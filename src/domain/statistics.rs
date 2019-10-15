use super::{
    amount::*,
    market::Market,
    offer::{OfferDirection, OfferId},
};
use crate::bisq::BisqHash;

#[derive(Clone)]
pub struct Trade {
    pub market: &'static Market,
    pub direction: OfferDirection,
    pub offer_id: OfferId,
    pub price: MonetaryAmount,
    pub amount: MonetaryAmount,
    pub volume: MonetaryAmount,
    pub payment_method_id: String,
    pub hash: BisqHash,
}
impl Trade {
    pub fn new(
        market: &'static Market,
        direction: OfferDirection,
        offer_id: OfferId,
        price: u64,
        amount: u64,
        payment_method_id: String,
        hash: BisqHash,
    ) -> Self {
        Self {
            market,
            direction,
            offer_id,
            price: MonetaryAmount::new(price, market.right),
            amount: MonetaryAmount::new(amount, market.left),
            volume: Self::determin_volume(price, amount, market),
            payment_method_id,
            hash,
        }
    }

    fn determin_volume(
        mut price: u64,
        mut amount: u64,
        Market { left, right, .. }: &Market,
    ) -> MonetaryAmount {
        let (left_precision, right_precision) = (left.precision(), right.precision());
        let mut res_precision = left_precision + right_precision;
        while res_precision > right_precision {
            if price % 10 == 0 {
                price = price / 10;
                res_precision -= 1;
            } else {
                break;
            }
        }
        while res_precision > right_precision {
            if amount % 10 == 0 {
                amount = amount / 10;
                res_precision -= 1;
            } else {
                break;
            }
        }
        let mut volume = amount * price;
        if res_precision > right_precision {
            volume = volume / 10_u64.pow(res_precision - right_precision);
        } else if res_precision < right_precision {
            volume = volume * 10_u64.pow(right_precision - res_precision);
        }
        MonetaryAmount::new(volume, right)
    }
}

#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use super::*;
    use crate::{
        bisq::BisqHash,
        domain::{CommandResult, FutureCommandResult},
        prelude::*,
    };
    use std::{collections::HashSet, str::FromStr, sync::Arc};

    #[derive(Clone)]
    pub struct StatsCache {
        inner: Arc<locks::RwLock<StatsCacheInner>>,
    }

    pub struct StatsCacheInner {
        trades: Vec<Trade>,
        hashes: HashSet<BisqHash>,
    }
    impl StatsCacheInner {
        fn add(&mut self, trade: Trade) -> CommandResult {
            if self.hashes.insert(trade.hash) {
                self.trades.push(trade);
                CommandResult::Accepted
            } else {
                CommandResult::Ignored
            }
        }
        pub fn trades(&self) -> &Vec<Trade> {
            &self.trades
        }
    }

    impl StatsCache {
        pub fn new() -> Option<Self> {
            Some(Self {
                inner: Arc::new(locks::RwLock::new(StatsCacheInner {
                    trades: Vec::new(),
                    hashes: HashSet::new(),
                })),
            })
        }

        pub fn add(&self, trade: Trade) -> impl FutureCommandResult {
            self.inner
                .write()
                .map(move |mut inner| inner.add(trade))
                .map_err(|_| MailboxError::Closed)
        }

        pub fn inner(
            &self,
        ) -> impl Future<Item = locks::RwLockReadGuard<StatsCacheInner>, Error = ()> {
            self.inner.read()
        }
    }
}

#[cfg(not(feature = "statistics"))]
pub use empty::*;
#[cfg(not(feature = "statistics"))]
mod empty {
    #[derive(Clone)]
    pub struct StatsCache;
    impl StatsCache {
        pub fn new() -> Option<Self> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{amount::MonetaryAmount, currency, market};

    #[test]
    fn determin_volume() {
        let eur = currency::from_code("EUR").unwrap();
        let market = market::from_pair(currency::from_code("BTC").unwrap(), eur).unwrap();
        let price = 9000 * 10_u64.pow(market.right.precision());
        let amount = 1 * 10_u64.pow(market.left.precision());
        let high_volume = super::Trade::determin_volume(price, amount, market);
        let low_volume = super::Trade::determin_volume(price, amount / 10000, market);
        assert!(&high_volume.format(8) == "9000.00000000");
        assert!(&low_volume.format(8) == "0.90000000");
    }
}
