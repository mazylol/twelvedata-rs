use crate::internal;

use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExchangeRate {
    pub symbol: String,
    pub rate: f32,
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

impl ExchangeRate {
    pub fn builder() -> Self {
        ExchangeRate {
            symbol: String::new(),
            rate: 0.0,
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

    pub async fn execute(&self) -> Result<ExchangeRate, Box<dyn Error>> {
        let params = vec![
            ("symbol", &self.symbol),
            ("date", &self.date),
            ("apikey", &self.apikey),
            ("dp", &self.dp),
            ("timezone", &self.timezone),
        ];

        internal::request::execute("/exchange_rate", params).await
    }
}

#[cfg(test)]
mod test {
    use super::ExchangeRate;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_exchangerate() {
        dotenv().expect(".env file not found");

        let response = ExchangeRate::builder()
            .symbol("EUR/USD")
            .apikey(&env::var("API_TOKEN").unwrap())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
