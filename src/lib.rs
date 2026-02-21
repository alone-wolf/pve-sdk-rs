//! Async Proxmox VE (PVE) SDK.
//!
//! # Quick start
//! ```no_run
//! use pve_sdk_rs::{ClientOption, NodeTasksQuery, QemuCreateRequest, WaitTaskOptions};
//! use std::time::Duration;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ClientOption::new("pve.example.com")
//!     .api_token("root@pam!ci=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
//!     .build()
//!     .await?;
//!
//! let nodes = client.nodes().await?;
//! let tasks = client.node_tasks_with("pve1", &NodeTasksQuery::default()).await?;
//!
//! let mut create = QemuCreateRequest::new(220);
//! create.name = Some("demo-220".to_string());
//! create.memory = Some(4096);
//! create.cores = Some(2);
//! create.net0 = Some("virtio,bridge=vmbr0".to_string());
//! create.scsi0 = Some("local-lvm:32".to_string());
//! let upid = client.qemu_create_with("pve1", &create).await?;
//! let _ = client
//!     .wait_for_task_with_options(
//!         "pve1",
//!         &upid,
//!         &WaitTaskOptions {
//!             poll_interval: Duration::from_secs(2),
//!             timeout: Some(Duration::from_secs(600)),
//!         },
//!     )
//!     .await?;
//! println!("nodes={}, tasks={}", nodes.len(), tasks.len());
//! # Ok(())
//! # }
//! ```

mod client;
mod client_option;
mod error;
mod models;
mod params;
mod requests;

pub use client::{Auth, PveClient};
pub use client_option::{ClientAuth, ClientOption};
pub use error::PveError;
pub use models::{
    ClusterResource, ClusterStatusItem, LxcStatus, LxcSummary, NetworkInterface, NodeStorageStatus,
    NodeSummary, NodeTask, QemuStatus, QemuVmSummary, SnapshotInfo, StorageContentItem,
    StorageIndexItem, TaskLogLine, TaskStatus, TicketInfo, VersionInfo,
};
pub use params::PveParams;
pub use requests::*;
