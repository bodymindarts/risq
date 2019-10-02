use super::open_offer::OpenOffer;
use crate::bisq::BisqHash;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use std::{collections::HashMap, time::Duration};

const CHECK_TTL_INTERVAL: Duration = Duration::from_secs(60);

pub struct OfferBook {
    open_offers: HashMap<BisqHash, OpenOffer>,
}
impl Actor for OfferBook {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(CHECK_TTL_INTERVAL, |offer_book, _ctx| {
            offer_book
                .open_offers
                .retain(|_, offer| !offer.is_expired());
        });
    }
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
        if !offer.is_expired() {
            if !offer.is_expired() {
                self.open_offers.insert(offer.bisq_hash, offer);
            }
        }
    }
}
pub struct RefreshOffer {
    pub bisq_hash: BisqHash,
    pub sequence: i32,
}
impl Message for RefreshOffer {
    type Result = ();
}
impl Handler<RefreshOffer> for OfferBook {
    type Result = ();
    fn handle(
        &mut self,
        RefreshOffer {
            ref bisq_hash,
            sequence,
        }: RefreshOffer,
        _ctx: &mut Self::Context,
    ) {
        if let Some(offer) = self.open_offers.get_mut(bisq_hash) {
            offer.refresh(sequence);
        }
    }
}
