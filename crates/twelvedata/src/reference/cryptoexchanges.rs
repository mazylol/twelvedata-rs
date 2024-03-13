use crate::Client;

use super::types::CryptoExchanges;

impl Client {
    pub async fn crypto_exchanges(&self) -> CryptoExchanges {
        let response: CryptoExchanges = reqwest::Client::new()
            .get("https://api.twelvedata.com/cryptocurrency_exchanges")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting crypto exchanges"));

        return response;
    }
}

pub mod test {
    #[tokio::test]
    async fn get_crypto_exchanges() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.crypto_exchanges().await;
    }
}
