use crate::bisq::payload::*;

#[allow(dead_code)]
pub enum StoragePayloadKind {
    Alert,
    Arbitrator,
    Mediator,
    Filter,
    /// not used anymore from v0.6 on. But leave it for receiving TradeStatistics objects from older
    /// versions and convert it to TradeStatistics2 objects.
    TradeStatistics,
    MailboxStoragePayload,
    OfferPayload,
    TempProposalPayload,
    Unknown,
}
impl Default for StoragePayloadKind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<&ProtectedStorageEntry> for StoragePayloadKind {
    fn from(msg: &ProtectedStorageEntry) -> Self {
        msg.storage_payload
            .as_ref()
            .and_then(|p| p.message.as_ref())
            .map(|m| match m {
                storage_payload::Message::OfferPayload(_) => StoragePayloadKind::OfferPayload,
                _ => StoragePayloadKind::Unknown,
            })
            .unwrap_or_default()
    }
}

#[allow(dead_code)]
pub enum PersistableNetworkPayloadKind {
    AccountAgeWitness,
    TradeStatistics2,
    ProposalPayload,
    BlindVotePayload,
    SignedWitness,
    Unknown,
}
impl Default for PersistableNetworkPayloadKind {
    fn default() -> Self {
        Self::Unknown
    }
}
impl From<&PersistableNetworkPayload> for PersistableNetworkPayloadKind {
    fn from(payload: &PersistableNetworkPayload) -> Self {
        payload
            .message
            .as_ref()
            .map(|m| match m {
                persistable_network_payload::Message::TradeStatistics2(_) => {
                    PersistableNetworkPayloadKind::TradeStatistics2
                }
                _ => PersistableNetworkPayloadKind::Unknown,
            })
            .unwrap_or_default()
    }
}
