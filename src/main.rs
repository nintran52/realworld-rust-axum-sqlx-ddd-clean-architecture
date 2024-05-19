use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;

mod app;
mod constants;
mod utils;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    println!("🌟 REST API Service 🌟");

    let state = {
        let pool = utils::db::establish_connection().await;
        use app::drivers::middlewares::state::AppState;
        AppState::new(pool.clone())
    };
    let app = app::drivers::routers::create_router(Arc::new(state.clone()))
        .layer(app::drivers::middlewares::cors::cors());

    println!("✅ Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind(constants::BIND).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
