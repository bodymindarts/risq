use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
pub struct GraphQLResponse<T: DeserializeOwned> {
    #[serde(bound = "T: DeserializeOwned")]
    pub data: T,
}

#[derive(Deserialize)]
pub struct Offers {
    pub offers: Vec<Offer>,
}

#[derive(Deserialize)]
pub struct Offer {
    pub id: String,
    pub direction: String,
}
