const P2P_NETWORK_VERSION: i32 = 1;
const MAX_PERMITTED_MESSAGE_SIZE: i32 = 10 * 1024 * 1024; // 10 MB (425 offers resulted in about 660 kb, mailbox msg will add more to it) offer has usually 2 kb, mailbox 3kb.

pub enum BaseCurrencyNetwork {
    BtcMainnet,
    BtcTestnet,
    BtcRegtest,
}

impl BaseCurrencyNetwork {
    pub fn get_message_version(&self) -> i32 {
        let base_currency_network = match self {
            BaseCurrencyNetwork::BtcMainnet => 0,
            BaseCurrencyNetwork::BtcTestnet => 1,
            BaseCurrencyNetwork::BtcRegtest => 2,
        };
        base_currency_network + 10 * P2P_NETWORK_VERSION
    }
}

pub mod message {
    include!("generated/io.bisq.protobuffer.rs");
}
