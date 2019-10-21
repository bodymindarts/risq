include!("../generated/io.bisq.protobuffer.rs");
include!("../generated/payload_macros.rs");

pub mod kind;

use super::{constants::*, hash::*};
use crate::prelude::{ripemd160, sha256, Hash};
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

impl StoragePayload {
    pub fn bisq_hash(&self) -> BisqHash {
        let mut serialized = Vec::with_capacity(self.encoded_len());
        self.encode(&mut serialized)
            .expect("Could not encode message");
        BisqHash::Sha256(sha256::Hash::hash(&serialized))
    }
}
impl ProtectedStorageEntry {
    pub fn verify(&self) -> Option<BisqHash> {
        let payload = self.storage_payload.as_ref()?;
        Some(payload.bisq_hash())
    }
}
impl PersistableNetworkPayload {
    pub fn bisq_hash(&self) -> BisqHash {
        let inner = match self
            .message
            .as_ref()
            .expect("PersistableNetworkPayload doesn't have message attached")
        {
            persistable_network_payload::Message::AccountAgeWitness(witness) => {
                ripemd160::Hash::from_slice(&witness.hash)
                    .expect("AccountAgeWitness.hash is not correct")
            }
            persistable_network_payload::Message::TradeStatistics2(stats) => {
                ripemd160::Hash::from_slice(&stats.hash)
                    .expect("TradeStatistics2.hash is not correct")
            }
            persistable_network_payload::Message::ProposalPayload(prop) => {
                ripemd160::Hash::from_slice(&prop.hash)
                    .expect("ProposalPayload.hash is not correct")
            }
            persistable_network_payload::Message::BlindVotePayload(vote) => {
                ripemd160::Hash::from_slice(&vote.hash)
                    .expect("BlindVotePayload.hash is not correct")
            }
            persistable_network_payload::Message::SignedWitness(witness) => {
                let mut data = witness.witness_hash.clone();
                data.extend_from_slice(&witness.signature);
                data.extend_from_slice(&witness.signer_pub_key);
                let hash = sha256::Hash::hash(&data);
                ripemd160::Hash::hash(&hash.into_inner())
            }
        };
        BisqHash::RIPEMD160(inner)
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
    type Extraction: Send;
    fn extract(msg: network_envelope::Message) -> Extract<Self::Extraction>;
}

macro_rules! extractor {
    ($caml:ident, $snake:ident) => {
        impl PayloadExtractor for $caml {
            type Extraction = $caml;
            fn extract(msg: network_envelope::Message) -> Extract<Self::Extraction> {
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
