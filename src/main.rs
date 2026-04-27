mod models;
mod rules;

use chrono::Utc;
use models::event::{EventType, SecurityEvent, Severity};
use models::policy::DevicePolicy;
use uuid::Uuid;

fn main() {
    let event = SecurityEvent {
        event_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        device_id: "device-1".to_string(),
        topic: "admin/root/access".to_string(),
        event_type: EventType::UnauthorizedPublish,
        severity: Severity::High,
        source: "mqtt-broker".to_string(),
    };

    let policy = DevicePolicy {
        device_id: "device-1".to_string(),
        allowed_topics: vec![
            "devices/device-1/".to_string(),
            "telemetry/device-1/".to_string(),
        ],
    };

    if let Some(detection) = rules::mqtt_topic::detect_unauthorized_publish(&event, &policy) {
        println!("DETECTION:");
        println!("{:#?}", detection);
    }
}
