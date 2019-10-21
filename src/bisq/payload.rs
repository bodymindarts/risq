include!("../generated/io.bisq.protobuffer.rs");
include!("../generated/payload_macros.rs");

pub mod kind;

use super::{constants::*, hash::*};
use crate::prelude::{ripemd160, sha256, Hash};
use openssl::{dsa::Dsa, hash::MessageDigest, pkey::*, sign::Verifier};
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
        BisqHash::Sha256(self.sha256())
    }

    fn signing_pub_key_bytes(&self) -> Option<&Vec<u8>> {
        match self.message.as_ref()? {
            storage_payload::Message::Alert(alert) => &alert.owner_pub_key_bytes,
            storage_payload::Message::Arbitrator(arb) => {
                &arb.pub_key_ring.as_ref()?.signature_pub_key_bytes
            }

            storage_payload::Message::Mediator(med) => {
                &med.pub_key_ring.as_ref()?.signature_pub_key_bytes
            }
            storage_payload::Message::Filter(filter) => &filter.owner_pub_key_bytes,
            storage_payload::Message::TradeStatistics(trade) => &trade.signature_pub_key_bytes,
            storage_payload::Message::MailboxStoragePayload(payload) => {
                &payload.sender_pub_key_for_add_operation_bytes
            }
            storage_payload::Message::OfferPayload(offer) => {
                &offer.pub_key_ring.as_ref()?.signature_pub_key_bytes
            }
            storage_payload::Message::TempProposalPayload(payload) => {
                &payload.owner_pub_key_encoded
            }
        }
        .into()
    }
}
impl ProtectedStorageEntry {
    fn owner_pub_key(&self) -> Option<PKey<Public>> {
        PKey::from_dsa(Dsa::public_key_from_der(&self.owner_pub_key_bytes).ok()?).ok()
    }
    pub fn verify(&self) -> Option<BisqHash> {
        let payload = self.storage_payload.as_ref()?;
        if payload.signing_pub_key_bytes()? != &self.owner_pub_key_bytes {
            warn!("Invalid public key in ProtectedStorageEntry");
            return None;
        }
        let pub_key = self.owner_pub_key()?;
        let verifier = Verifier::new_without_digest(&pub_key).ok()?;
        let hash = DataAndSeqNrPair {
            payload: Some(payload.clone()),
            sequence_number: self.sequence_number,
        }
        .sha256();
        verifier
            .verify_oneshot(&self.signature, &hash.into_inner())
            .ok()
            .and_then(|verified| {
                if verified {
                    Some(payload.bisq_hash())
                } else {
                    warn!(
                        "Detected invalid signature in ProtectedStorageEntry {:?}",
                        payload.bisq_hash()
                    );
                    None
                }
            })
    }
}
impl RefreshOfferMessage {
    pub fn payload_hash(&self) -> BisqHash {
        BisqHash::Sha256(
            sha256::Hash::from_slice(&self.hash_of_payload)
                .expect("Couldn't unwrap RefreshOfferMessage.hash_of_data"),
        )
    }
    pub fn verify(&self, owner_pub_key: &[u8], original_payload: &StoragePayload) -> Option<()> {
        let hash = DataAndSeqNrPair {
            payload: Some(original_payload.clone()),
            sequence_number: self.sequence_number,
        }
        .sha256();
        if hash.into_inner() != &*self.hash_of_data_and_seq_nr {
            warn!("Error with RefreshOfferMessage.hash_of_data_and_seq_nr");
            return None;
        }
        let pub_key = PKey::from_dsa(Dsa::public_key_from_der(owner_pub_key).ok()?).ok()?;
        let verifier = Verifier::new_without_digest(&pub_key).ok()?;
        verifier
            .verify_oneshot(&self.signature, &hash.into_inner())
            .ok()
            .and_then(|verified| {
                if verified {
                    Some(())
                } else {
                    warn!(
                        "Detected invalid signature in RefreshOfferMessage {:?}",
                        self.payload_hash()
                    );
                    None
                }
            })
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
