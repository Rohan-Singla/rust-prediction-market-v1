use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


// Define market structs with macros for easier code flow ...

#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum MarketStatus {
    Active,
    Pending,
    Resolved
}

#[derive(Serialize,Deserialize,Debug,Clone)]

pub enum Outcome {
    Yes,
    No
}


#[derive(Serialize,Deserialize,Debug,Clone)]

pub struct Market {
    pub id : String,
    pub question : String,
    pub status : MarketStatus,
    pub resolved_outcome : Option<Outcome>,
    pub createdAt : DateTime<Utc>
}

