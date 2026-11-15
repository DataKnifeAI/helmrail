//! Provisional CRD shapes for Helmrail.
//!
//! These are serde models for design review. M1 will switch them to
//! `kube::CustomResource` derives and generate installable CRD YAML.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const API_GROUP: &str = "helmrail.dataknife.ai";
pub const API_VERSION: &str = "v1alpha1";

/// Supported runtime adapters.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeKind {
    Hermes,
    Cursor,
    Custom,
}

/// Where inference comes from — typed common option.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelSpec {
    /// Provider hint: `vllm`, `openai`, `anthropic`, `custom`, …
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// OpenAI-compatible base URL (cluster DNS or external).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Model / deployment name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Preset: model, sandbox, dependencies, runtime.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentClassSpec {
    pub runtime: RuntimeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageSpec>,
    #[serde(default)]
    pub model: ModelSpec,
    /// Name of a SandboxProfile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox_ref: Option<String>,
    /// Names of DependencyBundles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_refs: Vec<String>,
    /// Escape hatch for adapter-specific config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentSpec {
    pub class_ref: String,
    #[serde(default)]
    pub model: ModelSpec,
    #[serde(default)]
    pub suspend: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentFleetSpec {
    pub class_ref: String,
    #[serde(default = "default_replicas")]
    pub replicas: u32,
}

fn default_replicas() -> u32 {
    1
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskMode {
    Ephemeral,
    Assign,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentTaskSpec {
    pub class_ref: String,
    pub mode: TaskMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: Option<u32>,
    pub prompt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EgressRule {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to_cidrs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_namespace: Option<String>,
    pub ports: Vec<EgressPort>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EgressPort {
    pub port: u16,
    #[serde(default = "default_tcp")]
    pub protocol: String,
}

fn default_tcp() -> String {
    "TCP".into()
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SandboxProfileSpec {
    #[serde(default)]
    pub allow_dns: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<EgressRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_class_roundtrip() {
        let spec = AgentClassSpec {
            runtime: RuntimeKind::Hermes,
            image: None,
            model: ModelSpec {
                provider: Some("vllm".into()),
                base_url: Some("http://vllm.inference.svc:8000/v1".into()),
                name: Some("hermes3:8b".into()),
            },
            sandbox_ref: Some("restricted".into()),
            dependency_refs: vec!["cluster-vllm-egress".into()],
            raw: None,
        };
        let json = serde_json::to_string(&spec).unwrap();
        let back: AgentClassSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(back.model.provider.as_deref(), Some("vllm"));
        assert_eq!(back.runtime, RuntimeKind::Hermes);
    }
}
