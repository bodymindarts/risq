use super::{message::*, *};
use crate::bisq::{payload::OfferPayload, BisqHash};
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, MessageResult};
use std::{collections::HashMap, time::Duration};

const CHECK_TTL_INTERVAL: Duration = Duration::from_secs(60);

pub struct OfferBook {
    open_offers: HashMap<BisqHash, OpenOffer>,
    offer_ids: HashMap<OfferId, (OfferPayload, Vec<u8>)>,
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
            offer_ids: HashMap::new(),
        }
        .start()
    }
}

impl Handler<AddOffer> for OfferBook {
    type Result = MessageResult<AddOffer>;
    fn handle(
        &mut self,
        AddOffer(offer, bytes): AddOffer,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        if let None = self.open_offers.get(&offer.bisq_hash) {
            if !offer.is_expired() {
                info!("Adding {:?}, {:?}", offer.id, offer.bisq_hash);
                self.open_offers.insert(offer.bisq_hash, offer.clone());
                if let Some((old_payload, old_bytes)) = self.offer_ids.get(&offer.id) {
                    if &bytes != old_bytes && &offer.payload == old_payload {
                        warn!("payload already exists: {:?}", old_bytes);
                        warn!("Now it is: {:?}", bytes);
                    }
                }
                self.offer_ids
                    .insert(offer.id.clone(), (offer.payload.clone(), bytes));
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
