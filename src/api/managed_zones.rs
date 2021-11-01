use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

use super::{managed_zone_operations::ManagedZoneOperation, ListEnvelope};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ManagedZone {
    pub kind: String, // "dns#managedZone"
    pub name: String,
    pub dns_name: String,
    pub description: String,
    pub id: u64,
    pub name_servers: Vec<String>,
    pub creation_time: String,
    pub dnssec_config: DnsSecConfig,
    pub name_server_set: String,
    pub visibility: String,
    pub private_visibility_config: PrivateVisibilityConfig,
    pub forwarding_config: ForwardingConfig,
    pub labels: HashMap<String, serde_json::Value>,
    pub peering_config: PeeringConfig,
    pub reverse_lookup_config: ReverseLookupConfig,
    pub service_directory_config: ServiceDirectoryConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ServiceDirectoryConfig {
    pub kind: String, // "dns#managedZoneServiceDirectoryConfig"
    pub namespace: ServiceDirectoryConfigNamespace,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ServiceDirectoryConfigNamespace {
    pub kind: String, // "dns#managedZoneServiceDirectoryConfigNamespace"
    pub namespace_url: String,
    pub deletion_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReverseLookupConfig {
    pub kind: String, // "dns#managedZoneReverseLookupConfig"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PeeringConfig {
    pub kind: String, // "dns#managedZonePeeringConfig"
    pub target_network: PeeringConfigTargetNetwork,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PeeringConfigTargetNetwork {
    pub kind: String, // "dns#managedZonePeeringConfigTargetNetwork"
    pub network_url: String,
    pub deactivate_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrivateVisibilityConfig {
    pub kind: String, // "dns#managedZonePrivateVisibilityConfig"
    pub networks: Vec<PrivateVisibilityConfigNetwork>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForwardingConfig {
    pub kind: String, // "dns#managedZoneForwardingConfig"
    pub target_name_servers: Vec<ForwardingConfigNameServerTarget>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForwardingConfigNameServerTarget {
    pub kind: String, // "dns#managedZoneForwardingConfigNameServerTarget"
    pub ipv4_address: String,
    pub forwarding_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrivateVisibilityConfigNetwork {
    pub kind: String, // "dns#managedZonePrivateVisibilityConfigNetwork"
    pub network_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DnsSecConfig {
    pub kind: String, // "dns#managedZoneDnsSecConfig"
    pub state: String,
    pub default_key_specs: Vec<DefaultKeySpec>,
    pub non_existence: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefaultKeySpec {
    pub kind: String, // "dns#dnsKeySpec"
    pub key_type: String,
    pub algorithm: String,
    pub key_length: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZones {
    #[serde(flatten)]
    pub envelope: ListEnvelope,
    pub managed_zones: Vec<ManagedZone>,
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
