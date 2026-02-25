use std::time::Duration;

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::PveError;
use crate::models::ApiEnvelope;

pub(crate) fn normalize_api_path(path: &str) -> String {
    if path.starts_with("/api2/json") {
        return path.to_string();
    }

    if path.starts_with('/') {
        format!("/api2/json{path}")
    } else {
        format!("/api2/json/{path}")
    }
}

pub(crate) fn enc(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

pub(crate) fn join_api_url(base_url: &Url, path: &str) -> Result<Url, PveError> {
    let normalized = normalize_api_path(path);
    base_url
        .join(normalized.trim_start_matches('/'))
        .map_err(|_| PveError::InvalidBaseUrl(format!("unable to join path: {normalized}")))
}

pub(crate) fn build_base_url(host: &str, port: u16, https: bool) -> Result<Url, PveError> {
    let mut host = host.trim().to_string();
    if host.starts_with("https://") || host.starts_with("http://") {
        let parsed = Url::parse(&host).map_err(|_| {
            PveError::InvalidBaseUrl(
                "invalid host URL, expected a hostname or IP without path/port".to_string(),
            )
        })?;
        if parsed.port().is_some() {
            return Err(PveError::InvalidBaseUrl(
                "host must not include port, use ClientOption::port()".to_string(),
            ));
        }
        if parsed.path() != "/" {
            return Err(PveError::InvalidBaseUrl(
                "host must not include path, use API methods with relative paths".to_string(),
            ));
        }
        if parsed.query().is_some() || parsed.fragment().is_some() {
            return Err(PveError::InvalidBaseUrl(
                "host must not include query or fragment".to_string(),
            ));
        }
        if !parsed.username().is_empty() || parsed.password().is_some() {
            return Err(PveError::InvalidBaseUrl(
                "host must not include credentials".to_string(),
            ));
        }
        host = parsed.host_str().unwrap_or_default().to_string();
    }

    host = host.trim_end_matches('/').to_string();
    if host.contains('/') {
        return Err(PveError::InvalidBaseUrl(
            "host must not include path, use ClientOption::host(\"pve.example.com\")".to_string(),
        ));
    }
    if host.contains('?') || host.contains('#') {
        return Err(PveError::InvalidBaseUrl(
            "host must not include query or fragment".to_string(),
        ));
    }
    if host.contains('@') {
        return Err(PveError::InvalidBaseUrl(
            "host must not include credentials".to_string(),
        ));
    }
    if host.contains("]:") || (host.matches(':').count() == 1 && !host.starts_with('[')) {
        return Err(PveError::InvalidBaseUrl(
            "host must not include port, use ClientOption::port()".to_string(),
        ));
    }

    if host.matches(':').count() > 1 && !host.starts_with('[') && !host.ends_with(']') {
        host = format!("[{host}]");
    }
    if host.is_empty() {
        return Err(PveError::InvalidBaseUrl("host is empty".to_string()));
    }

    let scheme = if https { "https" } else { "http" };
    let base = format!("{scheme}://{host}:{port}/");
    Url::parse(&base).map_err(|_| PveError::InvalidBaseUrl(base))
}

pub(crate) fn build_http_client(
    insecure_tls: bool,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
) -> Result<reqwest::Client, PveError> {
    let mut builder = reqwest::Client::builder().danger_accept_invalid_certs(insecure_tls);
    if let Some(timeout) = timeout {
        builder = builder.timeout(timeout);
    }
    if let Some(connect_timeout) = connect_timeout {
        builder = builder.connect_timeout(connect_timeout);
    }
    builder.build().map_err(PveError::from)
}

pub(crate) async fn execute<T>(request: RequestBuilder) -> Result<T, PveError>
where
    T: DeserializeOwned,
{
    let response = request.send().await?;
    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(PveError::ApiStatus {
            status: status.as_u16(),
            body,
        });
    }

    let payload: ApiEnvelope<T> = serde_json::from_str(&body)?;
    Ok(payload.data)
}
