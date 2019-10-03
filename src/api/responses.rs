use crate::domain::OpenOffer;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct GetOffers {
    pub offers: Vec<Offer>,
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
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub id: String,
}

impl From<OpenOffer> for Offer {
    fn from(offer: OpenOffer) -> Offer {
        Offer {
            created_at: offer.created_at,
            expires_at: offer.expires_at,
            id: offer.id().clone(),
        }
    }
}
