use crate::Client;

use super::types::Quote;

impl Client {
    pub async fn quote(&self, symbol: &str) -> Quote {
        let response: Quote = reqwest::Client::new()
            .get("https://api.twelvedata.com/quote")
            .query(&[("apikey", &self.api_key)])
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting quote for {}", symbol));

        return response;
    }
}

#[cfg(test)]
pub mod test {
    use super::Client;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_quote() {
        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.quote("AAPL").await;
    }
}
