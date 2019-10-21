use super::responses::*;
use reqwest::{self, *};
use serde::Serialize;

pub struct GrqphQLClient {
    url: Url,
    client: Client,
}

#[derive(Serialize)]
struct GraphQLQuery {
    query: &'static str,
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
    pub fn get_offers(&self) -> Result<Offers> {
        let response: GraphQLResponse<Offers> = self
            .client
            .post(self.url.clone())
            .json(&GraphQLQuery {
                query: "{offers { id direction } }",
            })
            .send()?
            .json()?;
        Ok(response.data)
    }
}
