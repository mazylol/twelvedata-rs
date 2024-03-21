use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct EarliestTimestamp {
    pub datetime: String,
    pub unix_time: i32,

    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    interval: String,
    #[serde(skip)]
    exchange: String,
    #[serde(skip)]
    mic_code: String,
    #[serde(skip)]
    apikey: String,
    #[serde(skip)]
    timezone: String,
}

impl EarliestTimestamp {
    pub fn builder() -> Self {
        EarliestTimestamp {
            datetime: String::new(),
            unix_time: 0,
            symbol: String::new(),
            interval: String::new(),
            exchange: String::new(),
            mic_code: String::new(),
            apikey: String::new(),
            timezone: String::new(),
        }
    }

    pub fn symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn interval(&mut self, interval: &str) -> &mut Self {
        self.interval = interval.to_string();
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

    pub fn apikey(&mut self, apikey: &str) -> &mut Self {
        self.apikey = apikey.to_string();
        self
    }

    pub fn timezone(&mut self, timezone: &str) -> &mut Self {
        self.timezone = timezone.to_string();
        self
    }

    pub async fn execute(&self) -> Result<EarliestTimestamp, Box<dyn Error>> {
        let params = vec![
            ("symbol", &self.symbol),
            ("interval", &self.interval),
            ("exchange", &self.exchange),
            ("mic_code", &self.mic_code),
            ("apikey", &self.apikey),
            ("timezone", &self.timezone),
        ];

        internal::request::execute("/earliest_timestamp", params).await
    }
}

#[cfg(test)]
mod test {
    use super::EarliestTimestamp;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_earliest_timestamp() {
        dotenv().expect(".env file not found");

        let earliest_timestamp = EarliestTimestamp::builder()
            .symbol("AAPL")
            .apikey(&env::var("API_TOKEN").unwrap())
            .interval("1min")
            .execute()
            .await;

        assert!(earliest_timestamp.is_ok());
    }
}
