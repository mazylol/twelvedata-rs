use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct RealtimePrice {
    pub price: String,

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
    apikey: String,
    #[serde(skip)]
    prepost: String,
    #[serde(skip)]
    dp: String,
}

/// Builder for the Realtime Price endpoint
impl RealtimePrice {
    pub fn builder() -> Self {
        RealtimePrice {
            price: String::new(),
            symbol: String::new(),
            exchange: String::new(),
            mic_code: String::new(),
            country: String::new(),
            type_field: String::new(),
            apikey: String::new(),
            prepost: String::new(),
            dp: String::new(),
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

    pub async fn execute(&self) -> Result<RealtimePrice, Box<dyn Error>> {
        let params = vec![
            ("apikey", &self.apikey),
            ("symbol", &self.symbol),
            ("exchange", &self.exchange),
            ("mic_code", &self.mic_code),
            ("country", &self.country),
            ("type", &self.type_field),
            ("prepost", &self.prepost),
            ("dp", &self.dp),
        ];

        internal::request::execute("https://api.twelvedata.com/price", params).await
    }
}

#[cfg(test)]
pub mod test {
    use super::RealtimePrice;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_stock_price() {
        dotenv().expect(".env file not found");

        let response = RealtimePrice::builder()
            .symbol("AAPL")
            .apikey(&env::var("API_TOKEN").unwrap().as_str())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
