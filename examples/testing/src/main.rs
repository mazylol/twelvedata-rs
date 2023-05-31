use twelvedata::core::get_realtime_price;

#[tokio::main]
async fn main() {
    println!("{}", get_realtime_price("AMZN", "TOKEN_HERE").await);
}
