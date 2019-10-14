use super::{market::Market, offer::OfferDirection};
use crate::bisq::BisqHash;

#[derive(Clone)]
pub struct Trade {
    pub market: &'static Market,
    pub direction: OfferDirection,
    pub hash: BisqHash,
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
            info!("ADDING TRADE TO INNER");
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
