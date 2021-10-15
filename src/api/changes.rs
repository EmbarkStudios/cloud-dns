use serde::{Deserialize, Serialize};

use crate::{DNSClient, Result};

use super::resource_record_sets::ResourceRecordSet;

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
    Pending
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangesListResponse {
    kind: String, // "dns#changesListResponse"
    header: Option<Header>,
    changes: Vec<Change>,
    next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Header {
    operation_id: String,
}

pub struct ChangesHandler<'client> {
    client: &'client DNSClient,
}

impl<'client> ChangesHandler<'client> {
    pub(crate) fn new(client: &'client DNSClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: String) -> Result<ChangesListResponse> {
        let route = format!(
            "managedZones/{}/changes",
            managed_zone,
        );

        self.client.get(route, None::<&()>).await
    }

    pub async fn get(&self, managed_zone: String, change_id: String) -> Result<Change> {
        let route = format!(
            "managedZones/{}/changes/{}",
            managed_zone,
            change_id,
        );

        self.client.get(route, None::<&()>).await
    }

    pub async fn create(&self, managed_zone: String, change: Change) -> Result<Change> {
        let route = format!(
            "managedZones/{}/changes",
            managed_zone,
        );

        self.client.post(route, Some(&change)).await
    }
}
