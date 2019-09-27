#[macro_use]
mod bisq;
mod alt_connection;
mod bootstrap;
mod connection;
mod error;
mod listener;
mod peers;
mod server;

use crate::error::Error;
use actix::{Arbiter, System};
use bisq::{constants::BaseCurrencyNetwork, payload::*};
use env_logger;
use listener::{Accept, Listener};
use peers::{message::SeedConnection, Peers};
use std::{error::Error as StdError, process};
use tokio::{
    self,
    prelude::{
        future::{self, ok, Future},
        stream::Stream,
    },
    sync::{mpsc, oneshot},
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
    let (start_send, start_rec) = oneshot::channel();
    let peers = Peers::start(network);
    let server = server::start(
        NodeAddress {
            host_name: "127.0.0.1".into(),
            port: 8000,
        },
        peers.clone(),
    );
    Arbiter::spawn(spawnable!(
        start_rec
            .map_err(|_| Error::ReceiveOneshotError)
            .and_then(|node_address| {
                bootstrap::execute(bootstrap::Config {
                    network: BaseCurrencyNetwork::BtcRegtest,
                    local_node_address: node_address,
                })
            })
            .and_then(
                move |result| future::join_all(result.seed_connections.into_iter().map(
                    move |(addr, id, conn)| {
                        peers
                            .send(SeedConnection(addr, id, conn))
                            .map_err(|e| e.into())
                    }
                ))
            ),
        "Error bootstrapping: {:?}"
    ));
    let _ = sys.run();
}
