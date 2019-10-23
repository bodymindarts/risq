use crate::{
    bisq::SequencedMessageHash,
    domain::{
        currency::{self, Currency},
        market::{self, Market},
        offer::{message::GetOpenOffers, OfferBook, OfferDirection, OpenOffer},
        statistics::*,
    },
    prelude::*,
};
use actix_web::{web, Error, HttpResponse};
use either::*;
use juniper::{
    self,
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, FieldResult,
};
use juniper_from_schema::graphql_schema_from_file;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn graphql(
    schema: web::Data<Arc<Schema>>,
    context: web::Data<GraphQLContextWrapper>,
    request: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    context
        .get()
        .and_then(|context| {
            web::block(move || {
                let res = request.execute(&schema, &context);
                Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
            })
            .map_err(Error::from)
        })
        .map(|result| {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(result)
        })
}
pub fn graphiql(port: web::Data<u16>) -> HttpResponse {
    let html = graphiql_source(&format!("http://localhost:{}/graphql", port.to_string()));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[derive(Clone)]
pub struct GraphQLContextWrapper {
    #[cfg(feature = "statistics")]
    pub stats_cache: StatsCache,
    pub offer_book: Addr<OfferBook>,
}
impl GraphQLContextWrapper {
    #[cfg(feature = "statistics")]
    pub fn get(&self) -> impl Future<Item = GraphQLContext, Error = Error> {
        Future::join(
            self.stats_cache.inner().map_err(Error::from),
            self.offer_book.send(GetOpenOffers).map_err(Error::from),
        )
        .map(|(stats_cache, open_offers)| GraphQLContext {
            stats_cache,
            open_offers,
        })
    }
    #[cfg(not(feature = "statistics"))]
    pub fn get(&self) -> impl Future<Item = GraphQLContext, Error = Error> {
        self.offer_book
            .send(GetOpenOffers)
            .map_err(Error::from)
            .map(|open_offers| GraphQLContext { open_offers })
    }
}
pub struct GraphQLContext {
    #[cfg(feature = "statistics")]
    stats_cache: locks::RwLockReadGuard<StatsCacheInner>,
    open_offers: Arc<HashMap<SequencedMessageHash, OpenOffer>>,
}
impl juniper::Context for GraphQLContext {}

graphql_schema_from_file!("src/api/schema.graphql", context_type: GraphQLContext);

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}

const ALL_MARKETS: &'static str = "all";

pub struct Query;
impl QueryFields for Query {
    #[cfg(feature = "statistics")]
    fn field_trades(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Trade, juniper_from_schema::Walked>,
        market: Option<MarketPair>,
        direction: Option<Direction>,
        timestamp_from: Option<UnixSecs>,
        timestamp_to: Option<UnixSecs>,
        limit: i32,
        sort: Sort,
    ) -> FieldResult<Option<Vec<Trade>>> {
        let stats = &executor.context().stats_cache;
        let market = market
            .as_ref()
            .map(|MarketPair(m)| m.as_ref())
            .unwrap_or_else(|| ALL_MARKETS);
        let direction = direction.map(OfferDirection::from);
        let timestamp_from = timestamp_from
            .and_then(|t| t.parse::<u64>().ok())
            .map(|secs| UNIX_EPOCH + Duration::from_secs(secs))
            .unwrap_or(UNIX_EPOCH);
        let timestamp_to = timestamp_to
            .and_then(|t| t.parse::<u64>().ok())
            .map(|secs| UNIX_EPOCH + Duration::from_secs(secs))
            .unwrap_or_else(SystemTime::now);
        let iter = stats
            .trades()
            .filter(|t| t.timestamp >= timestamp_from && t.timestamp < timestamp_to)
            .filter(|t| market == ALL_MARKETS || t.market.pair == market)
            .filter(|t| direction.is_none() || t.direction == direction.unwrap());
        let iter = if let Sort::Desc = sort {
            Left(iter.rev())
        } else {
            Right(iter)
        };
        Ok(Some(
            iter.take(usize::min(limit as usize, 2000))
                .cloned()
                .collect(),
        ))
    }
    #[cfg(not(feature = "statistics"))]
    fn field_hloc(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Hloc, juniper_from_schema::Walked>,
        _market: MarketPair,
    ) -> FieldResult<Option<Vec<Hloc>>> {
        Ok(None)
    }
    #[cfg(not(feature = "statistics"))]
    fn field_trades(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Trade, juniper_from_schema::Walked>,
        _market: Option<MarketPair>,
        _direction: Option<Direction>,
        _timestamp_from: Option<UnixSecs>,
        _timestamp_to: Option<UnixSecs>,
        _limit: i32,
        _sort: Sort,
    ) -> FieldResult<Option<Vec<Trade>>> {
        Ok(None)
    }

    fn field_currencies(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Currency, juniper_from_schema::Walked>,
    ) -> FieldResult<&Vec<Currency>> {
        Ok(&currency::ALL)
    }

    fn field_markets(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Market, juniper_from_schema::Walked>,
    ) -> FieldResult<&Vec<Market>> {
        Ok(&market::ALL)
    }

    fn field_offers(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, OpenOffer, juniper_from_schema::Walked>,
        market: Option<MarketPair>,
        direction: Option<Direction>,
    ) -> FieldResult<Vec<OpenOffer>> {
        let direction = direction.map(OfferDirection::from);
        let market = market
            .as_ref()
            .map(|MarketPair(m)| m.as_ref())
            .unwrap_or_else(|| ALL_MARKETS);
        Ok(executor
            .context()
            .open_offers
            .values()
            .filter(|o| &o.market.pair == market || market == ALL_MARKETS)
            .filter(|o| !o.is_expired())
            .filter(|t| direction.is_none() || t.direction == direction.unwrap())
            .cloned()
            .collect())
    }
}

const TARGET_PRECISION: u32 = 8;

impl From<OfferDirection> for Direction {
    fn from(direction: OfferDirection) -> Direction {
        match direction {
            OfferDirection::Buy => Direction::Buy,
            OfferDirection::Sell => Direction::Sell,
        }
    }
}
impl From<Direction> for OfferDirection {
    fn from(direction: Direction) -> OfferDirection {
        match direction {
            Direction::Buy => OfferDirection::Buy,
            Direction::Sell => OfferDirection::Sell,
        }
    }
}
impl TradeFields for Trade {
    fn field_market_pair(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<MarketPair> {
        Ok(MarketPair(self.market.pair.clone()))
    }
    fn field_direction(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Direction> {
        Ok(self.direction.into())
    }
    fn field_offer_id(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.offer_id.clone()))
    }

    fn field_payment_method_id(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.payment_method_id)
    }

    fn field_formatted_price(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.price.format(TARGET_PRECISION))
    }

    fn field_formatted_amount(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.amount.format(TARGET_PRECISION))
    }

    fn field_formatted_volume(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.volume.format(TARGET_PRECISION))
    }

    fn field_trade_date(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<UnixMillis> {
        Ok(self.timestamp.into())
    }
}

impl HlocFields for Hloc {
    fn field_period_start(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<UnixSecs> {
        Ok(self.period_start.into())
    }
}

impl From<SystemTime> for UnixMillis {
    fn from(time: SystemTime) -> Self {
        UnixMillis(
            time.duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis()
                .to_string(),
        )
    }
}
impl From<SystemTime> for UnixSecs {
    fn from(time: SystemTime) -> Self {
        UnixSecs(
            time.duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                .to_string(),
        )
    }
}

lazy_static! {
    static ref FIAT_LOWER: String = "fiat".to_string();
    static ref CRYPTO_LOWER: String = "crypto".to_string();
}

impl CurrencyFields for Currency {
    fn field_code(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.code)
    }

    fn field_name(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_precision(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<i32> {
        Ok(TARGET_PRECISION as i32)
    }

    fn field_currency_type_lower_case(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(self.currency_type.to_lowercase())
    }
}

impl MarketFields for Market {
    fn field_pair(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.pair)
    }

    fn field_name(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_l_name(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.left.name)
    }

    fn field_r_name(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.right.name)
    }

    fn field_l_symbol(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.left.code)
    }

    fn field_r_symbol(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.right.code)
    }

    fn field_l_precision(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<i32> {
        Ok(TARGET_PRECISION as i32)
    }

    fn field_r_precision(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<i32> {
        Ok(TARGET_PRECISION as i32)
    }

    fn field_l_type_lower_case(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(self.left.currency_type.to_lowercase())
    }

    fn field_r_type_lower_case(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(self.right.currency_type.to_lowercase())
    }
}

impl OpenOfferFields for OpenOffer {
    fn field_market_pair(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<MarketPair> {
        Ok(MarketPair::new(self.market.pair.clone()))
    }
    fn field_id(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.clone()))
    }
    fn field_direction(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Direction> {
        Ok(self.direction.into())
    }
    fn field_payment_method_id(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.payment_method_id)
    }
    fn field_offer_fee_tx_id(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&String> {
        Ok(&self.offer_fee_tx_id)
    }
    fn field_offer_date(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<UnixMillis> {
        Ok(self.created_at.into())
    }
    fn field_formatted_amount(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.amount.total.format(TARGET_PRECISION))
    }
    fn field_formatted_min_amount(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.amount.min.format(TARGET_PRECISION))
    }
    fn field_formatted_price(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.display_price.format(TARGET_PRECISION))
    }
    fn field_formatted_volume(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.display_volume.format(TARGET_PRECISION))
    }
}
