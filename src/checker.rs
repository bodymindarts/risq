use crate::{
    bisq::payload::*,
    p2p::{dispatch::*, Connection, ConnectionId},
    prelude::*,
};

struct DummyDispatcher;
impl Dispatcher for DummyDispatcher {
    fn dispatch(&self, conn: ConnectionId, msg: network_envelope::Message) -> Dispatch {
        Dispatch::Consumed
    }
}

pub fn check_node(addr: NodeAddress, proxy_port: u16) {
    let sys = System::new("risq");
    // let conn = Connection::open(
}
