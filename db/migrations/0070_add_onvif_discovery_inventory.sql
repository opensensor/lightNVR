-- Durable ONVIF discovery staging inventory. Credentials are deliberately absent.

-- migrate:up
CREATE TABLE onvif_discovery_inventory (
    uuid TEXT PRIMARY KEY,
    endpoint TEXT NOT NULL,
    device_service TEXT NOT NULL,
    media_service TEXT NOT NULL DEFAULT '',
    ptz_service TEXT NOT NULL DEFAULT '',
    imaging_service TEXT NOT NULL DEFAULT '',
    manufacturer TEXT NOT NULL DEFAULT '',
    model TEXT NOT NULL DEFAULT '',
    firmware_version TEXT NOT NULL DEFAULT '',
    serial_number TEXT NOT NULL DEFAULT '',
    hardware_id TEXT NOT NULL DEFAULT '',
    ip_address TEXT NOT NULL DEFAULT '',
    mac_address TEXT NOT NULL DEFAULT '' COLLATE NOCASE,
    first_seen_at INTEGER NOT NULL,
    last_seen_at INTEGER NOT NULL,
    last_scan_network TEXT NOT NULL DEFAULT 'auto',
    online INTEGER NOT NULL DEFAULT 1 CHECK (online IN (0, 1)),
    claim_state TEXT NOT NULL DEFAULT 'unclaimed'
        CHECK (claim_state IN ('unclaimed', 'claimed', 'ignored')),
    claimed_camera_uuid TEXT REFERENCES streams(camera_uuid) ON DELETE SET NULL,
    duplicate_suspected INTEGER NOT NULL DEFAULT 0
        CHECK (duplicate_suspected IN (0, 1)),
    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision >= 1),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX idx_onvif_inventory_seen
ON onvif_discovery_inventory(last_seen_at DESC, uuid);

CREATE INDEX idx_onvif_inventory_serial
ON onvif_discovery_inventory(serial_number)
WHERE serial_number != '';

CREATE INDEX idx_onvif_inventory_mac
ON onvif_discovery_inventory(mac_address)
WHERE mac_address != '';

CREATE INDEX idx_onvif_inventory_endpoint
ON onvif_discovery_inventory(endpoint);

CREATE INDEX idx_onvif_inventory_claim
ON onvif_discovery_inventory(claim_state, online, last_seen_at DESC);

CREATE TABLE onvif_discovery_addresses (
    inventory_uuid TEXT NOT NULL
        REFERENCES onvif_discovery_inventory(uuid) ON DELETE CASCADE,
    address_type TEXT NOT NULL CHECK (address_type IN ('endpoint', 'service', 'ip')),
    address TEXT NOT NULL,
    first_seen_at INTEGER NOT NULL,
    last_seen_at INTEGER NOT NULL,
    PRIMARY KEY (inventory_uuid, address_type, address)
);

CREATE INDEX idx_onvif_discovery_addresses_address
ON onvif_discovery_addresses(address, inventory_uuid);

CREATE TRIGGER trg_onvif_inventory_unclaim_deleted_stream
BEFORE DELETE ON streams
FOR EACH ROW BEGIN
    UPDATE onvif_discovery_inventory
    SET claim_state = 'unclaimed', claimed_camera_uuid = NULL,
        revision = revision + 1, updated_at = strftime('%s', 'now')
    WHERE claimed_camera_uuid = OLD.camera_uuid;
END;

-- migrate:down
DROP TRIGGER IF EXISTS trg_onvif_inventory_unclaim_deleted_stream;
DROP INDEX IF EXISTS idx_onvif_discovery_addresses_address;
DROP TABLE IF EXISTS onvif_discovery_addresses;
DROP INDEX IF EXISTS idx_onvif_inventory_claim;
DROP INDEX IF EXISTS idx_onvif_inventory_endpoint;
DROP INDEX IF EXISTS idx_onvif_inventory_mac;
DROP INDEX IF EXISTS idx_onvif_inventory_serial;
DROP INDEX IF EXISTS idx_onvif_inventory_seen;
DROP TABLE IF EXISTS onvif_discovery_inventory;
