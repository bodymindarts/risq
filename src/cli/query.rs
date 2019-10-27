use crate::{api::WithQueryFields, domain::market::Market};
use serde::Deserialize;
use std::{collections::HashMap, fmt, iter::Chain, vec::IntoIter};

#[derive(Deserialize)]
pub struct Offers {
    buys_and_sells: BuysAndSells,
}
#[derive(Deserialize)]
pub struct BuysAndSells {
    buys: Vec<Offer>,
    sells: Vec<Offer>,
}
impl Offers {
    pub fn add_variables(market: &Market, args: &mut HashMap<String, String>) {
        args.insert("market".to_string(), market.pair.clone());
    }
    fn buys(&self) -> &Vec<Offer> {
        &self.buys_and_sells.buys
    }
    fn sells(&self) -> &Vec<Offer> {
        &self.buys_and_sells.sells
    }
    pub fn len(&self) -> usize {
        self.buys().len() + self.sells().len()
    }
}
impl IntoIterator for Offers {
    type Item = Offer;
    type IntoIter = Chain<IntoIter<Offer>, IntoIter<Offer>>;

    fn into_iter(self) -> Self::IntoIter {
        let BuysAndSells { buys, sells } = self.buys_and_sells;
        buys.into_iter().chain(sells.into_iter())
    }
}
impl WithQueryFields for Offers {
    fn get_fields() -> String {
        format!(
            r#"query Offers($market: MarketPair = "all") {{
                 buys_and_sells: offers(market: $market) {{
                   buys {}
                   sells {}
               }} }}"#,
            Offer::get_fields(),
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
