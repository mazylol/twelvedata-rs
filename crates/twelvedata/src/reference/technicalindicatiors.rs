use std::{collections::HashMap, error::Error};

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
struct TechnicalIndicators {
    pub data: HashMap<String, TechnicalIndicator>,
}

#[derive(Deserialize, Debug)]
struct TechnicalIndicator {
    pub enable: bool,
    pub full_name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub overlay: bool,
    pub parameters: Parameters,
}

#[derive(Deserialize, Debug)]
struct Parameters {}

impl TechnicalIndicators {
    pub fn builder() -> Self {
        TechnicalIndicators {
            data: HashMap::new(),
        }
    }
}
