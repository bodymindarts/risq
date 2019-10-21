use crate::api::WithQueryFields;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Offers {
    pub offers: Vec<Offer>,
}
impl WithQueryFields for Offers {
    fn get_fields() -> String {
        format!("{{ offers {} }}", Offer::get_fields())
    }
}

#[derive(Deserialize)]
pub struct Offer {
    pub id: String,
    pub direction: String,
}

impl WithQueryFields for Offer {
    fn get_fields() -> String {
        "{ id direction }".to_string()
    }
}
