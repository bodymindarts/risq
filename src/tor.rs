extern crate socks;

use socks::Socks5Stream;
use socks::ToTargetAddr;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};

pub struct TorStream(TcpStream);

impl TorStream {
    pub fn connect(tor_proxy: SocketAddr, destination: impl ToTargetAddr) -> io::Result<TorStream> {
        Socks5Stream::connect(tor_proxy, destination).map(|stream| TorStream(stream.into_inner()))
    }
}

impl Read for TorStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for TorStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

#[cfg(test)]
mod tests {

    use super::TorStream;
    use std::io::{Read, Write};
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    #[test]
    fn check_clear_web() -> std::io::Result<()> {
        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050));
        let mut stream = TorStream::connect(address, "www.example.com:80")?;

        stream
            .write_all(b"GET / HTTP/1.1\r\nConnection: Close\r\nHost: www.example.com\r\n\r\n")
            .expect("Failed to send request");

        let mut buf = String::with_capacity(1633);
        stream
            .read_to_string(&mut buf)
            .expect("Failed to read response");

        assert!(buf.starts_with("HTTP/1.1 200 OK"));
        Ok(())
    }

    #[test]
    fn check_hidden_service() -> std::io::Result<()> {
        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050));
        let mut stream = TorStream::connect(
            address,
            (
                "p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion",
                80,
            ),
        )?;

        stream
            .write_all(b"GET / HTTP/1.1\r\nConnection: Close\r\nHost: p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion\r\n\r\n")
            .expect("Failed to send request");

        let mut buf = String::with_capacity(390);
        stream
            .read_to_string(&mut buf)
            .expect("Failed to read response");

        assert!(buf.starts_with("HTTP/1.1 302"));
        Ok(())
    }
}
