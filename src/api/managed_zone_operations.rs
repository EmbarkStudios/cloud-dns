use crate::{DNSClient, Result};

use super::managed_zones::ManagedZone;
use super::{dns_keys::DNSKey, ListEnvelope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ManagedZoneOperation {
    kind: String, // "dns#operation"
    id: String,
    start_time: String,
    status: String,
    user: String,
    r#type: String,
    zone_context: ZoneContext,
    dns_key_context: DNSKeyContext,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct ZoneContext {
    old_value: ManagedZone,
    new_value: ManagedZone,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct DNSKeyContext {
    old_value: DNSKey,
    new_value: DNSKey,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZoneOperations {
    #[serde(flatten)]
    envelope: ListEnvelope,
    operations: Vec<ManagedZoneOperation>,
}

pub struct ManagedZoneOperationsHandler<'client> {
    client: &'client DNSClient,
}

impl<'client> ManagedZoneOperationsHandler<'client> {
    pub(crate) fn new(client: &'client DNSClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: String) -> Result<ManagedZoneOperations> {
        let route = format!(
            "managedZones/{managed_zone}/operations",
            managed_zone = managed_zone,
        );

        self.client.get(route, None::<&()>).await
    }

    pub async fn get(
        &self,
        managed_zone: String,
        operation_id: String,
    ) -> Result<ManagedZoneOperation> {
        let route = format!(
            "managedZones/{managed_zone}/operations/{operation_id}",
            managed_zone = managed_zone,
            operation_id = operation_id,
        );

        self.client.get(route, None::<&()>).await
    }
}
