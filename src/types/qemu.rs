//! QEMU related request/response types.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;
pub use crate::types::common::SnapshotInfo;

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

#[derive(Debug, Clone, Copy)]
pub enum QemuBios {
    SeaBios,
    Ovmf,
}

impl QemuBios {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SeaBios => "seabios",
            Self::Ovmf => "ovmf",
        }
    }
}

impl fmt::Display for QemuBios {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QemuOsType {
    Other,
    Linux26,
    Win10,
    Win11,
}

impl QemuOsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Other => "other",
            Self::Linux26 => "l26",
            Self::Win10 => "win10",
            Self::Win11 => "win11",
        }
    }
}

impl fmt::Display for QemuOsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct QemuCreateRequest {
    pub vmid: u32,
    pub name: Option<String>,
    pub memory: Option<u32>,
    pub cores: Option<u32>,
    pub sockets: Option<u32>,
    pub cpu: Option<String>,
    pub bios: Option<QemuBios>,
    pub ostype: Option<QemuOsType>,
    pub agent: Option<String>,
    pub net0: Option<String>,
    pub scsi0: Option<String>,
    pub virtio0: Option<String>,
    pub machine: Option<String>,
    pub onboot: Option<bool>,
    pub tags: Option<String>,
    pub extra: PveParams,
}

impl QemuCreateRequest {
    pub fn new(vmid: u32) -> Self {
        Self {
            vmid,
            name: None,
            memory: None,
            cores: None,
            sockets: None,
            cpu: None,
            bios: None,
            ostype: None,
            agent: None,
            net0: None,
            scsi0: None,
            virtio0: None,
            machine: None,
            onboot: None,
            tags: None,
            extra: PveParams::new(),
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("vmid", self.vmid.to_string());
        params.insert_opt("name", self.name.clone());
        params.insert_opt("memory", self.memory.map(|v| v.to_string()));
        params.insert_opt("cores", self.cores.map(|v| v.to_string()));
        params.insert_opt("sockets", self.sockets.map(|v| v.to_string()));
        params.insert_opt("cpu", self.cpu.clone());
        params.insert_opt("bios", self.bios.map(|v| v.to_string()));
        params.insert_opt("ostype", self.ostype.map(|v| v.to_string()));
        params.insert_opt("agent", self.agent.clone());
        params.insert_opt("net0", self.net0.clone());
        params.insert_opt("scsi0", self.scsi0.clone());
        params.insert_opt("virtio0", self.virtio0.clone());
        params.insert_opt("machine", self.machine.clone());
        if let Some(onboot) = self.onboot {
            params.insert_bool("onboot", onboot);
        }
        params.insert_opt("tags", self.tags.clone());
        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct QemuConfigQuery {
    pub current: Option<bool>,
    pub snapshot: Option<String>,
}

impl QemuConfigQuery {
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
pub struct QemuSetConfigRequest {
    pub delete: Option<String>,
    pub digest: Option<String>,
    pub memory: Option<u32>,
    pub cores: Option<u32>,
    pub sockets: Option<u32>,
    pub cpu: Option<String>,
    pub agent: Option<String>,
    pub boot: Option<String>,
    pub bootdisk: Option<String>,
    pub net0: Option<String>,
    pub scsi0: Option<String>,
    pub virtio0: Option<String>,
    pub hotplug: Option<String>,
    pub onboot: Option<bool>,
    pub tags: Option<String>,
    pub extra: PveParams,
}

impl QemuSetConfigRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("delete", self.delete.clone());
        params.insert_opt("digest", self.digest.clone());
        params.insert_opt("memory", self.memory.map(|v| v.to_string()));
        params.insert_opt("cores", self.cores.map(|v| v.to_string()));
        params.insert_opt("sockets", self.sockets.map(|v| v.to_string()));
        params.insert_opt("cpu", self.cpu.clone());
        params.insert_opt("agent", self.agent.clone());
        params.insert_opt("boot", self.boot.clone());
        params.insert_opt("bootdisk", self.bootdisk.clone());
        params.insert_opt("net0", self.net0.clone());
        params.insert_opt("scsi0", self.scsi0.clone());
        params.insert_opt("virtio0", self.virtio0.clone());
        params.insert_opt("hotplug", self.hotplug.clone());
        if let Some(onboot) = self.onboot {
            params.insert_bool("onboot", onboot);
        }
        params.insert_opt("tags", self.tags.clone());
        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct QemuActionRequest {
    pub timeout: Option<u64>,
    pub skiplock: Option<bool>,
    pub force_stop: Option<bool>,
    pub keep_active: Option<bool>,
    pub overrule_shutdown: Option<bool>,
    pub todisk: Option<bool>,
    pub statestorage: Option<String>,
    pub nocheck: Option<bool>,
    pub migration_network: Option<String>,
    pub targetstorage: Option<String>,
    pub extra: PveParams,
}

impl QemuActionRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();

        params.insert_opt("timeout", self.timeout.map(|v| v.to_string()));

        if let Some(skiplock) = self.skiplock {
            params.insert_bool("skiplock", skiplock);
        }
        if let Some(force_stop) = self.force_stop {
            params.insert_bool("forceStop", force_stop);
        }
        if let Some(keep_active) = self.keep_active {
            params.insert_bool("keepActive", keep_active);
        }
        if let Some(overrule_shutdown) = self.overrule_shutdown {
            params.insert_bool("overrule-shutdown", overrule_shutdown);
        }
        if let Some(todisk) = self.todisk {
            params.insert_bool("todisk", todisk);
        }
        if let Some(nocheck) = self.nocheck {
            params.insert_bool("nocheck", nocheck);
        }

        params.insert_opt("statestorage", self.statestorage.clone());
        params.insert_opt("migration_network", self.migration_network.clone());
        params.insert_opt("targetstorage", self.targetstorage.clone());

        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone)]
pub struct QemuSnapshotCreateRequest {
    pub snapname: String,
    pub description: Option<String>,
    pub vmstate: Option<bool>,
}

impl QemuSnapshotCreateRequest {
    pub fn new(snapname: impl Into<String>) -> Self {
        Self {
            snapname: snapname.into(),
            description: None,
            vmstate: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("snapname", self.snapname.clone());
        params.insert_opt("description", self.description.clone());
        if let Some(vmstate) = self.vmstate {
            params.insert_bool("vmstate", vmstate);
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct QemuSnapshotRollbackRequest {
    pub snapname: String,
    pub start: Option<bool>,
}

impl QemuSnapshotRollbackRequest {
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
pub struct QemuCloneRequest {
    pub newid: u32,
    pub name: Option<String>,
    pub target: Option<String>,
    pub storage: Option<String>,
    pub full: Option<bool>,
    pub pool: Option<String>,
    pub snapname: Option<String>,
    pub bwlimit: Option<u64>,
    pub format: Option<String>,
    pub description: Option<String>,
    pub extra: PveParams,
}

impl QemuCloneRequest {
    pub fn new(newid: u32) -> Self {
        Self {
            newid,
            name: None,
            target: None,
            storage: None,
            full: None,
            pool: None,
            snapname: None,
            bwlimit: None,
            format: None,
            description: None,
            extra: PveParams::new(),
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("newid", self.newid.to_string());
        params.insert_opt("name", self.name.clone());
        params.insert_opt("target", self.target.clone());
        params.insert_opt("storage", self.storage.clone());
        if let Some(full) = self.full {
            params.insert_bool("full", full);
        }
        params.insert_opt("pool", self.pool.clone());
        params.insert_opt("snapname", self.snapname.clone());
        params.insert_opt("bwlimit", self.bwlimit.map(|v| v.to_string()));
        params.insert_opt("format", self.format.clone());
        params.insert_opt("description", self.description.clone());
        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone)]
pub struct QemuMigrateRequest {
    pub target: String,
    pub online: Option<bool>,
    pub with_local_disks: Option<bool>,
    pub targetstorage: Option<String>,
    pub migration_network: Option<String>,
    pub migration_type: Option<String>,
    pub bwlimit: Option<u64>,
    pub force: Option<bool>,
    pub with_conntrack_state: Option<bool>,
    pub extra: PveParams,
}

impl QemuMigrateRequest {
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            online: None,
            with_local_disks: None,
            targetstorage: None,
            migration_network: None,
            migration_type: None,
            bwlimit: None,
            force: None,
            with_conntrack_state: None,
            extra: PveParams::new(),
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("target", self.target.clone());
        if let Some(online) = self.online {
            params.insert_bool("online", online);
        }
        if let Some(with_local_disks) = self.with_local_disks {
            params.insert_bool("with-local-disks", with_local_disks);
        }
        if let Some(force) = self.force {
            params.insert_bool("force", force);
        }
        if let Some(with_conntrack_state) = self.with_conntrack_state {
            params.insert_bool("with-conntrack-state", with_conntrack_state);
        }
        params.insert_opt("targetstorage", self.targetstorage.clone());
        params.insert_opt("migration_network", self.migration_network.clone());
        params.insert_opt("migration_type", self.migration_type.clone());
        params.insert_opt("bwlimit", self.bwlimit.map(|v| v.to_string()));
        params.extend(&self.extra);
        params
    }
}

#[cfg(test)]
mod tests {
    use super::{QemuCreateRequest, QemuMigrateRequest};

    #[test]
    fn qemu_create_maps_bool_and_required_fields() {
        let mut req = QemuCreateRequest::new(220);
        req.onboot = Some(true);
        req.name = Some("demo".to_string());

        let params = req.to_params();
        assert_eq!(params.get("vmid"), Some("220"));
        assert_eq!(params.get("onboot"), Some("1"));
        assert_eq!(params.get("name"), Some("demo"));
    }

    #[test]
    fn qemu_migrate_maps_target_and_flags() {
        let mut req = QemuMigrateRequest::new("pve2");
        req.online = Some(true);
        req.with_local_disks = Some(false);
        let params = req.to_params();

        assert_eq!(params.get("target"), Some("pve2"));
        assert_eq!(params.get("online"), Some("1"));
        assert_eq!(params.get("with-local-disks"), Some("0"));
    }
}
