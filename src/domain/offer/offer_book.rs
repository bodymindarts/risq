use super::{message::*, *};
use crate::{
    bisq::BisqHash,
    domain::{price_feed::*, CommandResult},
    prelude::*,
};
use std::{collections::HashMap, sync::Arc, time::Duration};

const CHECK_TTL_INTERVAL: Duration = Duration::from_secs(40);

pub struct OfferBook {
    open_offers: Arc<HashMap<BisqHash, OpenOffer>>,
    price_feed: Addr<PriceFeed>,
    price_data: Arc<HashMap<&'static str, PriceData>>,
}
impl Actor for OfferBook {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(CHECK_TTL_INTERVAL, |offer_book, ctx| {
            ctx.spawn(
                fut::wrap_future(offer_book.price_feed.send(GetCurrentPrices)).then(
                    |maybe_data, offer_book: &mut OfferBook, _| {
                        if let Ok(price_data) = maybe_data {
                            offer_book.price_data = price_data;
                        }
                        let open_offers = offer_book
                            .open_offers
                            .iter()
                            .filter_map(|(hash, offer)| {
                                if offer.is_expired() {
                                    None
                                } else {
                                    let mut offer = offer.clone();
                                    offer.update_display_price(&offer_book.price_data);
                                    Some((hash.clone(), offer))
                                }
                            })
                            .collect();
                        offer_book.open_offers = Arc::new(open_offers);
                        fut::ok(())
                    },
                ),
            );
        });
    }
}
impl OfferBook {
    pub fn start(price_feed: Addr<PriceFeed>) -> Addr<OfferBook> {
        OfferBook {
            open_offers: Arc::new(HashMap::new()),
            price_feed,
            price_data: Arc::new(HashMap::new()),
        }
        .start()
    }
}

impl Handler<AddOffer> for OfferBook {
    type Result = MessageResult<AddOffer>;
    fn handle(&mut self, AddOffer(mut offer): AddOffer, _ctx: &mut Self::Context) -> Self::Result {
        if !offer.is_expired() {
            if let None = self.open_offers.get(&offer.bisq_hash) {
                info!("Adding offer");
                offer.update_display_price(&self.price_data);
                let offers = Arc::make_mut(&mut self.open_offers);
                offers.insert(offer.bisq_hash, offer);
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
        if let Some(offer) = self.open_offers.get(&bisq_hash) {
            if offer.would_refresh(sequence) {
                let offers = Arc::make_mut(&mut self.open_offers);
                let offer = offers.get_mut(&bisq_hash).unwrap();
                if offer.refresh(sequence) {
                    return MessageResult(CommandResult::Accepted);
                }
            }
        }
        MessageResult(CommandResult::Ignored)
    }
}

impl Handler<GetOpenOffers> for OfferBook {
    type Result = MessageResult<GetOpenOffers>;
    fn handle(&mut self, _: GetOpenOffers, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(Arc::clone(&self.open_offers))
    }
}
