#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

#[macro_use]
mod prelude;

mod api;
#[macro_use]
mod bisq;
#[cfg(feature = "checker")]
mod checker;
mod daemon;
mod domain;
mod error;
mod p2p;

#[cfg(not(target_os = "android"))]
pub mod cli;

#[macro_use]
extern crate log;

#[cfg(target_os = "android")]
pub use bisq::constants::BaseCurrencyNetwork;
#[cfg(target_os = "android")]
pub use daemon::*;
