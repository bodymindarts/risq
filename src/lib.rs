mod api;
#[macro_use]
mod bisq;
#[cfg(feature = "checker")]
mod checker;
mod daemon;
mod domain;
mod error;
mod p2p;
mod prelude;
#[cfg(feature = "stats")]
mod stats;

pub mod cli;

#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;

#[cfg(not(feature = "stats"))]
mod stats {
    use crate::prelude::*;
    use actix_web::{Error, HttpResponse};

    pub struct Schema;
    pub fn create_schema() -> Schema {
        Schema
    }
    pub fn graphql() -> impl Future<Item = HttpResponse, Error = Error> {
        future::ok(HttpResponse::Ok().finish())
    }
    pub fn graphiql() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
