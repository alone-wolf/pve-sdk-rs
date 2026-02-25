//! Backward-compatible model re-exports.
//!
//! New code should prefer domain-grouped modules under `crate::types`.

pub use crate::types::access::{
    AccessAcl, AccessGroup, AccessRole, AccessUser, AccessUserToken, TicketInfo,
};
pub use crate::types::cluster::{ClusterResource, ClusterStatusItem};
pub(crate) use crate::types::common::ApiEnvelope;
pub use crate::types::common::{SnapshotInfo, VersionInfo};
pub use crate::types::datacenter::DatacenterConfig;
pub use crate::types::lxc::{LxcStatus, LxcSummary};
pub use crate::types::node::{NetworkInterface, NodeSummary, NodeTask};
pub use crate::types::qemu::{QemuStatus, QemuVmSummary};
pub use crate::types::storage::{NodeStorageStatus, StorageContentItem, StorageIndexItem};
pub use crate::types::task::{TaskLogLine, TaskStatus};
