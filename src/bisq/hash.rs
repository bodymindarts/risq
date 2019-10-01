use super::payload::*;
use bitcoin_hashes::{ripemd160, sha256, Hash};
use prost::Message;

#[derive(PartialEq, Eq, Hash)]
pub struct BisqHash(Vec<u8>);
impl BisqHash {
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}
impl From<&StoragePayload> for BisqHash {
    fn from(payload: &StoragePayload) -> BisqHash {
        let mut serialized = Vec::with_capacity(payload.encoded_len());
        payload
            .encode(&mut serialized)
            .expect("Could not encode message");
        let hash = sha256::Hash::hash(&serialized);
        let mut ret = Vec::with_capacity(sha256::Hash::LEN);
        ret.extend_from_slice(&hash.into_inner());
        BisqHash(ret)
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
                witness.hash.clone()
            }
            persistable_network_payload::Message::TradeStatistics2(stats) => stats.hash.clone(),
            persistable_network_payload::Message::ProposalPayload(prop) => prop.hash.clone(),
            persistable_network_payload::Message::BlindVotePayload(vote) => vote.hash.clone(),
            persistable_network_payload::Message::SignedWitness(witness) => {
                let mut data = witness.witness_hash.clone();
                data.append(&mut witness.signature.clone());
                data.append(&mut witness.signer_pub_key.clone());
                let hash = sha256::Hash::hash(&data);
                let mut ret = Vec::with_capacity(20);
                ret.extend_from_slice(&ripemd160::Hash::hash(&hash.into_inner()).into_inner());
                ret
            }
        };
        BisqHash(inner)
    }
}
