use crate::bisq::{payload::OfferPayload, BisqHash};
use std::time::{Duration, SystemTime};

const OFFER_TTL: Duration = Duration::from_secs(9 * 60);

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
    pub expires_at: SystemTime,
    pub created_at: SystemTime,

    latest_sequence: OfferSequence,
    payload: OfferPayload,
}

impl OpenOffer {
    pub fn new(
        bisq_hash: BisqHash,
        created_at: SystemTime,
        sequence: OfferSequence,
        payload: OfferPayload,
    ) -> OpenOffer {
        Self {
            bisq_hash,
            created_at,
            expires_at: created_at + OFFER_TTL,
            payload,
            latest_sequence: sequence,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.elapsed().is_ok()
    }
    pub fn id(&self) -> &String {
        &self.payload.id
    }

    pub fn refresh(&mut self, sequence: OfferSequence) {
        if sequence > self.latest_sequence {
            self.expires_at = SystemTime::now() + OFFER_TTL;
            self.latest_sequence = sequence;
        }
    }
}
