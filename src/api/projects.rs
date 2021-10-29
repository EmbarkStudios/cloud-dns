use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub kind: String, // "dns#project"
    pub number: u64,
    pub id: String,
    pub quota: Quota,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quota {
    pub kind: String, // "dns#quota"
    pub managed_zones: i64,
    pub rrsets_per_managed_zone: i64,
    pub rrset_additions_per_change: i64,
    pub rrset_deletions_per_change: i64,
    pub total_rrdata_size_per_change: i64,
    pub resource_records_per_rrset: i64,
    pub dns_keys_per_managed_zone: i64,
    pub whitelisted_key_specs: Vec<WhitelistedKeySpec>,
    pub networks_per_managed_zone: i64,
    pub managed_zones_per_network: i64,
    pub policies: i64,
    pub networks_per_policy: i64,
    pub target_name_servers_per_policy: i64,
    pub target_name_servers_per_managed_zone: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhitelistedKeySpec {
    pub kind: String, // "dns#dnsKeySpec"
    pub key_type: String,
    pub algorithm: String,
    pub key_length: u64,
}

pub struct ProjectsHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> ProjectsHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Project> {
        self.client.get("/").await
    }
}
