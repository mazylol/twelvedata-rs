use crate::Client;

use super::types::ForexPairs;

impl Client {
    pub async fn forex_pairs_list(&self) -> ForexPairs {
        let response: ForexPairs = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/forex_pairs")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting forex pairs list"));

        return response;
    }
}
