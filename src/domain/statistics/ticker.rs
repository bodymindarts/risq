use crate::domain::{amount::*, market::Market};

#[derive(Clone)]
pub struct Ticker {
    pub market: &'static Market,
    pub last: Option<NumberWithPrecision>,
    pub high: Option<NumberWithPrecision>,
    pub low: Option<NumberWithPrecision>,
    pub volume_left: NumberWithPrecision,
    pub volume_right: NumberWithPrecision,
    pub buy: Option<NumberWithPrecision>,
    pub sell: Option<NumberWithPrecision>,
}

#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use super::Ticker;
    use crate::domain::{
        amount::*,
        market::*,
        offer::{OfferDirection, OpenOffer},
        statistics::trade::*,
    };
    use std::{
        collections::HashMap,
        time::{Duration, SystemTime},
    };

    impl Ticker {
        pub fn from_trades<'a>(
            history: &TradeHistory,
            market: Option<&'static Market>,
            offers: impl Iterator<Item = &'a OpenOffer>,
        ) -> Vec<Ticker> {
            let mut tickers = HashMap::new();
            match market {
                None => {
                    for market in ALL.iter() {
                        tickers.insert(&market.pair, Self::empty(market));
                    }
                }
                Some(market) => {
                    tickers.insert(&market.pair, Self::empty(market));
                }
            }
            let mut trades = history.iter().rev();
            let mut next = match trades.next() {
                None => {
                    return Self::to_return(tickers, offers);
                }
                Some(next) => next,
            };

            let earliest = SystemTime::now() - Duration::from_secs(24 * 60 * 60);
            while next.timestamp >= earliest {
                if let Some(mut ticker) = tickers.get_mut(&next.market.pair) {
                    ticker.last = Some(ticker.last.unwrap_or(next.price));
                    ticker.high =
                        Some(ticker.high.map(|h| h.max(next.price)).unwrap_or(next.price));
                    ticker.low = Some(ticker.low.map(|l| l.min(next.price)).unwrap_or(next.price));
                    ticker.volume_left += next.amount;
                    ticker.volume_right += next.volume;
                }
                next = match trades.next() {
                    None => {
                        return Self::to_return(tickers, offers);
                    }
                    Some(next) => next,
                };
            }
            let mut missing_markets: HashMap<&String, &mut Ticker> = tickers
                .iter_mut()
                .filter_map(|(market, ticker)| {
                    if ticker.last.is_none() {
                        Some((*market, ticker))
                    } else {
                        None
                    }
                })
                .collect();
            loop {
                if missing_markets.len() == 0 {
                    return Self::to_return(tickers, offers);
                }
                if let Some(ticker) = missing_markets.remove(&next.market.pair) {
                    ticker.last = Some(next.price);
                    ticker.high = Some(next.price);
                    ticker.low = Some(next.price);
                }

                next = match trades.next() {
                    None => {
                        return Self::to_return(tickers, offers);
                    }
                    Some(next) => next,
                };
            }
        }
        fn to_return<'a>(
            mut tickers: HashMap<&String, Ticker>,
            offers: impl Iterator<Item = &'a OpenOffer>,
        ) -> Vec<Ticker> {
            for offer in offers {
                if let Some(mut ticker) = tickers.get_mut(&offer.market.pair) {
                    match (offer.direction, ticker.buy, ticker.sell) {
                        (OfferDirection::Buy, _, None) => ticker.sell = Some(offer.display_price),
                        (OfferDirection::Sell, None, _) => ticker.buy = Some(offer.display_price),
                        (OfferDirection::Buy, _, Some(sell)) => {
                            ticker.sell = Some(sell.max(offer.display_price))
                        }
                        (OfferDirection::Sell, Some(buy), _) => {
                            ticker.buy = Some(buy.min(offer.display_price))
                        }
                    }
                }
            }
            let mut ret: Vec<Ticker> = tickers.into_iter().map(|(_, ticker)| ticker).collect();
            ret.sort_unstable_by(|a, b| a.market.pair.cmp(&b.market.pair));
            ret
        }
        fn empty(market: &'static Market) -> Self {
            Self {
                market,
                last: None,
                high: None,
                low: None,
                volume_left: ZERO,
                volume_right: ZERO,
                buy: None,
                sell: None,
            }
        }
    }
}
