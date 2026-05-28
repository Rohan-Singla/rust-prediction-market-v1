use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    engine::orderbook::OrderBook,
    models::{
        market::{Market, Outcome},
        trade::Trade,
        user::User,
    },
};

pub type SharedState = Arc<Mutex<AppState>>;

pub struct AppState {
    pub users: HashMap<String, User>,

    pub markets: HashMap<String, Market>,

    // (market_id, outcome)
    pub orderbooks: HashMap<(String, Outcome), OrderBook>,

    pub trades: Vec<Trade>,
}