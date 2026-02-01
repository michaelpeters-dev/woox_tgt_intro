use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::mpsc;

const WOOX_URL: &str = "wss://wss.woox.io/v3/public"; // WooX public market data endpoint
const SYMBOL: &str = "PERP_ETH_USDT";
const DEPTH: usize = 50;

// Connects to the WooX websocket and buffers incoming orderbook updates
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // channel
    let (tx, mut rx) = mpsc::channel::<String>(10_000);

    // connect
    let (ws, _) = connect_async(WOOX_URL).await?;
    println!("connected to {}", WOOX_URL);

    let (mut write, mut read) = ws.split();

    // subscribe
    let sub = json!({
        "cmd": "SUBSCRIBE",
        "params": [format!("orderbookupdate@{}@{}", SYMBOL, DEPTH)]
    }).to_string();
    write.send(Message::Text(sub)).await?;
    println!("subscribed");

    // websocket listener task
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if tx_clone.send(text).await.is_err() {
                        break;
                    }
                }
                Ok(Message::Ping(p)) => {
                    let _ = write.send(Message::Pong(p)).await;
                }
                _ => {}
            }
        }
    });

    // processing loop
    while let Some(msg) = rx.recv().await {
        println!("buffered update: {msg}\n");
    }

    Ok(())
}
