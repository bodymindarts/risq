use crate::bisq::payload::{network_envelope, NetworkEnvelope, PayloadEncoder};
use crate::error;
use crate::listener::{Accept, Listener};
use actix::{
    self,
    fut::{self, ActorFuture, IntoActorFuture},
    io::{FramedWrite, WriteHandler},
    Actor, Addr, AsyncContext, Context, Handler, ResponseActFuture, StreamHandler,
};
use prost::Message;
use std::{collections::VecDeque, io, net::SocketAddr};
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
struct ConnectionId(Uuid);
impl ConnectionId {
    fn new() -> ConnectionId {
        ConnectionId(Uuid::new_v4())
    }
}

pub struct Connection<L: Listener> {
    id: ConnectionId,
    writer: FramedWrite<WriteHalf<TcpStream>, PayloadEncoder>,
    listener: L,
    bootstrap_response_channel: Option<oneshot::Sender<network_envelope::Message>>,
}
impl<L: Listener + 'static> Actor for Connection<L> {
    type Context = Context<Connection<L>>;
}
impl<L: Listener + 'static> WriteHandler<error::Error> for Connection<L> {}
impl<L: Listener + 'static> StreamHandler<network_envelope::Message, error::Error>
    for Connection<L>
{
    fn handle(&mut self, msg: network_envelope::Message, _ctx: &mut Self::Context) {
        debug!("{:?} received message: {:?}", self.id, msg);
        match (
            self.listener.accept(&msg),
            self.bootstrap_response_channel.is_some(),
        ) {
            (Accept::Skipped, true) => self.bootstrap_response_channel.take().unwrap().send(msg),
            _ => {
                warn!("Received {:?} skipped", msg);
                Ok(())
            }
        }
        .expect("Couldn't respont on response_channel");
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        debug!("{:?} incoming stream has closed", self.id);
    }
}

impl<L: Listener + 'static> Connection<L> {
    pub fn open(
        addr: impl Into<SocketAddr>,
        encoder: PayloadEncoder,
        listener: L,
    ) -> impl Future<Item = Addr<Connection<L>>, Error = error::Error> {
        TcpStream::connect(&addr.into())
            .map(move |tcp| Connection::from_tcp_stream(tcp, encoder, listener))
            .map_err(|err| err.into())
    }
    pub fn from_tcp_stream(
        connection: TcpStream,
        encoder: PayloadEncoder,
        listener: L,
    ) -> Addr<Connection<L>> {
        connection
            .set_nodelay(true)
            .expect("Unable to set nodelayx");
        let (reader, writer) = connection.split();
        Connection::create(move |ctx| {
            ctx.add_stream(MessageStream::new(reader));
            Connection {
                id: ConnectionId::new(),
                writer: FramedWrite::new(writer, encoder, ctx),
                listener,
                bootstrap_response_channel: None,
            }
        })
    }
}

pub struct SendAndAwait<L: Listener>(pub network_envelope::Message, pub L);
impl<L: Listener + 'static> actix::Message for SendAndAwait<L> {
    type Result = Result<L, error::Error>;
}

impl<S: Listener + 'static, L: Listener + Send + 'static> Handler<SendAndAwait<L>>
    for Connection<S>
{
    type Result = ResponseActFuture<Connection<S>, L, error::Error>;
    fn handle(&mut self, msg: SendAndAwait<L>, _: &mut Self::Context) -> Self::Result {
        let SendAndAwait(msg, mut listener) = msg;
        let (send, receive) = oneshot::channel::<network_envelope::Message>();
        self.bootstrap_response_channel = Some(send);
        self.writer.write(msg);
        Box::new(
            fut::wrap_future(
                receive
                    .and_then(move |response| {
                        listener.accept(&response);
                        Ok(listener)
                    })
                    .map_err(|e| e.into()),
            )
            .then(|res, conn: &mut Connection<S>, _ctx| {
                conn.bootstrap_response_channel = None;
                fut::result(res)
            }),
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
