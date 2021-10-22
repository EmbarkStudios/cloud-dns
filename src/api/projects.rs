use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    kind: String, // "dns#project"
    number: u64,
    id: String,
    quota: Quota,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Quota {
    kind: String, // "dns#quota"
    managed_zones: i64,
    rrsets_per_managed_zone: i64,
    rrset_additions_per_change: i64,
    rrset_deletions_per_change: i64,
    total_rrdata_size_per_change: i64,
    resource_records_per_rrset: i64,
    dns_keys_per_managed_zone: i64,
    whitelisted_key_specs: Vec<WhitelistedKeySpec>,
    networks_per_managed_zone: i64,
    managed_zones_per_network: i64,
    policies: i64,
    networks_per_policy: i64,
    target_name_servers_per_policy: i64,
    target_name_servers_per_managed_zone: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WhitelistedKeySpec {
    kind: String, // "dns#dnsKeySpec"
    key_type: String,
    algorithm: String,
    key_length: u64,
}

pub struct ProjectsHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> ProjectsHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Project> {
        self.client.get("/", None::<&()>).await
    }
}
