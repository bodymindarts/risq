mod responses;

use crate::domain::offer_book::*;
use actix::Addr;
use actix_web::{
    web::{self, Data},
    App, HttpServer, Result,
};
use responses::*;
use std::io;
use tokio::prelude::future::{self, Future};

pub fn listen(port: u16, offer_book: Addr<OfferBook>) -> Result<(), io::Error> {
    let data = web::Data::new(offer_book);
    HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .route("/ping", web::get().to(|| "pong"))
            .route("/offers", web::get().to(get_offers))
    })
    .bind(("127.0.0.1", port))?
    .start();
    Ok(())
}

fn get_offers(Data: Data<Addr<OfferBook>>) -> Result<web::Json<GetOffers>> {
    Ok(web::Json(GetOffers {
        id: "ashoten".into(),
    }))
}
