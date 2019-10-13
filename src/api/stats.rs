use crate::prelude::Future;
use actix_web::{web, Error, HttpResponse};
use juniper::{
    self, graphql_object,
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, FieldResult, GraphQLInputObject, RootNode,
};
#[macro_use]
use juniper_from_schema::graphql_schema_from_file;
use std::sync::Arc;

pub fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &Context);
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

graphql_schema_from_file!("src/api/stats_schema.graphql");

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;
impl QueryFields for Query {
    fn field_hello_world(
        &self,
        executor: &juniper::Executor<'_, Context>,
        name: String,
    ) -> juniper::FieldResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}

type Mutation = EmptyMutation<Context>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}
