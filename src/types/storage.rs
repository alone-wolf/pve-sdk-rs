//! Storage related request/response types.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;

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
