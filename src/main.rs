mod client;
mod data;
mod orderbook;

// Starts the async orderbook client
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    client::run().await
}
