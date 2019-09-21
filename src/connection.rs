use crate::bisq::message::{network_envelope, Listener, MessageVersion, NetworkEnvelope};
use crate::error::Error;
use prost::Message;
use std::{collections::VecDeque, io, net::ToSocketAddrs};
use tokio::{
    io::{flush, write_all, AsyncRead, ReadHalf, WriteHalf},
    net::TcpStream,
    prelude::{
        future::{self, Future, Loop},
        stream::Stream,
        Async,
    },
};

#[derive(Debug, Clone, Copy)]
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
        future::done(
            addr.to_socket_addrs()
                .map_err(|_| Error::ToSocketError)
                .and_then(|mut i| i.next().ok_or(Error::ToSocketError)),
        )
        .and_then(move |addr| {
            TcpStream::connect(&addr)
                .map(move |tcp| {
                    let (reader, writer) = tcp.split();
                    let reader = Some(MessageStream::new(reader, None));
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
            reader: Some(MessageStream::new(reader, None)),
            conf: ConnectionConfig { message_version },
        }
    }

    pub fn send(
        self,
        msg: impl Into<network_envelope::Message>,
    ) -> impl Future<Item = Connection, Error = Error> {
        let envelope = NetworkEnvelope {
            message_version: self.conf.message_version.into(),
            message: Some(msg.into()),
        };
        let mut serialized = Vec::with_capacity(envelope.encoded_len() + 1);
        envelope
            .encode_length_delimited(&mut serialized)
            .expect("Could not encode message");
        let Connection {
            writer,
            conf,
            reader,
        } = self;
        write_all(writer, serialized)
            .and_then(|(writer, _)| flush(writer))
            .map(move |writer| Connection {
                writer,
                conf,
                reader,
            })
            .map_err(|err| err.into())
    }

    pub fn send_and_await<T>(
        self,
        listener: impl Listener<Option<T>>,
    ) -> impl Future<Item = (T, Connection), Error = (Error, Connection)> {
        future::loop_fn(
            (listener, self.into_message_stream()),
            |(mut listener, stream)| {
                stream.into_future().and_then(|(msg, stream)| {
                    match listener.accept_or_err(msg, Error::DidNotReceiveExpectedResponse) {
                        Ok(Some(response)) => Ok(Loop::Break((response, stream.into_inner()))),
                        Ok(None) => Ok(Loop::Continue((listener, stream))),
                        Err(err) => Err((err, stream)),
                    }
                })
            },
        )
        .map_err(|(err, stream)| (err, stream.into_inner()))
    }

    pub fn into_message_stream(self) -> MessageStream {
        let mut reader = self.reader.expect("Reader already removed");
        reader.conn = Some((self.conf, self.writer));
        reader
    }

    pub fn take_message_stream(
        &mut self,
    ) -> impl Stream<Item = network_envelope::Message, Error = Error> {
        self.reader.take().expect("Reader already removed")
    }
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
pub struct MessageStream {
    conn: Option<(ConnectionConfig, WriteHalf<TcpStream>)>,
    reader: ReadHalf<TcpStream>,
    state: MessageStreamState,
    buffer: VecDeque<NetworkEnvelope>,
}
impl MessageStream {
    fn new(
        reader: ReadHalf<TcpStream>,
        conn: Option<(ConnectionConfig, WriteHalf<TcpStream>)>,
    ) -> MessageStream {
        MessageStream {
            conn,
            reader,
            state: MessageStreamState::BetweenMessages,
            buffer: VecDeque::new(),
        }
    }
    pub fn into_inner(self) -> Connection {
        let (conf, writer) = self.conn.expect("Inner not present");
        Connection {
            conf,
            writer,
            reader: Some(MessageStream {
                reader: self.reader,
                state: self.state,
                buffer: self.buffer,
                conn: None,
            }),
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
                let n = try_ready!(self.reader.poll_read(&mut read_size));
                if n == 0 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "early eof").into());
                }
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
                        return Err(
                            io::Error::new(io::ErrorKind::UnexpectedEof, "early eof").into()
                        );
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
        constants::BaseCurrencyNetwork,
        message::{network_envelope::Message, Ping},
    };
    use crate::error::Error;
    use std::net::SocketAddr;
    use tokio::{
        net::TcpListener,
        prelude::{
            future::{lazy, ok, Future},
            stream::Stream,
        },
        sync::oneshot,
    };

    #[test]
    fn basic_send_and_receive() {
        let network = BaseCurrencyNetwork::BtcRegtest;
        let config = ConnectionConfig {
            message_version: network.into(),
        };
        let addr = "127.0.0.1:7477";
        let connection = Connection::new(addr, config.clone());
        let (tx, rx) = oneshot::channel();
        let ping = Ping {
            nonce: 0,
            last_round_trip_time: 0,
        };
        let ping2 = Ping {
            nonce: 0,
            last_round_trip_time: 0,
        };
        let receiver = TcpListener::bind(&addr.parse::<SocketAddr>().unwrap())
            .unwrap()
            .incoming()
            .into_future()
            .map_err(|e| println!("err"))
            .and_then(move |(tcp, _)| {
                Connection::from_tcp_stream(tcp.unwrap(), network.into())
                    .take_message_stream()
                    .into_future()
                    .map(|(msg, _)| tx.send(msg.unwrap()).unwrap())
                    .map_err(|e| println!("err"))
            });
        let sender = Connection::new(addr, config)
            .and_then(|conn| conn.send(ping).map(|_| ()))
            .map_err(|e| println!("stoen"));
        tokio::run(lazy(move || {
            tokio::spawn(receiver);
            tokio::spawn(sender);
            rx.then(move |msg| {
                ok(match msg {
                    Ok(msg) => assert!(msg == ping2.into()),
                    Err(_) => assert!(false),
                })
            })
        }));
    }
}
