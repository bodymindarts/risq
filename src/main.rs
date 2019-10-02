use env_logger;
use risq::{daemon, BaseCurrencyNetwork, DaemonConfig, TorConfig};
use std::fs;

fn main() {
    env_logger::init();

    let mut dir = dirs::home_dir().expect("Couldn't determin home dir");
    dir.push(".risq/tor");
    fs::create_dir_all(&dir).expect("Couldn't create risq dir");
    dir.push("service.key");

    let local_port = 5000;
    let mainnet_conf = DaemonConfig {
        network: BaseCurrencyNetwork::BtcMainnet,
        server_port: local_port,
        tor_proxy_port: Some(9050),
        tor_config: Some(TorConfig {
            hidden_service_port: 9999,
            tc_port: 9051,
            private_key_path: dir,
        }),
    };

    let regtest_conf = DaemonConfig {
        network: BaseCurrencyNetwork::BtcRegtest,
        server_port: local_port,
        tor_proxy_port: None,
        tor_config: None,
    };

    // Uncomment for regtest
    //
    daemon::run(regtest_conf);

    // Uncomment for mainnet
    //
    // daemon::run(mainnet_conf);
}
