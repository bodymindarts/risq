use crate::domain::OpenOffer;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
pub struct GetOffers {
    offers: Vec<Offer>,
}
impl From<Vec<OpenOffer>> for GetOffers {
    fn from(offers: Vec<OpenOffer>) -> GetOffers {
        GetOffers {
            offers: offers.into_iter().map(|o| o.into()).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct Offer {
    created_at: SystemTime,
    expires_at: SystemTime,
    id: String,
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
