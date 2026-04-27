use crate::models::{
    event::{EventType, SecurityEvent, Severity},
    policy::DevicePolicy,
};

pub fn detect_unauthorized_publish(
    event: &SecurityEvent,
    policy: &DevicePolicy,
) -> Option<SecurityEvent> {
    let is_allowed = policy
        .allowed_topics
        .iter()
        .any(|allowed_prefix| event.topic.starts_with(allowed_prefix));

    if is_allowed {
        return None;
    }

    Some(SecurityEvent {
        event_id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        device_id: event.device_id.clone(),
        topic: event.topic.clone(),
        event_type: EventType::UnauthorizedPublish,
        severity: Severity::High,
        source: "sentry-rule-engine".to_string(),
    })
}
