use crate::Client;

use super::types::ExchangeRate;

impl Client {
    pub async fn exchange_rate(&self, symbol: &str) -> ExchangeRate {
        let response: ExchangeRate = reqwest::Client::new()
            .get("https://api.twelvedata.com/exchange_rate")
            .query(&[("apikey", &self.api_key)])
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting exchange rate for {}", symbol));

        return response;
    }
}
