use thiserror::Error;

#[derive(Debug, Error)]
pub enum PveError {
    #[error("invalid base url: {0}")]
    InvalidBaseUrl(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to decode api response: {0}")]
    Decode(#[from] serde_json::Error),

    #[error("pve api returned status {status}: {body}")]
    ApiStatus { status: u16, body: String },

    #[error("missing csrf token for write request in ticket mode")]
    MissingCsrfToken,

    #[error("task {upid} failed with exitstatus {exitstatus}")]
    TaskFailed { upid: String, exitstatus: String },

    #[error("task {upid} timed out after {timeout_secs}s")]
    TaskTimeout { upid: String, timeout_secs: u64 },
}
