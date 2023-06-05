use crate::Client;

use super::types::Cryptocurrencies;

impl Client {
    pub async fn cryptocurrencies(&self) -> Cryptocurrencies {
        let response: Cryptocurrencies = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/cryptocurrencies")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting cryptocurrencies"));

        return response;
    }
}
