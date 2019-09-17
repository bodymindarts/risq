mod api;

mod bisq;
mod tor;

use std::{error::Error, io::Write, net::SocketAddr, process::exit};

use futures::{
    future::{ok, result, Future},
    stream::Stream,
    sync::oneshot,
};

use actix::Arbiter;
use bisq::{message::*, BaseCurrencyNetwork};
use tokio::{
    io::read_exact,
    net::{TcpListener, TcpStream},
};

use prost::Message;

use env_logger;
#[macro_use]
extern crate log;
fn process_socket(socket: TcpStream) -> impl Future<Item = (), Error = ()> {
    let read_size = vec![0];
    read_exact(socket, read_size)
        .and_then(|(socket, next_size)| {
            info!("size: {:?}", next_size[0]);
            let msg_bytes = vec![0; next_size[0].into()];
            read_exact(socket, msg_bytes)
        })
        .map_err(|e| error!("error reading from socket {:?}", e))
        .and_then(|(_socket, msg_bytes)| {
            info!("msg_bytes received {:?}", msg_bytes);
            result(
                NetworkEnvelope::decode(&msg_bytes)
                    .map(|msg| info!("message received {:?}", msg))
                    .map_err(|e| error!("error decoding msg from socket {:?}", e)),
            )
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let sys = actix::System::new("risq");

    let network = BaseCurrencyNetwork::BtcRegtest;
    let ping = NetworkEnvelope {
        message_version: 1,
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
        .and_then(move |_| {
            debug!("listener has started");
            info!("Sending ping {:?}", ping);
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

    api::listen(7477)?;

    sys.run()?;

    exit(0);
}
