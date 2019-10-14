#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use crate::{
        bisq::BisqHash,
        domain::{offer::OfferDirection, CommandResult, FutureCommandResult},
        prelude::*,
    };
    use iso4217::CurrencyCode;
    use std::{collections::HashSet, str::FromStr, sync::Arc};

    #[derive(Clone)]
    pub struct Trade {
        // pub currency: CurrencyCode,
        pub direction: OfferDirection,
        pub hash: BisqHash,
    }

    #[derive(Clone)]
    pub struct StatsCache {
        inner: Arc<locks::RwLock<StatsCacheInner>>,
    }
    impl juniper::Context for StatsCacheInner {}
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
            info!("Adding Trade");
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
    use crate::prelude::*;
    use actix_web::{Error, HttpResponse};

    #[derive(Clone)]
    pub struct StatsCache;
    impl StatsCache {
        pub fn new() -> Option<Self> {
            None
        }
    }
    pub struct Schema;
    pub fn create_schema() -> Schema {
        Schema
    }
    pub fn graphql() -> impl Future<Item = HttpResponse, Error = Error> {
        future::ok(HttpResponse::Ok().finish())
    }
    pub fn graphiql() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
