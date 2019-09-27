mod keep_alive;
mod receiver;

use crate::bisq::{
    constants::{self, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    payload::*,
};
use crate::connection::{Connection, ConnectionId, Request};
use crate::error::Error;
use crate::listener::{Accept, Listener};
use actix::{
    fut::{self, ActorFuture},
    Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message, WeakAddr,
};
use core::time::Duration;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
use tokio::prelude::{
    future::{self, Future, Loop},
    stream::Stream,
};

pub struct DummyListener {}
impl Listener for DummyListener {}

const REQUEST_PERIODICALLY_INTERVAL_MIN: Duration = Duration::from_secs(10 * 60 * 60);

pub struct Peers {
    network: BaseCurrencyNetwork,
    connections: HashMap<ConnectionId, Addr<Connection>>,
    reported_peers: HashMap<NodeAddress, Peer>,
    identified_connections: HashMap<ConnectionId, NodeAddress>,
    local_addr: Option<NodeAddress>,
}

impl Peers {
    pub fn start(network: BaseCurrencyNetwork) -> Addr<Self> {
        Self {
            network,
            connections: HashMap::new(),
            reported_peers: HashMap::new(),
            identified_connections: HashMap::new(),
            local_addr: None,
        }
        .start()
    }
}
impl Peers {
    fn request_peers(&self, ctx: &mut <Self as Actor>::Context) {
        self.connections.iter().for_each(move |(id, conn)| {
            self.request_peers_from(*id, conn, ctx);
        })
    }
    fn request_peers_from(
        &self,
        id: ConnectionId,
        conn: &Addr<Connection>,
        ctx: &mut <Self as Actor>::Context,
    ) {
        let request = GetPeersRequest {
            sender_node_address: self.local_addr.clone(),
            nonce: gen_nonce(),
            supported_capabilities: LOCAL_CAPABILITIES.clone(),
            reported_peers: self.collect_reported_peers(&id),
        };
        ctx.spawn(
            fut::wrap_future(conn.send(Request(request)).flatten())
                .map(move |mut res, peers: &mut Peers, _ctx| {
                    if let Some(node) = peers.identified_connections.get(&id) {
                        if let Some((node, mut peer)) = peers.reported_peers.remove_entry(node) {
                            peer.supported_capabilities = res.supported_capabilities;
                            peers.reported_peers.insert(node, peer);
                        }
                    }
                    peers.add_to_reported_peers(&mut res.reported_peers)
                })
                .map_err(|_, _, _| ()),
        );
    }
    fn collect_reported_peers(&self, exclude: &ConnectionId) -> Vec<Peer> {
        Vec::from_iter(
            self.identified_connections
                .iter()
                .filter_map(|(k, v)| {
                    if *k == *exclude {
                        None
                    } else {
                        self.reported_peers.get(v)
                    }
                })
                .cloned(),
        )
    }
    fn add_to_reported_peers(&mut self, reported: &mut Vec<Peer>) {
        self.reported_peers
            .extend(reported.drain(..).filter_map(|peer| {
                let addr = peer.node_address.clone();
                addr.map(|addr| (addr, peer))
            }));
    }
}
impl Actor for Peers {
    type Context = Context<Peers>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(REQUEST_PERIODICALLY_INTERVAL_MIN, |peers, ctx| {
            peers.request_peers(ctx);
        });
    }
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
    use super::receiver;
    use crate::bisq::{constants, payload::*};
    use crate::connection::{Connection, ConnectionId};
    use actix::{Addr, Arbiter, AsyncContext, Context, Handler, Message, MessageResult, WeakAddr};
    use rand::{seq::SliceRandom, thread_rng};
    use std::{
        iter::{Extend, FromIterator},
        time::{SystemTime, UNIX_EPOCH},
    };
    use tokio::{net::TcpStream, prelude::future::Future};

    pub struct ServerStarted(pub NodeAddress);
    impl Message for ServerStarted {
        type Result = ();
    }
    impl Handler<ServerStarted> for super::Peers {
        type Result = ();
        fn handle(&mut self, msg: ServerStarted, _: &mut Self::Context) -> Self::Result {
            self.local_addr = Some(msg.0);
        }
    }

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
            self.add_to_reported_peers(&mut msg.request.reported_peers);
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
                // Arbiter::spawn(addr.send(SendPayload(res.into())).then(|_| Ok(())))
            }
        }
    }

    pub struct IncomingConnection(pub TcpStream);
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
            // let mut tcp = connection.0;
            // let message_stream = connection.take_message_stream();
            // let id = connection.id;
            // let sender = Sender::start(connection);
            // receiver::listen(message_stream, sender.downgrade(), ctx.address());
            // self.connections.insert(id, sender);
        }
    }

    pub struct SeedConnection(pub NodeAddress, pub ConnectionId, pub Addr<Connection>);
    impl Message for SeedConnection {
        type Result = ();
    }
    impl Handler<SeedConnection> for super::Peers {
        type Result = ();
        fn handle(&mut self, connection: SeedConnection, ctx: &mut Self::Context) -> Self::Result {
            let SeedConnection(addr, id, connection) = connection;
            self.connections.insert(id, connection);
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
            self.request_peers_from(id, self.connections.get(&id).unwrap(), ctx);
        }
    }
}
