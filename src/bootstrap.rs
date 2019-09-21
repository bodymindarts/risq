use crate::bisq::{
    constants::{seed_nodes, BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    message::PreliminaryGetDataRequest,
};
use crate::connection::{Connection, ConnectionConfig};
use crate::error::Error;
use rand::{seq::SliceRandom, thread_rng, Rng};
use tokio::prelude::future::{self, Future};

pub struct Config {
    network: BaseCurrencyNetwork,
}
pub struct BootstrapResult {}

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
    );
    future::ok(BootstrapResult {})
}
