use serde::{Deserialize, Serialize};

pub mod changes;
pub mod dns_keys;
pub mod managed_zone_operations;
pub mod managed_zones;
pub mod policies;
pub mod projects;
pub mod resource_record_sets;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ListEnvelope {
    kind: String,
    header: Option<Header>,
    next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateEnvelope {
    header: Option<Header>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Header {
    operation_id: String,
}
