use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct ForexPairs {
    pub data: Vec<ForexPair>,
    pub status: String,

    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    currency_base: String,
    #[serde(skip)]
    currency_quote: String,
}

#[derive(Deserialize, Debug)]
pub struct ForexPair {
    pub symbol: String,
    pub currency_group: String,
    pub currency_base: String,
    pub currency_quote: String,
}

impl ForexPairs {
    pub fn builder() -> Self {
        ForexPairs {
            data: Vec::new(),
            status: String::new(),
            symbol: String::new(),
            currency_base: String::new(),
            currency_quote: String::new(),
        }
    }

    pub fn symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
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

    pub async fn execute(&self) -> Result<ForexPairs, Box<dyn Error>> {
        let params = vec![
            ("symbol", &self.symbol),
            ("currency_base", &self.currency_base),
            ("currency_quote", &self.currency_quote),
        ];

        internal::request::execute("/forex_pairs", params).await
    }
}

#[cfg(test)]
mod test {
    use super::ForexPairs;

    #[tokio::test]
    async fn get_forex_pairs() {
        let result = ForexPairs::builder().execute().await;

        assert!(result.is_ok());
    }
}
