use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct Cryptocurrencies {
    pub data: Vec<Cryptocurrency>,
    pub status: String,

    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    exchange: String,
    #[serde(skip)]
    currency_base: String,
    #[serde(skip)]
    currency_quote: String,
}

#[derive(Deserialize, Debug)]
pub struct Cryptocurrency {
    pub symbol: String,
    pub available_exchanges: Vec<String>,
    pub currency_base: String,
    pub currency_quote: String,
}

impl Cryptocurrencies {
    pub fn builder() -> Self {
        Cryptocurrencies {
            data: Vec::new(),
            status: String::new(),
            symbol: String::new(),
            exchange: String::new(),
            currency_base: String::new(),
            currency_quote: String::new(),
        }
    }

    pub fn symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn exchange(&mut self, exchange: &str) -> &mut Self {
        self.exchange = exchange.to_string();
        self
    }

    pub fn currency_base(&mut self, currency_base: &str) -> &mut Self {
        self.currency_base = currency_base.to_string();
        self
    }

    pub fn currency_quote(&mut self, currency_quote: &str) -> &mut Self {
        self.currency_quote = currency_quote.to_string();
        self
    }

    pub async fn execute(&self) -> Result<Cryptocurrencies, Box<dyn Error>> {
        let params = vec![
            ("symbol", &self.symbol),
            ("exchange", &self.exchange),
            ("currency_base", &self.currency_base),
            ("currency_quote", &self.currency_quote),
        ];

        internal::request::execute("/cryptocurrencies", params).await
    }
}

#[cfg(test)]
mod tests {
    use super::Cryptocurrencies;

    #[tokio::test]
    async fn get_cryptocurrencies() {
        let result = Cryptocurrencies::builder().execute().await;

        assert!(result.is_ok());
    }
}
