use axum::{Router, routing::get};

use super::handlers::get_alerts;

pub fn create_router() -> Router {
    Router::new().route("/alerts", get(get_alerts))
}
