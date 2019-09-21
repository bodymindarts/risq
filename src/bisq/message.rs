include!("../generated/io.bisq.protobuffer.rs");
include!("../generated/message_macros.rs");

use super::constants::*;

#[derive(Debug, Clone, Copy)]
pub struct MessageVersion(i32);
impl From<MessageVersion> for i32 {
    fn from(msg: MessageVersion) -> i32 {
        msg.0
    }
}
impl From<BaseCurrencyNetwork> for MessageVersion {
    fn from(network: BaseCurrencyNetwork) -> MessageVersion {
        MessageVersion((network as i32) + 10 * P2P_NETWORK_VERSION)
    }
}

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

macro_rules! into_message {
    ($caml:ident, $snake:ident) => {
        impl From<$caml> for network_envelope::Message {
            fn from(msg: $caml) -> network_envelope::Message {
                network_envelope::Message::$caml(msg)
            }
        }
    };
}
for_all_messages!(into_message);

#[cfg(test)]
mod tests {
    use super::{network_envelope, Listener, Ping};
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
