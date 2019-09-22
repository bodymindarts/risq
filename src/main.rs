#[macro_use]
mod bisq;
mod bootstrap;
mod connection;
mod error;
mod listener;
mod peers;

use bisq::{constants::BaseCurrencyNetwork, message::*};
use connection::ConnectionConfig;
use env_logger;
use listener::{Accept, Listener};
use std::process;
use tokio::{
    self,
    prelude::{
        future::{ok, Future},
        stream::Stream,
    },
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

fn main() -> () {
    env_logger::init();
    tokio::run(spawnable!(
        bootstrap::execute(bootstrap::Config {
            network: BaseCurrencyNetwork::BtcRegtest,
            local_node_address: NodeAddress {
                host_name: "localhost".into(),
                port: 8000
            }
        }),
        "Error bootstrapping: {:?}"
    ));
    process::exit(0);
}
