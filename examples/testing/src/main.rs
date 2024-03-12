use twelvedata::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("TOKEN_HERE");

    println!("{}", client.realtime_price("AMZN").await);
}
