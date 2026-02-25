//! Node related request/response types.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeSummary {
    pub node: String,
    pub status: Option<String>,
    pub cpu: Option<f64>,
    pub mem: Option<u64>,
    pub maxmem: Option<u64>,
    pub uptime: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkInterface {
    pub iface: Option<String>,
    #[serde(rename = "type")]
    pub interface_type: Option<String>,
    pub active: Option<u8>,
    pub autostart: Option<u8>,
    pub address: Option<String>,
    pub cidr: Option<String>,
    pub gateway: Option<String>,
    pub mtu: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeTask {
    pub upid: String,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub status: Option<String>,
    pub user: Option<String>,
    pub starttime: Option<u64>,
    pub endtime: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Copy)]
pub enum TaskSource {
    Archive,
    Active,
    All,
}

impl TaskSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Archive => "archive",
            Self::Active => "active",
            Self::All => "all",
        }
    }
}

impl fmt::Display for TaskSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub struct NodeTasksQuery {
    pub errors: Option<bool>,
    pub limit: Option<u64>,
    pub since: Option<u64>,
    pub source: Option<TaskSource>,
    pub start: Option<u64>,
    pub statusfilter: Option<String>,
    pub typefilter: Option<String>,
    pub until: Option<u64>,
    pub userfilter: Option<String>,
    pub vmid: Option<u32>,
}

impl NodeTasksQuery {
    #[allow(clippy::too_many_arguments)]
    pub fn all(
        errors: Option<bool>,
        limit: Option<u64>,
        since: Option<u64>,
        source: Option<TaskSource>,
        start: Option<u64>,
        statusfilter: Option<String>,
        typefilter: Option<String>,
        until: Option<u64>,
        userfilter: Option<String>,
        vmid: Option<u32>,
    ) -> Self {
        Self {
            errors,
            limit,
            since,
            source,
            start,
            statusfilter,
            typefilter,
            until,
            userfilter,
            vmid,
        }
    }

    pub fn errors(mut self, errors: bool) -> Self {
        self.errors = Some(errors);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn since(mut self, since: u64) -> Self {
        self.since = Some(since);
        self
    }

    pub fn source(mut self, source: TaskSource) -> Self {
        self.source = Some(source);
        self
    }

    pub fn start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    pub fn statusfilter(mut self, statusfilter: impl Into<String>) -> Self {
        self.statusfilter = Some(statusfilter.into());
        self
    }

    pub fn typefilter(mut self, typefilter: impl Into<String>) -> Self {
        self.typefilter = Some(typefilter.into());
        self
    }

    pub fn until(mut self, until: u64) -> Self {
        self.until = Some(until);
        self
    }

    pub fn userfilter(mut self, userfilter: impl Into<String>) -> Self {
        self.userfilter = Some(userfilter.into());
        self
    }

    pub fn vmid(mut self, vmid: u32) -> Self {
        self.vmid = Some(vmid);
        self
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();

        if let Some(value) = self.errors {
            params.insert_bool("errors", value);
        }

        params.insert_opt("limit", self.limit.map(|v| v.to_string()));
        params.insert_opt("since", self.since.map(|v| v.to_string()));
        params.insert_opt("source", self.source.map(|v| v.to_string()));
        params.insert_opt("start", self.start.map(|v| v.to_string()));
        params.insert_opt("statusfilter", self.statusfilter.clone());
        params.insert_opt("typefilter", self.typefilter.clone());
        params.insert_opt("until", self.until.map(|v| v.to_string()));
        params.insert_opt("userfilter", self.userfilter.clone());
        params.insert_opt("vmid", self.vmid.map(|v| v.to_string()));

        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct NodeNetworkQuery {
    pub interface_type: Option<String>,
}

impl NodeNetworkQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all(interface_type: Option<String>) -> Self {
        Self { interface_type }
    }

    pub fn interface_type(mut self, interface_type: impl Into<String>) -> Self {
        self.interface_type = Some(interface_type.into());
        self
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("type", self.interface_type.clone());
        params
    }
}
