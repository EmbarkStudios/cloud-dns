use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

use super::{ListEnvelope, UpdateEnvelope};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    kind: String, // "dns#policy"
    id: u64,
    name: String,
    enable_inbound_forwarding: bool,
    description: String,
    networks: PolicyNetwork,
    alternative_name_server_config: AlternativeNameServerConfig,
    enable_logging: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PolicyNetwork {
    kind: String, // "dns#policyNetwork"
    network_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlternativeNameServerConfig {
    kind: String, // "dns#policyAlternativeNameServerConfig"
    target_name_servers: Vec<AlternativeNameServerConfigTargetNameServers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlternativeNameServerConfigTargetNameServers {
    kind: String, // "dns#policyAlternativeNameServerConfigTargetNameServer"
    ipv4_address: String,
    forwarding_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Policies {
    #[serde(flatten)]
    envelope: ListEnvelope,
    policies: Vec<Policy>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicy {
    #[serde(flatten)]
    envelope: UpdateEnvelope,
    policy: Policy,
}

pub struct PoliciesHandler<'client> {
    client: &'client DnsClient,
}

impl<'client> PoliciesHandler<'client> {
    pub(crate) fn new(client: &'client DnsClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Policies> {
        let route = "policies".to_string();

        self.client.get(route).await
    }

    pub async fn get(&self, policy: &str) -> Result<Policy> {
        let route = format!("policies/{policy}", policy = policy,);

        self.client.get(route).await
    }

    pub async fn patch(&self, policy_id: &str, policy: Policy) -> Result<Policy> {
        let route = format!("policies/{policy_id}", policy_id = policy_id,);

        self.client.patch(route, Some(&policy)).await
    }

    pub async fn create(&self, policy: Policy) -> Result<Policy> {
        let route = "policies".to_string();

        self.client.post(route, Some(&policy)).await
    }

    pub async fn delete(&self, policy_id: &str) -> Result<()> {
        let route = format!("policies/{policy_id}", policy_id = policy_id,);

        self.client.delete(route).await
    }

    pub async fn update(&self, policy_id: &str, policy: Policy) -> Result<UpdatePolicy> {
        let route = format!("policies/{policy_id}", policy_id = policy_id,);

        self.client.put(route, Some(&policy)).await
    }
}
