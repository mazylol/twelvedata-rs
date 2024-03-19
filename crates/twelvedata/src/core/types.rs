use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeseries {
    pub meta: Meta,
    pub values: Vec<Value>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeRate {
    pub symbol: String,
    pub rate: f32,
    pub timestamp: i32,
}

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
