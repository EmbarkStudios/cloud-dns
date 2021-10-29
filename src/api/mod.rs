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
pub struct ListEnvelope {
    pub kind: String,
    pub header: Option<Header>,
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEnvelope {
    pub header: Option<Header>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub operation_id: String,
}
