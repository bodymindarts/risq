mod tor;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::process::exit;

fn main() -> () {
    let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050));

    println!("Tor address {}", address);

    exit(0);
}
