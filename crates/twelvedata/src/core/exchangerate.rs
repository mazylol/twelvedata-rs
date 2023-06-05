use crate::Client;

use super::types::ExchangeRate;

impl Client {
    pub async fn exchange_rate(&self, symbol: &str) -> ExchangeRate {
        let response: ExchangeRate = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/exchange_rate")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
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
