mod client;
mod server;
#[cfg(feature = "stats")]
mod stats;

pub mod responses;

pub use client::Client;
pub use server::listen;

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
