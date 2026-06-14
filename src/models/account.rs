use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub user_id: Uuid,

    pub available_balance: u64,

    pub locked_balance: u64,
}

impl Account {
    pub fn free_balance(&self) -> u64 {
        self.available_balance
    }

    pub fn total_balance(&self) -> u64 {
        self.available_balance + self.locked_balance
    }
}