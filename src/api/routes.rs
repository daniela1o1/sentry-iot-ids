use axum::{Router, routing::get};
use sqlx::PgPool;

use super::handlers::get_alerts;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/alerts", get(get_alerts))
        .with_state(pool)
}
