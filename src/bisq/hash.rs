use crate::prelude::{ripemd160, sha256, Hash};
use prost::Message;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PersistentMessageHash(ripemd160::Hash);
impl PersistentMessageHash {
    pub fn new(inner: ripemd160::Hash) -> Self {
        PersistentMessageHash(inner)
    }
}
impl From<PersistentMessageHash> for Vec<u8> {
    fn from(hash: PersistentMessageHash) -> Vec<u8> {
        hash.0.into_inner().to_vec()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SequencedMessageHash(sha256::Hash);
impl SequencedMessageHash {
    pub fn new(inner: sha256::Hash) -> Self {
        SequencedMessageHash(inner)
    }
}
impl From<SequencedMessageHash> for Vec<u8> {
    fn from(hash: SequencedMessageHash) -> Vec<u8> {
        hash.0.into_inner().to_vec()
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
