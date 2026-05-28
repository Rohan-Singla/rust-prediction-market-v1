use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Outcome {
    Yes,
    No,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketStatus {
    Active,
    Pending,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub market_id: String,

    pub question: String,

    pub status: MarketStatus,

    pub resolved_outcome: Option<Outcome>,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub volume: u64,

    pub open_interest: u64,
}