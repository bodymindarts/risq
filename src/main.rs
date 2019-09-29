#[macro_use]
mod bisq;
mod bootstrap;
mod connection;
mod dispatch;
mod error;
mod peers;
mod server;

use actix::{Arbiter, System};
use bisq::{constants::BaseCurrencyNetwork, payload::*};
use bootstrap::Bootstrap;
use env_logger;
use peers::{message::SeedConnection, Peers};
use tokio::{
    self,
    prelude::future::{self, Future},
};

#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;

macro_rules! spawnable {
    ($ex:expr, $f:tt) => {
        $ex.map(|_| ()).map_err(|e| {
            debug!($f, e);
        })
    };
}

fn main() {
    env_logger::init();
    let network = BaseCurrencyNetwork::BtcRegtest.into();
    let sys = System::new("risq");
    let peers = Peers::start(network);
    let bootstrap = Bootstrap::start(network, peers.clone());
    let listen_addr = NodeAddress {
        host_name: "127.0.0.1".into(),
        port: 8000,
    };
    let _ = server::start(listen_addr, peers, bootstrap);
    debug!("Bootstrap complete");
    let _ = sys.run();
}
