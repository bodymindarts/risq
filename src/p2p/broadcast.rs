use super::{
    connection::ConnectionId,
    dispatch::SendableDispatcher,
    peers::{Broadcast, Peers},
};
use crate::bisq::payload::network_envelope;
use actix::{Addr, Arbiter};
use tokio::prelude::future::Future;

pub struct Broadcaster<D: SendableDispatcher> {
    peers: Addr<Peers<D>>,
}

impl<D: SendableDispatcher> Broadcaster<D> {
    pub fn new(peers: Addr<Peers<D>>) -> Self {
        Self { peers }
    }
    pub fn broadcast<M>(&self, msg: M, exclude: Option<ConnectionId>)
    where
        M: Into<network_envelope::Message> + Clone + Send + 'static,
    {
        Arbiter::spawn(self.peers.send(Broadcast(msg, exclude)).then(|_| Ok(())));
    }
}
