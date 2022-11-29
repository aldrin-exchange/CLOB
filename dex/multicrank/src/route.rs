//! Routing of requests to handlers.

use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use tokio::sync::Mutex;
use tracing::instrument;

use crate::{error::AppError, handler, model::CrankRequest, state::State};

/// Start a crank with the provided market info and duration.
#[instrument]
pub async fn start_crank(
    Extension(state): Extension<Arc<Mutex<State>>>,
    req: Json<CrankRequest>,
) -> Result<impl IntoResponse, AppError> {
    handler::start_crank(req.0, state).await
}

/// Fetches a list of active markets in vector form.
#[instrument]
pub async fn active_cranks(
    Extension(state): Extension<Arc<Mutex<State>>>,
) -> Result<impl IntoResponse, AppError> {
    handler::active_cranks(state).await
}

/// TODO: impl some kind of file streaming (tail -n equivalent)
#[instrument]
pub async fn logs(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(market_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    handler::logs(state, market_id).await
}

/// Halt the crank for the provided market_id and remove it from the list.
#[instrument]
pub async fn purge(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(market_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    handler::purge(state, market_id).await
}
