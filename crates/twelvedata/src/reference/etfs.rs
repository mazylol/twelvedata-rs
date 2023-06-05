use crate::Client;

use super::types::ETFs;

impl Client {
    pub async fn etfs(&self) -> ETFs {
        let response: ETFs = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/etf")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting ETFs"));

        return response;
    }
}
