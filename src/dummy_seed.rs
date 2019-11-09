use crate::{
    bisq::{constants::BaseCurrencyNetwork, payload::*},
    p2p::{dispatch::*, server, Broadcaster, ConnectionId, Peers},
    prelude::*,
};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct DummyDispatcher;
impl Dispatcher for DummyDispatcher {
    fn dispatch(&self, _conn: ConnectionId, _msg: network_envelope::Message) -> Dispatch {
        Dispatch::Consumed
    }
}

pub fn run(server_port: u16, fixtures: Option<&Path>) {
    let sys = System::new("risq");
    let network = BaseCurrencyNetwork::BtcRegtest;

    let broadcaster = Broadcaster::start();
    let peers = Peers::start(network, broadcaster, DummyDispatcher, None);

    server::start(server_port, peers, None, None);

    let _ = sys.run();
}
