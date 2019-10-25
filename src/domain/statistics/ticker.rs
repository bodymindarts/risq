use crate::domain::{amount::*, market::Market};

#[derive(Clone)]
pub struct Ticker {
    pub market: &'static Market,
    pub last: Option<NumberWithPrecision>,
    pub high: Option<NumberWithPrecision>,
    pub low: Option<NumberWithPrecision>,
    pub volume_left: NumberWithPrecision,
    pub volume_right: NumberWithPrecision,
}

#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use super::Ticker;
    use crate::domain::{amount::*, market::*, statistics::trade::*};
    use std::{
        collections::HashMap,
        time::{Duration, SystemTime},
    };

    impl Ticker {
        pub fn from_trades(history: &TradeHistory, market: Option<&'static Market>) -> Vec<Ticker> {
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
                    return Self::to_return(tickers);
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
                        return Self::to_return(tickers);
                    }
                    Some(next) => next,
                };
            }

            Self::to_return(tickers)
        }
        fn to_return(tickers: HashMap<&String, Ticker>) -> Vec<Ticker> {
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
            }
        }
    }
}
