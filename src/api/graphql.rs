use crate::{
    domain::{offer::OfferDirection, statistics::*},
    prelude::*,
};
use actix_web::{web, Error, HttpResponse};
use juniper::{
    self,
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, FieldResult,
};
use juniper_from_schema::graphql_schema_from_file;
use std::sync::Arc;

pub fn graphql(
    schema: web::Data<Arc<Schema>>,
    context: web::Data<GraphQLContextWrapper>,
    request: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    context
        .get()
        .and_then(|context| {
            web::block(move || {
                let res = request.execute(&schema, &context);
                Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
            })
            .map_err(Error::from)
        })
        .map(|result| {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(result)
        })
}
pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:7477/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[derive(Clone)]
pub struct GraphQLContextWrapper {
    #[cfg(feature = "statistics")]
    pub stats_cache: StatsCache,
}
impl GraphQLContextWrapper {
    #[cfg(feature = "statistics")]
    pub fn get(&self) -> impl Future<Item = GraphQLContext, Error = Error> {
        self.stats_cache
            .inner()
            .map_err(Error::from)
            .map(|stats_cache| GraphQLContext { stats_cache })
    }
    #[cfg(not(feature = "statistics"))]
    pub fn get(&self) -> impl Future<Item = GraphQLContext, Error = Error> {
        future::ok(GraphQLContext {})
    }
}
pub struct GraphQLContext {
    #[cfg(feature = "statistics")]
    stats_cache: locks::RwLockReadGuard<StatsCacheInner>,
}
impl juniper::Context for GraphQLContext {}

graphql_schema_from_file!("src/api/schema.graphql", context_type: GraphQLContext);

type Mutation = EmptyMutation<GraphQLContext>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}

pub struct Query;
impl QueryFields for Query {
    #[cfg(feature = "statistics")]
    fn field_trades(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        trail: &QueryTrail<'_, Trade, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<Trade>> {
        let stats = &executor.context().stats_cache;
        Ok(stats.trades().iter().cloned().collect())
    }
    #[cfg(not(feature = "statistics"))]
    fn field_trades(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
        trail: &QueryTrail<'_, Trade, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<Trade>> {
        Ok(Vec::new())
    }
}

impl TradeFields for Trade {
    // fn field_currency(&self, executor: &juniper::Executor<'_, Inner>) -> FieldResult<String> {
    //     Ok(self.currency.alpha3.to_owned())
    // }
    fn field_direction(
        &self,
        executor: &juniper::Executor<'_, GraphQLContext>,
    ) -> FieldResult<Direction> {
        Ok(match self.direction {
            OfferDirection::Sell => Direction::Sell,
            OfferDirection::Buy => Direction::Buy,
        })
    }
}
