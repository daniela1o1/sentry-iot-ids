use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::database::alerts::{AlertRow, get_alerts as db_get_alerts};

pub async fn get_alerts(State(pool): State<PgPool>) -> Result<Json<Vec<AlertRow>>, StatusCode> {
    let alerts = db_get_alerts(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(alerts))
}
