use crate::bisq::message::Peer;
use crate::connection::Connection;

struct Peers {
    reported_peers: Vec<Peer>,
    connections: Vec<Connection>,
}
