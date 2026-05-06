mod detection;
mod models;

use chrono::Utc;
use detection::engine::run_detection;
use models::device::{DevicePolicy, DeviceState};
use models::event::{EventType, SecurityEvent, Severity};
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
            "device/device-1/".to_string(),
            "telemetry/device-1/".to_string(),
        ],
        state: DeviceState::Trusted,
    };

    let alerts = run_detection(&event, &policy);

    for alert in alerts {
        println!("ALERT:");
        println!("{:#?}", alert);
    }
}
