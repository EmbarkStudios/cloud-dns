use serde::{Deserialize, Serialize};

use crate::{DNSClient, Result};

use super::ListEnvelope;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRecordSet {
    kind: String, // "dns#resourceRecordSet"
    name: String,
    r#type: String,
    ttl: i32,
    rrdatas: Vec<String>,
    signature_rrdatas: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRecordSets {
    #[serde(flatten)]
    envelope: ListEnvelope,
    rrsets: Vec<ResourceRecordSet>,
}

pub struct ResourceRecordSetsHandler<'client> {
    client: &'client DNSClient,
}

impl<'client> ResourceRecordSetsHandler<'client> {
    pub(crate) fn new(client: &'client DNSClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: String) -> Result<ResourceRecordSets> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets",
            managed_zone = managed_zone,
        );

        self.client.get(route, None::<&()>).await
    }

    pub async fn get(
        &self,
        managed_zone: String,
        name: String,
        r#type: String,
    ) -> Result<ResourceRecordSet> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets/{name}/{typ}",
            managed_zone = managed_zone,
            name = name,
            typ = r#type,
        );

        self.client.get(route, None::<&()>).await
    }

    pub async fn patch(
        &self,
        managed_zone: String,
        name: String,
        r#type: String,
        record_set: ResourceRecordSet,
    ) -> Result<ResourceRecordSet> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets/{name}/{typ}",
            managed_zone = managed_zone,
            name = name,
            typ = r#type,
        );

        self.client.patch(route, Some(&record_set)).await
    }

    pub async fn create(
        &self,
        managed_zone: String,
        record_set: ResourceRecordSet,
    ) -> Result<ResourceRecordSet> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets",
            managed_zone = managed_zone,
        );

        self.client.post(route, Some(&record_set)).await
    }

    pub async fn delete(&self, managed_zone: String, name: String, r#type: String) -> Result<()> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets/{name}/{typ}",
            managed_zone = managed_zone,
            name = name,
            typ = r#type,
        );

        self.client.delete(route, None::<&()>).await
    }
}
