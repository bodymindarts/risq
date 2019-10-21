use reqwest::{self, *};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

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
struct GraphQLQuery {
    query: String,
    variables: HashMap<String, String>,
}
impl GraphQLQuery {
    fn new<T: WithQueryFields>(variables: HashMap<String, String>) -> Self {
        Self {
            query: <T as WithQueryFields>::get_fields(),
            variables,
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
    pub fn query<T: WithQueryFields>(&self, variables: HashMap<String, String>) -> Result<T> {
        let response: GraphQLResponse<T> = self
            .client
            .post(self.url.clone())
            .json(&GraphQLQuery::new::<T>(variables))
            .send()?
            .json()?;
        Ok(response.data)
    }
}
