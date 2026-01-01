mod error;
mod types;

pub use error::{ApiError, CanineError};
pub use types::*;

use serde::Serialize;
use url::Url;

#[derive(Clone, Debug)]
pub struct CanineClient {
    http: reqwest::Client,
    pub base_url: Url,
    auth: Auth,
    account: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Auth {
    Bearer(String),
    ApiKey(String),
}

impl CanineClient {
    pub fn new(
        url: impl AsRef<str>,
        auth: Auth,
        account: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            base_url: Url::parse(url.as_ref())?,
            auth,
            http: reqwest::Client::new(),
            account,
        })
    }

    async fn send_request<RBody, T: Serialize>(
        &self,
        path: &str,
        method: reqwest::Method,
        body: Option<&T>,
    ) -> Result<RBody, CanineError>
    where
        RBody: serde::de::DeserializeOwned,
    {
        let url = self
            .base_url
            .join(path)
            .map_err(|e| CanineError::UrlJoin(e.to_string()))?;

        let mut req = self.http.request(method, url);
        if let Some(account) = &self.account {
            req = req.header("X-ACCOUNT-ID", account);
        }

        if let Auth::ApiKey(token) = &self.auth {
            req = req.header("X-API-KEY", token);
        }

        if let Some(body) = body {
            req = req.json(body);
        }

        let res = req.send().await?;
        let status = res.status();

        let body = res.text().await?;
        if status.is_success() {
            let me: RBody = serde_json::from_str(&body)?;
            Ok(me)
        } else {
            Err(ApiError::ServerError { status, body }.into())
        }
    }

    pub async fn me(&self) -> Result<User, CanineError> {
        self.send_request::<User, ()>("/api/v1/me", reqwest::Method::GET, None)
            .await
    }

    pub async fn get_projects(&self) -> Result<ProjectsResponse, CanineError> {
        self.send_request::<ProjectsResponse, ()>("/api/v1/projects", reqwest::Method::GET, None)
            .await
    }

    pub async fn get_project(&self, project_id: &str) -> Result<Project, CanineError> {
        self.send_request::<Project, ()>(
            format!("/api/v1/projects/{}", project_id).as_str(),
            reqwest::Method::GET,
            None,
        )
        .await
    }

    pub async fn get_processes(&self, project_id: &str) -> Result<ProcessesResponse, CanineError> {
        self.send_request::<ProcessesResponse, ()>(
            format!("/api/v1/projects/{}/processes", project_id).as_str(),
            reqwest::Method::GET,
            None,
        )
        .await
    }

    pub async fn create_one_off_pod(&self, project_id: &str) -> Result<Pod, CanineError> {
        self.send_request::<Pod, ()>(
            format!("/api/v1/projects/{}/processes", project_id).as_str(),
            reqwest::Method::POST,
            None,
        )
        .await
    }

    pub async fn get_pod(&self, project_id: &str, pod_id: &str) -> Result<Pod, CanineError> {
        self.send_request::<Pod, ()>(
            format!("/api/v1/projects/{}/processes/{}", project_id, pod_id).as_str(),
            reqwest::Method::GET,
            None,
        )
        .await
    }

    pub async fn deploy_project(
        &self,
        project_id: &str,
        skip_build: bool,
    ) -> Result<DeployProjectResponse, CanineError> {
        self.send_request::<DeployProjectResponse, DeployProjectRequest>(
            format!("/api/v1/projects/{}/deploy", project_id).as_str(),
            reqwest::Method::POST,
            Some(&DeployProjectRequest { skip_build }),
        )
        .await
    }

    pub async fn download_kubeconfig_file(
        &self,
        cluster_id: &str,
    ) -> Result<ClusterKubeconfigResponse, CanineError> {
        self.send_request::<ClusterKubeconfigResponse, ()>(
            format!("/api/v1/clusters/{}/download_kubeconfig", cluster_id).as_str(),
            reqwest::Method::GET,
            None,
        )
        .await
    }
}
