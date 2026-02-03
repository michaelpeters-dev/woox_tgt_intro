use serde_json::json;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::mpsc;
use crate::data::{WsOrderBookUpdate, WsOrderBookUpdateData, OrderBookSnapshot};
use reqwest::get;
use futures_util::{SinkExt, StreamExt};
use crate::orderbook::OrderBook;

// WooX public endpoint for market data
const WOOX_URL: &str = "wss://wss.woox.io/v3/public";
const SYMBOL: &str = "SPOT_ETH_USDT";
const DEPTH: usize = 50;

// Connects to the WooX websocket and buffers incoming orderbook updates
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel::<WsOrderBookUpdateData>(10_000);

    // establish connection
    let (ws, _) = connect_async(WOOX_URL).await?;
    println!("Connected to {}", WOOX_URL);
    let (mut write, mut read) = ws.split();

    // subscribe to updates
    let sub = json!({
        "cmd": "SUBSCRIBE",
        "params": [format!("orderbookupdate@{}@{}", SYMBOL, DEPTH)]
    }).to_string();
    write.send(Message::Text(sub)).await?;
    println!("Subscribed");

    // websocket task to read messages and forward orderbook updates into the channel
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    //println!("outer");
                    if let Ok(update) = serde_json::from_str::<WsOrderBookUpdate>(&text) {
                        //println!("{:?}", update.data.bids);
                        let _ = tx_clone.send(update.data).await;
                    }
                }
                Ok(Message::Ping(p)) => {
                    let _ = write.send(Message::Pong(p)).await;
                }
                _ => {}
            }
        }
    });
    
    // fetch a full orderbook snapshot
    let snapshot = fetch_snapshot().await?;
    println!(
        "Snapshot loaded | ts={} bids={} asks={}",
        snapshot.timestamp,
        snapshot.data.bids.len(),
        snapshot.data.asks.len()
    );

    // initialize the local orderbook from the snapshot
    let mut orderbook = OrderBook::from_snapshot(snapshot);
    while let Some(update) = rx.recv().await {
        //println!("{:?}", update);
        if orderbook.last_ts >= update.prev_ts {
            orderbook.apply_print(update);
        } else if orderbook.last_ts < update.prev_ts {
            println!("Missed updates, refetching snapshot");
            let snapshot = fetch_snapshot().await?;
            orderbook = OrderBook::from_snapshot(snapshot);
        } else {
            println!("Ignoring old update");
        }
    }
    Ok(())
}

// Fetches a full orderbook snapshot from the WooX REST API
async fn fetch_snapshot() -> Result<OrderBookSnapshot, reqwest::Error> {
    let url = format!(
        "https://api.woox.io/v3/public/orderbook?symbol={}&maxlevel={}",
        SYMBOL,
        DEPTH
    );
    let resp = get(&url).await?;
    let snapshot = resp.json::<OrderBookSnapshot>().await?;
    Ok(snapshot)
}
