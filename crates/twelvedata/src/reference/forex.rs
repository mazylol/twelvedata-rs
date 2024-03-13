use crate::Client;

use super::types::ForexPairs;

impl Client {
    pub async fn forex_pairs(&self) -> ForexPairs {
        let response: ForexPairs = reqwest::Client::new()
            .get("https://api.twelvedata.com/forex_pairs")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting forex pairs list"));

        return response;
    }
}

pub mod test {
    #[tokio::test]
    async fn get_forex_pairs() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.forex_pairs().await;
    }
}
