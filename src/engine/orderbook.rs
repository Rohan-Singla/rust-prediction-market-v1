use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::order::OrderStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrder {
    pub user_id: String,

    pub quantity: u64,

    pub filled_quantity: u64,

    pub order_id: String,

    pub created_at: DateTime<Utc>,

    pub status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub available_quantity: u64,

    pub open_orders: Vec<OpenOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub bids: BTreeMap<u64, PriceLevel>,

    pub asks: BTreeMap<u64, PriceLevel>,

    pub last_traded_price: Option<u64>,
}