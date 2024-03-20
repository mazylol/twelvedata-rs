use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct Timeseries {
    pub meta: Meta,
    pub values: Vec<Value>,
    pub status: String,

    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    interval: String,
    #[serde(skip)]
    exchange: String,
    #[serde(skip)]
    mic_code: String,
    #[serde(skip)]
    country: String,
    #[serde(skip)]
    type_field: String,
    #[serde(skip)]
    outputsize: u16,
    #[serde(skip)]
    apikey: String,
    #[serde(skip)]
    prepost: String,
    #[serde(skip)]
    dp: String,
    #[serde(skip)]
    order: String,
    #[serde(skip)]
    timezone: String,
    #[serde(skip)]
    date: String,
    #[serde(skip)]
    start_date: String,
    #[serde(skip)]
    end_date: String,
    #[serde(skip)]
    previous_close: String,
    #[serde(skip)]
    adjust: String,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub symbol: String,
    pub interval: String,
    pub currency: String,
    pub exchange_timezone: String,
    pub exchange: String,
    pub mic_code: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Deserialize, Debug)]
pub struct Value {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

impl Timeseries {
    pub fn builder() -> Self {
        Timeseries {
            meta: Meta {
                symbol: String::new(),
                interval: String::new(),
                currency: String::new(),
                exchange_timezone: String::new(),
                exchange: String::new(),
                mic_code: String::new(),
                type_field: String::new(),
            },
            values: Vec::new(),
            status: String::new(),
            symbol: String::new(),
            interval: String::new(),
            exchange: String::new(),
            mic_code: String::new(),
            country: String::new(),
            type_field: String::new(),
            outputsize: 0,
            apikey: String::new(),
            prepost: String::new(),
            dp: String::new(),
            order: String::new(),
            timezone: String::new(),
            date: String::new(),
            start_date: String::new(),
            end_date: String::new(),
            previous_close: String::new(),
            adjust: String::new(),
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

    pub fn type_field(&mut self, type_field: &str) -> &mut Self {
        self.type_field = type_field.to_string();
        self
    }

    pub fn outputsize(&mut self, outputsize: u16) -> &mut Self {
        self.outputsize = outputsize;
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

    pub fn dp(&mut self, dp: &str) -> &mut Self {
        self.dp = dp.to_string();
        self
    }

    pub fn order(&mut self, order: &str) -> &mut Self {
        self.order = order.to_string();
        self
    }

    pub fn timezone(&mut self, timezone: &str) -> &mut Self {
        self.timezone = timezone.to_string();
        self
    }

    pub fn date(&mut self, date: &str) -> &mut Self {
        self.date = date.to_string();
        self
    }

    pub fn start_date(&mut self, start_date: &str) -> &mut Self {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(&mut self, end_date: &str) -> &mut Self {
        self.end_date = end_date.to_string();
        self
    }

    pub fn previous_close(&mut self, previous_close: &str) -> &mut Self {
        self.previous_close = previous_close.to_string();
        self
    }

    pub fn adjust(&mut self, adjust: &str) -> &mut Self {
        self.adjust = adjust.to_string();
        self
    }

    pub async fn execute(&self) -> Result<Timeseries, Box<dyn std::error::Error>> {
        let outputsize = self.outputsize.to_string();

        let params = vec![
            ("symbol", &self.symbol),
            ("interval", &self.interval),
            ("exchange", &self.exchange),
            ("mic_code", &self.mic_code),
            ("country", &self.country),
            ("type", &self.type_field),
            ("outputsize", &outputsize),
            ("apikey", &self.apikey),
            ("prepost", &self.prepost),
            ("dp", &self.dp),
            ("order", &self.order),
            ("timezone", &self.timezone),
            ("date", &self.date),
            ("start_date", &self.start_date),
            ("end_date", &self.end_date),
            ("previous_close", &self.previous_close),
            ("adjust", &self.adjust),
        ];

        internal::request::execute("https://api.twelvedata.com/time_series?", params).await
    }
}

#[cfg(test)]
pub mod test {
    use super::Timeseries;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_time_series() {
        dotenv().expect(".env file not found");

        let response = Timeseries::builder()
            .symbol("AAPL")
            .interval("1min")
            .apikey(&env::var("API_TOKEN").unwrap())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
