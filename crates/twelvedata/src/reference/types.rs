use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stocks {
    pub data: Vec<Stock>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
    pub country: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForexPairs {
    pub data: Vec<ForexPair>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForexPair {
    pub symbol: String,
    pub currency_group: String,
    pub currency_base: String,
    pub currency_quote: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cryptocurrencies {
    pub data: Vec<Cryptocurrency>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cryptocurrency {
    pub symbol: String,
    pub available_exchanges: Vec<String>,
    pub currency_base: String,
    pub currency_quote: String,
}
