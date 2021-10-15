use serde::{Deserialize, Serialize};

use crate::{DNSClient, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DNSKey {
    kind: String, // "dns#dnsKey"
    id: String,
    algorithm: Algorithm,
    key_length: u64,
    public_key: String,
    creation_time: String,
    is_active: bool,
    r#type: KeyType,
    key_tag: i32,
    digests: Vec<Digest>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum KeyType {
    KeySigning,
    ZoneSigning
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Algorithm {
    Ecdsap256sha256,
    Ecdsap384sha384,
    Rsasha1,
    Rsasha256,
    Rsasha512
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Digest {
    r#type: DigestType,
    digest: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum DigestType {
    Sha1,
    Sha256,
    Sha384,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DNSKeysListResponse {
    kind: String, // "dns#dnsKeysListResponse"
    header: Option<Header>,
    dns_keys: Vec<DNSKey>,
    next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Header {
    operation_id: String,
}

pub struct DNSKeysHandler<'client> {
    client: &'client DNSClient,
}

impl<'client> DNSKeysHandler<'client> {
    pub(crate) fn new(client: &'client DNSClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: String) -> Result<DNSKeysListResponse> {
        let route = format!(
            "managedZones/{managed_zone}/dnsKeys",
            managed_zone = managed_zone,
        );

        self.client.get(route, None::<&()>).await
    }

    pub async fn get(&self, managed_zone: String, dns_key_id: String) -> Result<DNSKey> {
        let route = format!(
            "managedZones/{managed_zone}/dnsKeys/{dns_key_id}",
            managed_zone = managed_zone,
            dns_key_id = dns_key_id,
        );

        self.client.get(route, None::<&()>).await
    }
}
