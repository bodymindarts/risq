use super::{message::*, *};
use crate::{bisq::BisqHash, domain::CommandResult, prelude::*};
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
    type Result = MessageResult<AddOffer>;
    fn handle(&mut self, AddOffer(offer): AddOffer, _ctx: &mut Self::Context) -> Self::Result {
        if !offer.is_expired() {
            if let None = self.open_offers.get(&offer.bisq_hash) {
                info!("Adding offer");
                self.open_offers.insert(offer.bisq_hash, offer);
                return MessageResult(CommandResult::Accepted);
            }
        }
        MessageResult(CommandResult::Ignored)
    }
}
impl Handler<RefreshOffer> for OfferBook {
    type Result = MessageResult<RefreshOffer>;
    fn handle(
        &mut self,
        RefreshOffer {
            bisq_hash,
            sequence,
        }: RefreshOffer,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        if let Some(offer) = self.open_offers.get_mut(&bisq_hash) {
            if offer.refresh(sequence) {
                return MessageResult(CommandResult::Accepted);
            }
        }
        MessageResult(CommandResult::Ignored)
    }
}

impl Handler<GetOpenOffers> for OfferBook {
    type Result = MessageResult<GetOpenOffers>;
    fn handle(&mut self, _: GetOpenOffers, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.open_offers.values().cloned().collect())
    }
}
