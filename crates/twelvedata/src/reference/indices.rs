use crate::Client;

use super::types::Indices;

impl Client {
    pub async fn indices(&self) -> Indices {
        let response: Indices = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/indices")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting indices"));

        return response;
    }
}
