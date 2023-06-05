use crate::Client;

use super::types::Timeseries;

impl Client {
    pub async fn timeseries(&self, symbol: &str, interval: &str) -> Timeseries {
        let response: Timeseries = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/time_series")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .query(&[("symbol", symbol), ("interval", interval)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting timeseries for {}", symbol));

        return response;
    }
}
