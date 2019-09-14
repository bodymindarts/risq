mod tor;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::process::exit;

mod tor_control;
use std::net::TcpStream;
use tor_control::*;

fn main() -> () {
    let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050));

    println!("Tor address {}", address);
    let tc: TCNoAuth<TcpStream> = TCNoAuth::connect("127.0.0.1:9051").unwrap();
    let mut tc = tc.auth(Some("\"password\"")).unwrap();
    println!("{:?}", tc.getconf(vec!["SOCKSPort", "Nickname"]).unwrap());
    exit(0);
}
