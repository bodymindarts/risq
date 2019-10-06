use super::{message::*, *};
use crate::bisq::BisqHash;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, MessageResult};
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

impl Handler<AddOffer> for OfferBook {
    type Result = ();
    fn handle(&mut self, AddOffer(offer): AddOffer, _ctx: &mut Self::Context) {
        if !offer.is_expired() {
            self.open_offers.insert(offer.bisq_hash, offer);
        }
    }
}
impl Handler<RefreshOffer> for OfferBook {
    type Result = ();
    fn handle(
        &mut self,
        RefreshOffer {
            bisq_hash,
            sequence,
        }: RefreshOffer,
        _ctx: &mut Self::Context,
    ) {
        if let Some(offer) = self.open_offers.get_mut(&bisq_hash) {
            offer.refresh(sequence);
        }
    }
}

impl Handler<GetOpenOffers> for OfferBook {
    type Result = MessageResult<GetOpenOffers>;
    fn handle(&mut self, _: GetOpenOffers, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.open_offers.values().cloned().collect())
    }
}
