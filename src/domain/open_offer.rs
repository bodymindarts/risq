use crate::bisq::{payload::OfferPayload, BisqHash};

pub struct OpenOffer {
    pub bisq_hash: BisqHash,

    payload: OfferPayload,
}

impl OpenOffer {
    pub fn new(bisq_hash: BisqHash, payload: OfferPayload) -> OpenOffer {
        Self { bisq_hash, payload }
    }
}
