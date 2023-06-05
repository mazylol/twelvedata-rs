use crate::Client;

use super::types::Exchanges;

impl Client {
    pub async fn exchanges(&self) -> Exchanges {
        let response: Exchanges = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/exchanges")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting exchanges"));

        return response;
    }
}
