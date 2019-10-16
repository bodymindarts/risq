use super::payload::*;
use crate::prelude::{ripemd160, sha256, Hash};
use prost::Message;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BisqHash {
    Sha256(sha256::Hash),
    RIPEMD160(ripemd160::Hash),
}
impl From<&BisqHash> for Vec<u8> {
    fn from(hash: &BisqHash) -> Vec<u8> {
        match hash {
            BisqHash::Sha256(hash) => hash.into_inner().to_vec(),
            BisqHash::RIPEMD160(hash) => hash.into_inner().to_vec(),
        }
    }
}

impl TryFrom<&ProtectedStorageEntry> for BisqHash {
    type Error = ();
    fn try_from(entry: &ProtectedStorageEntry) -> Result<BisqHash, Self::Error> {
        entry.storage_payload.as_ref().map(BisqHash::from).ok_or(())
    }
}

impl From<&StoragePayload> for BisqHash {
    fn from(payload: &StoragePayload) -> BisqHash {
        let mut serialized = Vec::with_capacity(payload.encoded_len());
        payload
            .encode(&mut serialized)
            .expect("Could not encode message");
        BisqHash::Sha256(sha256::Hash::hash(&serialized))
    }
}

impl From<&PersistableNetworkPayload> for BisqHash {
    fn from(payload: &PersistableNetworkPayload) -> BisqHash {
        let inner = match payload
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
