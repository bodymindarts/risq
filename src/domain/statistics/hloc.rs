use std::time::SystemTime;

pub struct Hloc {
    pub period_start: SystemTime,
}

#[cfg(feature = "statistics")]
pub use inner::*;
#[cfg(feature = "statistics")]
mod inner {
    use super::*;
    use crate::domain::{market::*, statistics::Trade};

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

    pub struct HlocQuery {
        pub market: &'static Market,
        pub timestamp_from: Option<SystemTime>,
        pub timestamp_to: Option<SystemTime>,
        pub interval: Option<HlocInterval>,
    }

    impl Hloc {
        pub fn from_trades(history: &Vec<Trade>, query: HlocQuery) -> Vec<Hloc> {
            Vec::new()
        }
    }
}
