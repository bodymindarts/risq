use crate::bisq::{constants, payload::*};
use crate::bootstrap::BootstrapResult;
use crate::connection::{Connection, ConnectionId};
use crate::error::Error;
use crate::listener::{Accept, Listener};
use actix::{Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message, MessageResult};
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
    connections: HashMap<ConnectionId, Connection>,
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
    fn get_peers_request(self, msg: GetPeersRequest) -> Accept<Self> {
        let PeersRequestListener { peers, from } = self;
        let peers_clone = peers.clone();
        Arbiter::spawn(spawnable!(
            peers
                .send(message::GetReportedPeers {})
                .and_then(move |reported_peers| {
                    let res = GetPeersResponse {
                        request_nonce: msg.nonce,
                        reported_peers,
                        supported_capabilities: constants::LOCAL_CAPABILITIES.clone(),
                    };
                    peers_clone.send(message::SendPayloadTo(from, res.into()))
                }),
            "Error responding to get_peers_request {:?}"
        ));
        Accept::Consumed(PeersRequestListener { peers, from })
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
        self.connections.insert(connection.id, connection);
        Arbiter::spawn(
            future::loop_fn((listener, message_stream), |(listener, stream)| {
                stream
                    .into_future()
                    .map_err(|(e, _)| e)
                    .and_then(|(msg, stream)| {
                        listener
                            .accept_or_err(msg, Error::ConnectionClosed)
                            .map(|accepted| match accepted {
                                Accept::Consumed(listener) => Loop::Continue((listener, stream)),
                                Accept::Skipped(msg, listener) => {
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
    use crate::bisq::payload::*;
    use crate::bootstrap::BootstrapResult;
    use crate::connection::ConnectionId;
    use actix::{Handler, Message, MessageResult};

    pub struct SendPayloadTo(pub ConnectionId, pub network_envelope::Message);
    impl Message for SendPayloadTo {
        type Result = ();
    }
    impl Handler<SendPayloadTo> for super::Peers {
        type Result = ();
        fn handle(&mut self, msg: SendPayloadTo, _: &mut Self::Context) -> Self::Result {
            if let Some(ref mut con) = self.connections.get_mut(&msg.0) {
                con.send_sync(msg.1);
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
                self.connections.insert(id, conn);
                self.known_connections.insert(addr, id);
            })
        }
    }
}
