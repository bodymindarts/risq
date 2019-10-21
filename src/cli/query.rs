use crate::{api::WithQueryFields, domain::market::Market};
use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(Deserialize)]
pub struct Offers {
    pub offers: Vec<Offer>,
}
impl Offers {
    pub fn add_variables(market: &Market, args: &mut HashMap<String, String>) {
        args.insert("market".to_string(), market.pair.clone());
    }
}
impl WithQueryFields for Offers {
    fn get_fields() -> String {
        format!(
            r#"query Offers($market: MarketPair = "all") {{
                 offers(market: $market) {}
               }}"#,
            Offer::get_fields()
        )
    }
}

#[derive(Deserialize)]
pub struct Offer {
    pub id: String,
    pub market: String,
    pub direction: String,
    pub price: String,
    pub amount: String,
    pub min_amount: String,
    pub volume: String,
    pub payment_method: String,
}
impl fmt::Display for Offer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} {} {}({}) {} {}",
            self.id,
            self.direction,
            self.price,
            self.amount,
            self.min_amount,
            self.volume,
            self.payment_method
        )
    }
}

impl WithQueryFields for Offer {
    fn get_fields() -> String {
        r#"{ id
             market: marketPair
             direction
             price: formattedPrice
             amount: formattedAmount
             min_amount: formattedMinAmount
             volume: formattedVolume
             payment_method: paymentMethodId
           }"#
        .to_string()
    }
}
