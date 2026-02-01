mod client;
mod data;
mod orderbook;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    client::run().await
}
