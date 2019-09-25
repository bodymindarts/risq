mod keep_alive;
mod receiver;
mod sender;
use crate::bisq::{constants, payload::*};
use crate::bootstrap::BootstrapResult;
use crate::connection::{Connection, ConnectionId, MessageStream};
use crate::error::Error;
use crate::listener::{Accept, Listener};
use actix::{Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message, WeakAddr};
use sender::{SendPayload, Sender};
use std::collections::HashMap;
use tokio::prelude::{
    future::{self, Future, Loop},
    stream::Stream,
};

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
impl Handler<Connection> for Peers {
    type Result = ();

    fn handle(&mut self, mut connection: Connection, ctx: &mut Self::Context) -> Self::Result {
        let message_stream = connection.take_message_stream();
        let id = connection.id.clone();
        let sender = Sender::start(connection);
        receiver::listen(message_stream, sender.downgrade(), ctx.address());
        self.connections.insert(id, sender);
    }
}

pub mod message {
    use super::sender::{SendPayload, Sender};
    use crate::bisq::{constants, payload::*};
    use crate::bootstrap::BootstrapResult;
    use crate::connection::ConnectionId;
    use actix::{Arbiter, Handler, Message, MessageResult, WeakAddr};
    use rand::{seq::SliceRandom, thread_rng};
    use tokio::prelude::future::Future;

    pub struct PeersExchange {
        pub request: GetPeersRequest,
        pub from: ConnectionId,
    }
    impl Message for PeersExchange {
        type Result = ();
    }
    impl Handler<PeersExchange> for super::Peers {
        type Result = ();
        fn handle(&mut self, mut msg: PeersExchange, _: &mut Self::Context) -> Self::Result {
            self.reported_peers.append(&mut msg.request.reported_peers);
            self.reported_peers.shuffle(&mut thread_rng());
            if self.reported_peers.len() > constants::MAX_REPORTED_PEERS {
                // Do something with removed reported peers
                let _ = self.reported_peers.drain(constants::MAX_REPORTED_PEERS..);
            }

            if let Some(addr) = self.connections.get(&msg.from) {
                let res = GetPeersResponse {
                    request_nonce: msg.request.nonce,
                    reported_peers: self.reported_peers.clone(),
                    supported_capabilities: constants::LOCAL_CAPABILITIES.clone(),
                };
                Arbiter::spawn(addr.send(SendPayload(res.into())).then(|_| Ok(())))
            }
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
