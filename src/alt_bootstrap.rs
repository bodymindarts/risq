use crate::alt_connection::{Connection, SendAndAwait};
use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    payload::{
        gen_nonce, GetDataResponse, GetPeersRequest, GetPeersResponse, GetUpdatedDataRequest,
        NodeAddress, PayloadEncoder, Peer, PreliminaryGetDataRequest,
    },
};
use actix::Addr;
// use crate::connection::{Connection, ConnectionConfig};
use crate::error::Error;
use crate::listener::{Accept, Listener};
use rand::{seq::SliceRandom, thread_rng};
use std::net::SocketAddr;
use tokio::{
    prelude::future::{Future, IntoFuture},
    sync::oneshot,
};

pub struct Config {
    pub network: BaseCurrencyNetwork,
    pub local_node_address: NodeAddress,
}
pub struct DummyListener {}
impl Listener for DummyListener {}

pub struct BootstrapResult {
    pub seed_connections: Vec<(NodeAddress, Addr<Connection<DummyListener>>)>,
}
struct GetDataListener {
    expecting_nonce: i32,
    response: Option<GetDataResponse>,
}
impl Listener for GetDataListener {
    fn get_data_response(&mut self, response: &GetDataResponse) -> Accept {
        if response.request_nonce == self.expecting_nonce {
            self.response = Some(response.to_owned());
            Accept::Processed
        } else {
            Accept::Skipped
        }
    }
}

pub fn execute(config: Config) -> impl Future<Item = BootstrapResult, Error = Error> {
    let mut seed_nodes = seed_nodes(config.network);
    seed_nodes.shuffle(&mut thread_rng());
    let addr = seed_nodes.pop().expect("No seed nodes defined");
    bootstrap_from_seed(addr.clone(), config.local_node_address, config.network).map(
        |seed_result| BootstrapResult {
            seed_connections: vec![(addr, seed_result.connection)],
        },
    )
}

struct SeedResult {
    preliminary_data_response: GetDataResponse,
    get_updated_data_response: GetDataResponse,
    connection: Addr<Connection<DummyListener>>,
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
    Connection::open(seed_addr, PayloadEncoder::from(network), DummyListener {})
        .and_then(move |conn| {
            let listener = GetDataListener {
                expecting_nonce: preliminary_get_data_request.nonce,
                response: None,
            };
            debug!("Sending PreliminaryGetDataRequest to seed.");
            conn.send(SendAndAwait(preliminary_get_data_request.into(), listener))
                .flatten()
                .map(|listener| (conn, listener.response.expect("Response not set")))
        })
        .and_then(move |(conn, preliminary_data_response)| {
            let listener = GetDataListener {
                expecting_nonce: get_updated_data_request.nonce,
                response: None,
            };
            debug!("Sending GetUpdatedDataRequest to seed.");
            conn.send(SendAndAwait(get_updated_data_request.into(), listener))
                .flatten()
                .map(|listener| SeedResult {
                    preliminary_data_response,
                    get_updated_data_response: listener.response.expect("Response not set"),
                    connection: conn,
                })
        })
}
