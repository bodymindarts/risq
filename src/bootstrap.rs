use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    message::{GetDataResponse, Listener, PreliminaryGetDataRequest},
};
use crate::connection::{Connection, ConnectionConfig};
use crate::error::Error;
use rand::{seq::SliceRandom, thread_rng, Rng};
use tokio::prelude::{
    future::{self, Future, IntoFuture, Loop},
    stream::Stream,
};

pub struct Config {
    network: BaseCurrencyNetwork,
}
pub struct BootstrapResult {}
struct GetDataResponseListener {
    expecting_nonce: i32,
}
impl Listener<Option<GetDataResponse>> for GetDataResponseListener {
    fn get_data_response(&mut self, response: GetDataResponse) -> Option<GetDataResponse> {
        if response.request_nonce == self.expecting_nonce {
            Some(response)
        } else {
            None
        }
    }
}

pub fn execute(config: Config) -> impl Future<Item = BootstrapResult, Error = Error> {
    let preliminary_get_data_request = PreliminaryGetDataRequest {
        nonce: thread_rng().gen(),
        excluded_keys: Vec::new(),
        supported_capabilities: LOCAL_CAPABILITIES.clone(),
    };
    let mut seed_nodes = seed_nodes(config.network);
    seed_nodes.shuffle(&mut thread_rng());
    let addr = seed_nodes.pop().expect("No seed nodes defined");
    let conn = Connection::new(
        addr,
        ConnectionConfig {
            message_version: config.network.into(),
        },
    )
    .and_then(|conn| {
        let listener = GetDataResponseListener {
            expecting_nonce: preliminary_get_data_request.nonce,
        };
        conn.send_and_await(preliminary_get_data_request, listener)
    });
    future::ok(BootstrapResult {})
}
