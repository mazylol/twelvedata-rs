use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::internal;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyConversion {
    pub symbol: String,
    pub rate: f32,
    pub amount: f32,
    pub timestamp: u32,
    #[serde(skip)]
    date: String,
    #[serde(skip)]
    apikey: String,
    #[serde(skip)]
    dp: String,
    #[serde(skip)]
    timezone: String,
}

impl CurrencyConversion {
    pub fn builder() -> Self {
        CurrencyConversion {
            symbol: String::new(),
            rate: 0.0,
            amount: 0.0,
            timestamp: 0,
            date: String::new(),
            apikey: String::new(),
            dp: String::new(),
            timezone: String::new(),
        }
    }

    pub fn symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn amount(&mut self, amount: f32) -> &mut Self {
        self.amount = amount;
        self
    }

    pub fn date(&mut self, date: &str) -> &mut Self {
        self.date = date.to_string();
        self
    }

    pub fn apikey(&mut self, apikey: &str) -> &mut Self {
        self.apikey = apikey.to_string();
        self
    }

    pub fn dp(&mut self, dp: &str) -> &mut Self {
        self.dp = dp.to_string();
        self
    }

    pub fn timezone(&mut self, timezone: &str) -> &mut Self {
        self.timezone = timezone.to_string();
        self
    }

    pub async fn execute(&self) -> Result<CurrencyConversion, Box<dyn Error>> {
        let amount = &self.amount.to_string();

        let params = vec![
            ("apikey", &self.apikey),
            ("symbol", &self.symbol),
            ("amount", amount),
            ("date", &self.date),
            ("dp", &self.dp),
            ("timezone", &self.timezone),
        ];

        internal::request::execute("https://api.twelvedata.com/currency_conversion?", params).await
    }
}

#[cfg(test)]
pub mod test {
    use super::CurrencyConversion;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_conversion() {
        dotenv().expect(".env file not found");

        let response = CurrencyConversion::builder()
            .symbol("USD/JPY")
            .amount(12.0)
            .apikey(&env::var("API_TOKEN").unwrap())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
