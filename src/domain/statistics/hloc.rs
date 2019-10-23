use std::time::SystemTime;

pub struct Hloc {
    pub period_start: SystemTime,
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
        static ref ONE_MINUTE: Duration = Duration::minutes(1);
        static ref THIRTY_MINUTES: Duration = Duration::minutes(30);
        static ref ONE_HOUR: Duration = Duration::hours(1);
        static ref TWELVE_HOURS: Duration = Duration::hours(12);
        static ref ONE_DAY: Duration = Duration::days(1);
        static ref THREE_DAYS: Duration = Duration::days(3);
        static ref ONE_WEEK: Duration = Duration::weeks(1);
        static ref SIXTY_DAYS: Duration = Duration::days(60);
        static ref ONE_YEAR: Duration = Duration::days(365);
        static ref FIVE_YEARS: Duration = Duration::days(1826);
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
        fn apropriate_floor(&self, time: NaiveDateTime) -> NaiveDateTime {
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
            info!("INTERVAL: {:?}", interval);

            let mut start_of_interval = interval.apropriate_floor(NaiveDateTime::from_timestamp(
                from.duration_since(UNIX_EPOCH)
                    .expect("Hloc time")
                    .as_secs() as i64,
                0,
            ));
            let end_time = interval.apropriate_floor(NaiveDateTime::from_timestamp(
                to.duration_since(UNIX_EPOCH).expect("Hloc time").as_secs() as i64,
                0,
            )) + interval;

            let interval_start_times =
                Hloc::interval_start_times(start_of_interval, end_time, interval);
            interval_start_times
                .into_iter()
                .map(|i| Hloc {
                    period_start: UNIX_EPOCH + std::time::Duration::from_secs(i.timestamp() as u64),
                })
                .collect()
        }

        fn interval_start_times(
            mut start: NaiveDateTime,
            end: NaiveDateTime,
            interval: HlocInterval,
        ) -> Vec<NaiveDateTime> {
            let mut intervals = Vec::new();
            while start < end {
                intervals.push(start);
                start = start + interval;
            }
            intervals
        }
    }

    impl Add<HlocInterval> for NaiveDateTime {
        type Output = NaiveDateTime;

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
            let date = NaiveDate::from_ymd(2016, 7, 8).and_hms(13, 45, 11);
            assert!(
                HlocInterval::Minute.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 7, 8).and_hms(13, 45, 0)
            );
            assert!(
                HlocInterval::HalfHour.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 7, 8).and_hms(13, 30, 0)
            );
            assert!(
                HlocInterval::Hour.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 7, 8).and_hms(13, 0, 0)
            );
            assert!(
                HlocInterval::HalfDay.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 7, 8).and_hms(12, 0, 0)
            );
            assert!(
                HlocInterval::Day.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0)
            );
            assert!(
                HlocInterval::Week.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 7, 4).and_hms(0, 0, 0)
            );
            assert!(
                HlocInterval::Month.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 7, 1).and_hms(0, 0, 0)
            );
            assert!(
                HlocInterval::Year.apropriate_floor(date)
                    == NaiveDate::from_ymd(2016, 1, 1).and_hms(0, 0, 0)
            );
        }

        #[test]
        fn interval_addition() {
            let date = NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0);
            assert!(
                date + HlocInterval::Minute == NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 1, 0)
            );
            assert!(
                date + HlocInterval::HalfHour == NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 30, 0)
            );
            assert!(date + HlocInterval::Hour == NaiveDate::from_ymd(2016, 7, 8).and_hms(1, 0, 0));
            assert!(
                date + HlocInterval::HalfDay == NaiveDate::from_ymd(2016, 7, 8).and_hms(12, 0, 0)
            );
            assert!(date + HlocInterval::Day == NaiveDate::from_ymd(2016, 7, 9).and_hms(0, 0, 0));
            assert!(date + HlocInterval::Week == NaiveDate::from_ymd(2016, 7, 15).and_hms(0, 0, 0));
            assert!(date + HlocInterval::Month == NaiveDate::from_ymd(2016, 8, 8).and_hms(0, 0, 0));
            assert!(date + HlocInterval::Year == NaiveDate::from_ymd(2017, 7, 8).and_hms(0, 0, 0));
        }

        #[test]
        fn interval_start_times() {
            let date = NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0);
            let intervals =
                Hloc::interval_start_times(date, date + Duration::days(1), HlocInterval::Day);
            assert!(intervals.len() == 1);
            assert!(intervals[0] == date);
            let intervals =
                Hloc::interval_start_times(date, date + Duration::weeks(3), HlocInterval::Week);
            assert!(intervals.len() == 3);
            assert!(intervals[0] == date);
            assert!(intervals[1] == date + Duration::weeks(1));
            assert!(intervals[2] == date + Duration::weeks(2));
        }
    }
}
