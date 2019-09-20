use std::net::SocketAddr;

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
pub fn seed_nodes(network: BaseCurrencyNetwork) -> Vec<SocketAddr> {
    match network {
        BaseCurrencyNetwork::BtcRegtest => vec![
            "127.0.0.1:2002".parse().unwrap(),
            "127.0.0.1:3002".parse().unwrap(),
        ],
        BaseCurrencyNetwork::BtcTestnet => vec!["m5izk3fvjsjbmkqi.onion:8001".parse().unwrap()],
        BaseCurrencyNetwork::BtcMainnet => vec![
            "5quyxpxheyvzmb2d.onion:8000".parse().unwrap(),
            "s67qglwhkgkyvr74.onion:8000".parse().unwrap(),
            "ef5qnzx6znifo3df.onion:8000".parse().unwrap(),
            "jhgcy2won7xnslrb.onion:8000".parse().unwrap(),
            "3f3cu2yw7u457ztq.onion:8000".parse().unwrap(),
            "723ljisnynbtdohi.onion:8000".parse().unwrap(),
            "rm7b56wbrcczpjvl.onion:8000".parse().unwrap(),
            "fl3mmribyxgrv63c.onion:8000".parse().unwrap(),
        ],
    }
}
