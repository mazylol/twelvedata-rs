use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct ETFs {
    pub data: Vec<ETF>,
    pub status: String,

    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    exchange: String,
    #[serde(skip)]
    mic_code: String,
    #[serde(skip)]
    country: String,
    #[serde(skip)]
    include_delisted: bool,
}

#[derive(Deserialize, Debug)]
pub struct ETF {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
    pub country: String,
}

impl ETFs {
    pub fn builder() -> Self {
        ETFs {
            data: Vec::new(),
            status: String::new(),
            symbol: String::new(),
            exchange: String::new(),
            mic_code: String::new(),
            country: String::new(),
            include_delisted: false,
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

    pub fn mic_code(&mut self, mic_code: &str) -> &mut Self {
        self.mic_code = mic_code.to_string();
        self
    }

    pub fn country(&mut self, country: &str) -> &mut Self {
        self.country = country.to_string();
        self
    }

    pub fn include_delisted(&mut self, include_delisted: bool) -> &mut Self {
        self.include_delisted = include_delisted;
        self
    }

    pub async fn execute(&self) -> Result<ETFs, Box<dyn Error>> {
        let include_delisted = &self.include_delisted.to_string();

        let params = vec![
            ("symbol", &self.symbol),
            ("exchange", &self.exchange),
            ("mic_code", &self.mic_code),
            ("country", &self.country),
            ("include_delisted", include_delisted),
        ];

        internal::request::execute("/etf", params).await
    }
}

#[cfg(test)]
mod test {
    use super::ETFs;

    #[tokio::test]
    async fn get_etfs() {
        let result = ETFs::builder().execute().await;

        assert!(result.is_ok());
    }
}
