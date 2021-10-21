use serde::{Deserialize, Serialize};

use crate::{DNSClient, Result};

use super::{ListEnvelope, UpdateEnvelope};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
struct PolicyNetwork {
    kind: String, // "dns#policyNetwork"
    network_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct AlternativeNameServerConfig {
    kind: String, // "dns#policyAlternativeNameServerConfig"
    target_name_servers: Vec<AlternativeNameServerConfigTargetNameServers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
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
    client: &'client DNSClient,
}

impl<'client> PoliciesHandler<'client> {
    pub(crate) fn new(client: &'client DNSClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Policies> {
        let route = "policies".to_string();

        self.client.get(route, None::<&()>).await
    }

    pub async fn get(&self, policy: String) -> Result<Policy> {
        let route = format!("policies/{policy}", policy = policy,);

        self.client.get(route, None::<&()>).await
    }

    pub async fn patch(&self, policy_id: String, policy: Policy) -> Result<Policy> {
        let route = format!("policies/{policy_id}", policy_id = policy_id,);

        self.client.patch(route, Some(&policy)).await
    }

    pub async fn create(&self, policy: Policy) -> Result<Policy> {
        let route = "policies".to_string();

        self.client.post(route, Some(&policy)).await
    }

    pub async fn delete(&self, policy_id: String) -> Result<()> {
        let route = format!("policies/{policy_id}", policy_id = policy_id,);

        self.client.delete(route, None::<&()>).await
    }

    pub async fn update(&self, policy_id: String, policy: Policy) -> Result<UpdatePolicy> {
        let route = format!("policies/{policy_id}", policy_id = policy_id,);

        self.client.put(route, Some(&policy)).await
    }
}
