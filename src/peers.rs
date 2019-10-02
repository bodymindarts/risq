mod keep_alive;

use crate::bisq::{
    constants::{BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    payload::*,
};
use crate::connection::{Connection, ConnectionId, Request};
use crate::dispatch::{self, ActorDispatcher, Dispatcher, SendableDispatcher};
use actix::{
    fut::{self, ActorFuture},
    Actor, Addr, Arbiter, AsyncContext, Context,
};
use core::time::Duration;
use keep_alive::{AddConnection, KeepAlive};
use std::{collections::HashMap, iter::FromIterator};
use tokio::prelude::future::Future;

const REQUEST_PERIODICALLY_INTERVAL_MIN: Duration = Duration::from_secs(10 * 60 * 60);

pub struct Peers<D: SendableDispatcher> {
    keep_alive: Addr<KeepAlive>,
    network: BaseCurrencyNetwork,
    connections: HashMap<ConnectionId, Addr<Connection>>,
    reported_peers: HashMap<NodeAddress, Peer>,
    identified_connections: HashMap<ConnectionId, NodeAddress>,
    local_addr: Option<NodeAddress>,
    dispatcher: D,
}

impl<D: SendableDispatcher> Peers<D> {
    pub fn start(network: BaseCurrencyNetwork, dispatcher: D) -> Addr<Self> {
        Self {
            keep_alive: KeepAlive::start(),
            network,
            connections: HashMap::new(),
            reported_peers: HashMap::new(),
            identified_connections: HashMap::new(),
            local_addr: None,
            dispatcher,
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
                .map(move |mut res, peers: &mut Peers<D>, _ctx| {
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
impl<D: SendableDispatcher> Actor for Peers<D> {
    type Context = Context<Peers<D>>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(REQUEST_PERIODICALLY_INTERVAL_MIN, |peers, ctx| {
            peers.request_peers(ctx);
        });
    }
}

pub mod message {
    use super::keep_alive::KeepAlive;
    use crate::bisq::{constants, payload::*};
    use crate::connection::{Connection, ConnectionId, Payload, SetDispatcher};
    use crate::dispatch::{Receive, SendableDispatcher};
    use crate::server::event::*;
    use actix::{Addr, Arbiter, AsyncContext, Handler, Message};
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

    impl<D: SendableDispatcher> Handler<Receive<GetPeersRequest>> for super::Peers<D> {
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
