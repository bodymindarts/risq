use crate::domain::offer::OpenOffer;
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
}
impl From<OpenOffer> for Offer {
    fn from(offer: OpenOffer) -> Offer {
        Offer {
            id: offer.id.into(),
            direction: format!("{:?}", offer.direction),
        }
    }
}
