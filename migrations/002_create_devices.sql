CREATE TABLE IF NOT EXISTS devices (
    device_id TEXT PRIMARY KEY,
    state TEXT NOT NULL,
    allowed_topics TEXT[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ
);

INSERT INTO devices (device_id, state, allowed_topics)
VALUES (
    'device-1',
    'Trusted',
    ARRAY[
        'devices/device-1/',
        'telemetry/device-1/',
        'mesh/provisioning/',
        'edge/provisioning/'
    ]
)
ON CONFLICT (device_id) DO NOTHING;