use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct Funds {
    pub result: FundResult,
    pub status: String,

    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    exchange: String,
    #[serde(skip)]
    country: String,
    #[serde(skip)]
    type_field: String,
    #[serde(skip)]
    include_delisted: bool,
    #[serde(skip)]
    page: u32,
    #[serde(skip)]
    outputsize: u32,
}

#[derive(Deserialize, Debug)]
pub struct Fund {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub currency: String,
    pub exchange: String,
    #[serde(rename = "type")]
    pub fund_type: String,
}

#[derive(Deserialize, Debug)]
pub struct FundResult {
    pub count: u32,
    pub list: Vec<Fund>,
}

impl Funds {
    pub fn builder() -> Self {
        Funds {
            result: FundResult {
                count: 0,
                list: Vec::new(),
            },
            status: String::new(),
            symbol: String::new(),
            exchange: String::new(),
            country: String::new(),
            type_field: String::new(),
            include_delisted: false,
            page: 1,
            outputsize: 5000,
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

    pub fn page(&mut self, page: u32) -> &mut Self {
        self.page = page;
        self
    }

    pub fn outputsize(&mut self, outputsize: u32) -> &mut Self {
        self.outputsize = outputsize;
        self
    }

    pub async fn execute(&self) -> Result<Funds, Box<dyn Error>> {
        let include_delisted = &self.include_delisted.to_string();
        let page = &self.page.to_string();
        let outputsize = &self.outputsize.to_string();

        let params = vec![
            ("symbol", &self.symbol),
            ("exchange", &self.exchange),
            ("country", &self.country),
            ("type", &self.type_field),
            ("include_delisted", include_delisted),
            ("page", page),
            ("outputsize", outputsize),
        ];

        internal::request::execute("/funds", params).await
    }
}

#[cfg(test)]
mod test {
    use super::Funds;

    #[tokio::test]
    async fn get_funds() {
        let funds = Funds::builder().execute().await;

        assert!(funds.is_ok());
    }
}
