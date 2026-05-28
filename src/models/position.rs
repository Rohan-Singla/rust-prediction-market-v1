use serde::{Deserialize, Serialize};

use super::market::Outcome;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub market_id: String,

    pub outcome: Outcome,

    pub quantity: u64,

    pub average_price: u64,
}