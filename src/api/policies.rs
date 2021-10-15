use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Policy {
    kind: String, // "dns#policy"
    id: u64,
    name: String,
    enable_inbound_forwarding: bool,
    description: String,
    networks: PolicyNetwork,
    alternative_name_server_config: AlternativeNameServerConfig,
    enable_logging: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct PolicyNetwork {
    kind: String, // "dns#policyNetwork"
    network_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct AlternativeNameServerConfig {
    kind: String, // "dns#policyAlternativeNameServerConfig"
    target_name_servers: Vec<AlternativeNameServerConfigTargetNameServers>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct AlternativeNameServerConfigTargetNameServers {
    kind: String, // "dns#policyAlternativeNameServerConfigTargetNameServer"
    ipv4_address: String,
    forwarding_path: String,
}
