use crate::{DnsClient, Result};

use super::managed_zones::ManagedZone;
use super::{dns_keys::DnsKey, ListEnvelope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZoneOperation {
    pub kind: String, // "dns#operation"
    pub id: String,
    pub start_time: String,
    pub status: String,
    pub user: String,
    pub r#type: String,
    pub zone_context: ZoneContext,
    pub dns_key_context: DnsKeyContext,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZoneContext {
    pub old_value: ManagedZone,
    pub new_value: ManagedZone,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DnsKeyContext {
    pub old_value: DnsKey,
    pub new_value: DnsKey,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZoneOperations {
    #[serde(flatten)]
    pub envelope: ListEnvelope,
    pub operations: Vec<ManagedZoneOperation>,
}

pub struct ManagedZoneOperationsHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> ManagedZoneOperationsHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: &str) -> Result<ManagedZoneOperations> {
        let route = format!(
            "managedZones/{managed_zone}/operations",
            managed_zone = managed_zone,
        );

        self.client.get(route).await
    }

    pub async fn get(
        &self,
        managed_zone: &str,
        operation_id: &str,
    ) -> Result<ManagedZoneOperation> {
        let route = format!(
            "managedZones/{managed_zone}/operations/{operation_id}",
            managed_zone = managed_zone,
            operation_id = operation_id,
        );

        self.client.get(route).await
    }
}
