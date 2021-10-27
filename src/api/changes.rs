use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

use super::{resource_record_sets::ResourceRecordSet, ListEnvelope};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    kind: String, // "dns#change"
    additions: Vec<ResourceRecordSet>,
    deletions: Vec<ResourceRecordSet>,
    start_time: String,
    id: String,
    status: Status,
    is_serving: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Status {
    Done,
    Pending,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Changes {
    #[serde(flatten)]
    envelope: ListEnvelope,
    changes: Vec<Change>,
}

pub struct ChangesHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> ChangesHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: &str) -> Result<Changes> {
        let route = format!("managedZones/{}/changes", managed_zone);

        self.client.get(route).await
    }

    pub async fn get(&self, managed_zone: &str, change_id: &str) -> Result<Change> {
        let route = format!("managedZones/{}/changes/{}", managed_zone, change_id);

        self.client.get(route).await
    }

    pub async fn create(&self, managed_zone: &str, change: Change) -> Result<Change> {
        let route = format!("managedZones/{}/changes", managed_zone);

        self.client.post(route, Some(&change)).await
    }
}
