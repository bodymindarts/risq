use crate::bisq::{payload::OfferPayload, BisqHash};
use std::time::{Duration, SystemTime};

const OFFER_TTL: Duration = Duration::from_secs(9 * 60);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct OfferId(String);
impl From<String> for OfferId {
    fn from(id: String) -> Self {
        OfferId(id)
    }
}
impl From<OfferId> for String {
    fn from(id: OfferId) -> Self {
        id.0
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd)]
pub struct OfferSequence(i32);
impl From<i32> for OfferSequence {
    fn from(s: i32) -> Self {
        OfferSequence(s)
    }
}

#[derive(Clone)]
pub struct OpenOffer {
    pub bisq_hash: BisqHash,
    pub id: OfferId,
    pub expires_at: SystemTime,
    pub created_at: SystemTime,

    latest_sequence: OfferSequence,
}

impl OpenOffer {
    pub fn new(
        bisq_hash: BisqHash,
        id: OfferId,
        created_at: SystemTime,
        sequence: OfferSequence,
    ) -> OpenOffer {
        Self {
            bisq_hash,
            id,
            created_at,
            expires_at: created_at + OFFER_TTL,
            latest_sequence: sequence,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.elapsed().is_ok()
    }

    pub fn refresh(&mut self, sequence: OfferSequence) {
        if sequence > self.latest_sequence {
            self.expires_at = SystemTime::now() + OFFER_TTL;
            self.latest_sequence = sequence;
        }
    }
}
