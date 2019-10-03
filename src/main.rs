use env_logger;
use risq::{cli, daemon, BaseCurrencyNetwork, DaemonConfig, TorConfig};
use std::fs;

fn main() {
    env_logger::init();

    cli::run();
}
