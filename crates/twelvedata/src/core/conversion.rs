use crate::Client;

use super::types::CurrencyConversion;

impl Client {
    pub async fn currency_conversion(&self, symbol: &str, amount: f32) -> CurrencyConversion {
        let response: CurrencyConversion = reqwest::Client::new()
            .get("https://api.twelvedata.com/currency_conversion")
            .query(&[("apikey", &self.api_key)])
            .query(&[("symbol", symbol), ("amount", &amount.to_string())])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting currency conversion for {}", symbol));

        return response;
    }
}

pub mod test {
    #[tokio::test]
    async fn get_conversion() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.currency_conversion("USD/JPY", 12.0).await;
    }
}