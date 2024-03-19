use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    delimiter: String,
    #[serde(skip)]
    api_key: String,
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
            delimiter: String::new(),
            api_key: String::new(),
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

    pub fn delimiter(&mut self, delimiter: &str) -> &mut Self {
        self.delimiter = delimiter.to_string();
        self
    }

    pub fn api_key(&mut self, api_key: &str) -> &mut Self {
        self.api_key = api_key.to_string();
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
        let client = reqwest::Client::new();

        let url = format!("https://api.twelvedata.com/price?");

        let params = vec![
            ("apikey", &self.api_key),
            ("symbol", &self.symbol),
            ("exchange", &self.exchange),
            ("mic_code", &self.mic_code),
            ("country", &self.country),
            ("type", &self.type_field),
            ("delimiter", &self.delimiter),
            ("prepost", &self.prepost),
            ("dp", &self.dp),
        ];

        let filtered_params: Vec<(&str, &str)> = params
            .into_iter()
            .filter(|(_, value)| !value.is_empty())
            .map(|(key, value)| (key, value.as_str()))
            .collect();

        let response = client.get(&url).query(&filtered_params).send().await?;

        if response.status().is_success() {
            let realtime_price = response.json::<RealtimePrice>().await?;
            Ok(realtime_price)
        } else {
            Err(format!("Request failed with status code: {}", response.status()).into())
        }
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
            .api_key(env::var("API_TOKEN").unwrap().as_str())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
