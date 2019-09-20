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
