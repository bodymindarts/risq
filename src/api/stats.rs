use crate::prelude::Future;
use actix_web::{web, Error, HttpResponse};
use juniper::{
    self, graphql_object,
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, FieldResult, GraphQLInputObject, RootNode,
};
use std::sync::Arc;

pub fn graphql(
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
pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:7477/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[derive(juniper::GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
