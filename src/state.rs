use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::models::{market::Market, order::Order, position::Position, trade::Trade, user::User};


pub struct AppState{
    pub users : HashMap<String,User>,

    pub markets : HashMap<String,Market>,

    pub orders : HashMap<String,Order>,

    pub positions : HashMap<(String,String),Position>,

    pub trades:  Vec<Trade>


}
pub type sharedState = Arc<Mutex<AppState>>;