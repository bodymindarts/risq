use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    payload::{
        gen_nonce, GetDataResponse, GetUpdatedDataRequest, NodeAddress, PreliminaryGetDataRequest,
    },
};
use crate::connection::{Connection, ConnectionId, Request};
use crate::dispatch::DummyDispatcher;
use crate::error::Error;
use crate::peers::{message::SeedConnection, Peers};
use crate::server::event::ServerStarted;
use actix::{
    fut::{self, ActorFuture},
    Actor, ActorContext, Addr, Arbiter, AsyncContext, Context, Handler,
};
use rand::{seq::SliceRandom, thread_rng};
use tokio::{prelude::future::Future, sync::oneshot};

pub struct Bootstrap {
    network: BaseCurrencyNetwork,
    proxy_port: Option<u16>,
    addr_notify: Option<oneshot::Sender<NodeAddress>>,
    addr_rec: Option<oneshot::Receiver<NodeAddress>>,
    seed_nodes: Vec<NodeAddress>,
    peers: Addr<Peers>,
}
impl Actor for Bootstrap {
    type Context = Context<Bootstrap>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = self.seed_nodes.pop().expect("No seed nodes defined");
        ctx.spawn(
            fut::wrap_future(bootstrap_from_seed(
                addr.clone(),
                self.addr_rec.take().expect("Receiver already removed"),
                self.network,
                self.proxy_port,
            ))
            .map_err(|_, _, _| ())
            .and_then(move |seed_result, bootstrap: &mut Bootstrap, _ctx| {
                fut::wrap_future(
                    bootstrap
                        .peers
                        .send(SeedConnection(
                            addr,
                            seed_result.connection_id,
                            seed_result.connection,
                        ))
                        .map_err(|_| ()),
                )
            })
            .map(|_, _, ctx| ctx.stop()),
        );
    }
}
impl Handler<ServerStarted> for Bootstrap {
    type Result = ();
    fn handle(&mut self, ServerStarted(local_addr): ServerStarted, _ctx: &mut Self::Context) {
        self.addr_notify
            .take()
            .expect("Local addr notifier already used")
            .send(local_addr)
            .map_err(|e| error!("ERR: {:?}", e))
            .expect("Couldn't send local address");
    }
}
impl Bootstrap {
    pub fn start(
        network: BaseCurrencyNetwork,
        peers: Addr<Peers>,
        proxy_port: Option<u16>,
    ) -> Addr<Bootstrap> {
        let mut seed_nodes = seed_nodes(&network);
        seed_nodes.shuffle(&mut thread_rng());
        let (addr_notify, addr_rec) = oneshot::channel();
        Self {
            network,
            addr_notify: Some(addr_notify),
            addr_rec: Some(addr_rec),
            proxy_port,
            seed_nodes,
            peers,
        }
        .start()
    }
}

struct SeedResult {
    preliminary_data_response: GetDataResponse,
    get_updated_data_response: GetDataResponse,
    connection: Addr<Connection>,
    connection_id: ConnectionId,
}

fn bootstrap_from_seed(
    seed_addr: NodeAddress,
    local_addr: oneshot::Receiver<NodeAddress>,
    network: BaseCurrencyNetwork,
    proxy_port: Option<u16>,
) -> impl Future<Item = SeedResult, Error = Error> {
    let preliminary_get_data_request = PreliminaryGetDataRequest {
        nonce: gen_nonce(),
        excluded_keys: Vec::new(),
        supported_capabilities: LOCAL_CAPABILITIES.clone(),
    };
    info!("Bootstrapping from seed: {:?}", seed_addr);
    Connection::open(seed_addr, network.into(), DummyDispatcher {}, proxy_port)
        .and_then(|(id, conn)| {
            debug!("Sending PreliminaryGetDataRequest to seed.");
            conn.send(Request(preliminary_get_data_request))
                .flatten()
                .map(move |response| (id, conn, response))
        })
        .and_then(|(id, conn, preliminary_data_response)| {
            local_addr
                .map(move |addr| {
                    (
                        GetUpdatedDataRequest {
                            sender_node_address: addr.into(),
                            nonce: gen_nonce(),
                            excluded_keys: Vec::new(),
                        },
                        id,
                        conn,
                        preliminary_data_response,
                    )
                })
                .map_err(|e| e.into())
        })
        .and_then(|(request, id, conn, preliminary_data_response)| {
            debug!("Sending GetUpdatedDataRequest to seed.");
            conn.send(Request(request))
                .flatten()
                .map(move |get_updated_data_response| SeedResult {
                    preliminary_data_response,
                    get_updated_data_response,
                    connection_id: id,
                    connection: conn,
                })
        })
}
