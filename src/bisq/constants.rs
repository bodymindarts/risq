use super::payload::NodeAddress;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub enum BaseCurrencyNetwork {
    BtcMainnet,
    BtcTestnet,
    BtcRegtest,
}

pub(super) const P2P_NETWORK_VERSION: i32 = 1;
pub const MAX_PERMITTED_MESSAGE_SIZE: i32 = 10 * 1024 * 1024; // 10 MB (425 offers resulted in about 660 kb, mailbox msg will add more to it) offer has usually 2 kb, mailbox 3kb.
pub const NUM_SEEDS_FOR_PRELIMINARY_REQUEST: u8 = 2;
pub const NUM_ADDITIONAL_SEEDS_FOR_UPDATE_REQUEST: u8 = 1;

pub fn seed_nodes(network: BaseCurrencyNetwork) -> Vec<NodeAddress> {
    match network {
        BaseCurrencyNetwork::BtcRegtest => vec![
            NodeAddress {
                host_name: "127.0.0.1".to_string(),
                port: 2002,
            },
            NodeAddress {
                host_name: "127.0.0.1".to_string(),
                port: 3002,
            },
        ],
        BaseCurrencyNetwork::BtcTestnet => vec![NodeAddress {
            host_name: "m5izk3fvjsjbmkqi.onion".to_string(),
            port: 8001,
        }],
        BaseCurrencyNetwork::BtcMainnet => vec![
            NodeAddress {
                host_name: "5quyxpxheyvzmb2d.onion".to_string(),
                port: 8000,
            },
            NodeAddress {
                host_name: "s67qglwhkgkyvr74.onion".to_string(),
                port: 8000,
            },
            NodeAddress {
                host_name: "ef5qnzx6znifo3df.onion".to_string(),
                port: 8000,
            },
            NodeAddress {
                host_name: "jhgcy2won7xnslrb.onion".to_string(),
                port: 8000,
            },
            NodeAddress {
                host_name: "3f3cu2yw7u457ztq.onion".to_string(),
                port: 8000,
            },
            NodeAddress {
                host_name: "723ljisnynbtdohi.onion".to_string(),
                port: 8000,
            },
            NodeAddress {
                host_name: "rm7b56wbrcczpjvl.onion".to_string(),
                port: 8000,
            },
            NodeAddress {
                host_name: "fl3mmribyxgrv63c.onion".to_string(),
                port: 8000,
            },
        ],
    }
}

lazy_static! {
    pub static ref LOCAL_CAPABILITIES: Vec<i32> = {
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
    SeedNode,        // Node is a seed node
    DaoFullNode,     // DAO full node can deliver BSQ blocks
    Proposal,        // Not required anymore as no old clients out there not having that support
    BlindVote,       // Not required anymore as no old clients out there not having that support
    AckMsg,          // Not required anymore as no old clients out there not having that support
    ReceiveBsqBlock, // Signaling that node which wants to receive BSQ blocks (DAO lite node)
    DaoState,        // Not required anymore as no old clients out there not having that support

    //TODO can be set deprecated after v1.1.6 as we
    //enforce update there
    BundleOfEnvelopes, // Supports bundling of messages if many messages are sent in short interval
    SignedAccountAgeWitness, // Supports the signed account age witness feature
    Mediation,         // Supports mediation feature
}
