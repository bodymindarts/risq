use crate::bisq::payload::*;
use crate::connection::{Connection, ConnectionId};
use crate::listener::{Accept, Listener};
use actix::{Actor, Addr, Context, Handler, Message, MessageResult};
use std::collections::HashMap;

pub struct Peers {
    connections: HashMap<ConnectionId, Connection>,
    reported_peers: Vec<Peer>,
}

impl Peers {
    pub fn start() -> Addr<Self> {
        Self {
            connections: HashMap::new(),
            reported_peers: Vec::new(),
        }
        .start()
    }
}

impl Actor for Peers {
    type Context = Context<Peers>;
}
struct GetReportedPeers {}
impl Message for GetReportedPeers {
    type Result = Vec<Peer>;
}
impl Handler<GetReportedPeers> for Peers {
    type Result = MessageResult<GetReportedPeers>;
    fn handle(&mut self, mut _msg: GetReportedPeers, _: &mut Self::Context) -> Self::Result {
        MessageResult(self.reported_peers.clone())
    }
}

impl Message for Connection {
    type Result = ();
}
struct PeersRequestListener {
    peers: Addr<Peers>,
    from: ConnectionId,
}
impl Listener for PeersRequestListener {
    fn get_peers_request(self, msg: GetPeersRequest) -> Accept<Self> {
        // Arbiter::spawn(self.peers.send(
        Accept::Consumed(self)
    }
}
impl Handler<Connection> for Peers {
    type Result = ();

    fn handle(&mut self, mut connection: Connection, ctx: &mut Self::Context) -> Self::Result {
        let message_stream = connection.take_message_stream();
        self.connections.insert(connection.id, connection);
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
