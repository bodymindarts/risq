use super::offer_book::RefreshOffer;
use super::open_offer::*;
use crate::bisq::{
    payload::{storage_payload, ProtectedStorageEntry, RefreshOfferMessage},
    BisqHash,
};
use bitcoin_hashes::{sha256, Hash};
use std::time::{Duration, SystemTime};

pub fn refresh_offer(msg: RefreshOfferMessage) -> RefreshOffer {
    RefreshOffer {
        sequence: msg.sequence_number,
        bisq_hash: BisqHash::Sha256(
            sha256::Hash::from_slice(&msg.hash_of_payload)
                .expect("Couldn't unwrap RefreshOfferMessage.hash_of_data"),
        ),
    }
}

pub fn open_offer(entry: ProtectedStorageEntry) -> Option<OpenOffer> {
    let created_at =
        SystemTime::UNIX_EPOCH + Duration::from_millis(entry.creation_time_stamp as u64);
    let storage_payload = entry.storage_payload?;
    let hash: BisqHash = (&storage_payload).into();
    if let storage_payload::Message::OfferPayload(payload) = storage_payload.message? {
        Some(OpenOffer::new(
            hash,
            created_at,
            entry.sequence_number,
            payload,
        ))
    } else {
        None
    }
}
