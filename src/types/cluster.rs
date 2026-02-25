//! Cluster related request/response types.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterStatusItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub nodeid: Option<u64>,
    pub online: Option<u64>,
    pub quorate: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterResource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub node: Option<String>,
    pub vmid: Option<u32>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub cpu: Option<f64>,
    pub mem: Option<u64>,
    pub maxmem: Option<u64>,
    pub disk: Option<u64>,
    pub maxdisk: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Copy)]
pub enum ClusterResourceType {
    Vm,
    Storage,
    Node,
    Sdn,
}

impl ClusterResourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vm => "vm",
            Self::Storage => "storage",
            Self::Node => "node",
            Self::Sdn => "sdn",
        }
    }
}

impl fmt::Display for ClusterResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClusterResourcesQuery {
    pub resource_type: Option<ClusterResourceType>,
}

impl ClusterResourcesQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all(resource_type: Option<ClusterResourceType>) -> Self {
        Self { resource_type }
    }

    pub fn resource_type(mut self, resource_type: ClusterResourceType) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("type", self.resource_type.map(|v| v.to_string()));
        params
    }
}
