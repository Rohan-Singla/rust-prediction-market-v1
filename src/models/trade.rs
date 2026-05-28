use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::market::Outcome;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,

    pub market_id: String,

    pub buyer_id: String,

    pub seller_id: String,

    pub outcome: Outcome,

    pub price: u64,

    pub quantity: u64,

    pub created_at: DateTime<Utc>,
}