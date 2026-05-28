use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Position {

    pub user_id : String,
    pub market_id : String,

    pub yes_shares: u64,
    pub no_shares : u64,
}