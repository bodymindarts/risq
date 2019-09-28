use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    payload::{
        gen_nonce, GetDataResponse, GetUpdatedDataRequest, NodeAddress, PreliminaryGetDataRequest,
    },
};
use crate::connection::{Connection, ConnectionId, Request};
use crate::dispatch::DummyDispatcher;
use crate::error::Error;
use actix::Addr;
use rand::{seq::SliceRandom, thread_rng};
use tokio::prelude::future::Future;

pub struct Config {
    pub network: BaseCurrencyNetwork,
    pub local_node_address: NodeAddress,
}

pub struct BootstrapResult {
    pub seed_connections: Vec<(NodeAddress, ConnectionId, Addr<Connection>)>,
}

pub fn execute(config: Config) -> impl Future<Item = BootstrapResult, Error = Error> {
    let mut seed_nodes = seed_nodes(config.network);
    seed_nodes.shuffle(&mut thread_rng());
    let addr = seed_nodes.pop().expect("No seed nodes defined");
    bootstrap_from_seed(addr.clone(), config.local_node_address, config.network).map(
        |seed_result| BootstrapResult {
            seed_connections: vec![(addr, seed_result.connection_id, seed_result.connection)],
        },
    )
}

struct SeedResult {
    preliminary_data_response: GetDataResponse,
    get_updated_data_response: GetDataResponse,
    connection: Addr<Connection>,
    connection_id: ConnectionId,
}

fn bootstrap_from_seed(
    seed_addr: NodeAddress,
    local_addr: NodeAddress,
    network: BaseCurrencyNetwork,
) -> impl Future<Item = SeedResult, Error = Error> {
    let preliminary_get_data_request = PreliminaryGetDataRequest {
        nonce: gen_nonce(),
        excluded_keys: Vec::new(),
        supported_capabilities: LOCAL_CAPABILITIES.clone(),
    };
    let get_updated_data_request = GetUpdatedDataRequest {
        sender_node_address: local_addr.clone().into(),
        nonce: gen_nonce(),
        excluded_keys: Vec::new(),
    };
    info!("Bootstrapping from seed: {:?}", seed_addr);
    Connection::open(seed_addr, network.into(), DummyDispatcher {})
        .and_then(|(id, conn)| {
            debug!("Sending PreliminaryGetDataRequest to seed.");
            conn.send(Request(preliminary_get_data_request))
                .flatten()
                .map(move |response| (id, conn, response))
        })
        .and_then(|(id, conn, preliminary_data_response)| {
            debug!("Sending GetUpdatedDataRequest to seed.");
            conn.send(Request(get_updated_data_request)).flatten().map(
                move |get_updated_data_response| SeedResult {
                    preliminary_data_response,
                    get_updated_data_response,
                    connection_id: id,
                    connection: conn,
                },
            )
        })
}
