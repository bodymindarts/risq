use super::{open_offer::OfferSequence, OpenOffer};
use crate::bisq::BisqHash;
use actix::Message;

pub enum CommandResult {
    Accepted,
    Ignored,
}

pub struct AddOffer(pub OpenOffer, pub Vec<u8>);
impl Message for AddOffer {
    type Result = CommandResult;
}

pub struct RefreshOffer {
    pub bisq_hash: BisqHash,
    pub sequence: OfferSequence,
}
impl Message for RefreshOffer {
    type Result = CommandResult;
}

pub struct GetOpenOffers;
impl Message for GetOpenOffers {
    type Result = Vec<OpenOffer>;
}
