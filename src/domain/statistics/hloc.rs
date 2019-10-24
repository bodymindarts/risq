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
    use crate::domain::{market::*, statistics::trade::*};
    use chrono::*;
    use lazy_static::lazy_static;
    use std::{
        ops::Add,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum HlocInterval {
        Minute,
        HalfHour,
        Hour,
        HalfDay,
        Day,
        Week,
        Month,
        Year,
    }

    lazy_static! {
        static ref ONE_MINUTE: chrono::Duration = chrono::Duration::minutes(1);
        static ref THIRTY_MINUTES: chrono::Duration = chrono::Duration::minutes(30);
        static ref ONE_HOUR: chrono::Duration = chrono::Duration::hours(1);
        static ref TWELVE_HOURS: chrono::Duration = chrono::Duration::hours(12);
        static ref ONE_DAY: chrono::Duration = chrono::Duration::days(1);
        static ref THREE_DAYS: chrono::Duration = chrono::Duration::days(3);
        static ref ONE_WEEK: chrono::Duration = chrono::Duration::weeks(1);
        static ref SIXTY_DAYS: chrono::Duration = chrono::Duration::days(60);
        static ref ONE_YEAR: chrono::Duration = chrono::Duration::days(365);
        static ref FIVE_YEARS: chrono::Duration = chrono::Duration::days(1826);
    }

    struct IntervalIterator {
        current: DateTime<Utc>,
        end: DateTime<Utc>,
        interval: HlocInterval,
    }
    impl Iterator for IntervalIterator {
        type Item = (SystemTime, SystemTime);
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let next = self.current + self.interval;
                let ret = (self.current.into(), next.into());
                self.current = next;
                Some(ret)
            } else {
                None
            }
        }
    }

    impl HlocInterval {
        fn from_range(from: &SystemTime, to: &SystemTime) -> Self {
            if let Ok(dur) = to.duration_since(*from) {
                match Duration::from_std(dur) {
                    Ok(dur) if dur <= *ONE_HOUR => HlocInterval::Minute,
                    Ok(dur) if dur <= *ONE_DAY => HlocInterval::HalfHour,
                    Ok(dur) if dur <= *THREE_DAYS => HlocInterval::Hour,
                    Ok(dur) if dur <= *ONE_WEEK => HlocInterval::HalfDay,
                    Ok(dur) if dur <= *SIXTY_DAYS => HlocInterval::Day,
                    Ok(dur) if dur <= *ONE_YEAR => HlocInterval::Week,
                    Ok(dur) if dur <= *FIVE_YEARS => HlocInterval::Month,
                    _ => HlocInterval::Year,
                }
            } else {
                HlocInterval::Year
            }
        }

        fn intervals(&self, start: SystemTime, end: SystemTime) -> IntervalIterator {
            let start: DateTime<Utc> = self.appropriate_floor(start.into());
            let end: DateTime<Utc> = self.appropriate_floor(end.into()) + *self;
            IntervalIterator {
                current: start,
                end,
                interval: *self,
            }
        }

        fn appropriate_floor(&self, time: DateTime<Utc>) -> DateTime<Utc> {
            let time = time.with_second(0).unwrap();
            let time = match *self {
                HlocInterval::Minute => return time,
                HlocInterval::HalfHour if time.minute() >= 30 => {
                    return time.with_minute(30).unwrap()
                }
                HlocInterval::Hour => return time.with_minute(0).unwrap(),
                _ => time.with_minute(0).unwrap(),
            };
            let time = match *self {
                HlocInterval::HalfDay if time.hour() >= 12 => return time.with_hour(12).unwrap(),
                HlocInterval::Day => return time.with_hour(0).unwrap(),
                _ => time.with_hour(0).unwrap(),
            };
            match *self {
                HlocInterval::Week => {
                    time - Duration::days(time.weekday().num_days_from_monday() as i64)
                }
                HlocInterval::Month => time.with_day0(0).unwrap(),
                HlocInterval::Year => time.with_day0(0).and_then(|t| t.with_month0(0)).unwrap(),
                _ => unreachable!(),
            }
        }
    }

    pub struct HlocQuery {
        pub market: &'static Market,
        pub timestamp_from: Option<SystemTime>,
        pub timestamp_to: Option<SystemTime>,
        pub interval: Option<HlocInterval>,
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
            let interval = interval.unwrap_or_else(|| HlocInterval::from_range(&from, &to));

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

    impl Add<HlocInterval> for DateTime<Utc> {
        type Output = DateTime<Utc>;

        fn add(self, interval: HlocInterval) -> Self {
            match interval {
                HlocInterval::Minute => self + *ONE_MINUTE,
                HlocInterval::HalfHour => self + *THIRTY_MINUTES,
                HlocInterval::Hour => self + *ONE_HOUR,
                HlocInterval::HalfDay => self + *TWELVE_HOURS,
                HlocInterval::Day => self + *ONE_DAY,
                HlocInterval::Week => self + *ONE_WEEK,
                HlocInterval::Month if self.month() != 12 => {
                    self.with_month(self.month() + 1).unwrap()
                }
                HlocInterval::Month => self
                    .with_month0(0)
                    .and_then(|d| d.with_year(self.year() + 1))
                    .unwrap(),
                HlocInterval::Year => self.with_year(self.year() + 1).unwrap(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn appropriate_floor() {
            let date = Utc.ymd(2016, 7, 8).and_hms(13, 45, 11);
            assert!(
                HlocInterval::Minute.appropriate_floor(date)
                    == Utc.ymd(2016, 7, 8).and_hms(13, 45, 0)
            );
            assert!(
                HlocInterval::HalfHour.appropriate_floor(date)
                    == Utc.ymd(2016, 7, 8).and_hms(13, 30, 0)
            );
            assert!(
                HlocInterval::Hour.appropriate_floor(date) == Utc.ymd(2016, 7, 8).and_hms(13, 0, 0)
            );
            assert!(
                HlocInterval::HalfDay.appropriate_floor(date)
                    == Utc.ymd(2016, 7, 8).and_hms(12, 0, 0)
            );
            assert!(
                HlocInterval::Day.appropriate_floor(date) == Utc.ymd(2016, 7, 8).and_hms(0, 0, 0)
            );
            assert!(
                HlocInterval::Week.appropriate_floor(date) == Utc.ymd(2016, 7, 4).and_hms(0, 0, 0)
            );
            assert!(
                HlocInterval::Month.appropriate_floor(date) == Utc.ymd(2016, 7, 1).and_hms(0, 0, 0)
            );
            assert!(
                HlocInterval::Year.appropriate_floor(date) == Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)
            );
        }

        #[test]
        fn interval_addition() {
            let date = Utc.ymd(2016, 7, 8).and_hms(0, 0, 0);
            assert!(date + HlocInterval::Minute == Utc.ymd(2016, 7, 8).and_hms(0, 1, 0));
            assert!(date + HlocInterval::HalfHour == Utc.ymd(2016, 7, 8).and_hms(0, 30, 0));
            assert!(date + HlocInterval::Hour == Utc.ymd(2016, 7, 8).and_hms(1, 0, 0));
            assert!(date + HlocInterval::HalfDay == Utc.ymd(2016, 7, 8).and_hms(12, 0, 0));
            assert!(date + HlocInterval::Day == Utc.ymd(2016, 7, 9).and_hms(0, 0, 0));
            assert!(date + HlocInterval::Week == Utc.ymd(2016, 7, 15).and_hms(0, 0, 0));
            assert!(date + HlocInterval::Month == Utc.ymd(2016, 8, 8).and_hms(0, 0, 0));
            assert!(date + HlocInterval::Year == Utc.ymd(2017, 7, 8).and_hms(0, 0, 0));
        }
    }
}
