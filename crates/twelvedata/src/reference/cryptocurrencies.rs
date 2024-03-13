use crate::Client;

use super::types::Cryptocurrencies;

impl Client {
    pub async fn cryptocurrencies(&self) -> Cryptocurrencies {
        let response: Cryptocurrencies = reqwest::Client::new()
            .get("https://api.twelvedata.com/cryptocurrencies")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting cryptocurrencies"));

        return response;
    }
}

#[cfg(test)]
pub mod test {
    use super::Client;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_cryptocurrencies() {
        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.cryptocurrencies().await;
    }
}
