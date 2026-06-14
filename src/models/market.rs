use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: Uuid,

    pub title: String,

    pub description: String,

    // e.g. "Will BTC > 100k by 2027?"
    pub resolution_rule: String,

    pub status: MarketStatus,

    pub created_at: DateTime<Utc>,

    pub closes_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketStatus {
    Active,
    Closed,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Yes,
    No,
}