use crate::prelude::{ripemd160, sha256, Hash};
use prost::Message;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BisqHash {
    Sha256(sha256::Hash),
    RIPEMD160(ripemd160::Hash),
}
impl From<BisqHash> for Vec<u8> {
    fn from(hash: BisqHash) -> Vec<u8> {
        match hash {
            BisqHash::Sha256(hash) => hash.into_inner().to_vec(),
            BisqHash::RIPEMD160(hash) => hash.into_inner().to_vec(),
        }
    }
}

pub trait Sha256: Message + Sized {
    fn sha256(&self) -> sha256::Hash {
        let mut serialized = Vec::with_capacity(self.encoded_len());
        self.encode(&mut serialized)
            .expect("Could not encode message");
        sha256::Hash::hash(&serialized)
    }
}

impl<T> Sha256 for T where T: Message + Sized {}
