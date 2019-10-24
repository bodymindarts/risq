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
#[cfg(feature = "statistics")]
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
            volume: (price * amount).with_precision(market.right.bisq_internal_precision()),
            payment_method_id,
            timestamp,
            hash,
        }
    }
}

#[cfg(feature = "statistics")]
pub struct TradeHistory {
    inner: Vec<Trade>,
}
#[cfg(feature = "statistics")]
impl TradeHistory {
    pub(super) fn new() -> Self {
        Self {
            inner: Vec::with_capacity(60000),
        }
    }
    pub(super) fn insert(&mut self, trade: Trade) {
        for n in (0..=self.inner.len()).rev() {
            if n == 0 || trade.timestamp > self.inner[n - 1].timestamp {
                self.inner.insert(n, trade);
                break;
            }
        }
    }
    pub(super) fn insert_all(&mut self, trades: impl IntoIterator<Item = Trade>) {
        self.inner.extend(trades.into_iter());
        self.inner
            .sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Trade> {
        self.inner.iter()
    }
    pub fn first_trade_time(&self) -> Option<SystemTime> {
        self.inner.get(0).map(|t| t.timestamp)
    }
}
