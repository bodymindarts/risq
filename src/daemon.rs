mod data_router;

use crate::{
    api,
    bisq::constants::BaseCurrencyNetwork,
    domain::offer_book::*,
    p2p::{dispatch::ActorDispatcher, server, Bootstrap, Peers, TorConfig},
};
use actix::{Arbiter, System};
use data_router::*;
use std::fs;

pub struct DaemonConfig {
    pub api_port: u16,
    pub server_port: u16,
    pub network: BaseCurrencyNetwork,
    pub tor_config: Option<TorConfig>,
    pub tor_proxy_port: Option<u16>,
}
pub fn run(
    DaemonConfig {
        api_port,
        server_port,
        network,
        tor_config,
        tor_proxy_port,
    }: DaemonConfig,
) {
    if let Some(tor_config) = tor_config.as_ref() {
        fs::create_dir_all(tor_config.private_key_path.parent().unwrap())
            .expect("Couldn't create risq dir");
    }

    let sys = System::new("risq");
    let offer_book = OfferBook::start();
    let data_router = DataRouter::start(offer_book.clone());
    let dispatcher = ActorDispatcher::<DataRouter, DataRouterDispatch>::new(data_router);

    Arbiter::new().exec_fn(move || {
        let peers = Peers::start(network, dispatcher.clone(), tor_proxy_port);
        let bootstrap = Bootstrap::start(network, peers.clone(), dispatcher, tor_proxy_port);
        server::start(server_port, peers, bootstrap, tor_config);
    });
    let _ = api::listen(api_port, offer_book);

    let _ = sys.run();
}
