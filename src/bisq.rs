const P2P_NETWORK_VERSION: i32 = 1;
pub const MAX_PERMITTED_MESSAGE_SIZE: i32 = 10 * 1024 * 1024; // 10 MB (425 offers resulted in about 660 kb, mailbox msg will add more to it) offer has usually 2 kb, mailbox 3kb.

#[derive(Debug, Clone, Copy)]
pub enum BaseCurrencyNetwork {
    BtcMainnet,
    BtcTestnet,
    BtcRegtest,
}

pub mod message {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct MessageVersion(i32);
    impl From<MessageVersion> for i32 {
        fn from(msg: MessageVersion) -> i32 {
            msg.0
        }
    }
    impl From<super::BaseCurrencyNetwork> for MessageVersion {
        fn from(network: BaseCurrencyNetwork) -> MessageVersion {
            let base_currency_network = match network {
                BaseCurrencyNetwork::BtcMainnet => 0,
                BaseCurrencyNetwork::BtcTestnet => 1,
                BaseCurrencyNetwork::BtcRegtest => 2,
            };
            MessageVersion(base_currency_network + 10 * P2P_NETWORK_VERSION)
        }
    }

    include!("generated/io.bisq.protobuffer.rs");
    include!("generated/for_all_messages.rs");
    macro_rules! into_message {
        ($caml:ident,$snake:ident) => {
            impl From<$caml> for network_envelope::Message {
                fn from(msg: $caml) -> network_envelope::Message {
                    network_envelope::Message::$caml(msg)
                }
            }
        };
    }
    for_all_messages!(into_message);
}

mod capabilities {
    pub(super) fn for_app() -> Vec<i32> {
        Vec::new()
    }
}
