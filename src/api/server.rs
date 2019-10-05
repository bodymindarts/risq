use super::responses::*;
use crate::domain::offer_book::*;
use actix::Addr;
use actix_web::{
    web::{self, Data},
    App, Error, HttpServer, Result,
};
use std::io;
use tokio::prelude::future::Future;

pub fn listen(port: u16, offer_book: Addr<OfferBook>) -> Result<(), io::Error> {
    let data = web::Data::new(offer_book);
    HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .route("/ping", web::get().to(|| "pong"))
            .route("/offers", web::get().to(get_offers))
    })
    .workers(1)
    .bind(("127.0.0.1", port))?
    .start();
    Ok(())
}

type FutureJsonResponse<T> = Box<dyn Future<Item = web::Json<T>, Error = Error>>;

fn get_offers(data: Data<Addr<OfferBook>>) -> FutureJsonResponse<GetOffers> {
    Box::new(
        data.get_ref()
            .send(GetOpenOffers)
            .map(|offers| web::Json(GetOffers::from(offers)))
            .map_err(|e| e.into()),
    )
}
