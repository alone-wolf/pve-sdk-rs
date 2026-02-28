//! Async Proxmox VE (PVE) SDK.
//!
//! # Quick start
//! ```no_run
//! use pve_sdk_rs::types;
//! use pve_sdk_rs::{ClientAuth, ClientOption};
//! use std::time::Duration;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ClientOption::new("pve.example.com")
//!     .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
//!     .build()
//!     .await?;
//!
//! let nodes = client.node().list().await?;
//! let tasks = client
//!     .node()
//!     .tasks_with("pve1", &types::node::NodeTasksQuery::default())
//!     .await?;
//!
//! let mut create = types::qemu::QemuCreateRequest::new(220);
//! create.name = Some("demo-220".to_string());
//! create.memory = Some(4096);
//! create.cores = Some(2);
//! create.net0 = Some("virtio,bridge=vmbr0".to_string());
//! create.scsi0 = Some("local-lvm:32".to_string());
//! let upid = client.qemu().create_with("pve1", &create).await?;
//! let _ = client
//!     .task()
//!     .wait_with_options(
//!         "pve1",
//!         &upid,
//!         &types::task::WaitTaskOptions {
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
mod client_api;
mod client_option;
mod core;
mod error;
mod models;
mod params;
mod requests;
mod services;
pub mod types;

pub use client::{Auth, PveClient};
pub use client_api::{
    AccessApi, BackupApi, ClusterApi, DatacenterApi, LxcApi, NodeApi, QemuApi, RawApi, StorageApi,
    TaskApi,
};
pub use client_option::{ClientAuth, ClientOption};
pub use error::PveError;
pub use params::PveParams;
