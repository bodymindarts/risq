use crate::bisq::payload::*;
use actix::{Actor, Context};

struct OfferBook {}
impl Actor for OfferBook {
    type Context = Context<Self>;
}
