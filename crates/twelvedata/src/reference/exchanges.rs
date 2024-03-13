use crate::Client;

use super::types::Exchanges;

impl Client {
    pub async fn exchanges(&self) -> Exchanges {
        let response: Exchanges = reqwest::Client::new()
            .get("https://api.twelvedata.com/exchanges")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting exchanges"));

        return response;
    }
}

#[cfg(test)]
pub mod test {
    use super::Client;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_exchanges() {
        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.exchanges().await;
    }
}
