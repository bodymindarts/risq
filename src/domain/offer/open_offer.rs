use crate::{
    bisq::BisqHash,
    domain::{amount::NumberWithPrecision, market::Market},
};
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

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct OfferSequence(i32);
impl From<i32> for OfferSequence {
    fn from(s: i32) -> Self {
        OfferSequence(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OfferDirection {
    Buy,
    Sell,
}

#[derive(Clone, Copy)]
pub enum OfferPrice {
    Fixed(NumberWithPrecision),
    MarketWithMargin(f64),
}

#[derive(Clone, Copy)]
pub struct OfferAmount {
    pub total: NumberWithPrecision,
    pub min: NumberWithPrecision,
}

#[derive(Clone)]
pub struct OpenOffer {
    pub bisq_hash: BisqHash,
    pub market: &'static Market,
    pub id: OfferId,
    pub direction: OfferDirection,
    pub price: OfferPrice,
    pub amount: OfferAmount,
    pub payment_method_id: String,
    pub offer_fee_tx_id: String,
    pub created_at: SystemTime,

    expires_at: SystemTime,
    latest_sequence: OfferSequence,
}

impl OpenOffer {
    pub fn new(
        bisq_hash: BisqHash,
        market: &'static Market,
        id: OfferId,
        direction: OfferDirection,
        price: OfferPrice,
        amount: OfferAmount,
        payment_method_id: String,
        offer_fee_tx_id: String,
        created_at: SystemTime,
        sequence: OfferSequence,
    ) -> OpenOffer {
        Self {
            bisq_hash,
            market,
            id,
            direction,
            price,
            amount,
            payment_method_id,
            created_at,
            expires_at: created_at + OFFER_TTL,
            latest_sequence: sequence,
            offer_fee_tx_id,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.elapsed().is_ok()
    }

    pub fn would_refresh(&self, sequence: OfferSequence) -> bool {
        sequence > self.latest_sequence
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
