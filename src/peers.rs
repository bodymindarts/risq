use crate::bisq::message::Peer;
use crate::bootstrap::BootstrapResult;
use crate::connection::Connection;
use actix::{Actor, Addr, Context, Handler, Message};
use tokio::net::TcpStream;

pub struct Peers {
    reported_peers: Vec<Peer>,
    connections: Vec<Connection>,
}

impl Peers {
    pub fn start(bootstrap: BootstrapResult) -> Addr<Self> {
        Self {
            reported_peers: bootstrap.reported_peers,
            connections: bootstrap.seed_connections,
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
impl Message for Bootstrapped {
    type Result = ();
}
impl Handler<Bootstrapped> for Peers {
    type Result = ();

    fn handle(&mut self, msg: Bootstrapped, ctx: &mut Self::Context) -> Self::Result {
        ()
    }
}
