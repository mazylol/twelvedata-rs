use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct Exchanges {
    pub data: Vec<Exchange>,
    pub status: String,

    #[serde(skip)]
    type_field: String,
    #[serde(skip)]
    name: String,
    #[serde(skip)]
    code: String,
    #[serde(skip)]
    country: String,
}

#[derive(Deserialize, Debug)]
pub struct Exchange {
    pub name: String,
    pub code: String,
    pub country: String,
    pub timezone: String,
}

impl Exchanges {
    pub fn builder() -> Self {
        Exchanges {
            data: Vec::new(),
            status: String::new(),
            type_field: String::new(),
            name: String::new(),
            code: String::new(),
            country: String::new(),
        }
    }

    pub fn type_field(&mut self, type_field: &str) -> &mut Self {
        self.type_field = type_field.to_string();
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn code(&mut self, code: &str) -> &mut Self {
        self.code = code.to_string();
        self
    }

    pub fn country(&mut self, country: &str) -> &mut Self {
        self.country = country.to_string();
        self
    }

    pub async fn execute(&self) -> Result<Exchanges, Box<dyn Error>> {
        let params = vec![
            ("type", &self.type_field),
            ("name", &self.name),
            ("code", &self.code),
            ("country", &self.country),
        ];

        internal::request::execute("/exchanges", params).await
    }
}

#[cfg(test)]
mod test {
    use super::Exchanges;

    #[tokio::test]
    async fn get_exchanges() {
        let exchanges = Exchanges::builder().execute().await;

        assert!(exchanges.is_ok());
    }
}
