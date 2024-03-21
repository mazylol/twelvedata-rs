use crate::internal;

use std::error::Error;

use serde::Deserialize;

#[derive(Debug)]
pub enum Type {
    Stocks,
    ETF,
    MutualFunds,
    Forex,
    Crypto,
}

impl Default for Type {
    fn default() -> Self {
        Type::Stocks
    }
}

/// # UNTESTED, USE WITH CAUTION (I don't have a paid account to test with)
/// There are multiple types of market movers, default is stocks. Use the `mover_type` method to change the type.
#[derive(Deserialize, Debug)]
pub struct MarketMovers {
    pub values: Vec<MarketMover>,
    pub status: String,

    #[serde(skip)]
    direction: String,
    #[serde(skip)]
    outputsize: String,
    #[serde(skip)]
    country: String,
    #[serde(skip)]
    apikey: String,
    #[serde(skip)]
    dp: u8,

    #[serde(skip)]
    mover_type: Type,
}

#[derive(Deserialize, Debug)]
pub struct MarketMover {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub mic_code: String,
    pub datetime: String,
    pub last: f32,
    pub high: f32,
    pub low: f32,
    pub volume: u32,
    pub change: f32,
    pub percent_change: f32,
}

impl MarketMovers {
    pub fn builder() -> Self {
        MarketMovers {
            values: Vec::new(),
            status: String::new(),
            direction: String::new(),
            outputsize: String::new(),
            country: String::new(),
            apikey: String::new(),
            dp: 5,
            mover_type: Type::default(),
        }
    }

    pub fn direction(&mut self, direction: &str) -> &mut Self {
        self.direction = direction.to_string();
        self
    }

    pub fn outputsize(&mut self, outputsize: &str) -> &mut Self {
        self.outputsize = outputsize.to_string();
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

    pub fn dp(&mut self, dp: u8) -> &mut Self {
        self.dp = dp;
        self
    }

    pub fn mover_type(&mut self, type_: Type) -> &mut Self {
        self.mover_type = type_;
        self
    }

    pub async fn execute(&self) -> Result<MarketMovers, Box<dyn Error>> {
        let dp = if self.dp > 11 {
            5.to_string()
        } else {
            self.dp.to_string()
        };

        let params = vec![
            ("direction", &self.direction),
            ("outputsize", &self.outputsize),
            ("country", &self.country),
            ("apikey", &self.apikey),
            ("dp", &dp),
        ];

        match self.mover_type {
            Type::Stocks => internal::request::execute("/market_movers/stocks", params).await,
            Type::ETF => internal::request::execute("/market_movers/etf", params).await,
            Type::MutualFunds => {
                internal::request::execute("/market_movers/mutual_funds", params).await
            }
            Type::Forex => internal::request::execute("/market_movers/forex", params).await,
            Type::Crypto => internal::request::execute("/market_movers/crypto", params).await,
        }
    }
}

// Untested until I get a paid account to test with
/*
#[cfg(test)]
mod tests {
    use super::MarketMovers;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_market_movers() {
        dotenv().expect(".env file not found");

        let response = MarketMovers::builder()
            .mover_type(super::Type::Forex)
            .apikey(&env::var("API_TOKEN").unwrap())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
*/
