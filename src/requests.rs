use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use crate::params::PveParams;

#[derive(Debug, Clone)]
pub struct TicketRequest {
    pub username: String,
    pub password: String,
    pub otp: Option<String>,
    pub realm: Option<String>,
    pub tfa_challenge: Option<String>,
}

impl TicketRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            otp: None,
            realm: None,
            tfa_challenge: None,
        }
    }

    pub fn all(
        username: impl Into<String>,
        password: impl Into<String>,
        otp: Option<String>,
        realm: Option<String>,
        tfa_challenge: Option<String>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            otp,
            realm,
            tfa_challenge,
        }
    }

    pub fn otp(mut self, otp: impl Into<String>) -> Self {
        self.otp = Some(otp.into());
        self
    }

    pub fn realm(mut self, realm: impl Into<String>) -> Self {
        self.realm = Some(realm.into());
        self
    }

    pub fn tfa_challenge(mut self, tfa_challenge: impl Into<String>) -> Self {
        self.tfa_challenge = Some(tfa_challenge.into());
        self
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("username", self.username.clone());
        params.insert("password", self.password.clone());
        params.insert_opt("otp", self.otp.clone());
        params.insert_opt("realm", self.realm.clone());
        params.insert_opt("tfa-challenge", self.tfa_challenge.clone());
        params
    }
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

#[derive(Debug, Clone, Default)]
pub struct NodeStorageQuery {
    pub content: Option<String>,
    pub enabled: Option<bool>,
    pub format: Option<bool>,
    pub storage: Option<String>,
    pub target: Option<String>,
}

impl NodeStorageQuery {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("content", self.content.clone());
        if let Some(enabled) = self.enabled {
            params.insert_bool("enabled", enabled);
        }
        if let Some(format) = self.format {
            params.insert_bool("format", format);
        }
        params.insert_opt("storage", self.storage.clone());
        params.insert_opt("target", self.target.clone());
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct StorageContentQuery {
    pub content: Option<String>,
    pub vmid: Option<u32>,
}

impl StorageContentQuery {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("content", self.content.clone());
        params.insert_opt("vmid", self.vmid.map(|v| v.to_string()));
        params
    }
}

#[derive(Debug, Clone)]
pub struct StorageAllocateDiskRequest {
    pub vmid: u32,
    pub filename: String,
    pub size: String,
    pub format: Option<String>,
    pub extra: PveParams,
}

impl StorageAllocateDiskRequest {
    pub fn new(vmid: u32, filename: impl Into<String>, size: impl Into<String>) -> Self {
        Self {
            vmid,
            filename: filename.into(),
            size: size.into(),
            format: None,
            extra: PveParams::new(),
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("vmid", self.vmid.to_string());
        params.insert("filename", self.filename.clone());
        params.insert("size", self.size.clone());
        params.insert_opt("format", self.format.clone());
        params.extend(&self.extra);
        params
    }
}

#[derive(Debug, Clone)]
pub struct StorageUploadRequest {
    pub content: String,
    pub file_path: PathBuf,
    pub checksum: Option<String>,
    pub checksum_algorithm: Option<String>,
}

impl StorageUploadRequest {
    pub fn new(content: impl Into<String>, file_path: impl Into<PathBuf>) -> Self {
        Self {
            content: content.into(),
            file_path: file_path.into(),
            checksum: None,
            checksum_algorithm: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StorageDeleteVolumeRequest {
    pub delay: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
pub enum VzdumpMode {
    Snapshot,
    Suspend,
    Stop,
}

impl VzdumpMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Suspend => "suspend",
            Self::Stop => "stop",
        }
    }
}

impl fmt::Display for VzdumpMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VzdumpCompress {
    None,
    Gzip,
    Lzo,
    Zstd,
}

impl VzdumpCompress {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Gzip => "gzip",
            Self::Lzo => "lzo",
            Self::Zstd => "zstd",
        }
    }
}

impl fmt::Display for VzdumpCompress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MailNotification {
    Always,
    Failure,
}

impl MailNotification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Failure => "failure",
        }
    }
}

impl fmt::Display for MailNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub struct VzdumpRequest {
    pub all: Option<bool>,
    pub vmid: Option<String>,
    pub mode: Option<VzdumpMode>,
    pub storage: Option<String>,
    pub compress: Option<VzdumpCompress>,
    pub mailnotification: Option<MailNotification>,
    pub mailto: Option<String>,
    pub notes_template: Option<String>,
    pub remove: Option<bool>,
    pub stopwait: Option<u64>,
    pub extra: PveParams,
}

impl VzdumpRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();

        if let Some(all) = self.all {
            params.insert_bool("all", all);
        }

        params.insert_opt("vmid", self.vmid.clone());
        params.insert_opt("mode", self.mode.map(|v| v.to_string()));
        params.insert_opt("storage", self.storage.clone());
        params.insert_opt("compress", self.compress.map(|v| v.to_string()));
        params.insert_opt(
            "mailnotification",
            self.mailnotification.map(|v| v.to_string()),
        );
        params.insert_opt("mailto", self.mailto.clone());
        params.insert_opt("notes-template", self.notes_template.clone());
        if let Some(remove) = self.remove {
            params.insert_bool("remove", remove);
        }
        params.insert_opt("stopwait", self.stopwait.map(|v| v.to_string()));

        params.extend(&self.extra);
        params
    }
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
