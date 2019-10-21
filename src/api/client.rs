use reqwest::{self, *};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;

pub struct GrqphQLClient {
    url: Url,
    client: Client,
}

pub trait WithQueryFields: DeserializeOwned {
    fn get_fields() -> String;
}

#[derive(Deserialize)]
struct GraphQLResponse<T: DeserializeOwned> {
    #[serde(bound = "T: DeserializeOwned")]
    pub data: T,
}

#[derive(Serialize)]
struct GraphQLQuery<T: WithQueryFields> {
    query: String,
    phantom: PhantomData<T>,
}
impl<T: WithQueryFields> GraphQLQuery<T> {
    fn new() -> Self {
        Self {
            query: <T as WithQueryFields>::get_fields(),
            phantom: PhantomData,
        }
    }
}

impl GrqphQLClient {
    pub fn new(api_port: u16) -> Self {
        Self {
            url: format!("http://127.0.0.1:{}/graphql", api_port)
                .parse()
                .unwrap(),
            client: Client::new(),
        }
    }
    pub fn query<T: WithQueryFields>(&self) -> Result<T> {
        let query: GraphQLQuery<T> = GraphQLQuery::new();
        let response: GraphQLResponse<T> = self
            .client
            .post(self.url.clone())
            .json(&query)
            .send()?
            .json()?;
        Ok(response.data)
    }
}
