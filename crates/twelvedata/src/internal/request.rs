use std::error::Error;

pub async fn execute<T: serde::de::DeserializeOwned>(
    endpoint: &str,
    params: Vec<(&str, &String)>,
) -> Result<T, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let filtered_params: Vec<(&str, &str)> = params
        .into_iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| (key, value.as_str()))
        .collect();

    let url = format!("https://api.twelvedata.com{}", endpoint);

    let response = client.get(url).query(&filtered_params).send().await?;

    if response.status().is_success() {
        let exchange_rate = response.json::<T>().await?;
        Ok(exchange_rate)
    } else {
        Err(format!("Request failed with status code: {}", response.status()).into())
    }
}
