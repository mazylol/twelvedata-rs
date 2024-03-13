use crate::Client;

use super::types::Indices;

impl Client {
    pub async fn indices(&self) -> Indices {
        let response: Indices = reqwest::Client::new()
            .get("https://api.twelvedata.com/indices")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting indices"));

        return response;
    }
}

pub mod test {
    #[tokio::test]
    async fn get_indices() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.indices().await;
    }
}
