use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct MarketState {
    pub data: Vec<Exchange>,

    #[serde(skip)]
    exchange: String,
    #[serde(skip)]
    code: String,
    #[serde(skip)]
    country: String,
    #[serde(skip)]
    apikey: String,
}

#[derive(Deserialize, Debug)]
pub struct Exchange {
    pub name: String,
    pub code: String,
    pub country: String,
    pub is_market_open: bool,
    pub time_to_open: String,
    pub time_to_close: String,
    pub time_after_open: String,
}

impl MarketState {
    pub fn builder() -> Self {
        MarketState {
            data: Vec::new(),
            exchange: String::new(),
            code: String::new(),
            country: String::new(),
            apikey: String::new(),
        }
    }

    pub fn exchange(&mut self, exchange: &str) -> &mut Self {
        self.exchange = exchange.to_string();
        self
    }

    pub fn code(&mut self, code: &str) -> &mut Self {
        self.code = code.to_string();
        self
    }

    pub fn country(&mut self, country: &str) -> &mut Self {
        self.country = country.to_string();
        self
    }

    pub fn apikey(&mut self, apikey: &str) -> &mut Self {
        self.apikey = apikey.to_string();
        self
    }

    pub async fn execute(&self) -> Result<MarketState, Box<dyn Error>> {
        let params = vec![
            ("exchange", &self.exchange),
            ("code", &self.code),
            ("country", &self.country),
            ("apikey", &self.apikey),
        ];

        internal::request::execute("/market_state", params).await
    }
}

#[cfg(test)]
mod test {
    use super::MarketState;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_market_state() {
        dotenv().expect(".env file not found");

        let result = MarketState::builder()
            .apikey(&env::var("API_TOKEN").unwrap())
            .execute()
            .await;

        assert!(result.is_ok());
    }
}
