use crate::bisq::{payload::OfferPayload, BisqHash};
use std::time::{Duration, SystemTime};

const OFFER_TTL: Duration = Duration::from_secs(9 * 60);

pub struct OpenOffer {
    pub bisq_hash: BisqHash,

    created_at: SystemTime,
    latest_sequence: i32,
    expires_at: SystemTime,
    payload: OfferPayload,
}

impl OpenOffer {
    pub fn new(
        bisq_hash: BisqHash,
        created_at: SystemTime,
        sequence: i32,
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

    pub fn refresh(&mut self, sequence: i32) {
        if sequence > self.latest_sequence {
            self.expires_at = SystemTime::now() + OFFER_TTL;
            self.latest_sequence = sequence;
        }
    }
}
