use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub user_id: Uuid,

    pub market_id: Uuid,

    pub yes_shares: u64,

    pub no_shares: u64,
}

impl Position {
    pub fn total_shares(&self) -> u64 {
        self.yes_shares + self.no_shares
    }
}