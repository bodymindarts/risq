mod keep_alive;
mod receiver;
mod sender;
use crate::bisq::{
    constants::{self, BaseCurrencyNetwork},
    payload::*,
};
use crate::connection::{Connection, ConnectionId, MessageStream};
use crate::error::Error;
use crate::listener::{Accept, Listener};
use actix::{Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message, WeakAddr};
use sender::{SendPayload, Sender};
use std::collections::{HashMap, HashSet};
use tokio::prelude::{
    future::{self, Future, Loop},
    stream::Stream,
};

pub struct Peers {
    network: BaseCurrencyNetwork,
    connections: HashMap<ConnectionId, Addr<Sender>>,
    reported_peers: HashMap<NodeAddress, Peer>,
    identified_connections: HashMap<ConnectionId, NodeAddress>,
}

impl Peers {
    pub fn start(network: BaseCurrencyNetwork) -> Addr<Self> {
        Self {
            network,
            connections: HashMap::new(),
            reported_peers: HashMap::new(),
            identified_connections: HashMap::new(),
        }
        .start()
    }
}

impl Actor for Peers {
    type Context = Context<Peers>;
}

struct ReportedPeersListener {
    pub peers: Addr<Peers>,
    pub from: ConnectionId,
}
impl Listener for ReportedPeersListener {
    fn get_peers_request(&mut self, msg: &GetPeersRequest) -> Accept {
        Arbiter::spawn(
            self.peers
                .send(message::PeersExchange {
                    request: msg.to_owned(),
                    from: self.from,
                })
                .then(|_| Ok(())),
        );
        Accept::Processed
    }
}

pub mod message {
    use super::{
        receiver,
        sender::{SendPayload, Sender},
    };
    use crate::bisq::{constants, payload::*};
    use crate::connection::{Connection, ConnectionConfig, ConnectionId};
    use actix::{Addr, Arbiter, AsyncContext, Context, Handler, Message, MessageResult, WeakAddr};
    use rand::{seq::SliceRandom, thread_rng};
    use std::{
        iter::{Extend, FromIterator},
        time::{SystemTime, UNIX_EPOCH},
    };
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
            self.reported_peers
                .extend(msg.request.reported_peers.drain(..).filter_map(|peer| {
                    let addr = peer.node_address.clone();
                    addr.map(|addr| (addr, peer))
                }));
            if let Some(addr) = msg.request.sender_node_address {
                self.reported_peers.insert(
                    addr.clone(),
                    Peer {
                        node_address: Some(addr.clone()),
                        supported_capabilities: msg.request.supported_capabilities,
                        date: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_millis() as i64,
                    },
                );
                if let None = self.identified_connections.insert(msg.from, addr) {
                    debug!("Identified connection {:?}", msg.from);
                }
            }
            if let Some(addr) = self.connections.get(&msg.from) {
                let from = msg.from;
                let res = GetPeersResponse {
                    request_nonce: msg.request.nonce,
                    reported_peers: Vec::from_iter(
                        self.identified_connections
                            .iter()
                            .filter_map(|(k, v)| {
                                if *k == from {
                                    None
                                } else {
                                    self.reported_peers.get(v)
                                }
                            })
                            .cloned(),
                    ),
                    supported_capabilities: constants::LOCAL_CAPABILITIES.clone(),
                };
                Arbiter::spawn(addr.send(SendPayload(res.into())).then(|_| Ok(())))
            }
        }
    }

    pub struct IncomingConnection(pub Connection);
    impl Message for IncomingConnection {
        type Result = ();
    }
    impl Handler<IncomingConnection> for super::Peers {
        type Result = ();

        fn handle(
            &mut self,
            connection: IncomingConnection,
            ctx: &mut Self::Context,
        ) -> Self::Result {
            let mut connection = connection.0;
            let message_stream = connection.take_message_stream();
            let id = connection.id;
            let sender = Sender::start(connection);
            receiver::listen(message_stream, sender.downgrade(), ctx.address());
            self.connections.insert(id, sender);
        }
    }

    pub struct OutgoingConnection(pub NodeAddress, pub Connection);
    impl Message for OutgoingConnection {
        type Result = ();
    }
    impl Handler<OutgoingConnection> for super::Peers {
        type Result = ();
        fn handle(
            &mut self,
            connection: OutgoingConnection,
            ctx: &mut Self::Context,
        ) -> Self::Result {
            let OutgoingConnection(addr, mut connection) = connection;
            let message_stream = connection.take_message_stream();
            let id = connection.id;
            let sender = Sender::start(connection);
            receiver::listen(message_stream, sender.downgrade(), ctx.address());
            self.connections.insert(id, sender);
            self.identified_connections.insert(id, addr.clone());
            self.reported_peers.insert(
                addr.clone(),
                Peer {
                    node_address: Some(addr),
                    date: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis() as i64,
                    supported_capabilities: Vec::new(),
                },
            );
        }
    }
}
