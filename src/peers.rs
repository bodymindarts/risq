use crate::bisq::message::{NodeAddress, Peer};
use crate::bootstrap::BootstrapResult;
use crate::connection::Connection;
use actix::{Actor, Addr, Context, Handler, Message};
use tokio::net::TcpStream;

pub struct Peers {
    local_node_address: Option<NodeAddress>,
    reported_peers: Vec<Peer>,
    connections: Vec<Connection>,
}

impl Peers {
    pub fn start() -> Addr<Self> {
        Self {
            local_node_address: None,
            reported_peers: Vec::new(),
            connections: Vec::new(),
        }
        .start()
    }
}

impl Actor for Peers {
    type Context = Context<Peers>;
}

pub struct Bootstrapped {
    reported_peers: Vec<Peer>,
    seed_connections: Vec<Connection>,
}
impl Message for Connection {
    type Result = ();
}
impl Handler<Connection> for Peers {
    type Result = ();

    fn handle(&mut self, msg: Connection, ctx: &mut Self::Context) -> Self::Result {
        ()
    }
}

// accept incoming conemtion and respond to peer requetss
