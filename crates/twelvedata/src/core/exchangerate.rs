use crate::Client;

use super::types::ExchangeRate;

impl Client {
    pub async fn exchange_rate(&self, symbol: &str) -> ExchangeRate {
        let response: ExchangeRate = reqwest::Client::new()
            .get("https://api.twelvedata.com/exchange_rate")
            .query(&[("apikey", &self.api_key)])
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting exchange rate for {}", symbol));

        return response;
    }
}

#[cfg(test)]
pub mod test {
    use super::Client;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_exchangerate() {
        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.exchange_rate("USD/JPY").await;
    }
}
