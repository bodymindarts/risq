include!("../generated/io.bisq.protobuffer.rs");
include!("../generated/payload_macros.rs");

use super::constants::*;
use crate::error;
use bytes::BytesMut;
use prost::Message;
use rand::{thread_rng, Rng};
use std::{
    io,
    net::{SocketAddr, ToSocketAddrs},
    vec,
};
use tokio::codec::Encoder;

pub fn gen_nonce() -> i32 {
    thread_rng().gen()
}

pub struct PayloadEncoder {
    message_version: MessageVersion,
}
impl PayloadEncoder {
    pub fn from(network: BaseCurrencyNetwork) -> PayloadEncoder {
        PayloadEncoder {
            message_version: network.into(),
        }
    }
}

impl Encoder for PayloadEncoder {
    type Item = network_envelope::Message;
    type Error = error::Error;
    fn encode(&mut self, msg: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let envelope = NetworkEnvelope {
            message_version: self.message_version.into(),
            message: Some(msg),
        };
        envelope.encode_length_delimited(dst).map_err(|e| e.into())
    }
}

impl ToSocketAddrs for NodeAddress {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        (&*self.host_name, self.port as u16).to_socket_addrs()
    }
}

impl From<NodeAddress> for SocketAddr {
    fn from(addr: NodeAddress) -> SocketAddr {
        addr.to_socket_addrs()
            .expect("SocketAddr from NodeAddress")
            .next()
            .expect("SocketAddr from NodeAddress")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MessageVersion(i32);
impl From<MessageVersion> for i32 {
    fn from(msg: MessageVersion) -> i32 {
        msg.0
    }
}
impl From<BaseCurrencyNetwork> for MessageVersion {
    fn from(network: BaseCurrencyNetwork) -> MessageVersion {
        MessageVersion((network as i32) + 10 * P2P_NETWORK_VERSION)
    }
}

macro_rules! into_message {
    ($caml:ident, $snake:ident) => {
        impl From<$caml> for network_envelope::Message {
            fn from(msg: $caml) -> network_envelope::Message {
                network_envelope::Message::$caml(msg)
            }
        }
    };
}
for_all_payloads!(into_message);
