use axum::Json;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct AlertResponse {
    pub device_id: String,
    pub rule_name: String,
    pub severity: String,
    pub reason: String,
}

pub async fn get_alerts() -> Json<Vec<AlertResponse>> {
    Json(vec![AlertResponse {
        device_id: "device-1".to_string(),
        rule_name: "unauthorized_topic_publish".to_string(),
        severity: "High".to_string(),
        reason: "demo alert".to_string(),
    }])
}
