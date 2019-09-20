const P2P_NETWORK_VERSION: i32 = 1;
pub const MAX_PERMITTED_MESSAGE_SIZE: i32 = 10 * 1024 * 1024; // 10 MB (425 offers resulted in about 660 kb, mailbox msg will add more to it) offer has usually 2 kb, mailbox 3kb.

#[derive(Debug, Clone, Copy)]
pub enum BaseCurrencyNetwork {
    BtcMainnet,
    BtcTestnet,
    BtcRegtest,
}

#[macro_use]
pub mod message {
    include!("generated/io.bisq.protobuffer.rs");
    include!("generated/message_macros.rs");

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
            MessageVersion((network as i32) + 10 * P2P_NETWORK_VERSION)
        }
    }

    macro_rules! listener_method {
        ($caml:ident, $snake:ident) => {
            fn $snake(&mut self, _msg: $caml) -> () {}
        };
    }
    pub trait Listener {
        fn accept(&mut self, msg: network_envelope::Message) -> () {
            match_message!(msg, self);
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
        impl super::Listener for PingListener {
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
}

pub mod capabilities {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref LOCAL: Vec<i32> = {
            let mut vec = Vec::with_capacity(SUPPORTED_CAPABILITIES.len());
            SUPPORTED_CAPABILITIES
                .iter()
                .for_each(|c| vec.push(*c as i32));
            vec
        };
    }
    static SUPPORTED_CAPABILITIES: [Capability; 10] = [
        Capability::TradeStatistics,
        Capability::TradeStatistics2,
        Capability::AccountAgeWitness,
        Capability::AckMsg,
        Capability::Proposal,
        Capability::BlindVote,
        Capability::DaoState,
        Capability::BundleOfEnvelopes,
        Capability::Mediation,
        Capability::ReceiveBsqBlock,
    ];
    #[derive(Debug, Clone, Copy)]
    pub enum Capability {
        TradeStatistics, // Not required anymore as no old clients out there not having that support
        TradeStatistics2, // Not required anymore as no old clients out there not having that support
        AccountAgeWitness, // Not required anymore as no old clients out there not having that support
        SeedNode,          // Node is a seed node
        DaoFullNode,       // DAO full node can deliver BSQ blocks
        Proposal, // Not required anymore as no old clients out there not having that support
        BlindVote, // Not required anymore as no old clients out there not having that support
        AckMsg,   // Not required anymore as no old clients out there not having that support
        ReceiveBsqBlock, // Signaling that node which wants to receive BSQ blocks (DAO lite node)
        DaoState, // Not required anymore as no old clients out there not having that support

        //TODO can be set deprecated after v1.1.6 as we
        //enforce update there
        BundleOfEnvelopes, // Supports bundling of messages if many messages are sent in short interval
        SignedAccountAgeWitness, // Supports the signed account age witness feature
        Mediation,         // Supports mediation feature
    }
}
