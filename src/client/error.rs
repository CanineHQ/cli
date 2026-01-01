use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("unauthorized (401)")]
    Unauthorized,
    #[error("forbidden (403)")]
    Forbidden,
    #[error("not found (404)")]
    NotFound,
    #[error("server error ({status})")]
    ServerError { status: StatusCode, body: String },
}

#[derive(Debug, Error)]
pub enum CanineError {
    #[error("account not found: {0}")]
    NoAccount(String),
    #[error("no token configured")]
    NoToken,
    #[error("one-off pod never became ready")]
    OneOffPodNeverReady,
    #[error("api error: {0}")]
    Api(#[from] ApiError),
    #[error("url join error: {0}")]
    UrlJoin(String),
    #[error(transparent)]
    Transport(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
