use sqlx::PgPool;

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
