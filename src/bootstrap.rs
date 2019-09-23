use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    message::{
        gen_nonce, GetDataResponse, GetPeersRequest, GetPeersResponse, GetUpdatedDataRequest,
        NodeAddress, Peer, PreliminaryGetDataRequest,
    },
};
use crate::connection::IdentifiedConnection;
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
pub struct BootstrapResult {
    pub reported_peers: Vec<Peer>,
    pub seed_connections: Vec<IdentifiedConnection>,
}
struct GetDataListener {
    expecting_nonce: i32,
    response: Option<GetDataResponse>,
}
impl Listener for GetDataListener {
    fn get_data_response(mut self, response: GetDataResponse) -> Accept<Self> {
        if response.request_nonce == self.expecting_nonce {
            self.response = Some(response);
            Accept::Consumed(self)
        } else {
            Accept::Skipped(response.into(), self)
        }
    }
}
struct GetPeersListener {
    expecting_nonce: i32,
    response: Option<GetPeersResponse>,
}
impl Listener for GetPeersListener {
    fn get_peers_response(mut self, response: GetPeersResponse) -> Accept<Self> {
        if response.request_nonce == self.expecting_nonce {
            self.response = Some(response);
            Accept::Consumed(self)
        } else {
            Accept::Skipped(response.into(), self)
        }
    }
}

pub fn execute(config: Config) -> impl Future<Item = BootstrapResult, Error = Error> {
    let mut seed_nodes = seed_nodes(config.network);
    seed_nodes.shuffle(&mut thread_rng());
    let addr = seed_nodes.pop().expect("No seed nodes defined");
    bootstrap_from_seed(addr, config.local_node_address, config.network).map(|seed_result| {
        BootstrapResult {
            reported_peers: seed_result.reported_peers,
            seed_connections: vec![seed_result.connection],
        }
    })
}

struct SeedResult {
    preliminary_data_response: GetDataResponse,
    get_updated_data_response: GetDataResponse,
    reported_peers: Vec<Peer>,
    connection: IdentifiedConnection,
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
    IdentifiedConnection::new(seed_addr.clone(), network.into())
        .and_then(move |conn| {
            let listener = GetDataListener {
                expecting_nonce: preliminary_get_data_request.nonce,
                response: None,
            };
            debug!("Sending PreliminaryGetDataRequest to seed.");
            conn.send_and_await(preliminary_get_data_request, listener)
                .map(|(listener, conn)| (listener.response.expect("Response not set"), conn))
        })
        .and_then(move |(preliminary_data_response, conn)| {
            let listener = GetDataListener {
                expecting_nonce: get_updated_data_request.nonce,
                response: None,
            };
            debug!("Sending GetUpdatedDataRequest to seed.");
            conn.send_and_await(get_updated_data_request, listener)
                .map(|(listener, conn)| {
                    (
                        preliminary_data_response,
                        listener.response.expect("Response not set"),
                        conn,
                    )
                })
        })
        .and_then(
            move |(preliminary_data_response, get_updated_data_response, conn)| {
                let get_peers_request = GetPeersRequest {
                    sender_node_address: local_addr.into(),
                    nonce: gen_nonce(),
                    supported_capabilities: LOCAL_CAPABILITIES.clone(),
                    reported_peers: Vec::new(),
                };
                let listener = GetPeersListener {
                    expecting_nonce: get_peers_request.nonce,
                    response: None,
                };
                debug!("Exchanging GetUpdatedDataRequest with seed");
                conn.send_and_await(get_peers_request, listener)
                    .map(|(listener, connection)| SeedResult {
                        preliminary_data_response,
                        get_updated_data_response,
                        connection,
                        reported_peers: listener.response.expect("Response not set").reported_peers,
                    })
            },
        )
}
