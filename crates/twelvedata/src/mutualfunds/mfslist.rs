use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct MutalFundsList {
    pub result: MutalFundsListResult,
    pub status: String,

    #[serde(skip)]
    pub symbol: String,
    #[serde(skip)]
    pub country: String,
    #[serde(skip)]
    pub fund_family: String,
    #[serde(skip)]
    pub fund_type: String,
    #[serde(skip)]
    pub performance_rating: u8,
    #[serde(skip)]
    pub risk_rating: u8,
    #[serde(skip)]
    pub page: u32,
    #[serde(skip)]
    pub outputsize: u32,
    #[serde(skip)]
    pub apikey: String,
}

#[derive(Deserialize, Debug)]
pub struct MutalFundsListResult {
    pub count: u32,
    pub list: Vec<MutalFund>,

}
#[derive(Deserialize, Debug)]
pub struct MutalFund {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub fund_family: String,
    pub fund_type: String,
    pub performance_rating: u8,
    pub risk_rating: u8,
    pub exchange: String,
    pub mic_code: String,
}

impl MutalFundsList {
    pub fn builder() -> Self {
        MutalFundsList {
            result: MutalFundsListResult {
                count: 0,
                list: Vec::new(),
            },
            status: String::new(),
            symbol: String::new(),
            country: String::new(),
            fund_family: String::new(),
            fund_type: String::new(),
            performance_rating: 6,
            risk_rating: 6,
            page: 1,
            outputsize: 100,
            apikey: String::new(),
        }
    }

    pub fn symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn country(&mut self, country: &str) -> &mut Self {
        self.country = country.to_string();
        self
    }

    pub fn fund_family(&mut self, fund_family: &str) -> &mut Self {
        self.fund_family = fund_family.to_string();
        self
    }

    pub fn fund_type(&mut self, fund_type: &str) -> &mut Self {
        self.fund_type = fund_type.to_string();
        self
    }

    pub fn performance_rating(&mut self, performance_rating: u8) -> &mut Self {
        self.performance_rating = performance_rating;
        self
    }

    pub fn risk_rating(&mut self, risk_rating: u8) -> &mut Self {
        self.risk_rating = risk_rating;
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

    pub fn apikey(&mut self, apikey: &str) -> &mut Self {
        self.apikey = apikey.to_string();
        self
    }

    pub async fn execute(&self) -> Result<MutalFundsList, Box<dyn Error>> {
        // since these values are ranges from 0 to 5 they will never be six, the default value
        let performance_rating = match self.performance_rating {
            6 => String::new(),
            _ => self.performance_rating.to_string()
        };
        let risk_rating = match self.risk_rating {
            6 => String::new(),
            _ => self.performance_rating.to_string()
        };

        let page = &self.page.to_string();
        let outputsize = &self.outputsize.to_string();

        let params = vec![
            ("symbol", &self.symbol),
            ("country", &self.country),
            ("fund_family", &self.fund_family),
            ("fund_type", &self.fund_type),
            ("performance_rating", &performance_rating),
            ("risk_rating", &risk_rating),
            ("page", page),
            ("outputsize", outputsize),
            ("apikey", &self.apikey),
        ];

        internal::request::execute("/mutual_funds/list", params).await
    }
}

#[cfg(test)]
mod test {
    use super::MutalFundsList;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_mutal_funds() {
        dotenv().expect(".env file not found");

        let response = MutalFundsList::builder()
            .apikey(&env::var("API_TOKEN").unwrap())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}