use crate::domain::amount::*;
use std::time::SystemTime;

pub struct Hloc {
    pub period_start: SystemTime,
    pub high: NumberWithPrecision,
    pub low: NumberWithPrecision,
    pub open: NumberWithPrecision,
    pub close: NumberWithPrecision,
    pub volume_left: NumberWithPrecision,
    pub volume_right: NumberWithPrecision,
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
    use chrono::*;
    use lazy_static::lazy_static;
    use std::{
        ops::Add,
        time::{SystemTime, UNIX_EPOCH},
    };

    pub struct HlocQuery {
        pub market: &'static Market,
        pub timestamp_from: Option<SystemTime>,
        pub timestamp_to: Option<SystemTime>,
        pub interval: Option<Interval>,
    }

    impl Hloc {
        pub fn from_trades(
            history: &TradeHistory,
            HlocQuery {
                market,
                timestamp_from,
                timestamp_to,
                interval,
            }: HlocQuery,
        ) -> Vec<Hloc> {
            let from = match history.first_trade_time() {
                None => return Vec::new(),
                Some(time) => time.max(timestamp_from.unwrap_or(UNIX_EPOCH)),
            };
            let to = from.max(timestamp_to.unwrap_or_else(SystemTime::now));
            let interval = interval.unwrap_or_else(|| Interval::from_range(&from, &to));

            let mut ret = Vec::new();
            let mut trades = history.iter().filter(|t| t.market.pair == market.pair);
            let mut trade = match trades.next() {
                None => return ret,
                Some(next) => next,
            };
            for (period_start, end) in interval.intervals(from, to) {
                while trade.timestamp < period_start {
                    trade = match trades.next() {
                        None => return ret,
                        Some(next) => next,
                    };
                }
                if trade.timestamp >= end {
                    continue;
                }

                let mut current = Hloc {
                    period_start,
                    high: trade.price,
                    low: trade.price,
                    open: trade.price,
                    close: trade.price,
                    volume_left: NumberWithPrecision::new(0, 4),
                    volume_right: NumberWithPrecision::new(0, 4),
                };
                while trade.timestamp < end {
                    current.high = current.high.max(trade.price);
                    current.low = current.low.min(trade.price);
                    current.close = trade.price;
                    current.volume_left += trade.amount;
                    current.volume_right += trade.volume;
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
