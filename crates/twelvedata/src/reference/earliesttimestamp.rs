use crate::Client;

use super::types::EarliestTimestamp;

impl Client {
    pub async fn earliest_timestamp(&self, symbol: &str, interval: &str) -> EarliestTimestamp {
        let response: EarliestTimestamp = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/earliest_timestamp")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .query(&[("symbol", symbol), ("interval", interval)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting earliest timestamp for {}", symbol));

        return response;
    }
}
