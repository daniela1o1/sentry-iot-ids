use crate::models::alert::Alert;
use crate::models::device::DevicePolicy;
use crate::models::event::SecurityEvent;

use super::mqtt_topic::detect_unauthorized_publish;

pub fn run_detection(event: &SecurityEvent, policy: &DevicePolicy) -> Vec<Alert> {
    let mut alerts = Vec::new();

    if let Some(alert) = detect_unauthorized_publish(event, policy) {
        alerts.push(alert);
    }

    alerts
}
