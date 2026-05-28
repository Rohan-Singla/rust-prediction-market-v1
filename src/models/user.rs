use serde::{Deserialize, Serialize};

use super::{
    order::Order,
    position::Position,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub auth_user: AuthUser,

    pub trading_account: TradingAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: String,

    pub username: String,

    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingAccount {
    pub collateral: Collateral,

    pub positions: Vec<Position>,

    pub orders: Vec<Order>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collateral {
    pub available: u64,

    pub locked: u64,
}