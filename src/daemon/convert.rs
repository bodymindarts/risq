use crate::{
    bisq::{
        payload::{
            offer_payload, persistable_network_payload, storage_payload, PersistableNetworkPayload,
            ProtectedStorageEntry, RefreshOfferMessage,
        },
        SequencedMessageHash,
    },
    domain::{
        amount::NumberWithPrecision,
        currency::Currency,
        market::Market,
        offer::{message::*, *},
        statistics,
    },
    prelude::{sha256, Hash},
};
use std::{
    convert::TryFrom,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

impl TryFrom<offer_payload::Direction> for OfferDirection {
    type Error = ();
    fn try_from(direction: offer_payload::Direction) -> Result<OfferDirection, Self::Error> {
        match direction {
            offer_payload::Direction::Buy => Ok(OfferDirection::Buy),
            offer_payload::Direction::Sell => Ok(OfferDirection::Sell),
            _ => Err(()),
        }
    }
}

pub fn refresh_offer(msg: &RefreshOfferMessage) -> RefreshOffer {
    RefreshOffer {
        sequence: msg.sequence_number.into(),
        bisq_hash: SequencedMessageHash::new(
            sha256::Hash::from_slice(&msg.hash_of_payload)
                .expect("Couldn't unwrap RefreshOfferMessage.hash_of_data"),
        ),
    }
}

pub fn open_offer(entry: ProtectedStorageEntry, hash: SequencedMessageHash) -> Option<OpenOffer> {
    let created_at =
        SystemTime::UNIX_EPOCH + Duration::from_millis(entry.creation_time_stamp as u64);
    let storage_payload = entry.storage_payload?;
    if let storage_payload::Message::OfferPayload(payload) = storage_payload.message? {
        let direction = offer_payload::Direction::from_i32(payload.direction)
            .ok_or(())
            .and_then(OfferDirection::try_from)
            .ok()?;
        let base = if let Some(currency) = Currency::from_code(&payload.base_currency_code) {
            currency
        } else {
            warn!(
                "Unsupported base currency in offer '{}'",
                payload.base_currency_code
            );
            return None;
        };
        let counter = if let Some(currency) = Currency::from_code(&payload.counter_currency_code) {
            currency
        } else {
            warn!(
                "Unsupported currency in offer '{}'",
                payload.counter_currency_code
            );
            return None;
        };
        let price = if payload.use_market_based_price {
            OfferPrice::MarketWithMargin(payload.market_price_margin)
        } else {
            OfferPrice::Fixed(NumberWithPrecision::new(
                payload.price as u64,
                counter.bisq_internal_precision(),
            ))
        };
        let market = Market::from_currency_pair(base, counter)?;
        Some(OpenOffer::new(
            hash,
            market,
            payload.id.into(),
            direction,
            price,
            OfferAmount {
                total: NumberWithPrecision::new(
                    payload.amount as u64,
                    base.bisq_internal_precision(),
                ),
                min: NumberWithPrecision::new(
                    payload.min_amount as u64,
                    base.bisq_internal_precision(),
                ),
            },
            payload.payment_method_id,
            payload.offer_fee_payment_tx_id,
            created_at,
            entry.sequence_number.into(),
        ))
    } else {
        None
    }
}

#[cfg(feature = "statistics")]
pub fn trade_statistics2(payload: PersistableNetworkPayload) -> Option<statistics::Trade> {
    let hash = payload.bisq_hash();
    if let persistable_network_payload::Message::TradeStatistics2(payload) = payload.message? {
        if payload.trade_price <= 0 || payload.trade_amount <= 0 {
            return None;
        }
        let direction = offer_payload::Direction::from_i32(payload.direction)
            .ok_or(())
            .and_then(OfferDirection::try_from)
            .ok()?;
        let base = Currency::from_code(&payload.base_currency)?;
        let counter = Currency::from_code(&payload.counter_currency)?;
        let market = Market::from_currency_pair(base, counter)?;
        Some(statistics::Trade::new(
            market,
            direction,
            payload.offer_id.into(),
            NumberWithPrecision::new(
                payload.trade_price as u64,
                counter.bisq_internal_precision(),
            ),
            NumberWithPrecision::new(payload.trade_amount as u64, base.bisq_internal_precision()),
            payload.payment_method_id,
            UNIX_EPOCH + Duration::from_millis(payload.trade_date as u64),
            hash,
        ))
    } else {
        None
    }
}
