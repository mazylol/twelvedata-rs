use crate::Client;

use super::types::CurrencyConversion;

impl Client {
    pub async fn currency_conversion(&self, symbol: &str, amount: f32) -> CurrencyConversion {
        let response: CurrencyConversion = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/currency_conversion")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .query(&[("symbol", symbol), ("amount", &amount.to_string())])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting currency conversion for {}", symbol));

        return response;
    }
}
