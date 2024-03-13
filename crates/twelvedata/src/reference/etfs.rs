use crate::Client;

use super::types::ETFs;

impl Client {
    pub async fn etfs(&self) -> ETFs {
        let response: ETFs = reqwest::Client::new()
            .get("https://api.twelvedata.com/etf")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting ETFs"));

        return response;
    }
}

pub mod test {
    #[tokio::test]
    async fn get_etfs() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.etfs().await;
    }
}
