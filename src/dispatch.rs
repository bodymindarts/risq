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
    A: Actor + Handler<Receive<<M as PayloadExtractor>::Extraction>>,
{
    addr: Addr<A>,
    phantom: PhantomData<M>,
}
impl<A, M> ActorDispatcher<A, M>
where
    M: PayloadExtractor,
    A: Actor + Handler<Receive<<M as PayloadExtractor>::Extraction>>,
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
    A: Actor + Handler<Receive<<M as PayloadExtractor>::Extraction>>,
    <A as Actor>::Context: ToEnvelope<A, Receive<<M as PayloadExtractor>::Extraction>>,
{
    fn dispatch(&self, conn: ConnectionId, msg: network_envelope::Message) -> Dispatch {
        match <M as PayloadExtractor>::extract(msg) {
            Extract::Succeeded(extraction) => {
                Arbiter::spawn(self.addr.send(Receive(conn, extraction)).then(|_| Ok(())));
                Dispatch::Forwarded
            }
            Extract::Failed(msg) => Dispatch::Retained(msg),
        }
    }
}
impl<A, M> Clone for ActorDispatcher<A, M>
where
    M: PayloadExtractor,
    A: Actor + Handler<Receive<<M as PayloadExtractor>::Extraction>>,
{
    fn clone(&self) -> Self {
        Self {
            addr: self.addr.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct Chain<F: Dispatcher + Sized> {
    first: F,
}
pub fn chain<F: Dispatcher + Sized>(first: F) -> Chain<F> {
    Chain { first }
}
impl<F: Dispatcher + Sized> Chain<F> {
    pub fn forward_to<N: Dispatcher + Sized>(self, next: N) -> ForwardTo<F, N> {
        ForwardTo {
            first: self.first,
            next,
        }
    }
}
pub struct ForwardTo<F: Dispatcher + Sized, N: Dispatcher + Sized> {
    first: F,
    next: N,
}
impl<F: Dispatcher + Sized, N: Dispatcher + Sized> ForwardTo<F, N> {
    fn forward_to<O: Dispatcher + Sized>(self, next: O) -> ForwardTo<Self, O> {
        ForwardTo {
            first: self,
            next: next,
        }
    }
}
impl<F: Dispatcher + Sized, N: Dispatcher + Sized> Dispatcher for ForwardTo<F, N> {
    fn dispatch(&self, conn: ConnectionId, msg: network_envelope::Message) -> Dispatch {
        match self.first.dispatch(conn, msg) {
            Dispatch::Forwarded => Dispatch::Forwarded,
            Dispatch::Retained(msg) => self.next.dispatch(conn, msg),
        }
    }
}
