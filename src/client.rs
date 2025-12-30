use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use url::Url;
use reqwest;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct CanineClient {
    http: reqwest::Client,
    base_url: Url,
    auth: Auth,
}

#[derive(Clone, Debug)]
pub enum Auth {
    None,
    Bearer(String),
    ApiKey(String),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("unauthorized (401)")]
    Unauthorized,
    #[error("forbidden (403)")]
    Forbidden,
    #[error("forbidden (404)")]
    NotFound,
    #[error("server error ({status})")]
    ServerError {
        status: StatusCode,
        body: String,
    },

}
#[derive(Debug, Error)]
pub enum CanineError {
    #[error("No token")]
    NoToken,
    #[error("api error")]
    Api(#[from] ApiError),
    #[error("url join error: {0}")]
    UrlJoin(String),
    #[error(transparent)]
    Transport(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeResponse {
    pub email: String,
}

impl CanineClient {
    pub fn new(url: impl AsRef<str>, auth: Auth) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            base_url: Url::parse(url.as_ref())?,
            auth,
            http: reqwest::Client::new()
        })
    }

    async fn get_json<RBody>(&self, path: &str) -> Result<RBody, CanineError> where RBody: serde::de::DeserializeOwned {
        let url = self.base_url
            .join(&path)
            .map_err(|e| CanineError::UrlJoin(e.to_string()))?;

        let mut req = self.http.get(url);
        if let Auth::ApiKey(token) = &self.auth {
            req = req.header("X-API-KEY", token);
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

    pub async fn me(&self) -> Result<MeResponse, CanineError> {
        self.get_json::<MeResponse>("/api/v1/me").await
    }
}