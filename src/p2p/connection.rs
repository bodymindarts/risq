mod message_stream;

use super::dispatch::{Dispatch, Dispatcher, SendableDispatcher};
use crate::{
    bisq::{constants::CloseConnectionReason, correlation::*, payload::*},
    error,
    prelude::{
        future::Either,
        io::{flush, write_all},
        net::TcpStream,
        reactor::Handle,
        sync::{mpsc, oneshot},
        *,
    },
};
use message_stream::MessageStream;
use prost::{encoding::encoded_len_varint, Message};
use socks::Socks5Stream;
use std::{collections::HashMap, net::ToSocketAddrs, thread};
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
        if let Some(id) = Option::<CorrelationId>::from(&msg) {
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
        info!("{:?} closed.", self.id);
        ctx.stop();
    }
}

impl Connection {
    pub fn open<D: SendableDispatcher>(
        addr: NodeAddress,
        message_version: MessageVersion,
        dispatcher: D,
        proxy_port: Option<u16>,
    ) -> impl Future<Item = (ConnectionId, Addr<Connection>), Error = error::Error> {
        match proxy_port {
            None => Either::A(
                TcpStream::connect(
                    &(addr.host_name.as_str(), addr.port as u16)
                        .to_socket_addrs()
                        .unwrap()
                        .next()
                        .unwrap(),
                )
                .map(move |tcp| Connection::from_tcp_stream(tcp, message_version, dispatcher))
                .map_err(|err| err.into()),
            ),
            Some(proxy_port) => {
                let (send, receive) = oneshot::channel::<Result<Socks5Stream, error::Error>>();
                thread::spawn(move || {
                    send.send(
                        Socks5Stream::connect(
                            ("127.0.0.1", proxy_port),
                            (addr.host_name.as_str(), addr.port as u16),
                        )
                        .map_err(|e| e.into()),
                    )
                    .expect("Couldn't send Socks5Stream");
                });
                Either::B(
                    receive
                        .map_err(|e| error::Error::from(e))
                        .flatten()
                        .and_then(|stream| {
                            TcpStream::from_std(stream.into_inner(), &Handle::default())
                                .map_err(|e| e.into())
                        })
                        .map(move |tcp| {
                            Connection::from_tcp_stream(tcp, message_version, dispatcher)
                        }),
                )
            }
        }
    }
    pub fn from_tcp_stream<D: SendableDispatcher>(
        connection: TcpStream,
        message_version: MessageVersion,
        dispatcher: D,
    ) -> (ConnectionId, Addr<Connection>) {
        let (reader, writer) = connection.split();
        let (send, rec) = mpsc::channel(10);
        let id = ConnectionId::new();
        arbiter_spawn!(future::loop_fn((rec, writer), move |(rec, writer)| {
            rec.into_future()
                .map_err(|(e, _)| e.into())
                .and_then(|(msg, rec)| {
                    msg.ok_or(error::Error::ReceiveMPSCError)
                        .map(|msg| (msg, rec))
                })
                .and_then(move |(msg, rec)| {
                    debug!("Sending message {:?}", msg);
                    let envelope = NetworkEnvelope {
                        message_version: message_version.into(),
                        message: Some(msg),
                    };
                    let len = envelope.encoded_len();
                    let required = len + encoded_len_varint(len as u64);
                    let mut serialized = Vec::with_capacity(required);
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
        }));
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

pub struct SetDispatcher<D: SendableDispatcher>(pub D);
impl<D: SendableDispatcher> actix::Message for SetDispatcher<D> {
    type Result = ();
}
impl<D: SendableDispatcher> Handler<SetDispatcher<D>> for Connection {
    type Result = ();
    fn handle(&mut self, SetDispatcher(dispatcher): SetDispatcher<D>, _ctx: &mut Self::Context) {
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
    fn handle(&mut self, Payload(msg): Payload<M>, _ctx: &mut Self::Context) -> Self::Result {
        Box::new(
            self.writer
                .clone()
                .sink_from_err::<error::Error>()
                .send(msg.into())
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
        let correlation_id =
            Option::<CorrelationId>::from(&msg).expect("Request without correlation_id");
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
pub struct Shutdown(pub CloseConnectionReason);
impl actix::Message for Shutdown {
    type Result = ();
}
impl Handler<Shutdown> for Connection {
    type Result = ();
    fn handle(&mut self, Shutdown(reason): Shutdown, ctx: &mut Self::Context) {
        let reason: String = reason.into();
        info!("Shutting down {:?} because {}", self.id, reason);
        ctx.spawn(
            fut::wrap_future(
                self.writer
                    .clone()
                    .sink_from_err::<error::Error>()
                    .send(CloseConnectionMessage { reason: reason }.into())
                    .then(|_| Ok(())),
            )
            .then(|_: Result<(), ()>, _, ctx: &mut Self::Context| fut::ok(ctx.stop())),
        );
    }
}
