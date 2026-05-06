use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::event::Severity;

#[derive(Debug, Clone)]
pub struct Alert {
    pub alert_id: Uuid,
    pub event_id: Uuid,
    pub device_id: String,
    pub rule_name: String,
    pub severity: Severity,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}
