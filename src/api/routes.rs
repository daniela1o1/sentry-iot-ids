use axum::{Router, routing::get};
use sqlx::PgPool;

use super::handlers::{get_alerts, get_devices};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/alerts", get(get_alerts))
        .route("/devices", get(get_devices))
        .with_state(pool)
}
