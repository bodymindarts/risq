use crate::bisq::{
    correlation::*,
    payload::{network_envelope, MessageVersion, NetworkEnvelope},
};
use crate::dispatch::{Dispatch, Dispatcher};
use crate::error;
use actix::{
    self, prelude::ActorContext, Actor, Addr, Arbiter, AsyncContext, Context, Handler,
    StreamHandler,
};
use prost::Message;
use std::{
    collections::{HashMap, VecDeque},
    io,
    net::SocketAddr,
};
use tokio::{
    io::{flush, write_all, AsyncRead, ReadHalf},
    net::TcpStream,
    prelude::{
        future::{self, Future, Loop},
        stream::Stream,
        Async, Sink,
    },
    sync::{mpsc, oneshot},
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
    writer: mpsc::Sender<network_envelope::Message>,
    dispatcher: Box<dyn Dispatcher>,
    response_channels: HashMap<CorrelationId, oneshot::Sender<network_envelope::Message>>,
}
impl Actor for Connection {
    type Context = Context<Connection>;
}
impl StreamHandler<network_envelope::Message, error::Error> for Connection {
    fn handle(&mut self, msg: network_envelope::Message, _ctx: &mut Self::Context) {
        debug!("{:?} received: {:?}", self.id, msg);
        if let Some(id) = msg.correlation_id() {
            if let Some(channel) = self.response_channels.remove(&id) {
                channel.send(msg).expect("Couldn't send response");
                return;
            }
        }
        if let Dispatch::Retained(msg) = self.dispatcher.dispatch(self.id, msg) {
            warn!("{:?} retained message: {:?}", self.id, msg)
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        info!("{:?} closed", self.id);
        ctx.stop();
    }
}

impl Connection {
    pub fn open<D>(
        addr: impl Into<SocketAddr>,
        message_version: MessageVersion,
        dispatcher: D,
    ) -> impl Future<Item = (ConnectionId, Addr<Connection>), Error = error::Error>
    where
        D: Dispatcher + 'static,
    {
        TcpStream::connect(&addr.into())
            .map(move |tcp| Connection::from_tcp_stream(tcp, message_version, dispatcher))
            .map_err(|err| err.into())
    }
    pub fn from_tcp_stream<D>(
        connection: TcpStream,
        message_version: MessageVersion,
        dispatcher: D,
    ) -> (ConnectionId, Addr<Connection>)
    where
        D: Dispatcher + 'static,
    {
        let (reader, writer) = connection.split();
        let (send, rec) = mpsc::channel(10);
        let id = ConnectionId::new();
        Arbiter::spawn(
            future::loop_fn((rec, writer), move |(rec, writer)| {
                rec.into_future()
                    .map_err(|(e, _)| e.into())
                    .and_then(|(msg, rec)| {
                        msg.ok_or(error::Error::ReceiveMPSCError)
                            .map(|msg| (msg, rec))
                    })
                    .and_then(move |(msg, rec)| {
                        debug!("{:?} sending: {:?}", id, msg);
                        let envelope = NetworkEnvelope {
                            message_version: message_version.into(),
                            message: Some(msg),
                        };
                        let mut serialized = Vec::with_capacity(envelope.encoded_len() + 1);
                        envelope
                            .encode_length_delimited(&mut serialized)
                            .expect("Could not encode message");
                        write_all(writer, serialized)
                            .and_then(|(writer, _)| flush(writer))
                            .then(|writer| match writer {
                                Ok(writer) => Ok(Loop::Continue((rec, writer))),
                                Err(e) => Ok(Loop::Break(e)),
                            })
                    })
                    .map_err(|_| ())
            })
            .map(|_| ()),
        );
        (
            id,
            Connection::create(move |ctx| {
                ctx.add_stream(MessageStream::new(reader));
                Connection {
                    id,
                    writer: send,
                    dispatcher: Box::new(dispatcher),
                    response_channels: HashMap::new(),
                }
            }),
        )
    }
}

pub struct SetDispatcher<L: Dispatcher>(pub L);
impl<L: Dispatcher> actix::Message for SetDispatcher<L> {
    type Result = ();
}
impl<L: Dispatcher + 'static> Handler<SetDispatcher<L>> for Connection {
    type Result = ();
    fn handle(&mut self, SetDispatcher(dispatcher): SetDispatcher<L>, _ctx: &mut Self::Context) {
        self.dispatcher = Box::new(dispatcher);
    }
}

pub struct Payload<M: Into<network_envelope::Message>>(pub M);
impl<M> actix::Message for Payload<M>
where
    M: Into<network_envelope::Message>,
{
    type Result = Result<(), error::Error>;
}
impl<M> Handler<Payload<M>> for Connection
where
    M: Into<network_envelope::Message>,
{
    type Result = Box<dyn Future<Item = (), Error = error::Error>>;
    fn handle(&mut self, msg: Payload<M>, _ctx: &mut Self::Context) -> Self::Result {
        Box::new(
            self.writer
                .clone()
                .sink_from_err::<error::Error>()
                .send(msg.0.into())
                .map(|_| ())
                .map_err(|e| e.into()),
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
        Box::new(
            self.writer
                .clone()
                .sink_from_err::<error::Error>()
                .send(msg)
                .and_then(|_| {
                    receive
                        .map(|response| <M as ResponseExtractor>::extract(response))
                        .map_err(|e| e.into())
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
