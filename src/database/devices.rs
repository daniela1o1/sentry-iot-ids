use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DeviceRow {
    pub device_id: String,
    pub state: String,
    pub allowed_topics: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

pub async fn get_devices(pool: &PgPool) -> anyhow::Result<Vec<DeviceRow>> {
    let devices = sqlx::query_as::<_, DeviceRow>(
        r#"
        SELECT
            device_id,
            state,
            allowed_topics,
            created_at,
            last_seen
        FROM devices
        ORDER BY device_id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(devices)
}
