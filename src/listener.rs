use crate::bisq::payload::*;
use crate::error::Error;

pub enum Accept {
    Processed,
    Skipped,
}
macro_rules! listener_method {
    ($caml:ident, $snake:ident) => {
        fn $snake(&mut self, msg: &$caml) -> Accept {
            Accept::Skipped
        }
    };
}
pub trait Listener {
    fn accept(&mut self, msg: &network_envelope::Message) -> Accept {
        match_payload!(msg, self)
    }
    fn accept_or_err(
        &mut self,
        msg: &Option<network_envelope::Message>,
        err: Error,
    ) -> Result<Accept, Error> {
        match msg {
            Some(msg) => Ok(self.accept(msg)),
            None => Err(err),
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
        fn ping(&mut self, msg: &Ping) -> Accept {
            assert!(msg.nonce == 5);
            self.called = true;
            Accept::Processed
        }
    }

    #[test]
    fn accept_ping() {
        let mut listener = PingListener { called: false };
        match listener.accept(&network_envelope::Message::Ping(Ping {
            nonce: 5,
            last_round_trip_time: 0,
        })) {
            Accept::Processed => assert!(listener.called),
            _ => assert!(false),
        }
    }
}
