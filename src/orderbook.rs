use crate::data::{PriceLevel, WsOrderBookUpdateData, OrderBookSnapshot};

// Representation of an orderbook
#[derive(Debug)]
pub struct OrderBook {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub last_ts: u64,
}

impl OrderBook {
    // Applies incremental updates to the orderbook and prints it to the console
    pub fn apply_print(&mut self, update: WsOrderBookUpdateData) {
        self.last_ts = update.ts;
        self.bids.extend(update.bids);
        self.asks.extend(update.asks);

        println!();
        println!("BIDS\t\t\tASKS");
        println!();

        for i in 0..5 {
            let bid = self.bids.get(i);
            let ask = self.asks.get(i);

            let bid_text = match bid {
                Some(b) => format!("{}\t{}", b.price, b.quantity),
                None => String::from(""),
            };

            let ask_text = match ask {
                Some(a) => format!("{}\t{}", a.price, a.quantity),
                None => String::from(""),
            };

            println!("{}\t\t{}", bid_text, ask_text);
        }
        println!("--------");

        println!();
    }

    // Creates an orderbook from a REST snapshot
    pub fn from_snapshot(snapshot: OrderBookSnapshot) -> Self {
        Self {
            bids: snapshot.data.bids,
            asks: snapshot.data.asks,
            last_ts: snapshot.timestamp,
        }
    }
}
