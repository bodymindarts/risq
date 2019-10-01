use crate::bisq::{payload::OfferPayload, BisqHash};

pub struct OpenOffer {
    pub bisq_hash: BisqHash,

    payload: OfferPayload,
}
