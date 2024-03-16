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

#[derive(Serialize, Deserialize, Debug)]
pub struct ETFs {
    pub data: Vec<ETF>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ETF {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Indices {
    pub data: Vec<Index>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exchanges {
    pub data: Vec<Exchange>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exchange {
    pub name: String,
    pub code: String,
    pub country: String,
    pub timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoExchanges {
    pub data: Vec<CryptoExchange>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoExchange {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolSearch {
    pub data: Vec<Symbol>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Symbol {
    pub symbol: String,
    pub instrument_name: String,
    pub exchange: String,
    pub mic_code: String,
    pub exchange_timezone: String,
    pub instrument_type: String,
    pub country: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EarliestTimestamp {
    pub datetime: String,
    pub unix_time: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Access {
    pub global: String,
    pub plan: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fund {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub currency: String,
    pub exchange: String,
    #[serde(rename = "type")]
    pub fund_type: String,
    pub access: Option<Access>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    pub count: u32,
    pub list: Vec<Fund>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Funds {
    pub result: Result,
    pub status: String,
}
