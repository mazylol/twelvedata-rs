use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RealtimePrice {
    pub price: String,
}
