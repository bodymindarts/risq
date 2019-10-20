use super::event::ConnectionAdded;
use crate::{
    bisq::payload::{gen_nonce, Ping, Pong},
    p2p::{
        connection::{Connection, ConnectionId, Payload, Request},
        dispatch::Receive,
    },
    prelude::*,
};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

lazy_static! {
    static ref LOOP_INTERVAL_SEC: u64 = thread_rng().gen::<u64>() % 5 + 30;
    static ref LOOP_INTERVAL: Duration = Duration::from_secs(*LOOP_INTERVAL_SEC);
    static ref LAST_ACTIVITY_AGE: Duration = Duration::from_secs(*LOOP_INTERVAL_SEC / 2);
}

struct Info {
    last_active: SystemTime,
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
        ctx.run_interval(*LOOP_INTERVAL, |keep_alive, ctx| {
            let infos = &mut keep_alive.infos;
            keep_alive.connections.retain(|id, conn| {
                if ping_peer(id.to_owned(), conn, infos.get(id), ctx) {
                    true
                } else {
                    infos.remove(id);
                    false
                }
            })
        });
    }
}
impl Handler<ConnectionAdded> for KeepAlive {
    type Result = ();
    fn handle(
        &mut self,
        ConnectionAdded(id, conn): ConnectionAdded,
        _: &mut Self::Context,
    ) -> Self::Result {
        self.connections.insert(id, conn);
    }
}
pub struct ReportLastActive;
impl Message for ReportLastActive {
    type Result = HashMap<ConnectionId, SystemTime>;
}
impl Handler<ReportLastActive> for KeepAlive {
    type Result = MessageResult<ReportLastActive>;

    fn handle(&mut self, _: ReportLastActive, _: &mut Self::Context) -> Self::Result {
        MessageResult(
            self.infos
                .iter()
                .map(|(id, info)| (*id, info.last_active.clone()))
                .collect(),
        )
    }
}

impl Handler<Receive<Ping>> for KeepAlive {
    type Result = ();
    fn handle(&mut self, Receive(id, ping): Receive<Ping>, _: &mut Self::Context) -> Self::Result {
        let now = SystemTime::now();
        self.infos.insert(
            id,
            Info {
                last_active: now,
                last_round_trip_time: Duration::from_millis(ping.last_round_trip_time as u64),
            },
        );
        if let Some(conn) = self.connections.get(&id) {
            if let Some(conn) = conn.upgrade() {
                arbiter_spawn!(conn.send(Payload(Pong {
                    request_nonce: ping.nonce,
                })));
            }
        }
    }
}

fn ping_peer(
    id: ConnectionId,
    conn: &WeakAddr<Connection>,
    info: Option<&Info>,
    ctx: &mut Context<KeepAlive>,
) -> bool {
    let send_time = SystemTime::now();
    let should_ping = match info {
        Some(info)
            if send_time
                .duration_since(info.last_active)
                .map(|t| t > *LAST_ACTIVITY_AGE)
                .unwrap_or(false) =>
        {
            true
        }
        None => true,
        _ => false,
    };
    if should_ping {
        if let Some(conn) = conn.upgrade() {
            let ping = Ping {
                nonce: gen_nonce(),
                last_round_trip_time: info.map_or(0, |i| i.last_round_trip_time.as_millis() as i32),
            };
            ctx.spawn(
                fut::wrap_future(conn.send(Request(ping)).flatten().map(move |_pong| {
                    let ret = SystemTime::now();
                    Info {
                        last_active: ret,
                        last_round_trip_time: ret
                            .duration_since(send_time)
                            .expect("Pong before Ping"),
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
