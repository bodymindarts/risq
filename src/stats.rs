#[cfg(feature = "stats")]
pub use inner::*;
#[cfg(feature = "stats")]
mod inner {
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
        schema: web::Data<Arc<Schema>>,
        cache: web::Data<StatsCache>,
        request: web::Json<GraphQLRequest>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        web::block(move || {
            let res = request.execute(&schema, &cache);
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

    graphql_schema_from_file!("src/api/stats_schema.graphql", context_type: StatsCache);

    pub struct Query;
    impl QueryFields for Query {
        fn field_hello_world(
            &self,
            executor: &juniper::Executor<'_, StatsCache>,
            name: String,
        ) -> juniper::FieldResult<String> {
            Ok(format!("Hello, {}!", name))
        }
    }

    type Mutation = EmptyMutation<StatsCache>;

    pub fn create_schema() -> Schema {
        Schema::new(Query {}, EmptyMutation::new())
    }

    type StatsLogInner = Arc<locks::RwLock<Vec<TradeStatistics2>>>;
    #[derive(Clone)]
    pub struct StatsCache {
        statistics: StatsLogInner,
    }
    impl juniper::Context for StatsCache {}

    impl StatsCache {
        pub fn new() -> Option<Self> {
            Some(Self {
                statistics: Arc::new(locks::RwLock::new(Vec::new())),
            })
        }
    }
}

#[cfg(not(feature = "stats"))]
pub use empty::*;
#[cfg(not(feature = "stats"))]
mod empty {
    use crate::prelude::*;
    use actix_web::{Error, HttpResponse};

    #[derive(Clone)]
    pub struct StatsCache;
    impl StatsCache {
        pub fn new() -> Option<Self> {
            None
        }
    }
    pub struct Schema;
    pub fn create_schema() -> Schema {
        Schema
    }
    pub fn graphql() -> impl Future<Item = HttpResponse, Error = Error> {
        future::ok(HttpResponse::Ok().finish())
    }
    pub fn graphiql() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
