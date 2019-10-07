use crate::bisq::payload::OfferPayload;
use crate::bisq::BisqHash;
use std::time::{Duration, SystemTime};

const OFFER_TTL: Duration = Duration::from_secs(9 * 60);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug)]
pub struct OfferSequence(i32);
impl From<i32> for OfferSequence {
    fn from(s: i32) -> Self {
        OfferSequence(s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OfferDirection {
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug)]
pub enum OfferPrice {
    Fixed(i64),
    MarketWithMargin(f64),
}

#[derive(Clone, Copy, Debug)]
pub struct OfferAmount {
    pub total: i64,
    pub min: i64,
}

#[derive(Clone, Debug)]
pub struct OpenOffer {
    pub bisq_hash: BisqHash,
    pub id: OfferId,
    pub direction: OfferDirection,
    pub price: OfferPrice,
    pub amount: OfferAmount,
    pub payload: OfferPayload,

    expires_at: SystemTime,
    created_at: SystemTime,
    latest_sequence: OfferSequence,
}

impl OpenOffer {
    pub fn new(
        payload: OfferPayload,
        bisq_hash: BisqHash,
        id: OfferId,
        direction: OfferDirection,
        price: OfferPrice,
        amount: OfferAmount,
        created_at: SystemTime,
        sequence: OfferSequence,
    ) -> OpenOffer {
        Self {
            payload,
            bisq_hash,
            id,
            direction,
            price,
            amount,
            created_at,
            expires_at: created_at + OFFER_TTL,
            latest_sequence: sequence,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.elapsed().is_ok()
    }

    pub fn refresh(&mut self, sequence: OfferSequence) -> bool {
        if sequence > self.latest_sequence {
            self.expires_at = SystemTime::now() + OFFER_TTL;
            self.latest_sequence = sequence;
            return true;
        }
        false
    }
}
