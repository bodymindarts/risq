use crate::bisq::message::*;

macro_rules! listener_method {
    ($caml:ident, $snake:ident) => {
        fn $snake(&mut self, _msg: $caml) -> T {
            T::default()
        }
    };
}
pub trait Listener<T>
where
    T: Default,
{
    fn accept(&mut self, msg: network_envelope::Message) -> T {
        match_message!(msg, self)
    }
    fn accept_or_err<E>(&mut self, msg: Option<network_envelope::Message>, err: E) -> Result<T, E> {
        match msg {
            Some(msg) => Ok(self.accept(msg)),
            None => Err(err),
        }
    }
    for_all_messages!(listener_method);
}

#[cfg(test)]
mod tests {
    use super::Listener;
    use crate::bisq::message::{network_envelope, Ping};
    struct PingListener {}
    impl super::Listener<()> for PingListener {
        fn ping(&mut self, msg: Ping) -> () {
            assert!(msg.nonce == 5);
        }
    }

    #[test]
    fn accept_ping() {
        let mut l = PingListener {};
        l.accept(network_envelope::Message::Ping(Ping {
            nonce: 5,
            last_round_trip_time: 0,
        }));
    }
}
