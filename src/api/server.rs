use super::{graphql::*, responses::*};
use crate::{
    domain::{
        offer::{message::GetOpenOffers, OfferBook},
        statistics::*,
    },
    prelude::*,
};
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, Error, HttpServer, Result,
};
use std::io;

pub fn listen(
    port: u16,
    offer_book: Addr<OfferBook>,
    stats_cache: Option<StatsCache>,
) -> Result<(), io::Error> {
    let gql_context = GraphQLContextWrapper {
        #[cfg(feature = "statistics")]
        stats_cache: stats_cache.expect("StatsCache empty"),
        offer_book,
    };
    let schema = if cfg!(feature = "statistics") {
        Some(std::sync::Arc::new(create_schema()))
    } else {
        None
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/ping", web::get().to(|| "pong"))
            .service(
                web::resource("/graphql")
                    .data(schema.clone().unwrap())
                    .data(gql_context.clone())
                    .route(web::post().to_async(graphql)),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(("127.0.0.1", port))?
    .start();
    Ok(())
}

// fn get_offers(
//     data: Data<Addr<OfferBook>>,
// ) -> impl Future<Item = web::Json<GetOffers>, Error = Error> {
//     data.get_ref()
//         .send(GetOpenOffers)
//         .map(|offers| web::Json(GetOffers::from(offers)))
//         .map_err(|e| e.into())
// }
