use crate::bisq::message::Peer;
use crate::bootstrap::BootstrapResult;
use crate::connection::Connection;

pub struct Peers {
    reported_peers: Vec<Peer>,
    connections: Vec<Connection>,
}

impl Peers {
    pub fn start(bootstrap: BootstrapResult) -> Peers {
        Peers {
            reported_peers: bootstrap.reported_peers,
            connections: bootstrap.seed_connections,
        }
    }
}
