#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::too_many_arguments)]

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
