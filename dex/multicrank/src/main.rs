//! A CLI server that wraps Serum's crank binary into a convenient API for
//! starting and monitoring market cranks.

mod crank;
mod error;
mod handler;
mod model;
mod monitor;
mod route;
mod state;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use clap::Parser;
use monitor::Monitor;
use route::*;
use serde::{Deserialize, Serialize};
use state::State;
use tokio::{join, sync::Mutex};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Arguments {
    /// Path to the crank binary
    #[arg(short, long)]
    crank: PathBuf,
    /// RPC endpoint
    #[arg(short, long)]
    rpc: String,
    /// Path to gas payer id.json
    #[arg(short, long)]
    gas_payer: PathBuf,
    /// Server socket
    #[arg(short, long)]
    socket: u16,
    /// Path to markets.json
    #[arg(short, long)]
    markets: Option<PathBuf>,
    /// Path to folder where state and logs will be stored
    #[arg(short, long)]
    persist: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Arguments::parse();

    let state = Arc::new(Mutex::new(State::from_args(&args)?));
    let app = Router::new()
        .route("/start_crank", post(start_crank))
        .route("/active_cranks", get(active_cranks))
        .route("/logs/:market_id", get(logs))
        .route("/purge/:market_id", get(purge))
        .layer(Extension(state.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], args.socket));
    tracing::info!("listening on {}", addr);
    let server_fut = axum::Server::bind(&addr).serve(app.into_make_service());
    let monitor = Monitor::new(state);
    let monitor_fut = monitor.init();

    let _ = join!(server_fut, monitor_fut);

    Ok(())
}
