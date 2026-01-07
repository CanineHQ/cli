use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;
use tabled::Tabled;

use crate::kubeconfig::Kubeconfig;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Account {
    pub id: i32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub current_account: Account,
    pub accounts: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Creating,
    Deployed,
    Destroying,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum ClusterStatus {
    Initializing,
    Installing,
    Running,
    Failed,
    Destroying,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum ClusterType {
    K8s,
    K3s,
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
pub enum ProcessStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub namespace: String,
    pub repository_url: String,
    pub branch: String,
    pub status: ProjectStatus,
    pub cluster_id: i32,
    pub cluster_name: String,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Cluster {
    pub id: i32,
    pub name: String,
    pub cluster_type: ClusterType,
    pub status: ClusterStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClustersResponse {
    pub clusters: Vec<Cluster>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildsResponse {
    pub builds: Vec<Build>,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Process {
    pub name: String,
    pub namespace: String,
    pub status: ProcessStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessesResponse {
    pub pods: Vec<Process>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployProjectResponse {
    pub message: String,
    pub build_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pod {
    pub name: String,
    pub namespace: String,
    pub status: ProcessStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployProjectRequest {
    pub skip_build: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterKubeconfigResponse {
    pub kubeconfig: Kubeconfig,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Build {
    pub id: i32,
    pub commit_sha: String,
    pub commit_message: String,
    pub project_id: i32,
    pub project_slug: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOnsResponse {
    pub add_ons: Vec<AddOn>,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct AddOn {
    pub id: i32,
    pub name: String,
    pub status: AddOnStatus,
    pub cluster_id: i32,
    pub cluster_name: String,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum AddOnStatus {
    Installing,
    Installed,
    Uninstalling,
    Uninstalled,
    Failed,
    Updating,
}