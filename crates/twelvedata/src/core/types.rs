use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RealtimePrice {
  pub price: String,
}