//! Task related request/response types.

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskStatus {
    pub upid: Option<String>,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub status: String,
    pub exitstatus: Option<String>,
    pub user: Option<String>,
    pub starttime: Option<u64>,
    pub node: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskLogLine {
    pub n: Option<u64>,
    pub t: String,
}

#[derive(Debug, Clone, Default)]
pub struct TaskLogQuery {
    pub start: Option<u64>,
    pub limit: Option<u64>,
}

impl TaskLogQuery {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("start", self.start.map(|v| v.to_string()));
        params.insert_opt("limit", self.limit.map(|v| v.to_string()));
        params
    }
}

#[derive(Debug, Clone)]
pub struct WaitTaskOptions {
    pub poll_interval: Duration,
    pub timeout: Option<Duration>,
}

impl Default for WaitTaskOptions {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(2),
            timeout: None,
        }
    }
}
