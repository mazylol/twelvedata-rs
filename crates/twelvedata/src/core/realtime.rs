use crate::Client;

use super::types;

impl Client {
    pub async fn realtime_price(&self, symbol: &str) -> f32 {
        let response: types::RealtimePrice = reqwest::Client::new()
            .get("https://api.twelvedata.com/price")
            .query(&[("apikey", &self.api_key)])
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting realtime price for {}", symbol));

        return response.price.parse::<f32>().unwrap();
    }
}

pub mod test {
    #[tokio::test]
    async fn get_stock_price() {
        use super::Client;
        use dotenvy::dotenv;
        use std::env;

        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.realtime_price("AMZN").await;
    }
}
