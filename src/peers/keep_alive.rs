use crate::bisq::payload::{gen_nonce, Ping, Pong};
use crate::connection::{Connection, ConnectionId, Payload, Request};
use crate::listener::{Accept, Listener};
use actix::{
    fut::{self, ActorFuture},
    Actor, Addr, Arbiter, AsyncContext, Context, Handler, Message, WeakAddr,
};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::{
    prelude::future::{self, Future, IntoFuture, Loop},
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
impl KeepAlive {
    pub fn start() -> Addr<KeepAlive> {
        KeepAlive {
            infos: HashMap::new(),
            connections: HashMap::new(),
        }
        .start()
    }
}
impl Actor for KeepAlive {
    type Context = Context<KeepAlive>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(LOOP_INTERVAL.to_owned(), |keep_alive, ctx| {
            let infos = &mut keep_alive.infos;
            keep_alive.connections.retain(|id, addr| {
                if ping_peer(id.to_owned(), addr, infos.get(id), ctx) {
                    true
                } else {
                    infos.remove(id);
                    false
                }
            })
        });
    }
}
pub struct AddConnection(pub ConnectionId, pub WeakAddr<Connection>);
impl Message for AddConnection {
    type Result = ();
}
impl Handler<AddConnection> for KeepAlive {
    type Result = ();
    fn handle(
        &mut self,
        AddConnection(id, addr): AddConnection,
        _: &mut Self::Context,
    ) -> Self::Result {
        self.connections.insert(id, addr);
    }
}

struct PingReceived(ConnectionId, Instant, Ping);
impl Message for PingReceived {
    type Result = ();
}
impl Handler<PingReceived> for KeepAlive {
    type Result = ();
    fn handle(
        &mut self,
        PingReceived(id, time, ping): PingReceived,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(info) = self.infos.get_mut(&id) {
            info.last_active = time;
        }
        if let Some(addr) = self.connections.get(&id) {
            if let Some(addr) = addr.upgrade() {
                Arbiter::spawn(
                    addr.send(Payload(Pong {
                        request_nonce: ping.nonce,
                    }))
                    .then(|_| Ok(())),
                );
            }
        }
    }
}

fn ping_peer(
    id: ConnectionId,
    addr: &WeakAddr<Connection>,
    info: Option<&Info>,
    ctx: &mut Context<KeepAlive>,
) -> bool {
    let send_time = Instant::now();
    let should_ping = match info {
        Some(info) if send_time.duration_since(info.last_active) > *LAST_ACTIVITY_AGE => true,
        None => true,
        _ => false,
    };
    if should_ping {
        if let Some(addr) = addr.upgrade() {
            let ping = Ping {
                nonce: gen_nonce(),
                last_round_trip_time: info.map_or(0, |i| i.last_round_trip_time.as_millis() as i32),
            };
            ctx.spawn(
                fut::wrap_future(addr.send(Request(ping)).flatten().map(move |_pong| {
                    let ret = Instant::now();
                    Info {
                        last_active: ret,
                        last_round_trip_time: ret - send_time,
                    }
                }))
                .map(move |info, keep_alive: &mut KeepAlive, _ctx| {
                    keep_alive.infos.insert(id, info)
                })
                .then(|_, _, _| fut::ok(())),
            );
            true
        } else {
            false
        }
    } else {
        true
    }
}

pub struct KeepAliveListener {
    pub keep_alive: Addr<KeepAlive>,
    pub connection_id: ConnectionId,
}
impl Listener for KeepAliveListener {
    fn ping(&mut self, ping: &Ping) -> Accept {
        Arbiter::spawn(
            self.keep_alive
                .clone()
                .send(PingReceived(
                    self.connection_id,
                    Instant::now(),
                    ping.to_owned(),
                ))
                .then(|_| Ok(())),
        );
        Accept::Processed
    }
}
