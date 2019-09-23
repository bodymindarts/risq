use crate::bisq::payload::{NodeAddress, Peer};
use crate::bootstrap::BootstrapResult;
use crate::connection::Connection;
use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;
use tokio::net::TcpStream;
use uuid::Uuid;

pub struct Peers {
    connections: HashMap<Uuid, Connection>,
}

impl Peers {
    pub fn start() -> Addr<Self> {
        Self {
            connections: HashMap::new(),
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
struct PeersRequestListener {
    peers: Addr<Peers>,
}
// impl Listener for PeersRequestListener {
//     fn get_peers_request(self, msg: GetPeersRequest){
//         Arbiter::spawn(self.peers.send(
//     }
// }
impl Handler<Connection> for Peers {
    type Result = ();

    fn handle(&mut self, mut connection: Connection, ctx: &mut Self::Context) -> Self::Result {
        let message_stream = connection.take_message_stream();
        self.connections.insert(connection.uuid, connection);
        // future::loop_fn(message_stream,|stream|
        // stream.into_future()
        //             .map_err(|(e, _)| e)
        //             .and_then(|(msg,stream)|
        //                 match msg {
        //                     Some(msg) =>
        //                 }
        //             )
        // )
    }
}

// accept incoming conemtion and respond to peer requetss
