use chrono::Utc;
use uuid::Uuid;

use crate::detection::engine::run_detection;
use crate::models::device::{DevicePolicy, DeviceState};
use crate::models::event::{EventType, SecurityEvent, Severity};
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;

pub async fn start_mqtt_ingest() -> anyhow::Result<()> {
    let mut mqtt_options = MqttOptions::new("sentry-ids", "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    client.subscribe("#", QoS::AtMostOnce).await?;

    println!("Sentry MQTT ingest started. Listening on topics...");

    loop {
        let event = eventloop.poll().await?;

        if let Event::Incoming(Incoming::Publish(packet)) = event {
            let topic = packet.topic;
            let payload = String::from_utf8_lossy(&packet.payload);

            let safe_payload = sanitize_payload(&topic, &payload);

            println!("MQTT EVENT:");
            println!("topic: {}", topic);
            println!("payload: {}", safe_payload);

            let security_event = SecurityEvent {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                device_id: "device-1".to_string(),
                topic: topic.clone(),
                event_type: EventType::MqttPublish,
                severity: Severity::Low,
                source: "mqtt-broker".to_string(),
            };

            let policy = DevicePolicy {
                device_id: "device-1".to_string(),
                allowed_topics: vec![
                    "devices/device-1/".to_string(),
                    "telemetry/device-1/".to_string(),
                    "mesh/provisioning/".to_string(),
                    "edge/provisioning/".to_string(),
                ],
                state: DeviceState::Trusted,
            };

            let alerts = run_detection(&security_event, &policy);

            for alert in alerts {
                println!("ALERT:");
                println!("{:#?}", alert);
            }
        }
    }
}

fn sanitize_payload(topic: &str, payload: &str) -> String {
    let topic_lower = topic.to_lowercase();

    if topic_lower.contains("hmac") {
        "[sensitive:hmac omitted]".to_string()
    } else if topic_lower.contains("ca") || payload.contains("BEGIN CERTIFICATE") {
        "[certificate omitted]".to_string()
    } else if payload.len() > 120 {
        format!("[payload omitted: {} bytes]", payload.len())
    } else {
        payload.to_string()
    }
}
