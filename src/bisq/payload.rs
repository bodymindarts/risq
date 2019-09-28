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

pub fn gen_nonce() -> i32 {
    thread_rng().gen()
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

pub enum Extract<P> {
    Succeeded(P),
    Failed(network_envelope::Message),
}
pub trait PayloadExtractor {
    type Payload: Send;
    fn extract(msg: network_envelope::Message) -> Extract<Self::Payload>;
}

macro_rules! extractor {
    ($caml:ident, $snake:ident) => {
        impl PayloadExtractor for $caml {
            type Payload = $caml;
            fn extract(msg: network_envelope::Message) -> Extract<Self::Payload> {
                if let network_envelope::Message::$caml(request) = msg {
                    Extract::Succeeded(request)
                } else {
                    Extract::Failed(msg)
                }
            }
        }
    };
}
for_all_payloads!(extractor);
