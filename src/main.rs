// mod api;
// mod peer;

// mod bisq;
// pub mod error;
// mod tor;

// use std::{error::Error, io::Write, net::SocketAddr, process::exit, thread, time};
use std::process;

// use futures::{
//     future::{ok, result, Future},
//     stream::Stream,
//     sync::oneshot,
// };

// use actix::{Actor, Arbiter};
// use bisq::{
//     message::{self, NetworkEnvelope},
//     BaseCurrencyNetwork,
// };
// use tokio::{
//     io::read_exact,
//     net::{TcpListener, TcpStream},
// };

// use prost::Message;

use env_logger;
#[macro_use]
extern crate log;

// fn process_socket(socket: TcpStream) -> impl Future<Item = (), Error = ()> {
//     let read_size = vec![0];
//     read_exact(socket, read_size)
//         .and_then(|(socket, next_size)| {
//             info!("size: {:?}", next_size[0]);
//             let msg_bytes = vec![0; next_size[0].into()];
//             read_exact(socket, msg_bytes)
//         })
//         .map_err(|e| error!("error reading from socket {:?}", e))
//         .and_then(|(_socket, msg_bytes)| {
//             info!("msg_bytes received {:?}", msg_bytes);
//             result(
//                 NetworkEnvelope::decode(&msg_bytes)
//                     .map(|msg| info!("message received {:?}", msg))
//                     .map_err(|e| error!("error decoding msg from socket {:?}", e)),
//             )
//         })
// }

fn main() -> () {
    env_logger::init();
    // let sys = actix::System::new("risq");

    // let network = BaseCurrencyNetwork::BtcRegtest;
    // let msgs = message::MessageFactory::new(&network);

    // let (start_up_notify, start_up_listen) = oneshot::channel();
    // TcpStream::connect(&"127.0.0.1:2002".parse::<SocketAddr>().expect("bla"))
    //     .and_then(|stream| ok(Connection::from_tcp_stream(stream).start()));
    // let con = Connection::start("127.0.0.1:2002");
    // let send_task = start_up_listen
    //     .map_err(|e| error!("error: {:?}", e))
    //     .and_then(move |_| {
    //         debug!("listener has started");
    //         let preliminary_get_data_request = msgs.preliminary_get_data_request();
    //         info!("Sending ping {:?}", preliminary_get_data_request);
    //         TcpStream::connect(&"127.0.0.1:2002".parse::<SocketAddr>().expect("bla"))
    //             .and_then(move |mut stream| {
    //                 let mut serialized =
    //                     Vec::with_capacity(preliminary_get_data_request.encoded_len() + 1);
    //                 preliminary_get_data_request
    //                     .encode_length_delimited(&mut serialized)
    //                     .expect("Could not encode ping");
    //                 info!("Sending msg {:?}", serialized);
    //                 stream.write(&serialized)?;
    //                 stream.flush();
    //                 info!("sent msg {:?}", serialized);
    //                 thread::sleep(time::Duration::from_secs(30));
    //                 info!("flush again");
    //                 stream.flush()
    //             })
    //             .and_then(|_| Ok(()))
    //             .map_err(|e| error!("error: {:?}", e))
    //     });
    // Arbiter::spawn(send_task);

    //     let addr = "127.0.0.1:4444".parse::<SocketAddr>()?;
    //     let listener = TcpListener::bind(&addr).and_then(|l| {
    //         debug!("listener is notifying");
    //         start_up_notify.send(());
    //         Ok(l)
    //     })?;
    //     let server = listener
    //         .incoming()
    //         .map_err(|e| error!("{}", e))
    //         .for_each(|socket| {
    //             info!("spawning incoming {:?}", socket);
    //             Arbiter::spawn(process_socket(socket));
    //             ok(())
    //         });

    //     Arbiter::spawn(server);

    //     api::listen(7477)?;

    //     sys.run()?;

    process::exit(0);
}
