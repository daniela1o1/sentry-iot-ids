#[derive(Debug, Clone)]
pub enum DeviceState {
    Unknown,
    Trusted,
    Suspicious,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct DevicePolicy {
    pub device_id: String,
    pub allowed_topics: Vec<String>,
    pub state: DeviceState,
}
