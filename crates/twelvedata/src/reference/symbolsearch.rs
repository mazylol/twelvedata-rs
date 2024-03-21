use std::error::Error;

use serde::Deserialize;

use crate::internal;

#[derive(Deserialize, Debug)]
pub struct SymbolSearch {
    pub data: Vec<Symbol>,
    pub status: String,

    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    outputsize: u32,
}

#[derive(Deserialize, Debug)]
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

impl SymbolSearch {
    pub fn builder() -> Self {
        SymbolSearch {
            data: Vec::new(),
            status: String::new(),
            symbol: String::new(),
            outputsize: 30,
        }
    }

    pub fn symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn outputsize(&mut self, outputsize: u32) -> &mut Self {
        self.outputsize = outputsize;
        self
    }

    pub async fn execute(&self) -> Result<SymbolSearch, Box<dyn Error>> {
        let outputsize = if self.outputsize > 120 {
            120.to_string()
        } else {
            self.outputsize.to_string()
        };

        let params = vec![("symbol", &self.symbol), ("outputsize", &outputsize)];

        internal::request::execute("/symbol_search", params).await
    }
}

#[cfg(test)]
mod test {
    use super::SymbolSearch;

    #[tokio::test]
    async fn get_symbol_search() {
        let symbol_search = SymbolSearch::builder()
            .symbol("AAPL")
            .outputsize(10)
            .execute()
            .await;

        assert!(symbol_search.is_ok());
    }
}
