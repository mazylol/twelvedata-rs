use crate::Client;

use super::types::EarliestTimestamp;

impl Client {
    pub async fn earliest_timestamp(&self, symbol: &str, interval: &str) -> EarliestTimestamp {
        let response: EarliestTimestamp = reqwest::Client::new()
            .get("https://api.twelvedata.com/earliest_timestamp")
            .query(&[("apikey", &self.api_key)])
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

pub mod test {
    #[tokio::test]
    async fn get_earliest_timestamp() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.earliest_timestamp("AAPL", "1day").await;
    }
}
