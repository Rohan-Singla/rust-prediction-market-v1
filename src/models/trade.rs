use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,

    pub market_id: Uuid,

    pub buyer_id: Uuid,

    pub seller_id: Uuid,

    pub price: u64,

    pub quantity: u64,

    pub created_at: DateTime<Utc>,
}