#[macro_use]
mod bisq;
mod bootstrap;
mod connection;
mod error;

use bisq::{
    constants::{BaseCurrencyNetwork, LOCAL_CAPABILITIES},
    message::*,
};
use connection::{Connection, ConnectionConfig};
use std::process;
use tokio::{
    self,
    prelude::{
        future::{ok, Future},
        stream::Stream,
    },
};

use env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;

macro_rules! debug_method {
    ($caml:ident, $snake:ident) => {
        fn $snake(&mut self, msg: $caml) -> () {
            debug!("Received message: {:?}", msg);
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
    let network = BaseCurrencyNetwork::BtcRegtest;
    let config = ConnectionConfig {
        message_version: network.into(),
    };
    let addr = "127.0.0.1:2002";
    let connection = Connection::new(addr, config);

    let msg = PreliminaryGetDataRequest {
        nonce: 0,
        excluded_keys: Vec::new(),
        supported_capabilities: LOCAL_CAPABILITIES.clone(),
    };

    let mut listener = DebugListener {};
    tokio::run(spawnable!(
        connection.and_then(|mut conn| {
            tokio::spawn(spawnable!(
                conn.take_message_stream().for_each(move |msg| {
                    listener.accept(msg);
                    ok(())
                }),
                "Error receiving: {:?}"
            ));
            conn.send(msg)
        }),
        "Error sending: {:?}"
    ));
    process::exit(0);
}
