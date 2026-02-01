use crate::data::{BidAsk, WsOrderBookUpdateData, OrderBookSnapshot};

// Representation of an orderbook
#[derive(Debug)]
pub struct OrderBook {
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
    pub last_ts: u64,
    pub initialized: bool,
}

impl OrderBook {
    // Creates an orderbook from a REST snapshot
    pub fn from_snapshot(snapshot: OrderBookSnapshot) -> Self {
        Self {
            bids: snapshot.data.bids,
            asks: snapshot.data.asks,
            last_ts: snapshot.timestamp,
            initialized: false,
        }
    }  
}

impl OrderBook {
    // Applies an incremental update to an initialized orderbook
    pub fn apply_update(&mut self, update: WsOrderBookUpdateData) -> bool {
        if update.prev_ts != self.last_ts {
            return false;
        }
        self.last_ts = update.ts;
        self.bids.extend(update.bids);
        self.asks.extend(update.asks);
        true
    }   
}

impl OrderBook {
    // Prints the order book (amount of level is dependant on DEPTH in client.rs)
    pub fn print(&self, depth: usize) {
        println!();
        println!("BIDS\t\t\tASKS");
        println!();

        for i in 0..depth {
            let bid = self.bids.get(i);
            let ask = self.asks.get(i);

            let bid_text = match bid {
                Some(b) => format!("{} {}", b.price, b.quantity),
                None => String::from(""),
            };

            let ask_text = match ask {
                Some(a) => format!("{} {}", a.price, a.quantity),
                None => String::from(""),
            };

            println!("{}\t\t{}", bid_text, ask_text);
            println!();
        }
    }
}
