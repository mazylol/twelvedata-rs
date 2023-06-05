use crate::Client;

use super::types;

impl Client {
    pub async fn realtime_price(&self, symbol: &str) -> f32 {
        let response: types::RealtimePrice = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/price")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
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
