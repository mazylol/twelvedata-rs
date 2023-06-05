pub mod core;

pub struct Client {
    pub api_key: String,
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
        }
    }
}
