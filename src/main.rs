use env_logger;
use risq::cli;

fn main() {
    env_logger::init();

    cli::run();
}
