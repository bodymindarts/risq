mod keep_alive;

use super::{
    connection::{Connection, ConnectionId, Request},
    dispatch::{self, ActorDispatcher, SendableDispatcher},
};
use crate::bisq::{
    constants::{BaseCurrencyNetwork, Capability, LOCAL_CAPABILITIES},
    payload::*,
};
use actix::{
    fut::{self, ActorFuture, ActorStream, Either},
    Actor, Addr, Arbiter, AsyncContext, Context,
};
use keep_alive::*;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    iter::FromIterator,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::prelude::{
    future::Future,
    stream::{self, Stream},
};

const REQUEST_PERIODICALLY_INTERVAL_MIN: Duration = Duration::from_secs(10 * 60);
const MAX_CONNECTIONS: usize = 12;
const MIN_CONNECTIONS: usize = MAX_CONNECTIONS / 7 * 10;

pub struct PeerInfo {
    reported_alive_at: SystemTime,
    gossiped_capabilities: Option<Vec<Capability>>,
    reported_capabilities: Option<Vec<Capability>>,
}
impl From<(NodeAddress, &PeerInfo)> for Peer {
    fn from((addr, info): (NodeAddress, &PeerInfo)) -> Peer {
        Peer {
            node_address: Some(addr),
            date: info
                .reported_alive_at
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as i64,
            supported_capabilities: info
                .reported_capabilities
                .as_ref()
                .or(info.gossiped_capabilities.as_ref())
                .map(|v| v.iter().map(|c| *c as i32).collect())
                .unwrap_or(Vec::new()),
        }
    }
}

pub struct Peers<D: SendableDispatcher> {
    keep_alive: Addr<KeepAlive>,
    network: BaseCurrencyNetwork,
    connections: HashMap<ConnectionId, Addr<Connection>>,
    identified_connections: HashMap<ConnectionId, NodeAddress>,
    peer_infos: HashMap<NodeAddress, PeerInfo>,
    local_addr: Option<NodeAddress>,
    dispatcher: D,
    proxy_port: Option<u16>,
}

impl<D: SendableDispatcher> Peers<D> {
    pub fn start(
        network: BaseCurrencyNetwork,
        dispatcher: D,
        proxy_port: Option<u16>,
    ) -> Addr<Self> {
        Self {
            keep_alive: KeepAlive::start(),
            network,
            connections: HashMap::new(),
            identified_connections: HashMap::new(),
            peer_infos: HashMap::new(),
            local_addr: None,
            dispatcher,
            proxy_port,
        }
        .start()
    }

    fn get_dispatcher(&self, addr: Addr<Peers<D>>) -> impl SendableDispatcher {
        dispatch::chain(self.dispatcher.clone())
            .forward_to(ActorDispatcher::<KeepAlive, Ping>::new(
                self.keep_alive.clone(),
            ))
            .forward_to(ActorDispatcher::<Self, GetPeersRequest>::new(addr))
    }

    fn add_connection(
        &mut self,
        id: ConnectionId,
        conn: Addr<Connection>,
        addr: Option<NodeAddress>,
    ) {
        info!("Adding {:?} @ {:?}", id, addr);
        let downgraded = conn.downgrade();
        self.connections.insert(id, conn);
        if let Some(addr) = addr {
            self.update_peer_info(&addr, SystemTime::now(), None, None);
            self.identified_connections.insert(id, addr);
        }
        Arbiter::spawn(
            self.keep_alive
                .send(AddConnection(id, downgraded))
                .then(|_| Ok(())),
        );
    }

    fn update_peer_info(
        &mut self,
        addr: &NodeAddress,
        reported_alive_at: SystemTime,
        gossiped_capabilities: Option<Vec<i32>>,
        reported_capabilities: Option<Vec<i32>>,
    ) {
        let gossiped_capabilities = gossiped_capabilities
            .map(|c| c.into_iter().filter_map(|i| i.try_into().ok()).collect());
        let reported_capabilities = reported_capabilities
            .map(|c| c.into_iter().filter_map(|i| i.try_into().ok()).collect());
        if let Some(info) = self.peer_infos.get_mut(addr) {
            if reported_alive_at > info.reported_alive_at {
                info.reported_alive_at = reported_alive_at;
            }
            if gossiped_capabilities.is_some() {
                info.gossiped_capabilities = gossiped_capabilities
            }
            if reported_capabilities.is_some() {
                info.reported_capabilities = reported_capabilities
            }
        } else {
            self.peer_infos.insert(
                addr.clone(),
                PeerInfo {
                    reported_alive_at,
                    gossiped_capabilities: gossiped_capabilities,
                    reported_capabilities: reported_capabilities,
                },
            );
        }
    }

    fn consolidate_connections(&self, ctx: &mut <Self as Actor>::Context) {
        info!("Consolidating peer connections");
        ctx.spawn(self.update_alive_times().then(|_, peers, ctx| {
            if peers.connections.len() < MIN_CONNECTIONS {
                let candidates = peers.new_connection_candidates();
                ctx.spawn(
                    if candidates.len() + peers.connections.len() < MIN_CONNECTIONS * 2 {
                        Either::A(peers.request_peers())
                    } else {
                        Either::B(fut::ok(()))
                    }
                    .then(|_, peers, ctx| fut::ok(peers.do_consolidate_connections(ctx))),
                );
            }
            fut::ok(())
        }));
    }
    fn new_connection_candidates(&self) -> HashSet<&NodeAddress> {
        let mut candidates: HashSet<&NodeAddress> = self.peer_infos.keys().collect();
        self.identified_connections.values().for_each(|v| {
            candidates.remove(&v);
        });
        candidates
    }
    fn do_consolidate_connections(&self, ctx: &mut <Self as Actor>::Context) {
        if self.connections.len() < MIN_CONNECTIONS {
            self.new_connection_candidates()
                .into_iter()
                .take(MAX_CONNECTIONS - self.connections.len())
                .cloned()
                .for_each(|addr| {
                    ctx.spawn(
                        fut::wrap_future(
                            Connection::open(
                                addr.clone(),
                                self.network.into(),
                                self.get_dispatcher(ctx.address()),
                                self.proxy_port,
                            )
                            .map_err(|_| ()),
                        )
                        .map(|(id, conn), peers: &mut Self, ctx| {
                            peers.add_connection(id, conn, Some(addr));
                            ctx.spawn(peers.request_peers_from(id));
                        }),
                    );
                });
        }
    }

    fn request_peers(&self) -> impl ActorFuture<Item = (), Error = (), Actor = Self> {
        let ids: Vec<ConnectionId> = self.connections.keys().cloned().collect();
        fut::wrap_stream(stream::iter_ok::<_, ()>(ids.into_iter()))
            .and_then(|id, peers: &mut Self, _ctx| peers.request_peers_from(id))
            .finish()
    }

    fn request_peers_from(
        &self,
        id: ConnectionId,
    ) -> impl ActorFuture<Item = (), Error = (), Actor = Self> {
        match self.connections.get(&id) {
            Some(conn) => {
                let request = GetPeersRequest {
                    sender_node_address: self.local_addr.clone(),
                    nonce: gen_nonce(),
                    supported_capabilities: LOCAL_CAPABILITIES.clone(),
                    reported_peers: self.peers_to_report(&id),
                };
                Either::A(
                    fut::wrap_future(conn.send(Request(request)).flatten())
                        .map(
                            move |GetPeersResponse {
                                      reported_peers,
                                      supported_capabilities,
                                      ..
                                  },
                                  peers: &mut Peers<D>,
                                  _ctx| {
                                peers
                                    .identified_connections
                                    .get(&id)
                                    .map(NodeAddress::clone)
                                    .map(|addr| {
                                        peers.update_peer_info(
                                            &addr,
                                            SystemTime::now(),
                                            None,
                                            Some(supported_capabilities),
                                        )
                                    });
                                peers.add_to_peer_infos(reported_peers)
                            },
                        )
                        .then(|_, _, _| fut::ok(())),
                )
            }
            None => Either::B(fut::ok(())),
        }
    }

    fn peers_to_report(&self, exclude: &ConnectionId) -> Vec<Peer> {
        self.identified_connections
            .iter()
            .filter_map(|(id, addr)| {
                if *id == *exclude {
                    None
                } else {
                    self.peer_infos
                        .get(addr)
                        .map(|info| (addr.clone(), info).into())
                }
            })
            .collect()
    }

    fn add_to_peer_infos(&mut self, mut reported: Vec<Peer>) {
        reported.drain(..).for_each(
            |Peer {
                 node_address,
                 date,
                 supported_capabilities,
             }| {
                node_address.map(|addr| {
                    self.update_peer_info(
                        &addr,
                        UNIX_EPOCH + Duration::from_secs(date as u64),
                        Some(supported_capabilities),
                        None,
                    )
                });
            },
        )
    }

    fn update_alive_times(&self) -> impl ActorFuture<Item = (), Error = (), Actor = Self> {
        fut::wrap_future(self.keep_alive.send(ReportLastActive))
            .and_then(|alive_times, peers: &mut Self, _| {
                alive_times.into_iter().for_each(|(id, last_active)| {
                    peers
                        .identified_connections
                        .get(&id)
                        .map(NodeAddress::clone)
                        .map(|addr| peers.update_peer_info(&addr, last_active, None, None));
                });
                fut::ok(())
            })
            .map_err(|_, _, _| ())
    }
}
impl<D: SendableDispatcher> Actor for Peers<D> {
    type Context = Context<Peers<D>>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(REQUEST_PERIODICALLY_INTERVAL_MIN, |peers, ctx| {
            peers.consolidate_connections(ctx);
        });
    }
}

pub mod message {
    use crate::{
        bisq::{constants, payload::*},
        p2p::{
            connection::{Connection, ConnectionId, Payload, SetDispatcher},
            dispatch::{Receive, SendableDispatcher},
            server::event::*,
        },
    };
    use actix::{
        fut::{self, ActorFuture},
        Addr, Arbiter, AsyncContext, Handler, Message,
    };
    use std::{
        iter::FromIterator,
        time::{SystemTime, UNIX_EPOCH},
    };
    use tokio::prelude::future::Future;

    pub struct SeedConnection(pub NodeAddress, pub ConnectionId, pub Addr<Connection>);
    impl Message for SeedConnection {
        type Result = ();
    }
    impl<D: SendableDispatcher> Handler<SeedConnection> for super::Peers<D> {
        type Result = ();
        fn handle(
            &mut self,
            SeedConnection(addr, id, connection): SeedConnection,
            ctx: &mut Self::Context,
        ) -> Self::Result {
            Arbiter::spawn(
                connection
                    .clone()
                    .send(SetDispatcher(self.get_dispatcher(ctx.address())))
                    .then(|_| Ok(())),
            );
            self.add_connection(id, connection, Some(addr));
            self.consolidate_connections(ctx);
        }
    }

    impl<D: SendableDispatcher> Handler<Receive<GetPeersRequest>> for super::Peers<D> {
        type Result = ();
        fn handle(
            &mut self,
            Receive(
                conn_id,
                GetPeersRequest {
                    nonce,
                    sender_node_address,
                    supported_capabilities,
                    reported_peers,
                },
            ): Receive<GetPeersRequest>,
            ctx: &mut Self::Context,
        ) -> Self::Result {
            self.add_to_peer_infos(reported_peers);
            if let Some(addr) = sender_node_address {
                self.update_peer_info(&addr, SystemTime::now(), None, Some(supported_capabilities));
                self.identified_connections.insert(conn_id, addr);
            }
            if let Some(conn) = self.connections.get(&conn_id).map(Addr::clone) {
                ctx.spawn(self.update_alive_times().then(move |_, peers, _| {
                    let res = GetPeersResponse {
                        request_nonce: nonce,
                        reported_peers: peers.peers_to_report(&conn_id),
                        supported_capabilities: constants::LOCAL_CAPABILITIES.clone(),
                    };
                    fut::wrap_future(conn.send(Payload(res)).then(|_| Ok(())))
                }));
            }
        }
    }

    impl<D: SendableDispatcher> Handler<ServerStarted> for super::Peers<D> {
        type Result = ();
        fn handle(
            &mut self,
            ServerStarted(addr): ServerStarted,
            _: &mut Self::Context,
        ) -> Self::Result {
            self.local_addr = Some(addr);
        }
    }

    impl<D: SendableDispatcher> Handler<IncomingConnection> for super::Peers<D> {
        type Result = ();

        fn handle(
            &mut self,
            IncomingConnection(tcp): IncomingConnection,
            ctx: &mut Self::Context,
        ) -> Self::Result {
            let dispatcher = self.get_dispatcher(ctx.address());
            let (id, conn) = Connection::from_tcp_stream(tcp, self.network.into(), dispatcher);
            self.add_connection(id, conn, None);
        }
    }
}
