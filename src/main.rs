use std::{
    sync::{Arc, Mutex},
};

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::state::AppState;

mod engine;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState {
        users: Default::default(),
        markets: Default::default(),
        orders: Default::default(),
        positions: Default::default(),
        trades: Default::default(),
    }));

    let app = Router::new().route("/", get(root())).with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on port 3000");

    axum::serve(listener, app).await.unwrap();
}

fn root() -> &'static str {
    return "backend is running !";
}
