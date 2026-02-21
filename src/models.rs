use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApiEnvelope<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketInfo {
    pub username: String,
    pub ticket: String,
    #[serde(rename = "CSRFPreventionToken")]
    pub csrf_prevention_token: String,
    pub clustername: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub release: Option<String>,
    pub repoid: Option<String>,
    pub console: Option<String>,
}

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QemuVmSummary {
    pub vmid: u32,
    pub name: Option<String>,
    pub status: Option<String>,
    pub cpu: Option<f64>,
    pub mem: Option<u64>,
    pub maxmem: Option<u64>,
    pub maxdisk: Option<u64>,
    pub uptime: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QemuStatus {
    pub vmid: Option<u32>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub qmpstatus: Option<String>,
    pub cpu: Option<f64>,
    pub mem: Option<u64>,
    pub maxmem: Option<u64>,
    pub netin: Option<u64>,
    pub netout: Option<u64>,
    pub diskread: Option<u64>,
    pub diskwrite: Option<u64>,
    pub uptime: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LxcSummary {
    pub vmid: u32,
    pub name: Option<String>,
    pub status: Option<String>,
    pub cpu: Option<f64>,
    pub mem: Option<u64>,
    pub maxmem: Option<u64>,
    pub maxdisk: Option<u64>,
    pub uptime: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LxcStatus {
    pub vmid: Option<u32>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub cpu: Option<f64>,
    pub mem: Option<u64>,
    pub maxmem: Option<u64>,
    pub maxdisk: Option<u64>,
    pub netin: Option<u64>,
    pub netout: Option<u64>,
    pub diskread: Option<u64>,
    pub diskwrite: Option<u64>,
    pub uptime: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageIndexItem {
    pub storage: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeStorageStatus {
    pub storage: String,
    #[serde(rename = "type")]
    pub storage_type: Option<String>,
    pub active: Option<u8>,
    pub enabled: Option<u8>,
    pub used: Option<u64>,
    pub avail: Option<u64>,
    pub total: Option<u64>,
    pub shared: Option<u8>,
    pub content: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageContentItem {
    pub volid: String,
    pub format: Option<String>,
    pub size: Option<u64>,
    pub used: Option<u64>,
    pub vmid: Option<u32>,
    pub ctime: Option<u64>,
    pub notes: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
