use crate::prelude::{ripemd160, sha256, Hash};

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
