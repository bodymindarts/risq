use crate::{
    bisq::payload::{network_envelope, NetworkEnvelope},
    error,
    prelude::{
        io::{AsyncRead, ReadHalf},
        net::TcpStream,
        Async, Stream,
    },
};
use futures::try_ready;
use prost::{encoding::decode_varint, Message};
use std::{collections::VecDeque, io};

enum MessageStreamState {
    MessageInProgress {
        size: usize,
        pos: usize,
        buf: Vec<u8>,
    },
    BetweenMessages {
        buf: [u8; 10],
        pos: usize,
    },
    Empty,
}
pub struct MessageStream {
    reader: ReadHalf<TcpStream>,
    state: MessageStreamState,
    buffer: VecDeque<NetworkEnvelope>,
}
impl MessageStream {
    pub fn new(reader: ReadHalf<TcpStream>) -> MessageStream {
        MessageStream {
            reader,
            state: MessageStreamState::BetweenMessages {
                buf: [0; 10],
                pos: 0,
            },
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
    type Error = error::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        if let Some(msg) = self.next_from_buffer() {
            debug!("Receiving msg: {:?}", msg);
            return Ok(Async::Ready(Some(msg)));
        }
        let next_read = match self.state {
            MessageStreamState::Empty => panic!("Stream is already finished"),
            MessageStreamState::BetweenMessages {
                ref mut buf,
                ref mut pos,
            } => {
                while *pos < buf.len() {
                    let n = try_ready!(self.reader.poll_read(&mut buf[*pos..(*pos + 1)]));
                    if n == 0 {
                        self.state = MessageStreamState::Empty;
                        return Err(
                            io::Error::new(io::ErrorKind::UnexpectedEof, "early eof").into()
                        );
                    }
                    let old_pos = *pos;
                    *pos += n;
                    if buf[old_pos] & 0b10000000 == 0 {
                        break;
                    }
                }
                let mut size_reader: VecDeque<u8> = buf.iter().take(*pos).cloned().collect();
                let size = decode_varint(&mut size_reader)? as usize;
                let buf = vec![0; size];
                self.state = MessageStreamState::MessageInProgress { size, pos: 0, buf };
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
                match NetworkEnvelope::decode(&*buf) {
                    Ok(res) => res,
                    Err(e) => {
                        self.state = MessageStreamState::Empty;
                        debug!("Decode error {:?}", e);
                        return Err(e.into());
                    }
                }
            }
        };
        self.buffer.push_back(next_read);
        self.state = MessageStreamState::BetweenMessages {
            buf: [0; 10],
            pos: 0,
        };
        self.poll()
    }
}
