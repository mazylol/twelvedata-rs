use crate::internal;

use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub mic_code: String,
    pub currency: String,
    pub datetime: String,
    pub timestamp: i32,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub previous_close: String,
    pub change: String,
    pub percent_change: String,
    pub average_volume: String,
    pub is_market_open: bool,
    pub fifty_two_week: FiftyTwoWeek,

    #[serde(skip)]
    interval: String,
    #[serde(skip)]
    country: String,
    #[serde(skip)]
    volume_time_period: String,
    #[serde(skip)]
    type_field: String,
    #[serde(skip)]
    apikey: String,
    #[serde(skip)]
    // Pro and Above
    prepost: String,
    #[serde(skip)]
    eod: String,
    #[serde(skip)]
    rolling_period: String,
    #[serde(skip)]
    dp: String,
    #[serde(skip)]
    timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FiftyTwoWeek {
    pub low: String,
    pub high: String,
    pub low_change: String,
    pub high_change: String,
    pub low_change_percent: String,
    pub high_change_percent: String,
    pub range: String,
}

impl Quote {
    pub fn builder() -> Self {
        Quote {
            symbol: String::new(),
            name: String::new(),
            exchange: String::new(),
            mic_code: String::new(),
            currency: String::new(),
            datetime: String::new(),
            timestamp: 0,
            open: String::new(),
            high: String::new(),
            low: String::new(),
            close: String::new(),
            volume: String::new(),
            previous_close: String::new(),
            change: String::new(),
            percent_change: String::new(),
            average_volume: String::new(),
            is_market_open: false,
            fifty_two_week: FiftyTwoWeek {
                low: String::new(),
                high: String::new(),
                low_change: String::new(),
                high_change: String::new(),
                low_change_percent: String::new(),
                high_change_percent: String::new(),
                range: String::new(),
            },
            interval: String::new(),
            country: String::new(),
            volume_time_period: String::new(),
            type_field: String::new(),
            apikey: String::new(),
            prepost: String::new(),
            eod: String::new(),
            rolling_period: String::new(),
            dp: String::new(),
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

    pub fn country(&mut self, country: &str) -> &mut Self {
        self.country = country.to_string();
        self
    }

    pub fn volume_time_period(&mut self, volume_time_period: &str) -> &mut Self {
        self.volume_time_period = volume_time_period.to_string();
        self
    }

    pub fn type_field(&mut self, type_field: &str) -> &mut Self {
        self.type_field = type_field.to_string();
        self
    }

    pub fn apikey(&mut self, apikey: &str) -> &mut Self {
        self.apikey = apikey.to_string();
        self
    }

    pub fn prepost(&mut self, prepost: &str) -> &mut Self {
        self.prepost = prepost.to_string();
        self
    }

    pub fn eod(&mut self, eod: &str) -> &mut Self {
        self.eod = eod.to_string();
        self
    }

    pub fn rolling_period(&mut self, rolling_period: &str) -> &mut Self {
        self.rolling_period = rolling_period.to_string();
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

    pub async fn execute(&self) -> Result<Quote, Box<dyn Error>> {
        let params = vec![
            ("symbol", &self.symbol),
            ("interval", &self.interval),
            ("exchange", &self.exchange),
            ("mic_code", &self.mic_code),
            ("country", &self.country),
            ("volume_time_period", &self.volume_time_period),
            ("type", &self.type_field),
            ("apikey", &self.apikey),
            ("prepost", &self.prepost),
            ("eod", &self.eod),
            ("rolling_period", &self.rolling_period),
            ("dp", &self.dp),
            ("timezone", &self.timezone),
        ];

        internal::request::execute("https://api.twelvedata.com/quote?", params).await
    }
}

#[cfg(test)]
pub mod test {
    use super::Quote;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_quote() {
        dotenv().expect(".env file not found");

        let response = Quote::builder()
            .symbol("AAPL")
            .apikey(&env::var("API_TOKEN").unwrap())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
