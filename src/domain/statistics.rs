mod hloc;
mod trade;

pub use hloc::*;
pub use trade::Trade;

#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use super::{trade::TradeHistory, *};
    use crate::{
        bisq::PersistentMessageHash,
        domain::{CommandResult, FutureCommandResult},
        prelude::*,
    };
    use std::{collections::HashSet, sync::Arc};

    pub struct StatsCacheInner {
        trades: TradeHistory,
        hashes: HashSet<PersistentMessageHash>,
    }
    impl StatsCacheInner {
        fn insert(&mut self, trade: Trade) -> CommandResult {
            if self.hashes.insert(trade.hash) {
                self.trades.insert(trade);
                CommandResult::Accepted
            } else {
                CommandResult::Ignored
            }
        }
        pub fn trades(&self) -> impl DoubleEndedIterator<Item = &Trade> {
            self.trades.iter()
        }
        pub fn hloc(&self, query: HlocQuery) -> Vec<Hloc> {
            Hloc::from_trades(&self.trades, query)
        }
    }

    #[derive(Clone)]
    pub struct StatsCache {
        inner: Arc<locks::RwLock<StatsCacheInner>>,
    }
    impl StatsCache {
        pub fn new() -> Option<Self> {
            Some(Self {
                inner: Arc::new(locks::RwLock::new(StatsCacheInner {
                    trades: TradeHistory::new(),
                    hashes: HashSet::new(),
                })),
            })
        }

        pub fn add(&self, trade: Trade) -> impl FutureCommandResult {
            self.inner
                .write()
                .map(move |mut inner| inner.insert(trade))
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
