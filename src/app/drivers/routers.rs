use super::middlewares::state::AppState;
use crate::app;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/health-check",
            get(app::module::health_check::controllers::index),
        )
        .with_state(app_state)
}
