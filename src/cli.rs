mod query;

#[cfg(feature = "checker")]
use crate::checker;
use crate::{
    api::Client,
    bisq::{constants::*, NodeAddress},
    daemon::{self, DaemonConfig},
    domain::{currency::Currency, market::Market},
    p2p::TorConfig,
};
use clap::{clap_app, crate_version, App, Arg, ArgMatches, SubCommand};
use query::*;
use reqwest;
use std::{collections::HashMap, str::FromStr};

fn app() -> App<'static, 'static> {
    let app = clap_app!(risq =>
        (version: crate_version!())
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand daemon =>
         (about: "Runs the risq p2p node")
         (visible_alias: "d")
         (@arg API_PORT: --("api-port") default_value("7477") {port} "API port")
         (@arg NETWORK: -n --network default_value[BtcMainnet] {network} "(BtcRegtest|BtcTestnet|BtcMainnet)")
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
    if cfg!(feature = "checker") {
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
    } else {
        app
    }
}

pub fn run() -> () {
    let matches = app().get_matches();
    match matches.subcommand() {
        ("daemon", Some(matches)) => daemon(matches),
        ("offers", Some(matches)) => offers(matches),
        #[cfg(feature = "checker")]
        ("check-node", Some(matches)) => check_node(matches),
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
        Err(_) => Err(format!("'{}' is not a valid port number", port).into()),
        Ok(_) => Ok(()),
    }
}
fn market(market: String) -> Result<(), String> {
    if &market == "all" {
        return Ok(());
    }
    if let None = Currency::from_code(&market) {
        return Err(format!("'{}' is not a valid currency code", market));
    }
    Ok(())
}
fn boolean(b: String) -> Result<(), String> {
    match bool::from_str(&b) {
        Err(_) => Err(format!("'{}' is not a valid boolean", b).into()),
        Ok(_) => Ok(()),
    }
}

fn daemon(matches: &ArgMatches) {
    let mut private_key_path = dirs::home_dir().expect("Couldn't determin home dir");
    private_key_path.push(".risq/tor/service.key");

    let network: BaseCurrencyNetwork = matches.value_of("NETWORK").unwrap().parse().unwrap();
    let api_port = matches.value_of("API_PORT").unwrap().parse().unwrap();
    let server_port = matches.value_of("P2P_PORT").unwrap().parse().unwrap();
    let tor_active: bool = matches.value_of("TOR_ACTIVE").unwrap().parse().unwrap();
    let (tor_proxy_port, tor_config) = if tor_active {
        (
            Some(matches.value_of("TOR_SOCKS_PORT").unwrap().parse().unwrap()),
            Some(TorConfig {
                hidden_service_port: matches
                    .value_of("TOR_HIDDEN_SERVICE_PORT")
                    .unwrap()
                    .parse()
                    .unwrap(),
                tc_port: matches
                    .value_of("TOR_CONTROL_PORT")
                    .unwrap()
                    .parse()
                    .unwrap(),
                private_key_path,
            }),
        )
    } else {
        (None, None)
    };
    daemon::run(DaemonConfig {
        api_port,
        server_port,
        network,
        tor_config,
        tor_proxy_port,
    });
}

fn offers(matches: &ArgMatches) {
    let api_port = matches.value_of("API_PORT").unwrap().parse().unwrap();
    let mut args = HashMap::new();
    let currency: Result<&Currency, ()> = matches.value_of("MARKET").unwrap().parse();
    if let Ok(currency) = currency {
        let market: &Market = currency.into();
        Offers::add_variables(&market, &mut args);
    }
    let response: reqwest::Result<Offers> = Client::new(api_port).query(args);
    match response {
        Ok(get_offers) => {
            println!("OPEN OFFERS");
            if get_offers.offers.len() == 0 {
                println!("<currently no offers available>");
                return;
            }
            for offer in get_offers.offers.into_iter() {
                println!("{}", offer)
            }
        }
        Err(_) => println!("Error trying to reach api"),
    }
}

#[cfg(feature = "checker")]
fn check_node(matches: &ArgMatches) {
    let socks_port = matches.value_of("TOR_SOCKS_PORT").unwrap().parse().unwrap();
    let host_name: String = matches.value_of("NODE_HOST").unwrap().into();
    let port = matches.value_of("NODE_PORT").unwrap().parse().unwrap();
    let network: BaseCurrencyNetwork = matches.value_of("NETWORK").unwrap().parse().unwrap();
    checker::check_node(network, NodeAddress { host_name, port }, socks_port);
}
