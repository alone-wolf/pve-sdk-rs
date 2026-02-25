//! LXC related request/response types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;
pub use crate::types::common::SnapshotInfo;

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

#[derive(Debug, Clone)]
pub struct LxcCreateRequest {
    pub vmid: u32,
    pub ostemplate: String,
    pub hostname: Option<String>,
    pub memory: Option<u32>,
    pub cores: Option<u32>,
    pub rootfs: Option<String>,
    pub net0: Option<String>,
    pub swap: Option<u32>,
    pub onboot: Option<bool>,
    pub unprivileged: Option<bool>,
    pub features: Option<String>,
    pub extra: PveParams,
}

impl LxcCreateRequest {
    pub fn new(vmid: u32, ostemplate: impl Into<String>) -> Self {
        Self {
            vmid,
            ostemplate: ostemplate.into(),
            hostname: None,
            memory: None,
            cores: None,
            rootfs: None,
            net0: None,
            swap: None,
            onboot: None,
            unprivileged: None,
            features: None,
            extra: PveParams::new(),
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("vmid", self.vmid.to_string());
        params.insert("ostemplate", self.ostemplate.clone());
        params.insert_opt("hostname", self.hostname.clone());
        params.insert_opt("memory", self.memory.map(|v| v.to_string()));
        params.insert_opt("cores", self.cores.map(|v| v.to_string()));
        params.insert_opt("rootfs", self.rootfs.clone());
        params.insert_opt("net0", self.net0.clone());
        params.insert_opt("swap", self.swap.map(|v| v.to_string()));
        if let Some(onboot) = self.onboot {
            params.insert_bool("onboot", onboot);
        }
        if let Some(unprivileged) = self.unprivileged {
            params.insert_bool("unprivileged", unprivileged);
        }
        params.insert_opt("features", self.features.clone());
        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct LxcConfigQuery {
    pub current: Option<bool>,
    pub snapshot: Option<String>,
}

impl LxcConfigQuery {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        if let Some(current) = self.current {
            params.insert_bool("current", current);
        }
        params.insert_opt("snapshot", self.snapshot.clone());
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct LxcSetConfigRequest {
    pub delete: Option<String>,
    pub digest: Option<String>,
    pub memory: Option<u32>,
    pub cores: Option<u32>,
    pub rootfs: Option<String>,
    pub net0: Option<String>,
    pub mp0: Option<String>,
    pub swap: Option<u32>,
    pub onboot: Option<bool>,
    pub unprivileged: Option<bool>,
    pub features: Option<String>,
    pub tags: Option<String>,
    pub extra: PveParams,
}

impl LxcSetConfigRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("delete", self.delete.clone());
        params.insert_opt("digest", self.digest.clone());
        params.insert_opt("memory", self.memory.map(|v| v.to_string()));
        params.insert_opt("cores", self.cores.map(|v| v.to_string()));
        params.insert_opt("rootfs", self.rootfs.clone());
        params.insert_opt("net0", self.net0.clone());
        params.insert_opt("mp0", self.mp0.clone());
        params.insert_opt("swap", self.swap.map(|v| v.to_string()));
        if let Some(onboot) = self.onboot {
            params.insert_bool("onboot", onboot);
        }
        if let Some(unprivileged) = self.unprivileged {
            params.insert_bool("unprivileged", unprivileged);
        }
        params.insert_opt("features", self.features.clone());
        params.insert_opt("tags", self.tags.clone());
        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct LxcActionRequest {
    pub timeout: Option<u64>,
    pub skiplock: Option<bool>,
    pub debug: Option<bool>,
    pub force_stop: Option<bool>,
    pub overrule_shutdown: Option<bool>,
    pub extra: PveParams,
}

impl LxcActionRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();

        params.insert_opt("timeout", self.timeout.map(|v| v.to_string()));
        if let Some(skiplock) = self.skiplock {
            params.insert_bool("skiplock", skiplock);
        }
        if let Some(debug) = self.debug {
            params.insert_bool("debug", debug);
        }
        if let Some(force_stop) = self.force_stop {
            params.insert_bool("forceStop", force_stop);
        }
        if let Some(overrule_shutdown) = self.overrule_shutdown {
            params.insert_bool("overrule-shutdown", overrule_shutdown);
        }

        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone)]
pub struct LxcSnapshotCreateRequest {
    pub snapname: String,
    pub description: Option<String>,
}

impl LxcSnapshotCreateRequest {
    pub fn new(snapname: impl Into<String>) -> Self {
        Self {
            snapname: snapname.into(),
            description: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("snapname", self.snapname.clone());
        params.insert_opt("description", self.description.clone());
        params
    }
}

#[derive(Debug, Clone)]
pub struct LxcSnapshotRollbackRequest {
    pub snapname: String,
    pub start: Option<bool>,
}

impl LxcSnapshotRollbackRequest {
    pub fn new(snapname: impl Into<String>) -> Self {
        Self {
            snapname: snapname.into(),
            start: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        if let Some(start) = self.start {
            params.insert_bool("start", start);
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct LxcMigrateRequest {
    pub target: String,
    pub online: Option<bool>,
    pub restart: Option<bool>,
    pub target_storage: Option<String>,
    pub bwlimit: Option<u64>,
    pub timeout: Option<u64>,
    pub extra: PveParams,
}

impl LxcMigrateRequest {
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            online: None,
            restart: None,
            target_storage: None,
            bwlimit: None,
            timeout: None,
            extra: PveParams::new(),
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("target", self.target.clone());
        if let Some(online) = self.online {
            params.insert_bool("online", online);
        }
        if let Some(restart) = self.restart {
            params.insert_bool("restart", restart);
        }
        params.insert_opt("target-storage", self.target_storage.clone());
        params.insert_opt("bwlimit", self.bwlimit.map(|v| v.to_string()));
        params.insert_opt("timeout", self.timeout.map(|v| v.to_string()));
        params.extend(&self.extra);
        params
    }
}
