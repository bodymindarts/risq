use super::payload::NodeAddress;
use lazy_static::lazy_static;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum BaseCurrencyNetwork {
    BtcMainnet,
    BtcTestnet,
    BtcRegtest,
}

impl FromStr for BaseCurrencyNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BtcMainnet" | "Mainnet" => Ok(BaseCurrencyNetwork::BtcMainnet),
            "BtcTestnet" | "Testnet" => Ok(BaseCurrencyNetwork::BtcTestnet),
            "BtcRegtest" | "Regtest" => Ok(BaseCurrencyNetwork::BtcRegtest),
            _ => Err(()),
        }
    }
}

pub(super) const P2P_NETWORK_VERSION: i32 = 1;

pub const MAX_REPORTED_PEERS: usize = 1000;
pub const MAX_CONNECTIONS_DEFAULT: u8 = 12;
pub const NUM_SEEDS_FOR_PRELIMINARY_REQUEST: u8 = 2;
pub const NUM_ADDITIONAL_SEEDS_FOR_UPDATE_REQUEST: u8 = 1;

pub fn seed_nodes(network: &BaseCurrencyNetwork) -> Vec<NodeAddress> {
    match network {
        BaseCurrencyNetwork::BtcRegtest => vec![
            NodeAddress {
                host_name: "127.0.0.1".to_string(),
                port: 2002,
            },
            // NodeAddress {
            //     host_name: "127.0.0.1".to_string(),
            //     port: 3002,
            // },
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
static SUPPORTED_CAPABILITIES: [Capability; 11] = [
    Capability::TradeStatistics,
    Capability::TradeStatistics2,
    Capability::AccountAgeWitness,
    Capability::Proposal,
    Capability::BlindVote,
    Capability::AckMsg,
    Capability::ReceiveBsqBlock,
    Capability::DaoState,
    Capability::BundleOfEnvelopes,
    Capability::SignedAccountAgeWitness,
    Capability::Mediation,
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
