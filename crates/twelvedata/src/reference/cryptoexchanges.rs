use crate::Client;

use super::types::CryptoExchanges;

impl Client {
    pub async fn crypto_exchanges(&self) -> CryptoExchanges {
        let response: CryptoExchanges = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/cryptocurrency_exchanges")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting crypto exchanges"));

        return response;
    }
}
