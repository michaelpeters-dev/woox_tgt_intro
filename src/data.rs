use serde::Deserialize;

// WebSocket wrapper for an order book update message
#[derive(Debug, Deserialize)]
pub struct WsOrderBookUpdate {
    pub data: WsOrderBookUpdateData,
}

// Data contained within an WsOrderBookUpdate
#[derive(Debug, Deserialize)]
pub struct WsOrderBookUpdateData {
    #[serde(rename = "prevTs")]
    pub prev_ts: u64,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub ts: u64,
}

// Represents a single bid or ask level
// can be used for both as they follow the same "price, quantity" structure
#[derive(Debug, Deserialize)]
pub struct PriceLevel {
    pub price: String,
    pub quantity: String,
}

// REST API orderbook snapshot response
#[derive(Debug, Deserialize)]
pub struct OrderBookSnapshot {
    pub timestamp: u64,
    pub data: OrderBookSnapshotData,
}

// Data contained within an OrderBookSnapshot
#[derive(Debug, Deserialize)]
pub struct OrderBookSnapshotData {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}
