//! Backward-compatible request/query re-exports.
//!
//! New code should prefer domain-grouped modules under `crate::types`.

pub use crate::types::access::{
    AccessAclQuery, AccessCreateGroupRequest, AccessCreateTokenRequest, AccessCreateUserRequest,
    AccessDeleteAclRequest, AccessSetAclRequest, AccessUpdateGroupRequest,
    AccessUpdateTokenRequest, AccessUpdateUserRequest, TicketRequest,
};
pub use crate::types::backup::{MailNotification, VzdumpCompress, VzdumpMode, VzdumpRequest};
pub use crate::types::cluster::{ClusterResourceType, ClusterResourcesQuery};
pub use crate::types::datacenter::DatacenterConfigUpdateRequest;
pub use crate::types::lxc::{
    LxcActionRequest, LxcConfigQuery, LxcCreateRequest, LxcMigrateRequest, LxcSetConfigRequest,
    LxcSnapshotCreateRequest, LxcSnapshotRollbackRequest,
};
pub use crate::types::node::{NodeNetworkQuery, NodeTasksQuery, TaskSource};
pub use crate::types::qemu::{
    QemuActionRequest, QemuBios, QemuCloneRequest, QemuConfigQuery, QemuCreateRequest,
    QemuMigrateRequest, QemuOsType, QemuSetConfigRequest, QemuSnapshotCreateRequest,
    QemuSnapshotRollbackRequest,
};
pub use crate::types::storage::{
    NodeStorageQuery, StorageAllocateDiskRequest, StorageContentQuery, StorageDeleteVolumeRequest,
    StorageUploadRequest,
};
pub use crate::types::task::{TaskLogQuery, WaitTaskOptions};
