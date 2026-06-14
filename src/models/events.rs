use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{order::Order, trade::Trade};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    OrderPlaced(Order),

    OrderCancelled {
        order_id: Uuid,
    },

    TradeExecuted(Trade),

    MarketClosed {
        market_id: Uuid,
    },

    MarketResolved {
        market_id: Uuid,
        outcome: Outcome,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Yes,
    No,
}