use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::alert::Alert;

pub async fn save_alert(pool: &PgPool, alert: &Alert) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO alerts (
            alert_id,
            event_id,
            device_id,
            rule_name,
            severity,
            reason,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(alert.alert_id)
    .bind(alert.event_id)
    .bind(&alert.device_id)
    .bind(&alert.rule_name)
    .bind(format!("{:?}", alert.severity))
    .bind(&alert.reason)
    .bind(alert.created_at)
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AlertRow {
    pub alert_id: Uuid,
    pub event_id: Uuid,
    pub device_id: String,
    pub rule_name: String,
    pub severity: String,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

pub async fn get_alerts(pool: &PgPool) -> anyhow::Result<Vec<AlertRow>> {
    let alerts = sqlx::query_as::<_, AlertRow>(
        r#"
        SELECT
        alert_id,
        event_id,
        device_id,
        rule_name,
        severity,
        reason,
        created_at
        FROM alerts
        ORDER BY created_at DESC
        LIMIT 50
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(alerts)
}
