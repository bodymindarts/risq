use crate::bisq::{
    correlation::*,
    payload::{network_envelope, NetworkEnvelope, PayloadEncoder},
};
use crate::error;
use crate::listener::Listener;
use actix::{
    self,
    fut::{self, ActorFuture},
    io::{FramedWrite, WriteHandler},
    prelude::ActorContext,
    Actor, Addr, AsyncContext, Context, Handler, ResponseActFuture, StreamHandler,
};
use prost::Message;
use std::{
    collections::{HashMap, VecDeque},
    io,
    net::SocketAddr,
};
use tokio::{
    io::{AsyncRead, ReadHalf, WriteHalf},
    net::TcpStream,
    prelude::{
        future::{self, Future, IntoFuture, Loop},
        stream::Stream,
        Async,
    },
    sync::oneshot,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ConnectionId(Uuid);
impl ConnectionId {
    fn new() -> ConnectionId {
        ConnectionId(Uuid::new_v4())
    }
}

pub struct Connection {
    id: ConnectionId,
    writer: FramedWrite<WriteHalf<TcpStream>, PayloadEncoder>,
    listener: Box<dyn Listener>,
    response_channels: HashMap<CorrelationId, oneshot::Sender<network_envelope::Message>>,
}
impl Actor for Connection {
    type Context = Context<Connection>;
}
impl WriteHandler<error::Error> for Connection {}
impl StreamHandler<network_envelope::Message, error::Error> for Connection {
    fn handle(&mut self, msg: network_envelope::Message, _ctx: &mut Self::Context) {
        debug!("{:?} received message: {:?}", self.id, msg);
        if let Some(id) = msg.correlation_id() {
            if let Some(channel) = self.response_channels.remove(&id) {
                channel.send(msg).expect("Couldn't send response");
            }
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        debug!("{:?} incoming stream has closed", self.id);
        ctx.stop();
    }
}

impl Connection {
    pub fn open(
        addr: impl Into<SocketAddr>,
        encoder: PayloadEncoder,
        listener: Box<dyn Listener>,
    ) -> impl Future<Item = (ConnectionId, Addr<Connection>), Error = error::Error> {
        TcpStream::connect(&addr.into())
            .map(move |tcp| Connection::from_tcp_stream(tcp, encoder, listener))
            .map_err(|err| err.into())
    }
    pub fn from_tcp_stream(
        connection: TcpStream,
        encoder: PayloadEncoder,
        listener: Box<dyn Listener>,
    ) -> (ConnectionId, Addr<Connection>) {
        connection
            .set_nodelay(true)
            .expect("Unable to set nodelayx");
        let (reader, writer) = connection.split();
        let id = ConnectionId::new();
        (
            id,
            Connection::create(move |ctx| {
                ctx.add_stream(MessageStream::new(reader));
                let mut writer = FramedWrite::new(writer, encoder, ctx);
                writer.set_buffer_capacity(0, 0);
                Connection {
                    id,
                    writer,
                    listener,
                    response_channels: HashMap::new(),
                }
            }),
        )
    }
}

pub struct Request<M: Into<network_envelope::Message> + ResponseExtractor>(pub M);
impl<M> actix::Message for Request<M>
where
    M: Into<network_envelope::Message> + ResponseExtractor + 'static,
{
    type Result = Result<<M as ResponseExtractor>::Response, error::Error>;
}
impl<M> Handler<Request<M>> for Connection
where
    M: Into<network_envelope::Message> + ResponseExtractor + 'static,
{
    type Result = Box<dyn Future<Item = <M as ResponseExtractor>::Response, Error = error::Error>>;
    fn handle(&mut self, request: Request<M>, _: &mut Self::Context) -> Self::Result {
        let msg: network_envelope::Message = request.0.into();
        let correlation_id = msg
            .correlation_id()
            .expect("Request without correlation_id");
        let (send, receive) = oneshot::channel::<network_envelope::Message>();
        self.response_channels.insert(correlation_id.clone(), send);
        self.writer.write(msg);
        Box::new(
            receive
                .map(|response| <M as ResponseExtractor>::extract(response))
                .map_err(|e| e.into()),
        )
    }
}

enum MessageStreamState {
    MessageInProgress {
        size: usize,
        pos: usize,
        buf: Vec<u8>,
    },
    BetweenMessages,
    Empty,
}
struct MessageStream {
    reader: ReadHalf<TcpStream>,
    state: MessageStreamState,
    buffer: VecDeque<NetworkEnvelope>,
}
impl MessageStream {
    fn new(reader: ReadHalf<TcpStream>) -> MessageStream {
        MessageStream {
            reader,
            state: MessageStreamState::BetweenMessages,
            buffer: VecDeque::new(),
        }
    }
    fn next_from_buffer(&mut self) -> Option<network_envelope::Message> {
        let msg = self.buffer.pop_front()?.message;
        match msg {
            Some(network_envelope::Message::BundleOfEnvelopes(msg)) => {
                msg.envelopes
                    .into_iter()
                    .rev()
                    .for_each(|envelope| self.buffer.push_front(envelope));
                self.next_from_buffer()
            }
            None => self.next_from_buffer(),
            _ => msg,
        }
    }
}
impl Stream for MessageStream {
    type Item = network_envelope::Message;
    type Error = error::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        let next_read = match self.state {
            MessageStreamState::Empty => panic!("Stream is already finished"),
            MessageStreamState::BetweenMessages => {
                if let Some(msg) = self.next_from_buffer() {
                    return Ok(Async::Ready(Some(msg)));
                }
                let mut read_size = vec![0];
                let n = try_ready!(self.reader.poll_read(&mut read_size));
                if n == 0 {
                    self.state = MessageStreamState::Empty;
                    return Ok(Async::Ready(None));
                }
                let size = read_size[0].into();
                self.state = MessageStreamState::MessageInProgress {
                    size,
                    pos: 0,
                    buf: vec![0; size],
                };
                return self.poll();
            }
            MessageStreamState::MessageInProgress {
                ref mut size,
                ref mut pos,
                ref mut buf,
            } => {
                while *pos < *size {
                    let n = try_ready!(self.reader.poll_read(&mut buf[*pos..]));
                    *pos += n;
                    if n == 0 {
                        return Err(
                            io::Error::new(io::ErrorKind::UnexpectedEof, "early eof").into()
                        );
                    }
                }
                NetworkEnvelope::decode(&*buf)?
            }
        };
        self.buffer.push_back(next_read);
        self.state = MessageStreamState::BetweenMessages;
        self.poll()
    }
}
