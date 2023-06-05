use crate::Client;

use super::types::Quote;

impl Client {
    pub async fn quote(&self, symbol: &str) -> Quote {
        let response: Quote = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/quote")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting quote for {}", symbol));

        return response;
    }
}
