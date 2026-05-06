use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub topic: String,
    pub event_type: EventType,
    pub severity: Severity,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    MqttPublish,
    UnauthorizedPublish,
    AuthFailed,
    ReplaySuspected,
    RogueProvisioning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}
