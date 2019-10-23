use crate::{
    bisq::PersistentMessageHash,
    domain::{
        amount::*,
        market::Market,
        offer::{OfferDirection, OfferId},
    },
};
use std::time::SystemTime;

#[derive(Clone)]
pub struct Trade {
    pub market: &'static Market,
    pub direction: OfferDirection,
    pub offer_id: OfferId,
    pub price: NumberWithPrecision,
    pub amount: NumberWithPrecision,
    pub volume: NumberWithPrecision,
    pub payment_method_id: String,
    pub timestamp: SystemTime,
    pub hash: PersistentMessageHash,
}
impl Trade {
    pub fn new(
        market: &'static Market,
        direction: OfferDirection,
        offer_id: OfferId,
        price: NumberWithPrecision,
        amount: NumberWithPrecision,
        payment_method_id: String,
        timestamp: SystemTime,
        hash: PersistentMessageHash,
    ) -> Self {
        Self {
            market,
            direction,
            offer_id,
            price,
            amount,
            volume: price * amount,
            payment_method_id,
            timestamp,
            hash,
        }
    }
}
