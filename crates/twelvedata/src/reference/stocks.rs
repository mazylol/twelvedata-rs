use crate::Client;

use super::types::Stocks;

impl Client {
    pub async fn stocks(&self) -> Stocks {
        let response: Stocks = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/stocks")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting stocks"));

        return response;
    }
}
