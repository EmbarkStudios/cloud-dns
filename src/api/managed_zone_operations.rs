use crate::{DnsClient, Result};

use super::managed_zones::ManagedZone;
use super::{dns_keys::DnsKey, ListEnvelope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZoneOperation {
    kind: String, // "dns#operation"
    id: String,
    start_time: String,
    status: String,
    user: String,
    r#type: String,
    zone_context: ZoneContext,
    dns_key_context: DnsKeyContext,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ZoneContext {
    old_value: ManagedZone,
    new_value: ManagedZone,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DnsKeyContext {
    old_value: DnsKey,
    new_value: DnsKey,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZoneOperations {
    #[serde(flatten)]
    envelope: ListEnvelope,
    operations: Vec<ManagedZoneOperation>,
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

        self.client.get(route, None::<&()>).await
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

        self.client.get(route, None::<&()>).await
    }
}
