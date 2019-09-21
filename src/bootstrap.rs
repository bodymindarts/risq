use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    message::{GetDataResponse, GetUpdatedDataRequest, NodeAddress, PreliminaryGetDataRequest},
};
use crate::connection::{Connection, ConnectionConfig};
use crate::error::Error;
use crate::listener::{Accept, Listener};
use rand::{seq::SliceRandom, thread_rng, Rng};
use tokio::prelude::{
    future::{self, Future, IntoFuture},
    stream::Stream,
};

pub struct Config {
    pub network: BaseCurrencyNetwork,
    pub local_node_address: NodeAddress,
}
pub struct BootstrapResult {}
struct GetDataResponseListener {
    expecting_nonce: i32,
    response: Option<GetDataResponse>,
}
impl Listener for GetDataResponseListener {
    fn get_data_response(self, response: GetDataResponse) -> Accept<Self> {
        if response.request_nonce == self.expecting_nonce {
            Accept::Consumed(GetDataResponseListener {
                expecting_nonce: self.expecting_nonce,
                response: Some(response),
            })
        } else {
            Accept::Skipped(response.into(), self)
        }
    }
}

pub fn execute(config: Config) -> impl Future<Item = BootstrapResult, Error = Error> {
    let preliminary_get_data_request = PreliminaryGetDataRequest {
        nonce: thread_rng().gen(),
        excluded_keys: Vec::new(),
        supported_capabilities: LOCAL_CAPABILITIES.clone(),
    };
    let get_updated_data_request = GetUpdatedDataRequest {
        sender_node_address: config.local_node_address.into(),
        nonce: thread_rng().gen(),
        excluded_keys: Vec::new(),
    };
    let mut seed_nodes = seed_nodes(config.network);
    seed_nodes.shuffle(&mut thread_rng());
    let addr = seed_nodes.pop().expect("No seed nodes defined");
    Connection::new(
        addr.clone(),
        ConnectionConfig {
            message_version: config.network.into(),
        },
    )
    .and_then(move |conn| {
        let listener = GetDataResponseListener {
            expecting_nonce: preliminary_get_data_request.nonce,
            response: None,
        };
        info!("Exchanging PreliminaryGetDataRequest with seed: {:?}", addr);
        conn.send_and_await(preliminary_get_data_request, listener)
            .and_then(move |(_listener, conn)| {
                let listener = GetDataResponseListener {
                    expecting_nonce: get_updated_data_request.nonce,
                    response: None,
                };
                info!("Exchanging GetUpdatedDataRequest with seed: {:?}", addr);
                conn.send_and_await(get_updated_data_request, listener)
                    .and_then(|(_listener, conn)| Ok(conn))
            })
    })
    .map(|_| BootstrapResult {})
}
