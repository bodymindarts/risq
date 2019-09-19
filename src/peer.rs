pub(super) mod connection {
    use crate::bisq::proto::{network_envelope, MessageVersion, NetworkEnvelope, Ping};
    use crate::error::{Error, ReceiveErrorKind};
    #[macro_use]
    use futures::{sync::oneshot, try_ready, future, Async, Future, Stream};
    use prost::Message;
    use std::{
        collections::VecDeque,
        io::{self, Write},
        mem,
        net::ToSocketAddrs,
    };
    use tokio::{
        io::{AsyncRead, ReadHalf, WriteHalf},
        net::TcpStream,
    };

    pub struct ConnectionConfig {
        pub message_version: MessageVersion,
    }
    pub struct Connection {
        writer: WriteHalf<TcpStream>,
        reader: Option<MessageStream>,
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
                    .map(|tcp| {
                        let (reader, writer) = tcp.split();
                        let reader = Some(MessageStream::new(reader));
                        Connection {
                            writer,
                            reader,
                            conf,
                        }
                    })
                    .map_err(|err| err.into())
            })
        }
        pub fn from_tcp_stream(stream: TcpStream, message_version: MessageVersion) -> Connection {
            let (reader, writer) = stream.split();
            Connection {
                writer,
                reader: Some(MessageStream::new(reader)),
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
                .expect("Could not encode message");
            self.writer.write(&serialized);
            self.writer.flush();
            ()
        }
        pub fn extract_message_stream(
            &mut self,
        ) -> impl Stream<Item = network_envelope::Message, Error = Error> {
            mem::replace(&mut self.reader, None).expect("Reader already removed")
        }

        // pub fn get_next(
        //     &mut self,
        // ) -> impl Future<Item = Option<network_envelope::Message>, Error = Error> {
        //     let (tx, rx) = oneshot::channel();
        //     // self.reader.and_then(|res| tx.send(res));
        //     let ping = network_envelope::Message::Ping(Ping {
        //         nonce: 0,
        //         last_round_trip_time: 0,
        //     });
        //     tx.send(Some(ping));
        //     rx.map_err(|e| e.into())
        // }
    }
    enum MessageStreamState {
        MessageInProgress {
            size: usize,
            pos: usize,
            buf: Vec<u8>,
        },
        BetweenMessages,
        Empty,
    }
    struct MessageStream {
        reader: ReadHalf<TcpStream>,
        state: MessageStreamState,
        buffer: VecDeque<NetworkEnvelope>,
    }
    impl MessageStream {
        fn new(reader: ReadHalf<TcpStream>) -> MessageStream {
            MessageStream {
                reader,
                state: MessageStreamState::BetweenMessages,
                buffer: VecDeque::new(),
            }
        }
        fn next_from_buffer(&mut self) -> Option<network_envelope::Message> {
            let msg = self.buffer.pop_front()?.message;
            match msg {
                Some(network_envelope::Message::BundleOfEnvelopes(msg)) => {
                    msg.envelopes
                        .into_iter()
                        .rev()
                        .for_each(|envelope| self.buffer.push_front(envelope));
                    self.next_from_buffer()
                }
                None => self.next_from_buffer(),
                _ => msg,
            }
        }
    }
    impl Stream for MessageStream {
        type Item = network_envelope::Message;
        type Error = Error;

        fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
            let next_read = match self.state {
                MessageStreamState::Empty => panic!("Stream is already finished"),
                MessageStreamState::BetweenMessages => {
                    if let Some(msg) = self.next_from_buffer() {
                        return Ok(Async::Ready(Some(msg)));
                    }
                    let mut read_size = vec![0];
                    let _ = try_ready!(self.reader.poll_read(&mut read_size));
                    let size = read_size[0].into();
                    self.state = MessageStreamState::MessageInProgress {
                        size,
                        pos: 0,
                        buf: vec![0; size],
                    };
                    return self.poll();
                }
                MessageStreamState::MessageInProgress {
                    ref mut size,
                    ref mut pos,
                    ref mut buf,
                } => {
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
            };
            self.buffer.push_back(next_read);
            self.state = MessageStreamState::BetweenMessages;
            self.poll()
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
        use futures::{
            future::{lazy, ok},
            stream::Stream,
            sync::oneshot,
            Future,
        };
        use std::net::SocketAddr;
        use tokio::net::TcpListener;

        #[test]
        fn send_and_receive() -> Result<(), Error> {
            let network = BaseCurrencyNetwork::BtcRegtest;
            let config = ConnectionConfig {
                message_version: network.into(),
            };
            // let ping = Message::Ping(Ping {
            //     nonce: 0,
            //     last_round_trip_time: 0,
            // });
            let addr = "127.0.0.1:7477";
            let connection = Connection::new(addr, config);
            let incoming_connection = TcpListener::bind(&addr.parse::<SocketAddr>().unwrap())?
                .incoming()
                .and_then(|stream| ok(Connection::from_tcp_stream(stream, network.into())))
                .map(|mut conn| {
                    let (tx, rx) = oneshot::channel();
                    tokio::spawn(lazy(move || {
                        conn.extract_message_stream()
                            .take(1)
                            .into_future()
                            .and_then(|(msg, _)| ok(tx.send(msg).unwrap()))
                            .map_err(|e| println!("err"))
                    }));
                });
            Ok(())
        }
    }
}
