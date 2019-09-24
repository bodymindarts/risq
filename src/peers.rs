mod sender;
use crate::bisq::{constants, payload::*};
use crate::bootstrap::BootstrapResult;
use crate::connection::{Connection, ConnectionId};
use crate::error::Error;
use crate::listener::{Accept, Listener};
use actix::{Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message};
use sender::Sender;
use std::collections::HashMap;
use tokio::prelude::{
    future::{self, Future, Loop},
    stream::Stream,
};

macro_rules! spawnable {
    ($ex:expr, $f:tt) => {
        $ex.map(|_| ()).map_err(|e| {
            debug!($f, e);
        })
    };
}

pub struct Peers {
    connections: HashMap<ConnectionId, Addr<Sender>>,
    reported_peers: Vec<Peer>,
    known_connections: HashMap<NodeAddress, ConnectionId>,
}

impl Peers {
    pub fn start() -> Addr<Self> {
        Self {
            connections: HashMap::new(),
            reported_peers: Vec::new(),
            known_connections: HashMap::new(),
        }
        .start()
    }
}

impl Actor for Peers {
    type Context = Context<Peers>;
}

impl Message for Connection {
    type Result = ();
}
struct PeersRequestListener {
    peers: Addr<Peers>,
    from: ConnectionId,
}
impl Listener for PeersRequestListener {
    fn get_peers_request(&mut self, msg: &GetPeersRequest) -> Accept {
        let request_nonce = msg.nonce.to_owned();
        let peers = self.peers.to_owned();
        let from = self.from.to_owned();
        Arbiter::spawn(spawnable!(
            self.peers
                .send(message::GetReportedPeers {})
                .and_then(move |reported_peers| {
                    let res = GetPeersResponse {
                        request_nonce,
                        reported_peers,
                        supported_capabilities: constants::LOCAL_CAPABILITIES.clone(),
                    };
                    peers.send(message::SendPayloadTo(from, res.into()))
                }),
            "Error responding to get_peers_request {:?}"
        ));
        Accept::Processed
    }
}
impl Handler<Connection> for Peers {
    type Result = ();

    fn handle(&mut self, mut connection: Connection, ctx: &mut Self::Context) -> Self::Result {
        let message_stream = connection.take_message_stream();
        let listener = PeersRequestListener {
            peers: ctx.address(),
            from: connection.id,
        };
        let id = connection.id.clone();
        let sender = Sender::start(connection);
        self.connections.insert(id, sender);
        Arbiter::spawn(
            future::loop_fn((listener, message_stream), |(mut listener, stream)| {
                stream
                    .into_future()
                    .map_err(|(e, _)| e)
                    .and_then(|(msg, stream)| {
                        listener
                            .accept_or_err(&msg, Error::ConnectionClosed)
                            .map(|accepted| match accepted {
                                Accept::Processed => Loop::Continue((listener, stream)),
                                Accept::Skipped => {
                                    warn!("Incoming listener skipped message: {:?}", msg);
                                    Loop::Continue((listener, stream))
                                }
                            })
                    })
            })
            .map_err(|e| info!("Connection closed: {:?}", e)),
        )
    }
}

pub mod message {
    use super::sender::{SendPayload, Sender};
    use crate::bisq::payload::*;
    use crate::bootstrap::BootstrapResult;
    use crate::connection::ConnectionId;
    use actix::{Arbiter, Handler, Message, MessageResult};
    use tokio::prelude::future::Future;

    pub struct SendPayloadTo(pub ConnectionId, pub network_envelope::Message);
    impl Message for SendPayloadTo {
        type Result = ();
    }
    impl Handler<SendPayloadTo> for super::Peers {
        type Result = ();
        fn handle(&mut self, msg: SendPayloadTo, _: &mut Self::Context) -> Self::Result {
            if let Some(sender) = self.connections.get(&msg.0) {
                Arbiter::spawn(sender.send(SendPayload(msg.1)).then(|_| Ok(())));
            }
        }
    }

    pub struct GetReportedPeers {}
    impl Message for GetReportedPeers {
        type Result = Vec<Peer>;
    }
    impl Handler<GetReportedPeers> for super::Peers {
        type Result = MessageResult<GetReportedPeers>;
        fn handle(&mut self, mut _msg: GetReportedPeers, _: &mut Self::Context) -> Self::Result {
            MessageResult(self.reported_peers.clone())
        }
    }

    impl Message for BootstrapResult {
        type Result = ();
    }
    impl Handler<BootstrapResult> for super::Peers {
        type Result = ();
        fn handle(&mut self, msg: BootstrapResult, _: &mut Self::Context) -> Self::Result {
            debug!("Inserting connections from bootstrap");
            msg.seed_connections.into_iter().for_each(|(addr, conn)| {
                let id = conn.id;
                self.connections.insert(id, Sender::start(conn));
                self.known_connections.insert(addr, id);
            })
        }
    }
}
