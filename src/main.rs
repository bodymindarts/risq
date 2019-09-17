mod api;

mod bisq;
mod tor;

use std::{error::Error, io::Write, net::SocketAddr, process::exit};

use futures::{
    future::{ok, Future},
    stream::Stream,
    sync::oneshot,
};

use actix::Arbiter;
use bisq::{message::*, BaseCurrencyNetwork};
use tokio::net::{TcpListener, TcpStream};

use prost::Message;

use env_logger;
#[macro_use]
extern crate log;
fn process_socket(socket: TcpStream) -> impl Future<Item = (), Error = ()> {
    ok::<(), ()>(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let sys = actix::System::new("risq");

    let network = BaseCurrencyNetwork::BtcRegtest;
    let ping = NetworkEnvelope {
        message_version: network.get_message_version(),
        message: Some(network_envelope::Message::Ping(Ping {
            nonce: 1,
            last_round_trip_time: 0,
        })),
    };

    let (start_up_notify, start_up_listen) = oneshot::channel();
    let mut serialized = Vec::with_capacity(ping.encoded_len() + 1);
    ping.encode_length_delimited(&mut serialized)
        .expect("Could not encode ping");
    let send_task = start_up_listen
        .map_err(|e| error!("error: {:?}", e))
        .and_then(|_| {
            debug!("listener has started");
            TcpStream::connect(&"127.0.0.1:4444".parse::<SocketAddr>().expect("bla"))
                .and_then(move |mut stream| {
                    info!("Sending msg {:?}", serialized);
                    stream.write(&serialized)?;
                    stream.flush()
                })
                .and_then(|_| Ok(()))
                .map_err(|e| error!("error: {:?}", e))
        });
    Arbiter::spawn(send_task);

    let addr = "127.0.0.1:4444".parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr).and_then(|l| {
        debug!("listener is notifying");
        start_up_notify.send(());
        Ok(l)
    })?;
    let server = listener
        .incoming()
        .map_err(|e| error!("{}", e))
        .for_each(|socket| {
            info!("spawning incoming {:?}", socket);
            Arbiter::spawn(process_socket(socket));
            ok(())
        });

    Arbiter::spawn(server);

    sys.run()?;
    //let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    //let listener = TcpListener::bind(&addr)?;
    ////
    //// accept connections and process them
    //tokio::run(
    //    listener
    //        .incoming()
    //        .map_err(|e| {
    //            eprintln!(
    //                "failed to accept socket; error =
    //{:?}",
    //                e
    //            )
    //        })
    //        .for_each(|socket| {
    //            process_socket(socket);
    //            Ok(())
    //        }),
    //);
    // listener.incoming()
    //         .map_err(|e| eprintln!("failed to accept socket; error = {:?}", e))
    //                 .for_each(|socket| {
    //                                 process_socket(socket);
    //                                             Ok(())
    //                                                         })
    // let api_thread = thread::spawn(move || api::listen(4444).expect("Failed to start api"));
    // let mut tc = tor::TorControl::connect("127.0.0.1:9051").unwrap();

    // let res = tc
    //     .add_v2_onion(AddOnionConfig {
    //         virtual_port: 4000,
    //         target_port: 4444,
    //         private_key_path: "/Users/jcarter/projects/bodymindarts/risq/.risq/risq_service_key"
    //             .into(),
    //     })
    //     .expect("Could not start api");

    // println!("{:?}", res);

    // api_thread.join().expect("Could not join api_thread");
    exit(0);
}
