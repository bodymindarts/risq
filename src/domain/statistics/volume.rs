use crate::domain::amount::*;
use std::time::SystemTime;

pub struct Volume {
    pub period_start: SystemTime,
    pub volume: NumberWithPrecision,
    pub num_trades: u32,
}
