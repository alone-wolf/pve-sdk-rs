//! Common/shared SDK types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApiEnvelope<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub release: Option<String>,
    pub repoid: Option<String>,
    pub console: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SnapshotInfo {
    pub name: String,
    pub description: Option<String>,
    pub parent: Option<String>,
    pub snaptime: Option<u64>,
    pub vmstate: Option<u8>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

pub use crate::params::PveParams;
