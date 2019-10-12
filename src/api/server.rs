use super::responses::*;
use super::stats::*;
use crate::{
    domain::offer::{message::GetOpenOffers, OfferBook},
    prelude::*,
};
use actix_web::{
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Result,
};
use std::{io, sync::Arc};

pub fn listen(port: u16, offer_book: Addr<OfferBook>) -> Result<(), io::Error> {
    let data = web::Data::new(offer_book);
    let schema = if cfg!(feature = "stats") {
        Some(std::sync::Arc::new(create_schema()))
    } else {
        None
    };
    HttpServer::new(move || {
        let app = App::new()
            .register_data(data.clone())
            .route("/ping", web::get().to(|| "pong"))
            .route("/offers", web::get().to_async(get_offers));

        if cfg!(feature = "stats") {
            app.data(schema.clone().unwrap())
                .service(web::resource("/graphql").route(web::post().to_async(graphql)))
                .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        } else {
            app
        }
    })
    .bind(("127.0.0.1", port))?
    .start();
    Ok(())
}

fn get_offers(
    data: Data<Addr<OfferBook>>,
) -> impl Future<Item = web::Json<GetOffers>, Error = Error> {
    data.get_ref()
        .send(GetOpenOffers)
        .map(|offers| web::Json(GetOffers::from(offers)))
        .map_err(|e| e.into())
}

#[cfg(feature = "stats")]
fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}
#[cfg(feature = "stats")]
fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:7477/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
