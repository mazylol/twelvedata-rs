use crate::Client;

use super::types::SymbolSearch;

impl Client {
    pub async fn symbol_search(&self, symbol: &str) -> SymbolSearch {
        let response: SymbolSearch = reqwest::Client::new()
            .get("https://api.twelvedata.com/symbol_search")
            .query(&[("apikey", &self.api_key)])
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

#[cfg(test)]
pub mod test {
    use super::Client;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_symbol_search() {
        dotenv().expect(".env file not found");

        let client = Client::new(env::var("API_TOKEN").unwrap().as_str());

        let _ = client.symbol_search("AA").await;
    }
}
