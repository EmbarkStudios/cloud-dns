use serde::{Deserialize, Serialize};

use crate::{DnsClient, Result};

use super::{ListEnvelope, UpdateEnvelope};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    pub kind: String, // "dns#policy"
    pub id: u64,
    pub name: String,
    pub enable_inbound_forwarding: bool,
    pub description: String,
    pub networks: PolicyNetwork,
    pub alternative_name_server_config: AlternativeNameServerConfig,
    pub enable_logging: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolicyNetwork {
    pub kind: String, // "dns#policyNetwork"
    pub network_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlternativeNameServerConfig {
    pub kind: String, // "dns#policyAlternativeNameServerConfig"
    pub target_name_servers: Vec<AlternativeNameServerConfigTargetNameServers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlternativeNameServerConfigTargetNameServers {
    pub kind: String, // "dns#policyAlternativeNameServerConfigTargetNameServer"
    pub ipv4_address: String,
    pub forwarding_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Policies {
    #[serde(flatten)]
    pub envelope: ListEnvelope,
    pub policies: Vec<Policy>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicy {
    #[serde(flatten)]
    pub envelope: UpdateEnvelope,
    pub policy: Policy,
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
