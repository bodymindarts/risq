mod api;
mod tor;

use std::process::exit;

use std::io::{Read, Write};
use tor::{AddOnionConfig, TCError, TorStream};

use std::thread;

fn main() -> () {
    let api_thread = thread::spawn(move || api::listen(4444).expect("Failed to start api"));
    let mut tc = tor::TorControl::connect("127.0.0.1:9051").unwrap();

    let res = tc
        .add_v2_onion(AddOnionConfig {
            virtual_port: 4000,
            target_port: 4444,
            private_key_path: "/Users/jcarter/projects/bodymindarts/risq/.risq/risq_service_key"
                .into(),
        })
        .expect("Could not start api");

    println!("{:?}", res);
    let mut stream = TorStream::connect("localhost:9050", (res.onion_service.as_str(), res.port))
        .expect("Could not connect to stream");
    stream
        .write_all(b"HELLO WORLD")
        .expect("Failed to send request");

    api_thread.join().expect("Could not join api_thread");
    exit(0);
}
