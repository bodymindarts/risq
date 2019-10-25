use crate::domain::{amount::*, market::Market};

#[derive(Clone)]
pub struct Ticker {
    pub market: &'static Market,
    pub last: NumberWithPrecision,
    pub high: NumberWithPrecision,
    pub low: NumberWithPrecision,
    pub volume_left: NumberWithPrecision,
    pub volume_right: NumberWithPrecision,
}
