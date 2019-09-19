pub(super) mod connection {
    use crate::bisq::proto::{network_envelope, MessageVersion, NetworkEnvelope};
    use crate::error::{Error, ReceiveErrorKind};
    #[macro_use]
    use futures::{try_ready, future, Async, Future, Stream};
    use prost::Message;
    use std::{
        io::{self, Write},
        net::ToSocketAddrs,
    };
    use tokio::{
        io::{AsyncRead, ReadHalf},
        net::TcpStream,
    };

    pub struct ConnectionConfig {
        pub message_version: MessageVersion,
    }
    pub struct Connection {
        tcp: TcpStream,
        conf: ConnectionConfig,
    }
    impl Connection {
        pub fn new(
            addr: impl ToSocketAddrs,
            conf: ConnectionConfig,
        ) -> impl Future<Item = Connection, Error = Error> {
            future::result(
                addr.to_socket_addrs()
                    .map_err(|_| Error::ToSocketError)
                    .and_then(|mut i| i.next().ok_or(Error::ToSocketError)),
            )
            .and_then(|addr| {
                TcpStream::connect(&addr)
                    .map(|tcp| Connection { tcp, conf })
                    .map_err(|err| err.into())
            })
        }
        pub fn from_tcp_stream(stream: TcpStream, message_version: MessageVersion) -> Connection {
            Connection {
                tcp: stream,
                conf: ConnectionConfig { message_version },
            }
        }
        pub fn send_sync(&mut self, msg: network_envelope::Message) -> () {
            let envelope = NetworkEnvelope {
                message_version: self.conf.message_version.into(),
                message: Some(msg),
            };
            let mut serialized = Vec::with_capacity(envelope.encoded_len() + 1);
            envelope
                .encode_length_delimited(&mut serialized)
                .map_err(|err| error!("Could not serialize message {:?}; Error {}", envelope, err))
                .expect("Could not encode message");
            self.tcp.write(&serialized);
            self.tcp.flush();
            ()
        }
        // pub fn next_msg(self) -> impl Future<Item = (), Error = ()> {
        // let read_size = vec![0];
        // read_exact(self.tcp, read_size)
        //     .and_then(|(socket, next_size)| {
        //         info!("size: {:?}", next_size[0]);
        //         let msg_bytes = vec![0; next_size[0].into()];
        //         read_exact(socket, msg_bytes)
        //     })
        //     .map_err(|e| error!("error reading from socket {:?}", e))
        //     .and_then(|(_socket, msg_bytes)| {
        //         info!("msg_bytes received {:?}", msg_bytes);
        //         future::result(
        //             NetworkEnvelope::decode(&msg_bytes)
        //                 .map(|msg| info!("message received {:?}", msg))
        //                 .map_err(|e| error!("error decoding msg from socket {:?}", e)),
        //         )
        //     })
        // }
    }
    enum OutStreamState {
        MessageInProgress {
            size: usize,
            pos: usize,
            buf: Vec<u8>,
        },
        BetweenMessages,
        Empty,
    }
    struct OutStream {
        reader: ReadHalf<TcpStream>,
        state: OutStreamState,
    }
    impl Stream for OutStream {
        type Item = network_envelope::Message;
        type Error = Error;

        fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
            let ping = network_envelope::Message::Ping(crate::bisq::proto::Ping {
                nonce: 0,
                last_round_trip_time: 0,
            });
            if let OutStreamState::BetweenMessages = self.state {
                let mut read_size = vec![0];
                let _ = try_ready!(self.reader.poll_read(&mut read_size));
                let size = read_size[0].into();
                self.state = OutStreamState::MessageInProgress {
                    size,
                    pos: 0,
                    buf: vec![0; size],
                };
            }
            let res = match (&mut self.state) {
                (OutStreamState::MessageInProgress {
                    ref mut size,
                    ref mut pos,
                    ref mut buf,
                }) => {
                    while *pos < *size {
                        let n = try_ready!(self.reader.poll_read(&mut buf[*pos..]));
                        *pos += n;
                        if n == 0 {
                            return Err(Error::IoError(io::Error::new(
                                io::ErrorKind::UnexpectedEof,
                                "early eof",
                            )));
                        }
                    }
                    NetworkEnvelope::decode(&*buf)?
                }
                OutStreamState::Empty => panic!("Stream is already finished"),
                _ => return Ok(Async::NotReady),
            };
            // let next_state = match self.state {
            //     OutStreamState::BetweenMessages => {
            //         let mut read_size = vec![0];
            //         let _ = try_ready!(self.reader.poll_read(&mut read_size));
            //         let size = read_size[0].into()
            //         OutStreamState::MessageInProgress {
            //             size ,
            //                 pos: 0,
            //                 buf: vec![0;size],
            //         }
            //     }
            //     OutStreamState::MessageInProgress => Ok(Async::Ready(Some(ping))),
            //     State::Empty => panic!("poll a ReadExact after it's done")
            // }
            match res.message {
                Some(message) => Ok(Async::Ready(Some(message))),
                None => Ok(Async::NotReady),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::{Connection, ConnectionConfig};
        use crate::bisq::{
            proto::{network_envelope::Message, Ping},
            BaseCurrencyNetwork,
        };
        use crate::error::Error;
        use futures::{future::ok, stream::Stream};
        use std::net::SocketAddr;
        use tokio::net::TcpListener;

        #[test]
        fn send_and_receive() -> Result<(), Error> {
            let network = BaseCurrencyNetwork::BtcRegtest;
            let config = ConnectionConfig {
                message_version: network.into(),
            };
            let addr = "127.0.0.1:7477";
            let connection = Connection::new(addr, config);

            TcpListener::bind(&addr.parse::<SocketAddr>().unwrap())?
                .incoming()
                .and_then(|stream| {
                    let ping = Message::Ping(Ping {
                        nonce: 0,
                        last_round_trip_time: 0,
                    });
                    ok(Connection::from_tcp_stream(stream, network.into()).send_sync(ping))
                });
            Ok(())
        }
    }
}
