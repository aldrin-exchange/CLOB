//! Endpoint handler implementations.

use std::sync::Arc;

use anyhow::anyhow;
use axum::Json;
use tokio::sync::Mutex;

use crate::{
    error::AppError,
    model::{CrankRequest, Market},
    state::State,
};

pub async fn start_crank(
    req: CrankRequest,
    state: Arc<Mutex<State>>,
) -> Result<(), AppError> {
    let mut state = state.lock().await;
    state.add_market(req.market_info, req.crank_duration)?;

    Ok(())
}

pub async fn active_cranks(
    state: Arc<Mutex<State>>,
) -> Result<Json<Vec<Market>>, AppError> {
    let state = state.lock().await;
    let markets = state
        .get_cranks()
        .iter()
        .map(|(_, crank)| crank.into())
        .collect::<Vec<Market>>();

    Ok(Json(markets))
}

pub async fn logs(
    _state: Arc<Mutex<State>>,
    _market_id: String,
) -> Result<(), AppError> {
    unimplemented!()
}

pub async fn purge(
    state: Arc<Mutex<State>>,
    market_id: String,
) -> Result<Json<Market>, AppError> {
    let mut state = state.lock().await;
    let market = state
        .purge_market(&market_id)?
        .map(Json)
        .ok_or_else(|| anyhow!("no market with id {market_id}"))?;

    Ok(market)
}
