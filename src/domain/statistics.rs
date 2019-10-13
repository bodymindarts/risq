#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use crate::{
        bisq::BisqHash,
        domain::{offer::OfferDirection, CommandResult, FutureCommandResult},
        prelude::*,
    };
    use actix_web::{web, Error, HttpResponse};
    use iso4217::CurrencyCode;
    use juniper::{
        self, graphql_object,
        http::{graphiql::graphiql_source, GraphQLRequest},
        EmptyMutation, FieldResult, GraphQLInputObject, RootNode,
    };
    use juniper_from_schema::graphql_schema_from_file;
    use std::{collections::HashSet, str::FromStr, sync::Arc};

    pub fn graphql(
        schema: web::Data<Arc<Schema>>,
        cache: web::Data<StatsCache>,
        request: web::Json<GraphQLRequest>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        cache
            .inner()
            .map_err(Error::from)
            .and_then(|cache| {
                web::block(move || {
                    let res = request.execute(&schema, &cache);
                    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
                })
                .map_err(Error::from)
            })
            .and_then(|result| {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(result))
            })
    }
    pub fn graphiql() -> HttpResponse {
        let html = graphiql_source("http://localhost:7477/graphql");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)
    }

    graphql_schema_from_file!("src/domain/schema.graphql", context_type: Inner);

    #[derive(Clone)]
    pub struct Trade {
        // pub currency: CurrencyCode,
        pub direction: OfferDirection,
        pub hash: BisqHash,
    }

    impl TradeFields for Trade {
        // fn field_currency(&self, executor: &juniper::Executor<'_, Inner>) -> FieldResult<String> {
        //     Ok(self.currency.alpha3.to_owned())
        // }
        fn field_direction(
            &self,
            executor: &juniper::Executor<'_, Inner>,
        ) -> FieldResult<Direction> {
            Ok(match self.direction {
                OfferDirection::Sell => Direction::Sell,
                OfferDirection::Buy => Direction::Buy,
            })
        }
    }

    pub struct Query;
    impl QueryFields for Query {
        fn field_trades<'a>(
            &self,
            executor: &juniper::Executor<'a, Inner>,
            trail: &QueryTrail<'_, Trade, juniper_from_schema::Walked>,
        ) -> FieldResult<Vec<Trade>> {
            let cache = executor.context();
            Ok(cache.trades.iter().cloned().collect())
        }
    }

    type Mutation = EmptyMutation<Inner>;

    pub fn create_schema() -> Schema {
        Schema::new(Query {}, EmptyMutation::new())
    }

    #[derive(Clone)]
    pub struct StatsCache {
        inner: Arc<locks::RwLock<Inner>>,
    }
    impl juniper::Context for Inner {}
    pub struct Inner {
        trades: Vec<Trade>,
        hashes: HashSet<BisqHash>,
    }
    impl Inner {
        fn add(&mut self, trade: Trade) -> CommandResult {
            if self.hashes.insert(trade.hash) {
                self.trades.push(trade);
                CommandResult::Accepted
            } else {
                CommandResult::Ignored
            }
        }
    }

    impl StatsCache {
        pub fn new() -> Option<Self> {
            Some(Self {
                inner: Arc::new(locks::RwLock::new(Inner {
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

        pub fn inner(&self) -> impl Future<Item = locks::RwLockReadGuard<Inner>, Error = ()> {
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
