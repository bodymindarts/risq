#[macro_use]
mod bisq;
mod bootstrap;
mod connection;
mod data_router;
mod dispatch;
mod error;
mod peers;
mod server;
mod tor;

use actix::{Arbiter, System};
use bisq::constants::BaseCurrencyNetwork;
use bootstrap::Bootstrap;
use data_router::*;
use dispatch::ActorDispatcher;
use env_logger;
use peers::Peers;
use server::TorConf;
use std::{fs, path::PathBuf};

#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;

fn main() {
    env_logger::init();

    let mut dir = dirs::home_dir().expect("Couldn't determin home dir");
    dir.push(".risq/tor");
    fs::create_dir_all(&dir).expect("Couldn't create risq dir");
    dir.push("service.key");

    // Uncomment for mainnet
    //
    let network = BaseCurrencyNetwork::BtcMainnet;
    let tor_proxy_port = Some(9050);
    let tor_conf = Some(TorConf {
        hidden_service_port: 9999,
        tc_port: 9051,
        private_key_path: dir,
    });

    // Uncomment for regtest
    // let network = BaseCurrencyNetwork::BtcRegtest;
    // let tor_proxy_port = None;
    // let tor_conf = None;

    let local_port = 5000;

    let sys = System::new("risq");
    let data_router = DataRouter::start();
    let dispatcher = ActorDispatcher::<DataRouter, DataRouterDispatch>::new(data_router);
    let peers = Peers::start(network);
    let bootstrap = Bootstrap::start(network, peers.clone(), dispatcher, tor_proxy_port);

    Arbiter::new().exec_fn(move || {
        server::start(local_port, peers, bootstrap, tor_conf);
    });
    let _ = sys.run();
}
