mod tor;

use std::process::exit;

use std::io::{Read, Write};
use tor::{AddOnionConfig, TCError, TorStream};

fn main() -> Result<(), TCError> {
    // let tc: TCNoAuth<TcpStream> = TCNoAuth::connect("127.0.0.1:9051").unwrap();
    let mut tc = tor::TorControl::connect("127.0.0.1:9051").unwrap();
    // let mut tc = tc.auth(Some("\"password\"")).unwrap();
    // let info = tc.protocol_info()?;
    // println!("{:?}", info);
    let res = tc.add_v2_onion(AddOnionConfig {
        virtual_port: 4000,
        target_port: 4444,
        private_key_path: "/Users/jcarter/projects/bodymindarts/risq/.tor/key".to_string(),
    })?;
    println!("{:?}", res);
    let mut stream = TorStream::connect("localhost:9050", (res.onion_service.as_str(), res.port))?;
    stream
        .write_all(b"HELLO WORLD")
        .expect("Failed to send request");

    exit(0);
}
