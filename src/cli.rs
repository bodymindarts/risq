use crate::{
    bisq::constants::*,
    daemon::{self, DaemonConfig},
    server::TorConfig,
};
use std::fs;
use std::str::FromStr;
#[macro_use]
use clap::{clap_app, crate_version, App};

fn app() -> App<'static, 'static> {
    clap_app!(risq =>
        (version: crate_version!())
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand daemon =>
         (about: "Runs the risq p2p node")
         (visible_alias: "d")
         (@arg NETWORK: -n --network default_value[BtcMainnet] {network} "(BtcRegtest|BtcTestnet|BtcMainnet)")
         (@arg API_PORT: -a --("api-port") default_value("7477") {port} "API port")
         (@arg P2P_PORT: -p --("p2p-port") default_value("5000") {port} "Port of p2p node")
         (@arg TOR_ACTIVE: --("tor-active") default_value("true") {boolean} "Run daemon behind tor")
         (@arg TOR_CONTROL_PORT: --("tor-controll-port") default_value("9051") {port} "Tor Control port")
         (@arg TOR_HIDDEN_SERVICE_PORT: --("tor-hidden-service-port") default_value("9999") {port} "Public port of the hidden service")
         (@arg TOR_SOCKS_PORT: --("tor-socks-port") default_value("9050") {port} "Tor SOCKSPort")
        )
    )
}

pub fn run() -> () {
    let mut private_key_path = dirs::home_dir().expect("Couldn't determin home dir");
    private_key_path.push(".risq/tor/service.key");

    let matches = app().get_matches();
    match matches.subcommand() {
        ("daemon", Some(matches)) => {
            let network: BaseCurrencyNetwork =
                matches.value_of("NETWORK").unwrap().parse().unwrap();
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
