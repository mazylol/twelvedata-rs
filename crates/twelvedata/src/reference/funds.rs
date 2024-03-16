use crate::Client;

use super::types::Funds;

impl Client {
    pub async fn funds(&self) -> Funds {
        let response: Funds = reqwest::Client::new()
            .get("https://api.twelvedata.com/funds")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting funds list"));

        return response;
    }
}

#[cfg(test)]
pub mod test {
    use super::Client;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_funds() {
        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.funds().await;
    }
}
