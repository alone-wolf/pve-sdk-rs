use reqwest::Method;
use serde_json::Value;

use crate::client::PveClient;
use crate::core::transport::enc;
use crate::error::PveError;
use crate::models::{LxcStatus, LxcSummary, SnapshotInfo};
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn lxc_list(&self, node: &str) -> Result<Vec<LxcSummary>, PveError> {
        let path = format!("/nodes/{}/lxc", enc(node));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn lxc_create(
        &self,
        node: &str,
        vmid: u32,
        ostemplate: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("vmid", vmid.to_string());
        body.insert("ostemplate", ostemplate);
        let path = format!("/nodes/{}/lxc", enc(node));
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn lxc_create_with(
        &self,
        node: &str,
        request: &requests::LxcCreateRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/lxc", enc(node));
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn lxc_config(
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

        let path = format!("/nodes/{}/lxc/{}/config", enc(node), vmid);
        self.send(Method::GET, &path, Some(&query), None).await
    }

    pub async fn lxc_config_with(
        &self,
        node: &str,
        vmid: u32,
        query: &requests::LxcConfigQuery,
    ) -> Result<Value, PveError> {
        let params = query.to_params();
        let path = format!("/nodes/{}/lxc/{}/config", enc(node), vmid);
        self.send(Method::GET, &path, Some(&params), None).await
    }

    pub async fn lxc_set_config(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/nodes/{}/lxc/{}/config", enc(node), vmid);
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn lxc_set_config_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcSetConfigRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.lxc_set_config(node, vmid, &params).await
    }

    pub async fn lxc_status(&self, node: &str, vmid: u32) -> Result<LxcStatus, PveError> {
        let path = format!("/nodes/{}/lxc/{}/status/current", enc(node), vmid);
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn lxc_start(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.lxc_action(node, vmid, "start", params).await
    }

    pub async fn lxc_start_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.lxc_start(node, vmid, &params).await
    }

    pub async fn lxc_shutdown(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.lxc_action(node, vmid, "shutdown", params).await
    }

    pub async fn lxc_shutdown_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.lxc_shutdown(node, vmid, &params).await
    }

    pub async fn lxc_stop(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.lxc_action(node, vmid, "stop", params).await
    }

    pub async fn lxc_stop_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.lxc_stop(node, vmid, &params).await
    }

    pub async fn lxc_reboot(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.lxc_action(node, vmid, "reboot", params).await
    }

    pub async fn lxc_reboot_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.lxc_reboot(node, vmid, &params).await
    }

    pub async fn lxc_snapshots(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<Vec<SnapshotInfo>, PveError> {
        let path = format!("/nodes/{}/lxc/{}/snapshot", enc(node), vmid);
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn lxc_snapshot_create(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("snapname", snapname);
        let path = format!("/nodes/{}/lxc/{}/snapshot", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn lxc_snapshot_create_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcSnapshotCreateRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/lxc/{}/snapshot", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn lxc_snapshot_rollback(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let path = format!(
            "/nodes/{}/lxc/{}/snapshot/{}/rollback",
            enc(node),
            vmid,
            enc(snapname)
        );
        self.send(Method::POST, &path, None, Some(params)).await
    }

    pub async fn lxc_snapshot_rollback_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcSnapshotRollbackRequest,
    ) -> Result<String, PveError> {
        let params = request.to_params();
        self.lxc_snapshot_rollback(node, vmid, &request.snapname, &params)
            .await
    }

    pub async fn lxc_migrate(
        &self,
        node: &str,
        vmid: u32,
        target: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let mut body = params.clone();
        body.insert("target", target);
        let path = format!("/nodes/{}/lxc/{}/migrate", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn lxc_migrate_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcMigrateRequest,
    ) -> Result<String, PveError> {
        let body = request.to_params();
        let path = format!("/nodes/{}/lxc/{}/migrate", enc(node), vmid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }
    async fn lxc_action(
        &self,
        node: &str,
        vmid: u32,
        action: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        let path = format!("/nodes/{}/lxc/{}/status/{}", enc(node), vmid, action);
        self.send(Method::POST, &path, None, Some(params)).await
    }
}
