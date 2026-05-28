use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::market::Outcome;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,

    pub market_id: String,

    pub user_id: String,

    pub outcome: Outcome,

    pub side: Side,

    pub quantity: u64,

    pub filled_quantity: u64,

    pub order_type: OrderType,

    pub price: u64,

    pub status: OrderStatus,

    pub created_at: DateTime<Utc>,
}