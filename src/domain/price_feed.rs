use super::currency::*;
use crate::{bisq::constants, prelude::*};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use reqwest::{r#async::Client, Proxy};
use serde::{self, Deserialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

lazy_static! {
    static ref LOOP_INTERVAL: Duration = Duration::from_secs(30);
}
const INVALID: &str = "INVALID";

pub struct PriceFeed {
    client: Client,
    price_data: Arc<HashMap<&'static str, PriceData>>,
    nodes: Vec<&'static str>,
}
impl Actor for PriceFeed {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.update_prices(ctx);
        ctx.run_interval(*LOOP_INTERVAL, |feed, ctx| feed.update_prices(ctx));
    }
}
impl PriceFeed {
    pub fn start(proxy_port: Option<u16>) -> Addr<PriceFeed> {
        let client = if let Some(proxy_port) = proxy_port {
            Client::builder()
                .proxy(
                    Proxy::http(&format!("socks5h://127.0.0.1:{}", proxy_port.to_string()))
                        .expect("Couldn't set proxy"),
                )
                .build()
                .expect("Couldn't create client")
        } else {
            Client::new()
        };
        PriceFeed {
            client,
            price_data: Arc::new(HashMap::new()),
            nodes: constants::price_nodes(proxy_port.is_some()),
        }
        .start()
    }
    fn update_prices(&mut self, ctx: &mut Context<Self>) {
        let node_index: usize = thread_rng().gen::<usize>() % self.nodes.len();
        let url = format!("{}/getAllMarketPrices", self.nodes[node_index]);
        info!("Updating price feed via {}", url);
        ctx.spawn(
            fut::wrap_future(
                self.client
                    .get(&url)
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
                                let code: &'static str = d.currency.code.as_ref();
                                let d = match feed.price_data.get(code) {
                                    Some(data) if data.timestamp > d.timestamp => data.clone(),
                                    _ => d,
                                };
                                Some((code, d))
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
            timestamp: UNIX_EPOCH + Duration::from_millis(timestamp_sec),
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
