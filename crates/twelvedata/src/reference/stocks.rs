use crate::Client;

use super::types::Stocks;

impl Client {
    pub async fn stocks(&self) -> Stocks {
        let response: Stocks = reqwest::Client::new()
            .get("https://api.twelvedata.com/stocks")
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting stocks"));

        return response;
    }
}

#[cfg(test)]
pub mod test {
    use super::Client;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_stocks() {
        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.stocks().await;
    }
}
