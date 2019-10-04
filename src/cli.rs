use crate::{
    api::{responses::*, Client},
    bisq::constants::*,
    daemon::{self, DaemonConfig},
    server::TorConfig,
};
use std::fs;
use std::str::FromStr;
#[macro_use]
use clap::{clap_app, crate_version, App, ArgMatches};

fn app() -> App<'static, 'static> {
    clap_app!(risq =>
        (version: crate_version!())
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@arg API_PORT: --("api-port") default_value("7477") +global {port} "API port")
        (@subcommand daemon =>
         (about: "Runs the risq p2p node")
         (visible_alias: "d")
         (@arg NETWORK: -n --network default_value[BtcMainnet] {network} "(BtcRegtest|BtcTestnet|BtcMainnet)")
         (@arg P2P_PORT: -p --("p2p-port") default_value("5000") {port} "Port of p2p node")
         (@arg TOR_ACTIVE: --("tor-active") default_value("true") {boolean} "Run daemon behind tor")
         (@arg TOR_CONTROL_PORT: --("tor-controll-port") default_value("9051") {port} "Tor Control port")
         (@arg TOR_HIDDEN_SERVICE_PORT: --("tor-hidden-service-port") default_value("9999") {port} "Public port of the hidden service")
         (@arg TOR_SOCKS_PORT: --("tor-socks-port") default_value("9050") {port} "Tor SOCKSPort")
        )
        (@subcommand offers =>
         (about: "Subcomand to interact with offers")
         (visible_alias: "o")
        )
    )
}

pub fn run() -> () {
    let matches = app().get_matches();
    match matches.subcommand() {
        ("daemon", Some(matches)) => daemon(matches),
        ("offers", Some(matches)) => offers(matches),
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
    match Client::new(api_port).get_offers() {
        Ok(get_offers) => {
            println!("OPEN OFFERS");
            if get_offers.any() {
                get_offers
                    .offers
                    .into_iter()
                    .for_each(display_offer_summary)
            } else {
                println!("<currently no offers available>")
            }
        }
        Err(_) => println!("Error trying to reach api"),
    }
}

fn display_offer_summary(offer: Offer) {
    let mut dis = format!("{} {} ", offer.direction, offer.price.r#type);
    dis.push_str(&if offer.price.r#type == "fixed" {
        format!("{}", offer.price.fixed.unwrap())
    } else {
        format!("{}", offer.price.market_margin.unwrap())
    });
    println!("{} {}({})", dis, offer.amount, offer.min_amount)
}
