use super::{open_offer::OfferSequence, OpenOffer};
use crate::{bisq::SequencedMessageHash, domain::CommandResult, prelude::Message};
use std::{collections::HashMap, sync::Arc};

pub struct AddOffer(pub OpenOffer);
impl Message for AddOffer {
    type Result = CommandResult;
}

pub struct RefreshOffer {
    pub bisq_hash: SequencedMessageHash,
    pub sequence: OfferSequence,
}
impl Message for RefreshOffer {
    type Result = CommandResult;
}

pub struct GetOpenOffers;
impl Message for GetOpenOffers {
    type Result = Arc<HashMap<SequencedMessageHash, OpenOffer>>;
}
