use crate::{
    bisq::{
        payload::{offer_payload, storage_payload, ProtectedStorageEntry, RefreshOfferMessage},
        BisqHash,
    },
    domain::offer::{message::*, *},
};
use bitcoin_hashes::{sha256, Hash};
use std::time::{Duration, SystemTime};

pub fn refresh_offer(msg: &RefreshOfferMessage) -> RefreshOffer {
    RefreshOffer {
        sequence: msg.sequence_number.into(),
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
        let direction = match offer_payload::Direction::from_i32(payload.direction) {
            Some(offer_payload::Direction::Buy) => Some(OfferDirection::Buy),
            Some(offer_payload::Direction::Sell) => Some(OfferDirection::Sell),
            _ => None,
        }?;
        let price = if payload.use_market_based_price {
            OfferPrice::MarketWithMargin(payload.market_price_margin)
        } else {
            OfferPrice::Fixed(payload.price)
        };
        Some(OpenOffer::new(
            payload.clone(),
            hash,
            payload.id.into(),
            direction,
            price,
            OfferAmount {
                total: payload.amount,
                min: payload.min_amount,
            },
            created_at,
            entry.sequence_number.into(),
        ))
    } else {
        None
    }
}
