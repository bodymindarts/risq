use super::graphql::*;
use crate::{
    bisq::NodeAddress,
    domain::{offer::OfferBook, statistics::*},
    p2p::status::*,
    prelude::*,
};
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use std::{collections::HashMap, io, time::UNIX_EPOCH};

#[allow(unused_variables)]
pub fn listen(
    port: u16,
    offer_book: Addr<OfferBook>,
    status: Status,
    stats_cache: Option<StatsCache>,
) -> Result<(), io::Error> {
    let gql_context = GraphQLContextWrapper {
        #[cfg(feature = "statistics")]
        stats_cache: stats_cache.unwrap(),
        offer_book,
    };
    listen_with_context(port, status, gql_context)
}

fn listen_with_context(
    port: u16,
    p2p_status: Status,
    gql_context: GraphQLContextWrapper,
) -> Result<(), io::Error> {
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/ping", web::get().to(|| "pong"))
            .service(
                web::resource("/status")
                    .data(p2p_status.clone())
                    .route(web::get().to(status)),
            )
            .service(
                web::resource("/graphql")
                    .data(schema.clone())
                    .data(gql_context.clone())
                    .route(web::post().to_async(graphql)),
            )
            .service(
                web::resource("/graphiql")
                    .data(port)
                    .route(web::get().to(graphiql)),
            )
    })
    .bind(("127.0.0.1", port))?
    .start();
    Ok(())
}

#[derive(serde::Serialize)]
struct ConnInfo {
    addr: Option<String>,
    alive_at: u64,
}
#[derive(serde::Serialize)]
struct StatusResponse {
    connections: HashMap<String, ConnInfo>,
}

fn status(status: web::Data<Status>) -> HttpResponse {
    let connections: HashMap<String, ConnInfo> = status
        .connections()
        .iter()
        .map(|(id, status)| {
            (
                String::from(*id),
                ConnInfo {
                    addr: status.addr.as_ref().map(NodeAddress::to_string),
                    alive_at: status
                        .alive_at
                        .duration_since(UNIX_EPOCH)
                        .expect("Time reversed")
                        .as_secs(),
                },
            )
        })
        .collect();
    HttpResponse::Ok().json(StatusResponse { connections })
}
