use serde::{Deserialize, Serialize};

use crate::{DNSClient, Result};

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
pub struct ResourceRecordSetListResponse {
    kind: String, // "dns#resourceRecordSetsListResponse"
    header: Option<Header>,
    rrsets: Vec<ResourceRecordSet>,
    next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Header {
    operation_id: String,
}

pub struct ResourceRecordSetsHandler<'client> {
    client: &'client DNSClient,
}

impl<'client> ResourceRecordSetsHandler<'client> {
    pub(crate) fn new(client: &'client DNSClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: String) -> Result<ResourceRecordSetListResponse> {
        let route = format!(
            "managedZones/{}/rrsets",
            managed_zone,
        );

        self.client.get(route, None::<&()>).await
    }
}
