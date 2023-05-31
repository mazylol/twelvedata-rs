pub mod types;

pub async fn get_realtime_price(ticker: &str, api_key: &str) -> f32 {
  let response: types::RealtimePrice = reqwest::Client::new()
    .get("https://twelve-data1.p.rapidapi.com/price")
    .header("X-RapidAPI-Key", api_key)
    .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
    .query(&[("symbol", ticker)])
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap_or_else(|_| panic!("Error getting realtime price for {}", ticker));

  return response.price.parse::<f32>().unwrap();

}