use crate::{
    bisq::{constants::BaseCurrencyNetwork, payload::*},
    p2p::{dispatch::*, Connection, ConnectionId, Request},
    prelude::*,
};
use std::{process, time::SystemTime};

#[derive(Debug, Clone, Copy)]
struct DummyDispatcher;
impl Dispatcher for DummyDispatcher {
    fn dispatch(&self, conn: ConnectionId, msg: network_envelope::Message) -> Dispatch {
        Dispatch::Consumed
    }
}

pub fn check_node(network: BaseCurrencyNetwork, addr: NodeAddress, proxy_port: u16) {
    let _ = System::run(move || {
        Arbiter::spawn(
            Connection::open(
                addr.clone(),
                network.into(),
                DummyDispatcher,
                Some(proxy_port),
            )
            .map_err(|_| {
                eprintln!("CRITICAL - Unable to connect to node");
                process::exit(2);
            })
            .and_then(move |(_id, conn)| {
                //println!("Sending Ping to {}:{}", addr.host_name, addr.port);
                let ping = Ping {
                    nonce: gen_nonce(),
                    last_round_trip_time: 0,
                };
                let send_time = SystemTime::now();
                conn.send(Request(ping))
                    .map_err(|_| {
                        eprintln!("CRITICAL - Unable to send ping");
                        process::exit(2)
                    })
                    .map(move |res| match res {
                        Ok(_) => {
                            let res_time = SystemTime::now();
                            println!(
                                "OK - PONG|time={}ms",
                                res_time
                                    .duration_since(send_time)
                                    .expect("Pong before Ping")
                                    .as_millis()
                            );
                            process::exit(0)
                        }
                        Err(_) => {
                            eprintln!("CRITICAL - No response from host");
                            process::exit(2)
                        }
                    })
            }),
        )
    });
}
