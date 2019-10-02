use super::offer_book::RefreshOffer;
use crate::bisq::{payload::*, BisqHash};
use bitcoin_hashes::{ripemd160, Hash};

pub fn refresh_offer(msg: RefreshOfferMessage) -> RefreshOffer {
    RefreshOffer {
        sequence: msg.sequence_number,
        bisq_hash: BisqHash::RIPEMD160(
            ripemd160::Hash::from_slice(&msg.hash_of_payload)
                .expect("Couldn't unwrap RefreshOfferMessage.hash_of_data"),
        ),
    }
}
