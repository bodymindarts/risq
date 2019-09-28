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
    let _ = server::start(
        NodeAddress {
            host_name: "127.0.0.1".into(),
            port: 8000,
        },
        peers.clone(),
    );
    Arbiter::spawn(spawnable!(
        bootstrap::execute(bootstrap::Config {
            network: BaseCurrencyNetwork::BtcRegtest,
            local_node_address: NodeAddress {
                host_name: "127.0.0.1".into(),
                port: 8000,
            },
        })
        .and_then(move |result| future::join_all(
            result
                .seed_connections
                .into_iter()
                .map(move |(addr, id, conn)| {
                    peers
                        .send(SeedConnection(addr, id, conn))
                        .map_err(|e| e.into())
                })
        )),
        "Error bootstrapping: {:?}"
    ));
    let _ = sys.run();
}
