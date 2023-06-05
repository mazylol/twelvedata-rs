use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RealtimePrice {
    pub price: String,
}

#[derive(Serialize, Deserialize)]
pub struct Timeseries {
    pub meta: Meta,
    pub values: Vec<Value>,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}
