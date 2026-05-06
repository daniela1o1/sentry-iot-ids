use chrono::Utc;
use uuid::Uuid;

use crate::database::alerts::save_alert;
use crate::detection::engine::run_detection;
use crate::models::device::{DevicePolicy, DeviceState};
use crate::models::event::{EventType, SecurityEvent, Severity};
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use sqlx::PgPool;
use std::time::Duration;

pub async fn start_mqtt_ingest(pool: PgPool) -> anyhow::Result<()> {
    let mut mqtt_options = MqttOptions::new("sentry-ids", "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    client.subscribe("#", QoS::AtLeastOnce).await?; //QoS 1, just for not missing out on something

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

            let device_id = extract_device_id(&topic);

            let security_event = SecurityEvent {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                device_id: device_id.clone(),
                topic: topic.clone(),
                event_type: EventType::MqttPublish,
                severity: Severity::Low,
                source: "mqtt-broker".to_string(),
            };

            let policy = DevicePolicy {
                device_id: device_id.clone(),
                allowed_topics: vec![
                    format!("devices/{}/", device_id),
                    format!("telemetry/{}/", device_id),
                    "mesh/provisioning/".to_string(),
                    "edge/provisioning/".to_string(),
                ],
                state: DeviceState::Trusted,
            };

            let alerts = run_detection(&security_event, &policy);

            for alert in alerts {
                println!("ALERT:");
                println!("{:#?}", alert);

                save_alert(&pool, &alert).await?;
            }
        }
    }
}

fn sanitize_payload(topic: &str, payload: &str) -> String {
    let topic_lower = topic.to_lowercase(); // normalizing input for stable detection logic

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

fn extract_device_id(topic: &str) -> String {
    let parts: Vec<&str> = topic.split('/').collect();

    if parts.len() >= 2 && (parts[0] == "devices" || parts[0] == "telemetry") {
        return parts[1].to_string();
    }

    "unknown".to_string()
}
