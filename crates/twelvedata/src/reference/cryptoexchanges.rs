use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct CryptoExchanges {
    pub data: Vec<CryptoExchange>,
}

#[derive(Deserialize, Debug)]
pub struct CryptoExchange {
    pub name: String,
}

impl CryptoExchanges {
    pub fn builder() -> Self {
        CryptoExchanges { data: Vec::new() }
    }

    pub async fn execute(&self) -> Result<CryptoExchanges, Box<dyn Error>> {
        internal::request::execute("/cryptocurrency_exchanges", Vec::new()).await
    }
}

#[cfg(test)]
mod test {
    use super::CryptoExchanges;

    #[tokio::test]
    async fn get_crypto_exchanges() {
        let exchanges = CryptoExchanges::builder().execute().await;

        assert!(exchanges.is_ok());
    }
}
