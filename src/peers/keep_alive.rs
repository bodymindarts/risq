use crate::bisq::payload::{Ping, Pong};
use crate::listener::{Accept, Listener};
use crate::peers::sender::{SendPayload, Sender};
use actix::{Arbiter, WeakAddr};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::time::Duration;
use tokio::prelude::future::Future;

lazy_static! {
    static ref LOOP_INTERVAL_SEC: u64 = thread_rng().gen::<u64>() % 5 + 30;
    static ref LOOP_INTERVAL: Duration = Duration::from_secs(*LOOP_INTERVAL_SEC);
    static ref LAST_ACTIVITY_AGE: Duration = Duration::from_secs(*LOOP_INTERVAL_SEC / 2);
}

pub struct KeepAliveListener {
    pub return_addr: WeakAddr<Sender>,
}
impl Listener for KeepAliveListener {
    fn ping(&mut self, msg: &Ping) -> Accept {
        let pong = Pong {
            request_nonce: msg.nonce,
        };
        if let Some(addr) = self.return_addr.upgrade() {
            Arbiter::spawn(addr.send(SendPayload(pong.into())).then(|_| Ok(())));
        }
        Accept::Processed
    }
}
