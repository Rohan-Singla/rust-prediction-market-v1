use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::market::Outcome;


#[derive(Clone,Deserialize,Serialize,Debug)]
pub struct Trade {
    pub id : String,

    pub user_id : String,

    pub buyer_id : String,

    pub seller_id : String,

    pub outcome : Outcome,

    pub price : u64,

    pub quantity : u64,

    pub createdAt : DateTime<Utc>
}