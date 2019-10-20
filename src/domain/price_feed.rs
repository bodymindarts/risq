use super::{amount::NumberWithPrecision, currency::*};
use crate::prelude::*;
use lazy_static::lazy_static;
use reqwest::r#async::Client;
use serde::{self, Deserialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

lazy_static! {
    static ref LOOP_INTERVAL: Duration = Duration::from_secs(60);
}
const INVALID: &str = "INVALID";

pub struct PriceFeed {
    client: Client,
    price_data: Arc<HashMap<&'static str, PriceData>>,
}
impl Actor for PriceFeed {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.update_prices(ctx);
        ctx.run_interval(*LOOP_INTERVAL, |feed, ctx| feed.update_prices(ctx));
    }
}
impl PriceFeed {
    pub fn start() -> Addr<PriceFeed> {
        PriceFeed {
            client: Client::new(),
            price_data: Arc::new(HashMap::new()),
        }
        .start()
    }
    fn update_prices(&self, ctx: &mut Context<Self>) {
        ctx.spawn(
            fut::wrap_future(
                self.client
                    .get("http://174.138.104.137:8080/getAllMarketPrices")
                    .send()
                    .map_err(|e| {
                        error!("error getting price {:?}", e);
                        e
                    })
                    .and_then(|mut response| response.json()),
            )
            .map(
                |response: GetAllMarketPricesResponse, feed: &mut PriceFeed, _| {
                    let new_data = response
                        .data
                        .into_iter()
                        .filter_map(|d| {
                            if d.provider == INVALID {
                                None
                            } else {
                                Some((d.currency.code.as_ref(), d))
                            }
                        })
                        .collect();
                    feed.price_data = Arc::new(new_data);
                },
            )
            .map_err(|e, _, _| error!("deserializing price: {:?}", e)),
        );
    }
}

pub struct GetCurrentPrices;
impl Message for GetCurrentPrices {
    type Result = Arc<HashMap<&'static str, PriceData>>;
}
impl Handler<GetCurrentPrices> for PriceFeed {
    type Result = MessageResult<GetCurrentPrices>;
    fn handle(&mut self, _: GetCurrentPrices, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(Arc::clone(&self.price_data))
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(from = "PriceDataRaw")]
pub struct PriceData {
    pub currency: &'static Currency,
    pub price: f64,
    timestamp: SystemTime,
    provider: String,
}

#[derive(Deserialize)]
struct GetAllMarketPricesResponse {
    data: Vec<PriceData>,
}
const PRICE_NODE_FIAT_DECIMALS: u32 = 2;
const PRICE_NODE_CRYPTO_DECIMALS: u32 = 8;

impl From<PriceDataRaw> for PriceData {
    fn from(
        PriceDataRaw {
            currency_code,
            price,
            timestamp_sec,
            provider,
        }: PriceDataRaw,
    ) -> Self {
        let currency = match Currency::from_code(&currency_code) {
            Some(currency) => currency,
            _ => {
                return Self {
                    currency: Currency::from_code("EUR").unwrap(),
                    price: 0.0,
                    timestamp: UNIX_EPOCH,
                    provider: INVALID.to_string(),
                }
            }
        };
        Self {
            currency,
            price,
            timestamp: UNIX_EPOCH + Duration::from_secs(timestamp_sec),
            provider,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PriceDataRaw {
    currency_code: String,
    price: f64,
    timestamp_sec: u64,
    provider: String,
}
