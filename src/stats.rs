use crate::bisq::payload::TradeStatistics2;
use crate::prelude::*;
use actix_web::{web, Error, HttpResponse};
use juniper::{
    self, graphql_object,
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, FieldResult, GraphQLInputObject, RootNode,
};
use juniper_from_schema::graphql_schema_from_file;
use std::sync::Arc;

pub fn graphql(
    data: web::Data<(Arc<Schema>, Arc<StatsLog>)>,
    request: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = request.execute(&data.0, &data.1);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}
pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:7477/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

graphql_schema_from_file!("src/api/stats_schema.graphql", context_type: StatsLog);

pub struct Query;
impl QueryFields for Query {
    fn field_hello_world(
        &self,
        executor: &juniper::Executor<'_, StatsLog>,
        name: String,
    ) -> juniper::FieldResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}

type Mutation = EmptyMutation<StatsLog>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}

type StatsLogInner = Arc<locks::RwLock<Vec<TradeStatistics2>>>;
#[derive(Clone)]
pub struct StatsLog(pub StatsLogInner);
impl juniper::Context for StatsLog {}

pub struct Stats {
    statistics: StatsLogInner,
}
impl Stats {
    pub fn start() -> (StatsLog, Addr<Self>) {
        let statistics = Arc::new(locks::RwLock::new(Vec::new()));
        (StatsLog(statistics.clone()), Self { statistics }.start())
    }
}

impl Actor for Stats {
    type Context = Context<Self>;
}
