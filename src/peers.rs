mod keep_alive;
mod receiver;

use crate::bisq::{
    constants::{self, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    payload::*,
};
use crate::connection::{Connection, ConnectionId, Request};
use crate::error::Error;
use actix::{
    fut::{self, ActorFuture},
    Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message, WeakAddr,
};
use core::time::Duration;
use keep_alive::{AddConnection, KeepAlive};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
use tokio::prelude::{
    future::{self, Future, Loop},
    stream::Stream,
};

const REQUEST_PERIODICALLY_INTERVAL_MIN: Duration = Duration::from_secs(10 * 60 * 60);

pub struct Peers {
    keep_alive: Addr<KeepAlive>,
    network: BaseCurrencyNetwork,
    connections: HashMap<ConnectionId, Addr<Connection>>,
    reported_peers: HashMap<NodeAddress, Peer>,
    identified_connections: HashMap<ConnectionId, NodeAddress>,
    local_addr: Option<NodeAddress>,
}

impl Peers {
    pub fn start(network: BaseCurrencyNetwork) -> Addr<Self> {
        Self {
            keep_alive: KeepAlive::start(),
            network,
            connections: HashMap::new(),
            reported_peers: HashMap::new(),
            identified_connections: HashMap::new(),
            local_addr: None,
        }
        .start()
    }

    fn add_connection(
        &mut self,
        id: ConnectionId,
        conn: Addr<Connection>,
        addr: Option<&NodeAddress>,
    ) {
        let downgraded = conn.downgrade();
        self.connections.insert(id, conn);
        if let Some(addr) = addr {
            self.identified_connections.insert(id, addr.to_owned());
        }
        Arbiter::spawn(
            self.keep_alive
                .send(AddConnection(id, downgraded))
                .then(|_| Ok(())),
        );
    }

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
        debug!("Adding reported peers {:?}", reported);
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

pub mod message {
    use super::receiver;
    use crate::bisq::{constants, payload::*};
    use crate::connection::{Connection, ConnectionId, Payload};
    use crate::dispatch::*;
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

    pub struct IncomingConnection(pub TcpStream);
    impl Message for IncomingConnection {
        type Result = ();
    }
    impl Handler<IncomingConnection> for super::Peers {
        type Result = ();

        fn handle(
            &mut self,
            IncomingConnection(tcp): IncomingConnection,
            ctx: &mut Self::Context,
        ) -> Self::Result {
            let dispatcher = ActorDispatcher::<Self, GetPeersRequest>::new(ctx.address());
            let (id, conn) = Connection::from_tcp_stream(tcp, self.network.into(), dispatcher);
            self.add_connection(id, conn, None);
        }
    }

    pub struct SeedConnection(pub NodeAddress, pub ConnectionId, pub Addr<Connection>);
    impl Message for SeedConnection {
        type Result = ();
    }
    impl Handler<SeedConnection> for super::Peers {
        type Result = ();
        fn handle(
            &mut self,
            SeedConnection(addr, id, connection): SeedConnection,
            ctx: &mut Self::Context,
        ) -> Self::Result {
            self.add_connection(id, connection, Some(&addr));
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

    impl Handler<Receive<GetPeersRequest>> for super::Peers {
        type Result = ();
        fn handle(
            &mut self,
            Receive(conn_id, mut request): Receive<GetPeersRequest>,
            _: &mut Self::Context,
        ) -> Self::Result {
            self.add_to_reported_peers(&mut request.reported_peers);
            if let Some(addr) = request.sender_node_address {
                self.reported_peers.insert(
                    addr.clone(),
                    Peer {
                        node_address: Some(addr.clone()),
                        supported_capabilities: request.supported_capabilities,
                        date: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_millis() as i64,
                    },
                );
                if let None = self.identified_connections.insert(conn_id, addr) {
                    debug!("Identified connection {:?}", conn_id);
                }
            }
            if let Some(addr) = self.connections.get(&conn_id) {
                let from = conn_id;
                let res = GetPeersResponse {
                    request_nonce: request.nonce,
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
                Arbiter::spawn(addr.send(Payload(res)).then(|_| Ok(())))
            }
        }
    }
}
