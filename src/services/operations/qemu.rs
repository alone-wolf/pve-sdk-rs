use reqwest::Method;
use serde_json::Value;

use crate::client::PveClient;
use crate::core::transport::enc;
use crate::error::PveError;
use crate::models::{QemuStatus, QemuVmSummary, SnapshotInfo};
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn qemu_list(
        &self,
        node: &str,
        full: Option<bool>,
    ) -> Result<Vec<QemuVmSummary>, PveError> {
        let mut query = PveParams::new();
        if let Some(full) = full {
            query.insert_bool("full", full);
        }

        let path = format!("/nodes/{}/qemu", enc(node));
        self.send(Method::GET, &path, Some(&query), None).await
    }

    pub async fn qemu_create(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("vmid", vmid.to_string());
        let path = format!("/nodes/{}/qemu", enc(node));
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn qemu_create_with(
        &self,
        node: &str,
        request: &requests::QemuCreateRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/qemu", enc(node));
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn qemu_config(
        &self,
        node: &str,
        vmid: u32,
        current: Option<bool>,
        snapshot: Option<&str>,
    ) -> Result<Value, PveError> {
        let mut query = PveParams::new();
        if let Some(current) = current {
            query.insert_bool("current", current);
        }
        query.insert_opt("snapshot", snapshot);

        let path = format!("/nodes/{}/qemu/{}/config", enc(node), vmid);
        self.send(Method::GET, &path, Some(&query), None).await
    }

    pub async fn qemu_config_with(
        &self,
        node: &str,
        vmid: u32,
        query: &requests::QemuConfigQuery,
    ) -> Result<Value, PveError> {
        let params = query.to_params();
        let path = format!("/nodes/{}/qemu/{}/config", enc(node), vmid);
        self.send(Method::GET, &path, Some(&params), None).await
    }

    pub async fn qemu_set_config_async(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let path = format!("/nodes/{}/qemu/{}/config", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(params)).await
    }

    pub async fn qemu_set_config_async_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSetConfigRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_set_config_async(node, vmid, &params).await
    }

    pub async fn qemu_set_config_sync(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/nodes/{}/qemu/{}/config", enc(node), vmid);
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn qemu_set_config_sync_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSetConfigRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.qemu_set_config_sync(node, vmid, &params).await
    }

    pub async fn qemu_status(&self, node: &str, vmid: u32) -> Result<QemuStatus, PveError> {
        let path = format!("/nodes/{}/qemu/{}/status/current", enc(node), vmid);
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn qemu_start(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.qemu_action(node, vmid, "start", params).await
    }

    pub async fn qemu_start_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_start(node, vmid, &params).await
    }

    pub async fn qemu_shutdown(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.qemu_action(node, vmid, "shutdown", params).await
    }

    pub async fn qemu_shutdown_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_shutdown(node, vmid, &params).await
    }

    pub async fn qemu_stop(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.qemu_action(node, vmid, "stop", params).await
    }

    pub async fn qemu_stop_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_stop(node, vmid, &params).await
    }

    pub async fn qemu_reboot(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.qemu_action(node, vmid, "reboot", params).await
    }

    pub async fn qemu_reboot_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_reboot(node, vmid, &params).await
    }

    pub async fn qemu_suspend(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.qemu_action(node, vmid, "suspend", params).await
    }

    pub async fn qemu_suspend_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_suspend(node, vmid, &params).await
    }

    pub async fn qemu_resume(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.qemu_action(node, vmid, "resume", params).await
    }

    pub async fn qemu_resume_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_resume(node, vmid, &params).await
    }

    pub async fn qemu_snapshots(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<SnapshotInfo>, PveError> {
        let path = format!("/nodes/{}/qemu/{}/snapshot", enc(node), vmid);
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn qemu_snapshot_create(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("snapname", snapname);
        let path = format!("/nodes/{}/qemu/{}/snapshot", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn qemu_snapshot_create_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSnapshotCreateRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/qemu/{}/snapshot", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn qemu_snapshot_rollback(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let path = format!(
            "/nodes/{}/qemu/{}/snapshot/{}/rollback",
            enc(node),
            vmid,
            enc(snapname)
        );
        self.send(Method::POST, &path, None, Some(params)).await
    }

    pub async fn qemu_snapshot_rollback_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSnapshotRollbackRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.qemu_snapshot_rollback(node, vmid, &request.snapname, &params)
            .await
    }

    pub async fn qemu_clone(
        &self,
        node: &str,
        vmid: u32,
        newid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("newid", newid.to_string());
        let path = format!("/nodes/{}/qemu/{}/clone", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn qemu_clone_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuCloneRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/qemu/{}/clone", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn qemu_migrate(
        &self,
        node: &str,
        vmid: u32,
        target: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("target", target);
        let path = format!("/nodes/{}/qemu/{}/migrate", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn qemu_migrate_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuMigrateRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/qemu/{}/migrate", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }
    async fn qemu_action(
        &self,
        node: &str,
        vmid: u32,
        action: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let path = format!("/nodes/{}/qemu/{}/status/{}", enc(node), vmid, action);
        self.send(Method::POST, &path, None, Some(params)).await
    }
}
