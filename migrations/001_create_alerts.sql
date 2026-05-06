CREATE TABLE IF NOT EXISTS alerts (
    alert_id UUID PRIMARY KEY,
    event_id UUID NOT NULL,
    device_id TEXT NOT NULL,
    rule_name TEXT NOT NULL,
    severity TEXT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
