use crate::domain::amount::*;
use std::time::SystemTime;

pub struct Volume {
    pub period_start: SystemTime,
    pub volume: NumberWithPrecision,
    pub num_trades: u32,
}

#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use super::*;
    use crate::domain::{
        market::*,
        statistics::{interval::*, trade::*},
    };
    use std::time::SystemTime;

    impl Volume {
        pub fn from_trades(
            history: &TradeHistory,
            market: Option<&'static Market>,
            interval: Option<Interval>,
        ) -> Vec<Volume> {
            let interval = interval.unwrap_or(Interval::Year);
            let from = match history.first_trade_time() {
                None => return Vec::new(),
                Some(time) => time,
            };

            let mut ret = Vec::new();
            let mut trades = history
                .iter()
                .filter(|t| market.map(|m| t.market.pair == m.pair).unwrap_or(true));
            let mut trade = match trades.next() {
                None => return ret,
                Some(next) => next,
            };
            for (period_start, end) in interval.intervals(from, SystemTime::now()) {
                while trade.timestamp < period_start {
                    trade = match trades.next() {
                        None => return ret,
                        Some(next) => next,
                    };
                }
                if trade.timestamp >= end {
                    continue;
                }
                let mut current = Volume {
                    period_start,
                    volume: ZERO,
                    num_trades: 0,
                };
                while trade.timestamp < end {
                    current.volume += trade.volume;
                    current.num_trades += 1;
                    trade = match trades.next() {
                        None => {
                            ret.push(current);
                            return ret;
                        }
                        Some(next) => next,
                    };
                }
                ret.push(current);
            }
            ret
        }
    }
}
