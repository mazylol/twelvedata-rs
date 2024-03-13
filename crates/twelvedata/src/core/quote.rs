use crate::Client;

use super::types::Quote;

impl Client {
    pub async fn quote(&self, symbol: &str) -> Quote {
        let response: Quote = reqwest::Client::new()
            .get("https://api.twelvedata.com/quote")
            .query(&[("apikey", &self.api_key)])
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
