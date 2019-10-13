use super::responses::*;
use crate::{
    domain::offer::{message::GetOpenOffers, OfferBook},
    prelude::*,
    stats::*,
};
use actix_web::{
    web::{self, Data},
    App, Error, HttpServer, Result,
};
use std::io;

pub fn listen(
    port: u16,
    offer_book: Addr<OfferBook>,
    stats_log: Option<StatsLog>,
) -> Result<(), io::Error> {
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
            app.data((
                schema.clone().unwrap(),
                stats_log.clone().expect("No stats log"),
            ))
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
