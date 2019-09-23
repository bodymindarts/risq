use crate::bisq::payload::*;
use crate::error::Error;

pub enum Accept<T> {
    Consumed(T),
    Skipped(network_envelope::Message, T),
    Error(Error),
}
macro_rules! listener_method {
    ($caml:ident, $snake:ident) => {
        fn $snake(self, msg: $caml) -> Accept<Self> {
            Accept::Skipped(msg.into(), self)
        }
    };
}
pub trait Listener: Sized {
    fn accept(self, msg: network_envelope::Message) -> Accept<Self> {
        match_payload!(msg, self)
    }
    fn accept_or_err(self, msg: Option<network_envelope::Message>, err: Error) -> Accept<Self> {
        match msg {
            Some(msg) => self.accept(msg),
            None => Accept::Error(err),
        }
    }
    for_all_payloads!(listener_method);
}

#[cfg(test)]
mod tests {
    use super::{Accept, Listener};
    use crate::bisq::payload::{network_envelope, Ping};
    struct PingListener {
        pub called: bool,
    }
    impl Listener for PingListener {
        fn ping(self, msg: Ping) -> Accept<Self> {
            assert!(msg.nonce == 5);
            Accept::Consumed(PingListener { called: true })
        }
    }

    #[test]
    fn accept_ping() {
        let listener = PingListener { called: false };
        match listener.accept(network_envelope::Message::Ping(Ping {
            nonce: 5,
            last_round_trip_time: 0,
        })) {
            Accept::Consumed(listener) => assert!(listener.called),
            _ => assert!(false),
        }
    }
}
