use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyConversion {
    pub symbol: String,
    pub rate: f32,
    pub amount: f32,
    pub timestamp: u32,
    #[serde(skip)]
    date: String,
    #[serde(skip)]
    delimiter: String,
    #[serde(skip)]
    api_key: String,
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
            delimiter: String::new(),
            api_key: String::new(),
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

    pub fn delimiter(&mut self, delimiter: &str) -> &mut Self {
        self.delimiter = delimiter.to_string();
        self
    }

    pub fn api_key(&mut self, api_key: &str) -> &mut Self {
        self.api_key = api_key.to_string();
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
        let client = reqwest::Client::new();

        let url = format!("https://api.twelvedata.com/currency_conversion?");

        let amount = &self.amount.to_string();

        let params = vec![
            ("apikey", &self.api_key),
            ("symbol", &self.symbol),
            ("amount", amount),
            ("date", &self.date),
            ("delimiter", &self.delimiter),
            ("dp", &self.dp),
            ("timezone", &self.timezone),
        ];

        let filtered_params: Vec<(&str, &str)> = params
            .into_iter()
            .filter(|(_, value)| !value.is_empty())
            .map(|(key, value)| (key, value.as_str()))
            .collect();

        let response = client.get(&url).query(&filtered_params).send().await?;

        if response.status().is_success() {
            let currency_conversion = response.json::<CurrencyConversion>().await?;
            Ok(currency_conversion)
        } else {
            Err(format!("Request failed with status code: {}", response.status()).into())
        }
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
            .api_key(env::var("API_TOKEN").unwrap().as_str())
            .execute()
            .await;

        assert!(response.is_ok());
    }
}
