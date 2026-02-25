//! Domain-grouped request/response types.
//!
//! This module introduces namespaced type access (for example `types::qemu::QemuCreateRequest`)
//! while keeping existing flat exports at crate root for backward compatibility.

pub mod access;
pub mod backup;
pub mod cluster;
pub mod common;
pub mod datacenter;
pub mod lxc;
pub mod node;
pub mod qemu;
pub mod storage;
pub mod task;
