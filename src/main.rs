mod tor;

use std::process::exit;

use tor::TCError;

fn main() -> Result<(), TCError> {
    // let tc: TCNoAuth<TcpStream> = TCNoAuth::connect("127.0.0.1:9051").unwrap();
    let mut tc = tor::TorControl::connect("127.0.0.1:9051").unwrap();
    // let mut tc = tc.auth(Some("\"password\"")).unwrap();
    // let info = tc.protocol_info()?;
    // println!("{:?}", info);
    exit(0);
}
