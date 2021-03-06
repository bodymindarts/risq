mod btc_offer;

use crate::{
    bisq::SequencedMessageHash,
    domain::{
        currency::{self, Currency},
        market::{self, Market},
        offer::{message::GetOpenOffers, OfferBook, OfferDirection, OpenOffer},
        statistics::*,
    },
    p2p::{BootstrapState, Status},
    prelude::*,
};
use actix_web::{web, Error, HttpResponse};
use btc_offer::BtcOffer;
use chrono::{DateTime, TimeZone, Utc};
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
    time::{Duration, UNIX_EPOCH},
};

pub fn graphql(
    schema: web::Data<Arc<Schema>>,
    context: web::Data<GraphQLContextWrapper>,
    status: web::Data<Status>,
    request: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match status.bootstrap_state() {
        BootstrapState::Bootstrapped => (),
        state => {
            return future::Either::A(future::ok(
                HttpResponse::ServiceUnavailable().body(state.to_string()),
            ))
        }
    }
    future::Either::B(
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
            }),
    )
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

const ALL_MARKETS: &str = "all";

pub struct Offers {
    market: MarketPair,
    offers: Vec<OpenOffer>,
}

impl Offers {
    fn direction(&self, direction: OfferDirection) -> impl DoubleEndedIterator<Item = &OpenOffer> {
        self.offers.iter().filter(move |o| o.direction == direction)
    }

    fn btc_direction(
        &self,
        direction: OfferDirection,
    ) -> impl DoubleEndedIterator<Item = &OpenOffer> {
        self.offers
            .iter()
            .filter(move |o| BtcOffer::new(o).direction() == direction)
    }
}

pub struct Query;
impl QueryFields for Query {
    fn field_offers(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Offers, juniper_from_schema::Walked>,
        market: Option<MarketPair>,
        direction: Option<Direction>,
    ) -> FieldResult<Offers> {
        let market_cmp = market
            .as_ref()
            .map(|MarketPair(m)| m.as_ref())
            .unwrap_or(ALL_MARKETS);
        let direction = direction.map(OfferDirection::from);
        let mut offers: Vec<OpenOffer> = executor
            .context()
            .open_offers
            .values()
            .filter(|o| market_cmp == ALL_MARKETS || o.market.pair == market_cmp)
            .filter(|o| !o.is_expired())
            .filter(|o| direction.is_none() || o.direction == direction.unwrap())
            .cloned()
            .collect();
        offers.sort_unstable_by(|a, b| a.display_price.cmp(&b.display_price));
        Ok(Offers {
            market: market.unwrap_or_else(|| MarketPair(ALL_MARKETS.to_string())),
            offers,
        })
    }

    fn field_markets(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Market, juniper_from_schema::Walked>,
    ) -> FieldResult<&Vec<Market>> {
        Ok(&market::ALL)
    }

    fn field_currencies(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Currency, juniper_from_schema::Walked>,
    ) -> FieldResult<&Vec<Currency>> {
        Ok(&currency::ALL)
    }

    #[cfg(not(feature = "statistics"))]
    fn field_ticker(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Ticker, juniper_from_schema::Walked>,
        _market: Option<MarketPair>,
    ) -> FieldResult<Option<Vec<Ticker>>> {
        Ok(None)
    }
    #[cfg(feature = "statistics")]
    fn field_ticker(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Ticker, juniper_from_schema::Walked>,
        market: Option<MarketPair>,
    ) -> FieldResult<Option<Vec<Ticker>>> {
        let context = executor.context();
        let stats = &context.stats_cache;
        Ok(Some(stats.ticker(
            market.and_then(|m| Market::from_pair(&m)),
            context.open_offers.values(),
        )))
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
        use either::*;
        use std::{convert::TryInto, time::SystemTime};

        let stats = &executor.context().stats_cache;
        let market = market
            .as_ref()
            .map(|MarketPair(m)| m.as_ref())
            .unwrap_or(ALL_MARKETS);
        let direction = direction.map(OfferDirection::from);
        let timestamp_from = timestamp_from
            .and_then(|t| t.try_into().ok())
            .unwrap_or(UNIX_EPOCH);
        let timestamp_to = timestamp_to
            .and_then(|t| t.try_into().ok())
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
        _timestamp_from: Option<UnixSecs>,
        _timestamp_to: Option<UnixSecs>,
        _interval: Option<Interval>,
    ) -> FieldResult<Option<Vec<Hloc>>> {
        Ok(None)
    }
    #[cfg(feature = "statistics")]
    fn field_hloc(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Hloc, juniper_from_schema::Walked>,
        MarketPair(market): MarketPair,
        timestamp_from: Option<UnixSecs>,
        timestamp_to: Option<UnixSecs>,
        interval: Option<Interval>,
    ) -> FieldResult<Option<Vec<Hloc>>> {
        use std::convert::TryInto;

        let stats = &executor.context().stats_cache;
        Ok(Some(
            stats.hloc(HlocQuery {
                market: Market::from_pair(&market)
                    .ok_or_else(|| format!("MarketPair '{}' does not exist", market))?,
                timestamp_from: timestamp_from.and_then(|t| t.try_into().ok()),
                timestamp_to: timestamp_to.and_then(|t| t.try_into().ok()),
                interval: interval.map(interval::Interval::from),
            }),
        ))
    }

    #[cfg(not(feature = "statistics"))]
    fn field_volumes(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Volume, juniper_from_schema::Walked>,
        _market: Option<MarketPair>,
        _interval: Option<Interval>,
    ) -> FieldResult<Option<Vec<Volume>>> {
        Ok(None)
    }
    #[cfg(feature = "statistics")]
    fn field_volumes(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, Volume, juniper_from_schema::Walked>,
        market: Option<MarketPair>,
        interval: Option<Interval>,
    ) -> FieldResult<Option<Vec<Volume>>> {
        let stats = &executor.context().stats_cache;
        Ok(Some(stats.volumes(
            market.and_then(|m| Market::from_pair(&m)),
            interval.map(interval::Interval::from),
        )))
    }
}

const TARGET_PRECISION: u32 = 8;

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
    fn field_period_start_date_time(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<DateTime<Utc>> {
        let secs = self
            .period_start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Ok(Utc.timestamp(secs as i64, 0))
    }
    fn field_formatted_high(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.high.format(TARGET_PRECISION))
    }
    fn field_formatted_low(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.low.format(TARGET_PRECISION))
    }
    fn field_formatted_open(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.open.format(TARGET_PRECISION))
    }
    fn field_formatted_close(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.close.format(TARGET_PRECISION))
    }
    fn field_formatted_volume_left(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.volume_left.format(TARGET_PRECISION))
    }
    fn field_formatted_volume_right(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.volume_right.format(TARGET_PRECISION))
    }
    fn field_formatted_avg(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok((self.volume_right / self.volume_left).format(TARGET_PRECISION))
    }
}

impl VolumeFields for Volume {
    fn field_period_start(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<UnixSecs> {
        Ok(self.period_start.into())
    }
    fn field_period_start_date_time(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<DateTime<Utc>> {
        let secs = self
            .period_start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Ok(Utc.timestamp(secs as i64, 0))
    }
    fn field_formatted_volume(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.volume.format(TARGET_PRECISION))
    }
    fn field_num_trades(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<i32> {
        Ok(self.num_trades as i32)
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

    fn field_btc_direction(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Direction> {
        Ok(BtcOffer::new(self).direction().into())
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

    fn field_formatted_btc_amount(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        let btc_offer = BtcOffer::new(self);
        Ok(btc_offer.amount().format(TARGET_PRECISION))
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
        let display_volume = self.display_price * self.amount.total;
        Ok(display_volume.format(TARGET_PRECISION))
    }

    fn field_formatted_btc_volume(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        let btc_offer = BtcOffer::new(self);
        Ok(btc_offer.volume().format(TARGET_PRECISION))
    }
}

impl TickerFields for Ticker {
    fn field_market_pair(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<MarketPair> {
        Ok(MarketPair(self.market.pair.clone()))
    }
    fn field_formatted_last(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Option<String>> {
        Ok(self.last.map(|l| l.format(TARGET_PRECISION)))
    }
    fn field_formatted_high(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Option<String>> {
        Ok(self.high.map(|h| h.format(TARGET_PRECISION)))
    }
    fn field_formatted_low(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Option<String>> {
        Ok(self.low.map(|l| l.format(TARGET_PRECISION)))
    }
    fn field_formatted_volume_left(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.volume_left.format(TARGET_PRECISION))
    }
    fn field_formatted_volume_right(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<String> {
        Ok(self.volume_right.format(TARGET_PRECISION))
    }
    fn field_formatted_buy(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Option<String>> {
        Ok(self.buy.map(|n| n.format(TARGET_PRECISION)))
    }
    fn field_formatted_sell(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Option<String>> {
        Ok(self.sell.map(|n| n.format(TARGET_PRECISION)))
    }
}

impl OffersFields for Offers {
    fn field_market_pair(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<&MarketPair> {
        Ok(&self.market)
    }

    fn field_buys(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, OpenOffer, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<&OpenOffer>> {
        Ok(self.direction(OfferDirection::Buy).rev().collect())
    }

    fn field_sells(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, OpenOffer, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<&OpenOffer>> {
        Ok(self.direction(OfferDirection::Sell).collect())
    }

    fn field_btc_buys(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, OpenOffer, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<&OpenOffer>> {
        Ok(self.btc_direction(OfferDirection::Buy).rev().collect())
    }

    fn field_btc_sells(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
        _trail: &QueryTrail<'_, OpenOffer, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<&OpenOffer>> {
        Ok(self.btc_direction(OfferDirection::Sell).collect())
    }

    fn field_formatted_buy_prices(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Vec<String>> {
        Ok(self
            .direction(OfferDirection::Buy)
            .map(|o| o.display_price.format(TARGET_PRECISION))
            .collect())
    }

    fn field_formatted_sell_prices(
        &self,
        _executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Vec<String>> {
        Ok(self
            .direction(OfferDirection::Sell)
            .map(|o| o.display_price.format(TARGET_PRECISION))
            .collect())
    }
}

mod convert {
    use super::*;
    use crate::domain::offer::OfferDirection;
    use std::{convert::TryFrom, time::SystemTime};

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
    impl TryFrom<UnixSecs> for SystemTime {
        type Error = std::num::ParseIntError;
        fn try_from(secs: UnixSecs) -> Result<Self, Self::Error> {
            secs.parse::<u64>()
                .map(|secs| UNIX_EPOCH + Duration::from_secs(secs))
        }
    }

    #[cfg(feature = "statistics")]
    impl From<Interval> for interval::Interval {
        fn from(interval: Interval) -> Self {
            match interval {
                Interval::Minute => interval::Interval::Minute,
                Interval::Halfhour => interval::Interval::HalfHour,
                Interval::Hour => interval::Interval::Hour,
                Interval::Halfday => interval::Interval::HalfDay,
                Interval::Day => interval::Interval::Day,
                Interval::Week => interval::Interval::Week,
                Interval::Month => interval::Interval::Month,
                Interval::Year => interval::Interval::Year,
            }
        }
    }
}
