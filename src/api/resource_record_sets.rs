use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

use super::ListEnvelope;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRecordSet {
    pub kind: String, // "dns#resourceRecordSet"
    pub name: String,
    pub r#type: String,
    pub ttl: i32,
    pub rrdatas: Vec<String>,
    pub signature_rrdatas: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRecordSets {
    #[serde(flatten)]
    pub envelope: ListEnvelope,
    pub rrsets: Vec<ResourceRecordSet>,
}

pub struct ResourceRecordSetsHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> ResourceRecordSetsHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: &str) -> Result<ResourceRecordSets> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets",
            managed_zone = managed_zone,
        );

        self.client.get(route).await
    }

    pub async fn get(
        &self,
        managed_zone: &str,
        name: &str,
        r#type: &str,
    ) -> Result<ResourceRecordSet> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets/{name}/{typ}",
            managed_zone = managed_zone,
            name = name,
            typ = r#type,
        );

        self.client.get(route).await
    }

    pub async fn patch(
        &self,
        managed_zone: &str,
        name: &str,
        r#type: &str,
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
        managed_zone: &str,
        record_set: ResourceRecordSet,
    ) -> Result<ResourceRecordSet> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets",
            managed_zone = managed_zone,
        );

        self.client.post(route, Some(&record_set)).await
    }

    pub async fn delete(&self, managed_zone: &str, name: &str, r#type: &str) -> Result<()> {
        let route = format!(
            "managedZones/{managed_zone}/rrsets/{name}/{typ}",
            managed_zone = managed_zone,
            name = name,
            typ = r#type,
        );

        self.client.delete(route).await
    }
}
