const P2P_NETWORK_VERSION: i32 = 1;
pub const MAX_PERMITTED_MESSAGE_SIZE: i32 = 10 * 1024 * 1024; // 10 MB (425 offers resulted in about 660 kb, mailbox msg will add more to it) offer has usually 2 kb, mailbox 3kb.

#[derive(Debug, Clone, Copy)]
pub enum BaseCurrencyNetwork {
    BtcMainnet,
    BtcTestnet,
    BtcRegtest,
}

pub mod proto {
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

    // pub struct MessageFactory(MessageVersion);
    // impl MessageFactory {
    //     pub fn new(network: &BaseCurrencyNetwork) -> MessageFactory {
    //         MessageFactory(message_version(network))
    //     }

    //     pub fn preliminary_get_data_request(&self) -> NetworkEnvelope {
    //         NetworkEnvelope {
    //             message_version: (&(self.0)).into(),
    //             message: Some(network_envelope::Message::PreliminaryGetDataRequest(
    //                 PreliminaryGetDataRequest {
    //                     nonce: rand::thread_rng().gen::<i32>(),
    //                     excluded_keys: Vec::new(),
    //                     supported_capabilities: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    //                 },
    //             )),
    //         }
    //     }
    // }

    include!("generated/io.bisq.protobuffer.rs");
}

mod capabilities {
    pub(super) fn for_app() -> Vec<i32> {
        Vec::new()
    }
}
