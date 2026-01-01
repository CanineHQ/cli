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
    pub cluster_id: i32,
    pub name: String,
    pub namespace: String,
    pub repository_url: String,
    pub branch: String,
    pub status: ProjectStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
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
