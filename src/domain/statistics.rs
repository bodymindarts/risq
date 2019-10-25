mod hloc;
mod ticker;
mod trade;

pub use hloc::*;
pub use ticker::Ticker;
pub use trade::Trade;

#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use super::{trade::TradeHistory, *};
    use crate::{
        bisq::PersistentMessageHash,
        domain::{market::Market, offer::OfferId, CommandResult, FutureCommandResult},
        prelude::*,
    };
    use std::{collections::HashSet, sync::Arc};

    pub struct StatsCacheInner {
        trades: TradeHistory,
        ids: HashSet<PersistentMessageHash>,
    }
    impl StatsCacheInner {
        fn insert(&mut self, trade: Trade) -> CommandResult {
            if self.ids.insert(trade.hash) {
                self.trades.insert(trade);
                CommandResult::Accepted
            } else {
                CommandResult::Ignored
            }
        }
        fn bootstrap(&mut self, trades: Vec<Trade>) {
            let mut ids = self.ids.clone();
            self.trades
                .insert_all(trades.into_iter().filter(|t| ids.insert(t.hash)));
            self.ids = ids;
        }
        pub fn trades(&self) -> impl DoubleEndedIterator<Item = &Trade> {
            self.trades.iter()
        }
        pub fn hloc(&self, query: HlocQuery) -> Vec<Hloc> {
            Hloc::from_trades(&self.trades, query)
        }
        pub fn ticker(&self, market: Option<&'static Market>) -> Vec<Ticker> {
            Ticker::from_trades(&self.trades, market)
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
                    ids: HashSet::new(),
                })),
            })
        }

        pub fn add(&self, trade: Trade) -> impl FutureCommandResult {
            self.inner
                .write()
                .map(move |mut inner| inner.insert(trade))
                .map_err(|_| MailboxError::Closed)
        }
        pub fn bootstrap(&self, trades: Vec<Trade>) -> impl Future<Item = (), Error = ()> {
            self.inner
                .write()
                .map(move |mut inner| {
                    inner.bootstrap(trades);
                })
                .then(|_| Ok(()))
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
