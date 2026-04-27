#[derive(Debug)]
pub enum DeviceState {
    Unknown,
    Provisioning,
    Trusted,
    Suspect,
    Quarantined,
    Revoked,
}
