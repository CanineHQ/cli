//! Kubeconfig schema + (YAML <-> struct <-> JSON) using serde.
//!
//! Add deps:
//! serde = { version = "1", features = ["derive"] }
//! serde_yaml = "0.9"
//! serde_json = "1"

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kubeconfig {
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    pub kind: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clusters: Vec<NamedCluster>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<NamedUser>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<NamedContext>,

    #[serde(rename = "current-context", default, skip_serializing_if = "Option::is_none")]
    pub current_context: Option<String>,

    /// Kubeconfig supports arbitrary extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<NamedExtension>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Preferences {
    /// Often an empty object in kubeconfigs.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedCluster {
    pub name: String,
    pub cluster: Cluster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub server: String,

    #[serde(rename = "certificate-authority", default, skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<String>,

    #[serde(
        rename = "certificate-authority-data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_authority_data: Option<String>,

    #[serde(rename = "insecure-skip-tls-verify", default, skip_serializing_if = "Option::is_none")]
    pub insecure_skip_tls_verify: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<NamedExtension>,

    /// Preserve unknown keys (future-proof).
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedContext {
    pub name: String,
    pub context: Context,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub cluster: String,
    pub user: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<NamedExtension>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedUser {
    pub name: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    // Common static auth fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "token-file", default, skip_serializing_if = "Option::is_none")]
    pub token_file: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "client-certificate", default, skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,

    #[serde(
        rename = "client-certificate-data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_certificate_data: Option<String>,

    #[serde(rename = "client-key", default, skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,

    #[serde(rename = "client-key-data", default, skip_serializing_if = "Option::is_none")]
    pub client_key_data: Option<String>,

    // Dynamic auth mechanisms
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecConfig>,

    #[serde(rename = "auth-provider", default, skip_serializing_if = "Option::is_none")]
    pub auth_provider: Option<AuthProvider>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impersonate: Option<String>,

    #[serde(rename = "impersonate-groups", default, skip_serializing_if = "Vec::is_empty")]
    pub impersonate_groups: Vec<String>,

    #[serde(rename = "impersonate-user-extra", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub impersonate_user_extra: BTreeMap<String, Vec<String>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<NamedExtension>,

    /// Preserve unknown keys (kubeconfig allows arbitrary user fields)
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecConfig {
    /// K8s client auth exec API version (often: client.authentication.k8s.io/v1beta1 or /v1)
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    pub command: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,

    #[serde(rename = "interactiveMode", default, skip_serializing_if = "Option::is_none")]
    pub interactive_mode: Option<String>,

    #[serde(rename = "provideClusterInfo", default, skip_serializing_if = "Option::is_none")]
    pub provide_cluster_info: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_hint: Option<String>,

    #[serde(rename = "installHint", default, skip_serializing_if = "Option::is_none")]
    pub install_hint_alt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProvider {
    pub name: String,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub config: BTreeMap<String, String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedExtension {
    pub name: String,

    /// Extension payload is arbitrary JSON/YAML.
    #[serde(default)]
    pub extension: serde_json::Value,
}

// -------------------- helpers / example usage --------------------

pub fn parse_kubeconfig_yaml(yaml: &str) -> Result<Kubeconfig, serde_yaml::Error> {
    serde_yaml::from_str::<Kubeconfig>(yaml)
}

pub fn kubeconfig_to_pretty_json(cfg: &Kubeconfig) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(cfg)
}

pub fn kubeconfig_to_yaml(cfg: &Kubeconfig) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_yaml_to_json() {
        let yaml = r#"
apiVersion: v1
kind: Config
clusters:
- name: demo
  cluster:
    server: https://1.2.3.4:6443
contexts:
- name: demo
  context:
    cluster: demo
    user: demo
current-context: demo
users:
- name: demo
  user:
    token: abc123
"#;

        let cfg = parse_kubeconfig_yaml(yaml).unwrap();
        let json = kubeconfig_to_pretty_json(&cfg).unwrap();
        assert!(json.contains(r#""current_context""#) || json.contains(r#""current_context":"#));
        // Also ensure we can dump back to YAML
        let yaml2 = kubeconfig_to_yaml(&cfg).unwrap();
        assert!(yaml2.contains("apiVersion"));
    }
}


use std::process::Command;
use std::io;

#[derive(Debug)]
pub enum KubectlError {
    NotFound,
    NotExecutable(io::Error),
    FailedToRun(String),
}

pub fn ensure_kubectl() -> Result<(), KubectlError> {
    let output = Command::new("kubectl")
        .arg("version")
        .arg("--client")
        .output()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                KubectlError::NotFound
            } else {
                KubectlError::NotExecutable(e)
            }
        })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(KubectlError::FailedToRun(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}
