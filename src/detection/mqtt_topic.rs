use crate::models::alert::Alert;
use crate::models::device::DevicePolicy;
use crate::models::event::{SecurityEvent, Severity};

pub fn detect_unauthorized_publish(event: &SecurityEvent, policy: &DevicePolicy) -> Option<Alert> {
    let is_allowed = policy
        .allowed_topics
        .iter()
        .any(|allowed| event.topic.starts_with(allowed));

    if is_allowed {
        return None;
    }

    Some(Alert {
        alert_id: uuid::Uuid::new_v4(),
        event_id: event.event_id,
        device_id: event.device_id.clone(),
        rule_name: "unauthorized_topic_publish".to_string(),
        severity: Severity::High,
        reason: format!(
            "Device '{}' published to unauthorized topic '{}'",
            event.device_id, event.topic
        ),
        created_at: chrono::Utc::now(),
    })
}
