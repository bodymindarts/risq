use chrono::*;
use lazy_static::lazy_static;
use std::{ops::Add, time::SystemTime};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Interval {
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

pub struct IntervalIterator {
    current: DateTime<Utc>,
    end: DateTime<Utc>,
    interval: Interval,
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

impl Interval {
    pub fn from_range(from: &SystemTime, to: &SystemTime) -> Self {
        if let Ok(dur) = to.duration_since(*from) {
            match Duration::from_std(dur) {
                Ok(dur) if dur <= *ONE_HOUR => Interval::Minute,
                Ok(dur) if dur <= *ONE_DAY => Interval::HalfHour,
                Ok(dur) if dur <= *THREE_DAYS => Interval::Hour,
                Ok(dur) if dur <= *ONE_WEEK => Interval::HalfDay,
                Ok(dur) if dur <= *SIXTY_DAYS => Interval::Day,
                Ok(dur) if dur <= *ONE_YEAR => Interval::Week,
                Ok(dur) if dur <= *FIVE_YEARS => Interval::Month,
                _ => Interval::Year,
            }
        } else {
            Interval::Year
        }
    }

    pub fn intervals(&self, start: SystemTime, end: SystemTime) -> IntervalIterator {
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
            Interval::Minute => return time,
            Interval::HalfHour if time.minute() >= 30 => return time.with_minute(30).unwrap(),
            Interval::Hour => return time.with_minute(0).unwrap(),
            _ => time.with_minute(0).unwrap(),
        };
        let time = match *self {
            Interval::HalfDay if time.hour() >= 12 => return time.with_hour(12).unwrap(),
            Interval::Day => return time.with_hour(0).unwrap(),
            _ => time.with_hour(0).unwrap(),
        };
        match *self {
            Interval::Week => time - Duration::days(time.weekday().num_days_from_monday() as i64),
            Interval::Month => time.with_day0(0).unwrap(),
            Interval::Year => time.with_day0(0).and_then(|t| t.with_month0(0)).unwrap(),
            _ => unreachable!(),
        }
    }
}

impl Add<Interval> for DateTime<Utc> {
    type Output = DateTime<Utc>;

    fn add(self, interval: Interval) -> Self {
        match interval {
            Interval::Minute => self + *ONE_MINUTE,
            Interval::HalfHour => self + *THIRTY_MINUTES,
            Interval::Hour => self + *ONE_HOUR,
            Interval::HalfDay => self + *TWELVE_HOURS,
            Interval::Day => self + *ONE_DAY,
            Interval::Week => self + *ONE_WEEK,
            Interval::Month if self.month() != 12 => self.with_month(self.month() + 1).unwrap(),
            Interval::Month => self
                .with_month0(0)
                .and_then(|d| d.with_year(self.year() + 1))
                .unwrap(),
            Interval::Year => self.with_year(self.year() + 1).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn appropriate_floor() {
        let date = Utc.ymd(2016, 7, 8).and_hms(13, 45, 11);
        assert!(Interval::Minute.appropriate_floor(date) == Utc.ymd(2016, 7, 8).and_hms(13, 45, 0));
        assert!(
            Interval::HalfHour.appropriate_floor(date) == Utc.ymd(2016, 7, 8).and_hms(13, 30, 0)
        );
        assert!(Interval::Hour.appropriate_floor(date) == Utc.ymd(2016, 7, 8).and_hms(13, 0, 0));
        assert!(Interval::HalfDay.appropriate_floor(date) == Utc.ymd(2016, 7, 8).and_hms(12, 0, 0));
        assert!(Interval::Day.appropriate_floor(date) == Utc.ymd(2016, 7, 8).and_hms(0, 0, 0));
        assert!(Interval::Week.appropriate_floor(date) == Utc.ymd(2016, 7, 4).and_hms(0, 0, 0));
        assert!(Interval::Month.appropriate_floor(date) == Utc.ymd(2016, 7, 1).and_hms(0, 0, 0));
        assert!(Interval::Year.appropriate_floor(date) == Utc.ymd(2016, 1, 1).and_hms(0, 0, 0));
    }

    #[test]
    fn interval_addition() {
        let date = Utc.ymd(2016, 7, 8).and_hms(0, 0, 0);
        assert!(date + Interval::Minute == Utc.ymd(2016, 7, 8).and_hms(0, 1, 0));
        assert!(date + Interval::HalfHour == Utc.ymd(2016, 7, 8).and_hms(0, 30, 0));
        assert!(date + Interval::Hour == Utc.ymd(2016, 7, 8).and_hms(1, 0, 0));
        assert!(date + Interval::HalfDay == Utc.ymd(2016, 7, 8).and_hms(12, 0, 0));
        assert!(date + Interval::Day == Utc.ymd(2016, 7, 9).and_hms(0, 0, 0));
        assert!(date + Interval::Week == Utc.ymd(2016, 7, 15).and_hms(0, 0, 0));
        assert!(date + Interval::Month == Utc.ymd(2016, 8, 8).and_hms(0, 0, 0));
        assert!(date + Interval::Year == Utc.ymd(2017, 7, 8).and_hms(0, 0, 0));
    }
}
