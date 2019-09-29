#[macro_use]
mod bisq;
mod bootstrap;
mod connection;
mod dispatch;
mod error;
mod peers;
mod server;
mod tor;

use actix::{Arbiter, System};
use bisq::constants::BaseCurrencyNetwork;
use bootstrap::Bootstrap;
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

    let network = BaseCurrencyNetwork::BtcMainnet;
    let sys = System::new("risq");
    let peers = Peers::start(network);
    let proxy_port = Some(9050);
    let bootstrap = Bootstrap::start(network, peers.clone(), proxy_port);
    let local_port = 5000;

    Arbiter::new().exec_fn(move || {
        server::start(
            local_port,
            peers,
            bootstrap,
            Some(TorConf {
                hidden_service_port: 8000,
                tc_port: 9051,
                private_key_path: dir,
            }),
        );
    });
    let _ = sys.run();
}
