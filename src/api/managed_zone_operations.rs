use super::dns_keys::DNSKey;
use super::managed_zones::ManagedZone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct ManagedZoneOperation {
    kind: String, // "dns#operation"
    id: String,
    start_time: String,
    status: String,
    user: String,
    r#type: String,
    zone_context: ZoneContext,
    dns_key_context: DNSKeyContext,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct ZoneContext {
    old_value: ManagedZone,
    new_value: ManagedZone,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct DNSKeyContext {
    old_value: DNSKey,
    new_value: DNSKey,
}
