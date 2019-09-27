use crate::bisq::payload::{gen_nonce, Ping, Pong};
use crate::connection::{Connection, ConnectionId, Request};
use crate::error;
use crate::listener::{Accept, Listener};
use actix::{
    fut::{self, ActorFuture},
    Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message, StreamHandler, WeakAddr,
};
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
    last_round_trip_time: Duration,
}
pub struct KeepAlive {
    infos: HashMap<ConnectionId, Info>,
    connections: HashMap<ConnectionId, WeakAddr<Connection>>,
}
fn ping_peer(
    id: ConnectionId,
    addr: Addr<Connection>,
    info: Option<&Info>,
    ctx: &mut Context<KeepAlive>,
) {
    let ping = Ping {
        nonce: gen_nonce(),
        last_round_trip_time: info.map_or(0, |i| i.last_round_trip_time.as_millis() as i32),
    };
    let send = Instant::now();
    ctx.spawn(
        fut::wrap_future(addr.send(Request(ping)).flatten().map(move |_pong| {
            let ret = Instant::now();
            Info {
                last_active: ret,
                last_round_trip_time: ret - send,
            }
        }))
        .map(move |info, keep_alive: &mut KeepAlive, _ctx| keep_alive.infos.insert(id, info))
        .then(|_, _, _| fut::ok(())),
    );
}
impl Actor for KeepAlive {
    type Context = Context<KeepAlive>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(LOOP_INTERVAL.to_owned(), |keep_alive, ctx| {
            let now = Instant::now();
            let infos = &keep_alive.infos;
            keep_alive
                .connections
                .retain(|id, addr| match infos.get(id) {
                    Some(info) if now.duration_since(info.last_active) > *LAST_ACTIVITY_AGE => {
                        if let Some(addr) = addr.upgrade() {
                            ping_peer(id.to_owned(), addr, info.into(), ctx);
                            true
                        } else {
                            false
                        }
                    }
                    None => {
                        if let Some(addr) = addr.upgrade() {
                            ping_peer(id.to_owned(), addr, None, ctx);
                            true
                        } else {
                            false
                        }
                    }
                    Some(_) => true,
                })
        });
    }
}
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
