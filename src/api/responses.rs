use crate::domain::{OfferPrice, OpenOffer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetOffers {
    pub offers: Vec<Offer>,
}
impl GetOffers {
    pub fn any(&self) -> bool {
        self.offers.len() > 0
    }
}

impl From<Vec<OpenOffer>> for GetOffers {
    fn from(offers: Vec<OpenOffer>) -> GetOffers {
        GetOffers {
            offers: offers.into_iter().map(|o| o.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Offer {
    pub id: String,
    pub direction: String,
    pub price: Price,
    pub amount: i64,
    pub min_amount: i64,
}
impl From<OpenOffer> for Offer {
    fn from(offer: OpenOffer) -> Offer {
        Offer {
            id: offer.id.into(),
            direction: format!("{:?}", offer.direction),
            price: offer.price.into(),
            amount: offer.amount.total,
            min_amount: offer.amount.min,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub r#type: String,
    pub market_margin: Option<f64>,
    pub fixed: Option<i64>,
}
impl From<OfferPrice> for Price {
    fn from(price: OfferPrice) -> Price {
        match price {
            OfferPrice::Fixed(value) => Price {
                r#type: "fixed".into(),
                market_margin: None,
                fixed: Some(value),
            },
            OfferPrice::MarketWithMargin(value) => Price {
                r#type: "market".into(),
                market_margin: Some(value),
                fixed: None,
            },
        }
    }
}
