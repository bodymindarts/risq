mod convert;
mod data_router;

use crate::{
    api,
    bisq::{constants::BaseCurrencyNetwork, NodeAddress},
    domain::{offer::*, price_feed::PriceFeed, statistics::StatsCache},
    p2p::{
        dispatch::ActorDispatcher, server, status::Status, Bootstrap, Broadcaster, Peers, TorConfig,
    },
    prelude::*,
};
use data_router::*;
use std::{fs, path::PathBuf};

pub struct DaemonConfig {
    pub api_port: u16,
    pub server_port: u16,
    pub network: BaseCurrencyNetwork,
    pub force_seed: Option<NodeAddress>,
    pub risq_home: PathBuf,
    pub tor_control_port: Option<u16>,
    pub tor_proxy_port: Option<u16>,
    pub hidden_service_port: Option<u16>,
}

const SERIVCE_PRIVATE_KEY_PATH: &str = "tor/service.key";

pub fn run(
    DaemonConfig {
        api_port,
        server_port,
        network,
        force_seed,
        risq_home,
        tor_control_port,
        tor_proxy_port,
        hidden_service_port,
    }: DaemonConfig,
) {
    let private_key_path = risq_home.join(SERIVCE_PRIVATE_KEY_PATH);
    fs::create_dir_all(private_key_path.parent().unwrap()).expect("Couldn't create risq dir");
    let tor_config = match (tor_control_port, hidden_service_port) {
        (Some(tc_port), Some(hidden_service_port)) => Some(TorConfig {
            hidden_service_port,
            tc_port,
            private_key_path,
        }),
        _ => None,
    };

    let sys = System::new("risq");

    // Domain Thread
    let price_feed = PriceFeed::start(tor_proxy_port);
    let offer_book = OfferBook::start(price_feed);
    let stats_cache = StatsCache::new();

    let status = Status::new();
    // Api Thread
    let _ = api::listen(
        api_port,
        offer_book.clone(),
        status.clone(),
        stats_cache.as_ref().map(Clone::clone),
    );

    Arbiter::new().exec_fn(move || {
        // Daemon Thread
        let broadcaster = Broadcaster::start();
        let data_router = DataRouter::start(offer_book, broadcaster.clone(), stats_cache);
        let dispatcher = ActorDispatcher::<DataRouter, DataRouterDispatch>::new(data_router);

        Arbiter::new().exec_fn(move || {
            // P2P Thread
            let peers = Peers::start(
                network,
                broadcaster,
                status,
                dispatcher.clone(),
                tor_proxy_port,
            );
            let bootstrap = Bootstrap::start(
                network,
                peers.clone(),
                dispatcher,
                tor_proxy_port,
                force_seed,
            );
            server::start(server_port, peers, Some(bootstrap), tor_config);
        });
    });

    let _ = sys.run();
}
