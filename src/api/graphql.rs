use crate::{
    bisq::BisqHash,
    domain::{offer::OfferDirection, statistics::*, CommandResult, FutureCommandResult},
    prelude::*,
};
use actix_web::{web, Error, HttpResponse};
use juniper::{
    self, graphql_object,
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, FieldResult, GraphQLInputObject, RootNode,
};
use juniper_from_schema::graphql_schema_from_file;
use std::sync::Arc;

pub fn graphql(
    schema: web::Data<Arc<Schema>>,
    cache: web::Data<StatsCache>,
    request: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    cache
        .inner()
        .map_err(Error::from)
        .and_then(|cache| {
            web::block(move || {
                let res = request.execute(&schema, &cache);
                Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
            })
            .map_err(Error::from)
        })
        .and_then(|result| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(result))
        })
}
pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:7477/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

graphql_schema_from_file!("src/api/schema.graphql", context_type: StatsCacheInner);

pub struct Query;
impl QueryFields for Query {
    fn field_trades<'a>(
        &self,
        executor: &juniper::Executor<'a, StatsCacheInner>,
        trail: &QueryTrail<'_, Trade, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<Trade>> {
        let cache = executor.context();
        Ok(cache.trades().iter().cloned().collect())
    }
}

type Mutation = EmptyMutation<StatsCacheInner>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}

impl TradeFields for Trade {
    // fn field_currency(&self, executor: &juniper::Executor<'_, Inner>) -> FieldResult<String> {
    //     Ok(self.currency.alpha3.to_owned())
    // }
    fn field_direction(
        &self,
        executor: &juniper::Executor<'_, StatsCacheInner>,
    ) -> FieldResult<Direction> {
        Ok(match self.direction {
            OfferDirection::Sell => Direction::Sell,
            OfferDirection::Buy => Direction::Buy,
        })
    }
}
