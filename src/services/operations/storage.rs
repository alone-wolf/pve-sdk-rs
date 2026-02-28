use std::path::Path;

use reqwest::{Method, multipart};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::client::PveClient;
use crate::core::transport::enc;
use crate::error::PveError;
use crate::models::{NodeStorageStatus, StorageContentItem, StorageIndexItem};
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn storage_index(
        &self,
        storage_type: Option<&str>,
    ) -> Result<Vec<StorageIndexItem>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("type", storage_type);
        self.send(Method::GET, "/storage", Some(&query), None).await
    }

    pub async fn node_storage(
        &self,
        node: &str,
        query: &PveParams,
    ) -> Result<Vec<NodeStorageStatus>, PveError> {
        let path = format!("/nodes/{}/storage", enc(node));
        self.send(Method::GET, &path, Some(query), None).await
    }

    pub async fn node_storage_with(
        &self,
        node: &str,
        query: &requests::NodeStorageQuery,
    ) -> Result<Vec<NodeStorageStatus>, PveError> {
        let params = query.to_params();
        self.node_storage(node, &params).await
    }

    pub async fn storage_content(
        &self,
        node: &str,
        storage: &str,
        query: &PveParams,
    ) -> Result<Vec<StorageContentItem>, PveError> {
        let path = format!("/nodes/{}/storage/{}/content", enc(node), enc(storage));
        self.send(Method::GET, &path, Some(query), None).await
    }

    pub async fn storage_content_with(
        &self,
        node: &str,
        storage: &str,
        query: &requests::StorageContentQuery,
    ) -> Result<Vec<StorageContentItem>, PveError> {
        let params = query.to_params();
        self.storage_content(node, storage, &params).await
    }

    pub async fn storage_allocate_disk(
        &self,
        node: &str,
        storage: &str,
        vmid: u32,
        filename: &str,
        size: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("vmid", vmid.to_string());
        body.insert("filename", filename);
        body.insert("size", size);

        let path = format!("/nodes/{}/storage/{}/content", enc(node), enc(storage));
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn storage_allocate_disk_with(
        &self,
        node: &str,
        storage: &str,
        request: &requests::StorageAllocateDiskRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/storage/{}/content", enc(node), enc(storage));
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn storage_upload_file(
        &self,
        node: &str,
        storage: &str,
        content: &str,
        file_path: impl AsRef<Path>,
        checksum: Option<&str>,
        checksum_algorithm: Option<&str>,
    ) -> Result<String, PveError> {
        let file_path = file_path.as_ref();
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| PveError::InvalidArgument("invalid upload file path".into()))?;

        let file_size = tokio::fs::metadata(file_path).await?.len();
        let file = File::open(file_path).await?;
        let stream = ReaderStream::new(file);
        let body = reqwest::Body::wrap_stream(stream);
        let file_part =
            multipart::Part::stream_with_length(body, file_size).file_name(file_name.to_string());
        let mut form = multipart::Form::new()
            .text("content", content.to_string())
            .part("filename", file_part);

        if let Some(checksum) = checksum {
            form = form.text("checksum", checksum.to_string());
        }
        if let Some(checksum_algorithm) = checksum_algorithm {
            form = form.text("checksum-algorithm", checksum_algorithm.to_string());
        }

        let path = format!("/nodes/{}/storage/{}/upload", enc(node), enc(storage));
        self.send_multipart(Method::POST, &path, form).await
    }

    pub async fn storage_upload_form(
        &self,
        node: &str,
        storage: &str,
        form: multipart::Form,
    ) -> Result<String, PveError> {
        let path = format!("/nodes/{}/storage/{}/upload", enc(node), enc(storage));
        self.send_multipart(Method::POST, &path, form).await
    }

    pub async fn storage_upload_with(
        &self,
        node: &str,
        storage: &str,
        request: &requests::StorageUploadRequest,
    ) -> Result<String, PveError> {
        self.storage_upload_file(
            node,
            storage,
            &request.content,
            &request.file_path,
            request.checksum.as_deref(),
            request.checksum_algorithm.as_deref(),
        )
        .await
    }

    pub async fn storage_delete_volume(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        delay: Option<u32>,
    ) -> Result<String, PveError> {
        let path = format!(
            "/nodes/{}/storage/{}/content/{}",
            enc(node),
            enc(storage),
            enc(volume)
        );

        let mut query = PveParams::new();
        query.insert_opt("delay", delay.map(|d| d.to_string()));

        self.send(Method::DELETE, &path, Some(&query), None).await
    }

    pub async fn storage_delete_volume_with(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        request: &requests::StorageDeleteVolumeRequest,
    ) -> Result<String, PveError> {
        self.storage_delete_volume(node, storage, volume, request.delay)
            .await
    }

    pub async fn vzdump_backup(&self, node: &str, params: &PveParams) -> Result<String, PveError> {
        let path = format!("/nodes/{}/vzdump", enc(node));
        self.send(Method::POST, &path, None, Some(params)).await
    }

    pub async fn vzdump_backup_with(
        &self,
        node: &str,
        request: &requests::VzdumpRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.vzdump_backup(node, &params).await
    }
}
