use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::market::Outcome;


#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Order{
    pub id : String,

    pub user_id : String,

    pub order_id : String,

    pub side : Side,
    pub outcome : Outcome,

    pub price : u64,

    pub quantity : u64,

    pub remaining : u64,

    pub createdAt : DateTime<Utc>
}