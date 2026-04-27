#[derive(Debug)]
pub struct DevicePolicy {
    pub device_id: String,
    pub allowed_topics: Vec<String>,
}
