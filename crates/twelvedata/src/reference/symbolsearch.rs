use crate::Client;

use super::types::SymbolSearch;

impl Client {
    pub async fn symbol_search(&self, symbol: &str) -> SymbolSearch {
        let response: SymbolSearch = reqwest::Client::new()
            .get("https://twelve-data1.p.rapidapi.com/symbol_search")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap_or_else(|_| panic!("Error getting symbol search for {}", symbol));

        return response;
    }
}
