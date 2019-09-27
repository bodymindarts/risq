use crate::bisq::payload::{Ping, Pong};
use crate::connection::{Connection, ConnectionId};
use crate::listener::{Accept, Listener};
use actix::{Actor, Arbiter, AsyncContext, Context, Handler, Message, StreamHandler, WeakAddr};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::{
    prelude::{
        future::{self, Future, IntoFuture, Loop},
        stream::Stream,
    },
    timer::{self, Interval},
};

lazy_static! {
    static ref LOOP_INTERVAL_SEC: u64 = thread_rng().gen::<u64>() % 5 + 30;
    static ref LOOP_INTERVAL: Duration = Duration::from_secs(*LOOP_INTERVAL_SEC);
    static ref LAST_ACTIVITY_AGE: Duration = Duration::from_secs(*LOOP_INTERVAL_SEC / 2);
}

struct Info {
    last_active: Instant,
    last_ping: Instant,
    last_pong: Instant,
}
pub struct KeepAlive {
    info: HashMap<ConnectionId, Info>,
    connections: HashMap<ConnectionId, WeakAddr<Connection>>,
}
impl Actor for KeepAlive {
    type Context = Context<KeepAlive>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(LOOP_INTERVAL.to_owned(), |keep_alive, ctx| ());
    }
}
impl StreamHandler<Instant, timer::Error> for KeepAlive {
    fn handle(&mut self, item: Instant, ctx: &mut Context<KeepAlive>) {
        println!("PING");
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        println!("finished");
    }
}

// struct Tick {}
// impl Message for Tick {
//     type Result = ();
// }
// impl Handler<Tick> for KeepAlive {
//     type Result = ();
//     fn handle(&mut self, _: Tick, _: &mut Self::Context) -> Self::Result {
//         self.senders
//             .iter()
//             .for_each(|(id, addr)| match (addr.upgrade(), self.info.get(id)) {
//                 (Some(addr), Some(info))
//                     if Instant::now().duration_since(info.last_active) > *LAST_ACTIVITY_AGE =>
//                 {
//                     ()
//                 }
//                 _ => (),
//             });
//     }
// }
pub struct KeepAliveListener {
    // pub return_addr: WeakAddr<Sender>,
}
impl Listener for KeepAliveListener {
    fn ping(&mut self, msg: &Ping) -> Accept {
        let pong = Pong {
            request_nonce: msg.nonce,
        };
        // if let Some(addr) = self.return_addr.upgrade() {
        // Arbiter::spawn(addr.send(SendPayload(pong.into())).then(|_| Ok(())));
        // }
        Accept::Processed
    }
}
