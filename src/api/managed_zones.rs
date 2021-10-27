use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

use super::{managed_zone_operations::ManagedZoneOperation, ListEnvelope};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ManagedZone {
    kind: String, // "dns#managedZone"
    name: String,
    dns_name: String,
    description: String,
    id: u64,
    name_servers: Vec<String>,
    creation_time: String,
    dnssec_config: DnsSecConfig,
    name_server_set: String,
    visibility: String,
    private_visibility_config: PrivateVisibilityConfig,
    forwarding_config: ForwardingConfig,
    labels: HashMap<String, serde_json::Value>,
    peering_config: PeeringConfig,
    reverse_lookup_config: ReverseLookupConfig,
    service_directory_config: ServiceDirectoryConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct ServiceDirectoryConfig {
    kind: String, // "dns#managedZoneServiceDirectoryConfig"
    namespace: ServiceDirectoryConfigNamespace,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct ServiceDirectoryConfigNamespace {
    kind: String, // "dns#managedZoneServiceDirectoryConfigNamespace"
    namespace_url: String,
    deletion_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReverseLookupConfig {
    kind: String, // "dns#managedZoneReverseLookupConfig"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PeeringConfig {
    kind: String, // "dns#managedZonePeeringConfig"
    target_network: PeeringConfigTargetNetwork,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PeeringConfigTargetNetwork {
    kind: String, // "dns#managedZonePeeringConfigTargetNetwork"
    network_url: String,
    deactivate_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PrivateVisibilityConfig {
    kind: String, // "dns#managedZonePrivateVisibilityConfig"
    networks: Vec<PrivateVisibilityConfigNetwork>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ForwardingConfig {
    kind: String, // "dns#managedZoneForwardingConfig"
    target_name_servers: Vec<ForwardingConfigNameServerTarget>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ForwardingConfigNameServerTarget {
    kind: String, // "dns#managedZoneForwardingConfigNameServerTarget"
    ipv4_address: String,
    forwarding_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PrivateVisibilityConfigNetwork {
    kind: String, // "dns#managedZonePrivateVisibilityConfigNetwork"
    network_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DnsSecConfig {
    kind: String, // "dns#managedZoneDnsSecConfig"
    state: String,
    default_key_specs: Vec<DefaultKeySpec>,
    non_existence: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DefaultKeySpec {
    kind: String, // "dns#dnsKeySpec"
    key_type: String,
    algorithm: String,
    key_length: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZones {
    #[serde(flatten)]
    envelope: ListEnvelope,
    managed_zones: Vec<ManagedZone>,
}

pub struct ManagedZonesHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> ManagedZonesHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<ManagedZones> {
        let route = "managedZones".to_string();

        self.client.get(route).await
    }

    pub async fn get(&self, managed_zone: String) -> Result<ManagedZone> {
        let route = format!("managedZones/{managed_zone}", managed_zone = managed_zone,);

        self.client.get(route).await
    }

    pub async fn patch(
        &self,
        managed_zone_id: &str,
        managed_zone: ManagedZone,
    ) -> Result<ManagedZoneOperation> {
        let route = format!(
            "managedZones/{managed_zone_id}",
            managed_zone_id = managed_zone_id,
        );
        self.client.patch(route, Some(&managed_zone)).await
    }

    pub async fn create(&self, managed_zone: ManagedZone) -> Result<ManagedZone> {
        let route = "managedZones".to_string();

        self.client.post(route, Some(&managed_zone)).await
    }

    pub async fn delete(&self, managed_zone: &str) -> Result<()> {
        let route = format!("managedZones/{managed_zone}", managed_zone = managed_zone,);

        self.client.delete(route).await
    }

    pub async fn update(
        &self,
        managed_zone_id: &str,
        managed_zone: ManagedZone,
    ) -> Result<ManagedZoneOperation> {
        let route = format!(
            "managedZones/{managed_zone_id}",
            managed_zone_id = managed_zone_id,
        );

        self.client.put(route, Some(&managed_zone)).await
    }
}
