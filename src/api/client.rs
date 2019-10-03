use super::responses::*;
use reqwest::{self, *};

pub struct Client {
    api_port: u16,
}

impl Client {
    pub fn new(api_port: u16) -> Client {
        Client { api_port }
    }
    pub fn get_offers(&self) -> Result<GetOffers> {
        reqwest::get(url(self.api_port, "offers"))?.json()
    }
}

fn url(port: u16, path: &str) -> Url {
    format!("http://127.0.0.1:{}/{}", port, path)
        .parse()
        .unwrap()
}
