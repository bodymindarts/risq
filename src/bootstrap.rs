use crate::bisq::constants::{seed_nodes, BaseCurrencyNetwork};
use crate::error::Error;
use rand::{seq::SliceRandom, thread_rng};
use tokio::prelude::future::{self, Future};

pub struct Config {
    network: BaseCurrencyNetwork,
}
pub struct BootstrapResult {}

pub fn execute(config: Config) -> impl Future<Item = BootstrapResult, Error = Error> {
    let mut seed_nodes = seed_nodes(config.network);
    seed_nodes.shuffle(&mut thread_rng());
    let addr = seed_nodes.pop().expect("No seed nodes defined");

    future::ok(BootstrapResult {})
}
