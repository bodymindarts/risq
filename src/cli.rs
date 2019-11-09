mod query;

use crate::{
    api::Client,
    bisq::constants::*,
    daemon::{self, DaemonConfig},
    domain::{currency::Currency, market::Market},
};
use clap::{clap_app, crate_version, App, ArgMatches};
use env_logger::Env;
use log::Level;
use query::*;
use reqwest;
use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

fn app() -> App<'static, 'static> {
    let app = clap_app!(risq =>
        (version: crate_version!())
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand daemon =>
         (about: "Runs the risq p2p node")
         (visible_alias: "d")
         (@arg API_PORT: --("api-port") default_value("7477") {port} "API port")
         (@arg LOG_LEVEL: -l --("log-level") default_value("info") {level} "(error|warn|info|debug|trace)")
         (@arg NETWORK: -n --network default_value("BtcMainnet") {network} "(BtcRegtest|BtcTestnet|BtcMainnet)")
         (@arg P2P_PORT: -p --("p2p-port") default_value("5000") {port} "Port of p2p node")
         (@arg TOR_ACTIVE: --("tor-active") default_value("true") {boolean} "Run daemon behind tor")
         (@arg TOR_CONTROL_PORT: --("tor-controll-port") default_value("9051") {port} "Tor Control port")
         (@arg TOR_HIDDEN_SERVICE_PORT: --("tor-hidden-service-port") default_value("9999") {port} "Public port of the hidden service")
         (@arg TOR_SOCKS_PORT: --("tor-socks-port") default_value("9050") {port} "Tor SOCKSPort")
        )
        (@subcommand offers =>
         (about: "Subcomand to interact with offers")
         (@arg API_PORT: --("api-port") default_value("7477") {port} "API port")
         (@arg MARKET: --("market") default_value("all") {market} "Filter by market pair")
        )
    );

    let app = add_checker_cmd(app);
    add_dummy_seed_cmd(app)
}

pub fn run() {
    let matches = app().get_matches();
    match matches.subcommand() {
        ("daemon", Some(matches)) => daemon(matches),
        ("offers", Some(matches)) => offers(matches),
        #[cfg(feature = "checker")]
        ("check-node", Some(matches)) => check_node(matches),
        #[cfg(feature = "dummy-seed")]
        ("dummy-seed", Some(matches)) => dummy_seed(matches),
        _ => unreachable!(),
    }
}

fn network(network: String) -> Result<(), String> {
    match BaseCurrencyNetwork::from_str(&network) {
        Err(_) => Err("(BtcMainnet|BtcTestnet|BtcRegtest)".into()),
        Ok(_) => Ok(()),
    }
}
fn port(port: String) -> Result<(), String> {
    match u16::from_str(&port) {
        Err(_) => Err(format!("'{}' is not a valid port number", port)),
        Ok(_) => Ok(()),
    }
}
fn market(market: String) -> Result<(), String> {
    if &market == "all" {
        return Ok(());
    }
    if Currency::from_code(&market).is_none() {
        return Err(format!("'{}' is not a valid currency code", market));
    }
    Ok(())
}
fn boolean(b: String) -> Result<(), String> {
    match bool::from_str(&b) {
        Err(_) => Err(format!("'{}' is not a valid boolean", b)),
        Ok(_) => Ok(()),
    }
}
fn level(level: String) -> Result<(), String> {
    match Level::from_str(&level) {
        Err(_) => Err(format!("'{}' is not a valid logging level", level)),
        Ok(_) => Ok(()),
    }
}
#[cfg(feature = "dummy-seed")]
fn file(file: String) -> Result<(), String> {
    let path = Path::new(&file);
    match (path.exists(), path.is_file()) {
        (true, true) => Ok(()),
        (false, _) => Err(format!("File '{}' does not exist", file)),
        (_, false) => Err(format!("'{}' is not a file", file)),
    }
}

const RISQ_HOME_VAR: &str = "RISQ_HOME";

fn daemon(matches: &ArgMatches) {
    let risq_home = env::var_os(RISQ_HOME_VAR)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let mut risq_dir = dirs::home_dir().expect("Couldn't determin home dir");
            risq_dir.push(".risq");
            risq_dir
        });

    let network: BaseCurrencyNetwork = matches.value_of("NETWORK").unwrap().parse().unwrap();
    let api_port = matches.value_of("API_PORT").unwrap().parse().unwrap();
    let server_port = matches.value_of("P2P_PORT").unwrap().parse().unwrap();
    let tor_active: bool = matches.value_of("TOR_ACTIVE").unwrap().parse().unwrap();
    let level: String = matches.value_of("LOG_LEVEL").unwrap().parse().unwrap();
    let env = Env::default().filter_or("RUST_LOG", level);
    env_logger::init_from_env(env);
    let (tor_proxy_port, tor_control_port, hidden_service_port) = if tor_active {
        (
            Some(matches.value_of("TOR_SOCKS_PORT").unwrap().parse().unwrap()),
            Some(
                matches
                    .value_of("TOR_CONTROL_PORT")
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
            Some(
                matches
                    .value_of("TOR_HIDDEN_SERVICE_PORT")
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
        )
    } else {
        (None, None, None)
    };
    daemon::run(DaemonConfig {
        api_port,
        server_port,
        network,
        risq_home,
        tor_control_port,
        tor_proxy_port,
        hidden_service_port,
    });
}

fn offers(matches: &ArgMatches) {
    let api_port = matches.value_of("API_PORT").unwrap().parse().unwrap();
    let mut vars = HashMap::new();
    let currency: Result<&Currency, ()> = matches.value_of("MARKET").unwrap().parse();
    if let Ok(currency) = currency {
        let market: &Market = currency.into();
        Offers::add_variables(&market, &mut vars);
    }
    let response: reqwest::Result<Offers> = Client::new(api_port).query(vars);
    match response {
        Ok(offers) => {
            println!("OPEN OFFERS");
            if offers.len() == 0 {
                println!("<currently no offers available>");
                return;
            }
            for offer in offers {
                println!("{}", offer)
            }
        }
        Err(_) => println!("Error trying to reach api"),
    }
}
#[cfg(not(feature = "checker"))]
fn add_checker_cmd(app: App<'static, 'static>) -> App<'static, 'static> {
    app
}
#[cfg(feature = "checker")]
fn add_checker_cmd(app: App<'static, 'static>) -> App<'static, 'static> {
    use clap::{Arg, SubCommand};
    app.subcommand(
        SubCommand::with_name("check-node")
            .about("Send a ping to a node. Used for monitoring.")
            .arg(
                Arg::with_name("TOR_SOCKS_PORT")
                    .long("tor-socks-port")
                    .validator(port)
                    .default_value("9050"),
            )
            .arg(
                Arg::with_name("NETWORK")
                    .long("network")
                    .short("n")
                    .validator(network)
                    .default_value("BtcMainnet"),
            )
            .arg(Arg::with_name("NODE_HOST").index(1).required(true))
            .arg(
                Arg::with_name("NODE_PORT")
                    .index(2)
                    .required(true)
                    .validator(port),
            )
            .after_help("Returns exit code 0 on success, 2 otherwise."),
    )
}

#[cfg(not(feature = "dummy-seed"))]
fn add_dummy_seed_cmd(app: App<'static, 'static>) -> App<'static, 'static> {
    app
}
#[cfg(feature = "dummy-seed")]
fn add_dummy_seed_cmd(app: App<'static, 'static>) -> App<'static, 'static> {
    use clap::{Arg, SubCommand};
    app.subcommand(
        SubCommand::with_name("dummy-seed")
            .about("Start a seed node used for testing")
            .arg(
                Arg::with_name("P2P_PORT")
                    .short("p")
                    .validator(port)
                    .default_value("4004"),
            )
            .arg(
                Arg::with_name("FIXTURES")
                    .short("f")
                    .takes_value(true)
                    .validator(file),
            ),
    )
}

#[cfg(feature = "checker")]
fn check_node(matches: &ArgMatches) {
    use crate::bisq::NodeAddress;
    use crate::checker;

    let socks_port = matches.value_of("TOR_SOCKS_PORT").unwrap().parse().unwrap();
    let host_name: String = matches.value_of("NODE_HOST").unwrap().into();
    let port = matches.value_of("NODE_PORT").unwrap().parse().unwrap();
    let network: BaseCurrencyNetwork = matches.value_of("NETWORK").unwrap().parse().unwrap();
    checker::check_node(network, NodeAddress { host_name, port }, socks_port);
}

#[cfg(feature = "dummy-seed")]
fn dummy_seed(matches: &ArgMatches) {
    use crate::dummy_seed;;

    let port = matches.value_of("P2P_PORT").unwrap().parse().unwrap();
    let fixtures: Option<&Path> = matches.value_of("FIXTURES").map(Path::new);
    dummy_seed::run(port, fixtures);
}
