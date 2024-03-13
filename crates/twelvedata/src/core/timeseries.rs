use crate::Client;

use super::types::Timeseries;

impl Client {
    pub async fn timeseries(&self, symbol: &str, interval: &str) -> Timeseries {
        let response: Timeseries = reqwest::Client::new()
            .get("https://api.twelvedata.com/time_series")
            .query(&[("apikey", &self.api_key)])
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

pub mod test {
    #[tokio::test]
    async fn get_time_series() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.timeseries("AAPL", "1min").await;
    }
}
