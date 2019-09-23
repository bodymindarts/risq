#[macro_use]
mod bisq;
mod bootstrap;
mod connection;
mod error;
mod listener;
mod peers;
mod server;

use crate::error::Error;
use actix::{Arbiter, System};
use bisq::{constants::BaseCurrencyNetwork, message::*};
use env_logger;
use listener::{Accept, Listener};
use peers::Peers;
use std::{error::Error as StdError, process};
use tokio::{
    self,
    prelude::{
        future::{ok, Future},
        stream::Stream,
    },
    sync::{mpsc, oneshot},
};

#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;

macro_rules! debug_method {
    ($caml:ident, $snake:ident) => {
        fn $snake(self, msg: $caml) -> Accept<Self> {
            debug!("Received message: {:?}", msg);
            Accept::Consumed(self)
        }
    };
}
struct DebugListener {}
impl Listener for DebugListener {
    for_all_messages!(debug_method);
}

macro_rules! spawnable {
    ($ex:expr, $f:tt) => {
        $ex.map(|_| ()).map_err(|e| {
            debug!($f, e);
        })
    };
}

fn main() {
    env_logger::init();
    let sys = System::new("risq");
    let (start_send, start_rec) = oneshot::channel();
    let peers_addr = Peers::start();
    Arbiter::spawn(spawnable!(
        server::start(
            NodeAddress {
                host_name: "127.0.0.1".into(),
                port: 8000
            },
            BaseCurrencyNetwork::BtcRegtest.into(),
            start_send,
            peers_addr
        ),
        "Server error {:?}"
    ));
    Arbiter::spawn(spawnable!(
        start_rec
            .map_err(|_| Error::ReceiveOneshotError)
            .and_then(|node_address| {
                bootstrap::execute(bootstrap::Config {
                    network: BaseCurrencyNetwork::BtcRegtest,
                    local_node_address: node_address,
                })
            }),
        "Error bootstrapping: {:?}"
    ));
    let _ = sys.run();
}
