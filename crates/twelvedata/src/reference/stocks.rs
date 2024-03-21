use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct Stocks {
    pub data: Vec<Stock>,
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
    type_field: String,
    #[serde(skip)]
    include_delisted: bool,
}

#[derive(Deserialize, Debug)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
    pub country: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl Stocks {
    pub fn builder() -> Self {
        Stocks {
            data: Vec::new(),
            status: String::new(),
            symbol: String::new(),
            exchange: String::new(),
            mic_code: String::new(),
            country: String::new(),
            type_field: String::new(),
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

    pub fn type_field(&mut self, type_field: &str) -> &mut Self {
        self.type_field = type_field.to_string();
        self
    }

    pub fn include_delisted(&mut self, include_delisted: bool) -> &mut Self {
        self.include_delisted = include_delisted;
        self
    }

    pub async fn execute(&self) -> Result<Stocks, Box<dyn Error>> {
        let include_delisted = &self.include_delisted.to_string();

        let params = vec![
            ("symbol", &self.symbol),
            ("exchange", &self.exchange),
            ("mic_code", &self.mic_code),
            ("country", &self.country),
            ("type", &self.type_field),
            ("include_delisted", include_delisted),
        ];

        internal::request::execute("/stocks", params).await
    }
}

#[cfg(test)]
mod test {
    use super::Stocks;

    #[tokio::test]
    async fn get_stocks() {
        let result = Stocks::builder().execute().await;

        assert!(result.is_ok());
    }
}
