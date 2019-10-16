use super::{
    connection::{Connection, ConnectionId, Payload},
    peers::event::ConnectionAdded,
};
use crate::{bisq::payload::network_envelope, prelude::*};
use std::collections::HashMap;

pub struct Broadcaster {
    connections: HashMap<ConnectionId, WeakAddr<Connection>>,
}
impl Actor for Broadcaster {
    type Context = Context<Broadcaster>;
}

impl Broadcaster {
    pub fn start() -> Addr<Self> {
        Self {
            connections: HashMap::new(),
        }
        .start()
    }
}

pub struct Broadcast<M: Into<network_envelope::Message>>(pub M, pub Option<ConnectionId>);
impl<M> Message for Broadcast<M>
where
    M: Into<network_envelope::Message>,
{
    type Result = ();
}
impl<M: 'static> Handler<Broadcast<M>> for Broadcaster
where
    M: Into<network_envelope::Message> + Send + Clone,
{
    type Result = ();
    fn handle(&mut self, Broadcast(message, exclude): Broadcast<M>, _ctx: &mut Self::Context) {
        self.connections.retain(|id, conn| {
            conn.upgrade()
                .map(|conn| match exclude {
                    Some(exclude) if id == &exclude => (),
                    _ => arbiter_spawn!(conn.send(Payload(message.clone()))),
                })
                .is_some()
        });
    }
}
impl Handler<ConnectionAdded> for Broadcaster {
    type Result = ();
    fn handle(
        &mut self,
        ConnectionAdded(id, conn): ConnectionAdded,
        _: &mut Self::Context,
    ) -> Self::Result {
        self.connections.insert(id, conn);
    }
}
