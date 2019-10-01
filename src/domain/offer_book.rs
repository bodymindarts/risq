use super::open_offer::OpenOffer;
use crate::bisq::BisqHash;
use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;

pub struct OfferBook {
    open_offers: HashMap<BisqHash, OpenOffer>,
}
impl Actor for OfferBook {
    type Context = Context<Self>;
}
impl OfferBook {
    pub fn start() -> Addr<OfferBook> {
        OfferBook {
            open_offers: HashMap::new(),
        }
        .start()
    }
}

pub struct AddOffer(pub OpenOffer);
impl Message for AddOffer {
    type Result = ();
}
impl Handler<AddOffer> for OfferBook {
    type Result = ();
    fn handle(&mut self, AddOffer(offer): AddOffer, _ctx: &mut Self::Context) {
        self.open_offers.insert(offer.bisq_hash, offer);
        debug!("Inserted offer into OfferBook");
    }
}
