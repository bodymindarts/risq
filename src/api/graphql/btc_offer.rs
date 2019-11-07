use crate::domain::{amount::*, offer::*};

pub struct BtcOffer<'a> {
    inner: &'a OpenOffer,
}

impl<'a> BtcOffer<'a> {
    pub fn new(offer: &'a OpenOffer) -> Self {
        Self { inner: offer }
    }

    pub fn direction(&self) -> OfferDirection {
        if self.inner.market.non_btc_side().is_crypto() {
            self.inner.direction.oposite()
        } else {
            self.inner.direction
        }
    }

    pub fn volume(&self) -> NumberWithPrecision {
        if self.inner.market.non_btc_side().is_crypto() {
            self.inner.amount.total
        } else {
            self.inner.display_price * self.inner.amount.total
        }
    }

    pub fn amount(&self) -> NumberWithPrecision {
        if self.inner.market.non_btc_side().is_crypto() {
            self.inner.amount.total / self.inner.display_price
        } else {
            self.inner.amount.total
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bisq::SequencedMessageHash,
        domain::{amount::*, market::*, offer::*},
    };
    use bitcoin_hashes::sha256;
    use std::{str::FromStr, time::UNIX_EPOCH};

    fn fiat_offer() -> OpenOffer {
        OpenOffer::new(
            SequencedMessageHash::new(
                sha256::Hash::from_str(
                    &"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                )
                .unwrap(),
            ),
            Market::from_pair("btc_eur").unwrap(),
            "fiat-offer".to_string().into(),
            OfferDirection::Buy,
            OfferPrice::Fixed(NumberWithPrecision::new(1000, 0)),
            OfferAmount {
                total: NumberWithPrecision::new(1, 0),
                min: NumberWithPrecision::new(5, 1),
            },
            "PAYMENT_METHOD".into(),
            "OFFER_FEE_TX_ID".into(),
            UNIX_EPOCH,
            0.into(),
        )
    }

    fn crypto_offer() -> OpenOffer {
        OpenOffer::new(
            SequencedMessageHash::new(
                sha256::Hash::from_str(
                    &"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                )
                .unwrap(),
            ),
            Market::from_pair("bsq_btc").unwrap(),
            "fiat-offer".to_string().into(),
            OfferDirection::Sell,
            OfferPrice::Fixed(NumberWithPrecision::new(7800, 8)),
            OfferAmount {
                total: NumberWithPrecision::new(5, 1),
                min: NumberWithPrecision::new(5, 1),
            },
            "PAYMENT_METHOD".into(),
            "OFFER_FEE_TX_ID".into(),
            UNIX_EPOCH,
            0.into(),
        )
    }

    #[test]
    fn test_fiat_offer() {
        let fiat_offer = fiat_offer();
        let btc_offer = BtcOffer::new(&fiat_offer);
        assert!(btc_offer.direction() == OfferDirection::Buy);
        assert!(btc_offer.volume() == NumberWithPrecision::new(1000, 0));
        assert!(btc_offer.amount() == NumberWithPrecision::new(1, 0));
    }

    #[test]
    fn test_crypto_offer() {
        let crypto_offer = crypto_offer();
        let btc_offer = BtcOffer::new(&crypto_offer);
        assert!(btc_offer.direction() == OfferDirection::Buy);
        assert!(btc_offer.volume() == NumberWithPrecision::new(5, 1));
        assert!(btc_offer.amount() == NumberWithPrecision::new(641025641025, 8));
    }
}
