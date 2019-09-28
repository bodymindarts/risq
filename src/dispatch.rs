use crate::bisq::payload::*;
use crate::connection::ConnectionId;
use actix::{dev::ToEnvelope, Actor, Addr, Arbiter, Handler, Message};
use std::marker::PhantomData;
use tokio::prelude::future::Future;

pub enum Dispatch {
    Forwarded,
    Retained(network_envelope::Message),
}

pub trait Dispatcher {
    fn dispatch(&self, conn: ConnectionId, msg: network_envelope::Message) -> Dispatch;
}
pub struct Receive<M>(pub ConnectionId, pub M);
impl<M> Message for Receive<M> {
    type Result = ();
}
pub struct ActorDispatcher<A, M>
where
    M: PayloadExtractor,
    A: Actor + Handler<Receive<<M as PayloadExtractor>::Payload>>,
{
    addr: Addr<A>,
    phantom: PhantomData<M>,
}
impl<A, M> ActorDispatcher<A, M>
where
    M: PayloadExtractor,
    A: Actor + Handler<Receive<<M as PayloadExtractor>::Payload>>,
{
    pub fn new(addr: Addr<A>) -> Self {
        ActorDispatcher {
            addr,
            phantom: PhantomData,
        }
    }
}
impl<A, M> Dispatcher for ActorDispatcher<A, M>
where
    M: PayloadExtractor + 'static,
    A: Actor + Handler<Receive<<M as PayloadExtractor>::Payload>>,
    <A as Actor>::Context: ToEnvelope<A, Receive<<M as PayloadExtractor>::Payload>>,
{
    fn dispatch(&self, conn: ConnectionId, msg: network_envelope::Message) -> Dispatch {
        match <M as PayloadExtractor>::extract(msg) {
            Extract::Succeeded(payload) => {
                Arbiter::spawn(self.addr.send(Receive(conn, payload)).then(|_| Ok(())));
                Dispatch::Forwarded
            }
            Extract::Failed(msg) => Dispatch::Retained(msg),
        }
    }
}
pub struct DummyDispatcher {}
impl Dispatcher for DummyDispatcher {
    fn dispatch(&self, _conn: ConnectionId, msg: network_envelope::Message) -> Dispatch {
        Dispatch::Retained(msg)
    }
}
