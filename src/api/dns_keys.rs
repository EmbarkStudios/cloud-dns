use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

use super::ListEnvelope;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DnsKey {
    pub kind: String, // "dns#dnsKey"
    pub id: String,
    pub algorithm: Algorithm,
    pub key_length: u64,
    pub public_key: String,
    pub creation_time: String,
    pub is_active: bool,
    pub r#type: KeyType,
    pub key_tag: i32,
    pub digests: Vec<Digest>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum KeyType {
    KeySigning,
    ZoneSigning,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
    Ecdsap256sha256,
    Ecdsap384sha384,
    Rsasha1,
    Rsasha256,
    Rsasha512,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Digest {
    pub r#type: DigestType,
    pub digest: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DigestType {
    Sha1,
    Sha256,
    Sha384,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DnsKeys {
    #[serde(flatten)]
    pub envelope: ListEnvelope,
    pub dns_keys: Vec<DnsKey>,
}

pub struct DnsKeysHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> DnsKeysHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, managed_zone: &str) -> Result<DnsKeys> {
        let route = format!(
            "managedZones/{managed_zone}/dnsKeys",
            managed_zone = managed_zone,
        );

        self.client.get(route).await
    }

    pub async fn get(&self, managed_zone: &str, dns_key_id: &str) -> Result<DnsKey> {
        let route = format!(
            "managedZones/{managed_zone}/dnsKeys/{dns_key_id}",
            managed_zone = managed_zone,
            dns_key_id = dns_key_id,
        );

        self.client.get(route).await
    }
}
