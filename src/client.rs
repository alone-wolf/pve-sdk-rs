use std::path::Path;
use std::time::Duration;

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::header::{AUTHORIZATION, COOKIE, HeaderValue};
use reqwest::{Method, RequestBuilder, multipart};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::fs::File;
use tokio::time::{Instant, sleep};
use tokio_util::io::ReaderStream;
use url::Url;

use crate::client_option::{ClientAuth, ClientOption, validate_api_token_format};
use crate::error::PveError;
use crate::models::{
    ApiEnvelope, ClusterResource, ClusterStatusItem, LxcStatus, LxcSummary, NetworkInterface,
    NodeStorageStatus, NodeSummary, NodeTask, QemuStatus, QemuVmSummary, SnapshotInfo,
    StorageContentItem, StorageIndexItem, TaskLogLine, TaskStatus, TicketInfo, VersionInfo,
};
use crate::params::PveParams;
use crate::requests;

#[derive(Debug, Clone)]
pub enum Auth {
    None,
    ApiToken(String),
    Ticket {
        ticket: String,
        csrf: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct PveClient {
    base_url: Url,
    http: reqwest::Client,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    auth: Auth,
}

impl PveClient {
    pub const DEFAULT_PORT: u16 = 8006;

    pub async fn from_option(option: ClientOption) -> Result<Self, PveError> {
        let parsed = build_base_url(&option.host, option.port, option.https)?;
        let http = build_http_client(option.insecure_tls, option.timeout, option.connect_timeout)?;
        let mut client = Self {
            base_url: parsed,
            http,
            timeout: option.timeout,
            connect_timeout: option.connect_timeout,
            auth: Auth::None,
        };

        match option.auth {
            ClientAuth::None => {}
            ClientAuth::ApiToken(token) => {
                validate_api_token_format(&token)?;
                client.auth = Auth::ApiToken(token);
            }
            ClientAuth::ApiTokenPartial {
                user,
                realm,
                token_id: tokenid,
                secret,
            } => {
                let user = user.trim();
                let realm = realm.trim();
                let tokenid = tokenid.trim();
                let secret = secret.trim();
                if user.is_empty() || realm.is_empty() || tokenid.is_empty() || secret.is_empty() {
                    return Err(PveError::InvalidArgument(
                        "api token partial fields must be non-empty".to_string(),
                    ));
                }
                let token = format!("{user}@{realm}!{tokenid}={secret}");
                validate_api_token_format(&token)?;
                client.auth = Auth::ApiToken(token);
            }
            ClientAuth::Ticket { ticket, csrf } => {
                client.auth = Auth::Ticket { ticket, csrf };
            }
            ClientAuth::Password {
                username,
                password,
                otp,
                realm,
                tfa_challenge,
            } => {
                let ticket = client
                    .request_ticket(
                        &username,
                        &password,
                        otp.as_deref(),
                        realm.as_deref(),
                        tfa_challenge.as_deref(),
                    )
                    .await?;
                client.auth = Auth::Ticket {
                    ticket: ticket.ticket,
                    csrf: Some(ticket.csrf_prevention_token),
                };
            }
        }

        Ok(client)
    }

    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    pub fn set_auth(&mut self, auth: Auth) {
        self.auth = auth;
    }

    pub fn set_tls_insecure(self, insecure: bool) -> Result<Self, PveError> {
        let http = build_http_client(insecure, self.timeout, self.connect_timeout)?;
        Ok(Self { http, ..self })
    }

    pub async fn connect(&self) -> Result<(), PveError> {
        let _: VersionInfo = self.connect_with_version().await?;
        Ok(())
    }

    pub async fn connect_with_version(&self) -> Result<VersionInfo, PveError> {
        self.version().await
    }

    pub async fn request_ticket(
        &self,
        username: &str,
        password: &str,
        otp: Option<&str>,
        realm: Option<&str>,
        tfa_challenge: Option<&str>,
    ) -> Result<TicketInfo, PveError> {
        let mut params = PveParams::new();
        params.insert("username", username);
        params.insert("password", password);
        params.insert_opt("otp", otp);
        params.insert_opt("realm", realm);
        params.insert_opt("tfa-challenge", tfa_challenge);

        self.send(Method::POST, "/access/ticket", None, Some(&params))
            .await
    }

    pub async fn request_ticket_with(
        &self,
        request: &requests::TicketRequest,
    ) -> Result<TicketInfo, PveError> {
        let params = request.to_params();
        self.send(Method::POST, "/access/ticket", None, Some(&params))
            .await
    }

    pub async fn version(&self) -> Result<VersionInfo, PveError> {
        self.send(Method::GET, "/version", None, None).await
    }

    pub async fn nodes(&self) -> Result<Vec<NodeSummary>, PveError> {
        self.send(Method::GET, "/nodes", None, None).await
    }

    pub async fn cluster_status(&self) -> Result<Vec<ClusterStatusItem>, PveError> {
        self.send(Method::GET, "/cluster/status", None, None).await
    }

    pub async fn cluster_resources(
        &self,
        resource_type: Option<&str>,
    ) -> Result<Vec<ClusterResource>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("type", resource_type);
        self.send(Method::GET, "/cluster/resources", Some(&query), None)
            .await
    }

    pub async fn cluster_resources_with(
        &self,
        query: &requests::ClusterResourcesQuery,
    ) -> Result<Vec<ClusterResource>, PveError> {
        let params = query.to_params();
        self.send(Method::GET, "/cluster/resources", Some(&params), None)
            .await
    }

    pub async fn cluster_next_id(&self, vmid: Option<u32>) -> Result<u32, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("vmid", vmid.map(|v| v.to_string()));
        self.send(Method::GET, "/cluster/nextid", Some(&query), None)
            .await
    }

    pub async fn node_status(&self, node: &str) -> Result<Value, PveError> {
        let path = format!("/nodes/{}/status", enc(node));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn node_tasks(
        &self,
        node: &str,
        query: &PveParams,
    ) -> Result<Vec<NodeTask>, PveError> {
        let path = format!("/nodes/{}/tasks", enc(node));
        self.send(Method::GET, &path, Some(query), None).await
    }

    pub async fn node_tasks_with(
        &self,
        node: &str,
        query: &requests::NodeTasksQuery,
    ) -> Result<Vec<NodeTask>, PveError> {
        let params = query.to_params();
        self.node_tasks(node, &params).await
    }

    pub async fn node_network(
        &self,
        node: &str,
        interface_type: Option<&str>,
    ) -> Result<Vec<NetworkInterface>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("type", interface_type);
        let path = format!("/nodes/{}/network", enc(node));
        self.send(Method::GET, &path, Some(&query), None).await
    }

    pub async fn node_network_with(
        &self,
        node: &str,
        query: &requests::NodeNetworkQuery,
    ) -> Result<Vec<NetworkInterface>, PveError> {
        let params = query.to_params();
        let path = format!("/nodes/{}/network", enc(node));
        self.send(Method::GET, &path, Some(&params), None).await
    }

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

    pub async fn task_status(&self, node: &str, upid: &str) -> Result<TaskStatus, PveError> {
        let path = format!("/nodes/{}/tasks/{}/status", enc(node), enc(upid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn task_log(
        &self,
        node: &str,
        upid: &str,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<TaskLogLine>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("start", start.map(|v| v.to_string()));
        query.insert_opt("limit", limit.map(|v| v.to_string()));

        let path = format!("/nodes/{}/tasks/{}/log", enc(node), enc(upid));
        self.send(Method::GET, &path, Some(&query), None).await
    }

    pub async fn task_log_with(
        &self,
        node: &str,
        upid: &str,
        query: &requests::TaskLogQuery,
    ) -> Result<Vec<TaskLogLine>, PveError> {
        let params = query.to_params();
        let path = format!("/nodes/{}/tasks/{}/log", enc(node), enc(upid));
        self.send(Method::GET, &path, Some(&params), None).await
    }

    pub async fn wait_for_task(
        &self,
        node: &str,
        upid: &str,
        poll_interval: Duration,
        timeout: Option<Duration>,
    ) -> Result<TaskStatus, PveError> {
        let started = Instant::now();

        loop {
            let status = self.task_status(node, upid).await?;
            if status.status == "stopped" {
                if status.exitstatus.as_deref() == Some("OK") {
                    return Ok(status);
                }

                return Err(PveError::TaskFailed {
                    upid: upid.to_string(),
                    exitstatus: status
                        .exitstatus
                        .clone()
                        .unwrap_or_else(|| "UNKNOWN".to_string()),
                });
            }

            if let Some(timeout) = timeout
                && started.elapsed() > timeout
            {
                return Err(PveError::TaskTimeout {
                    upid: upid.to_string(),
                    timeout_secs: timeout.as_secs(),
                });
            }

            sleep(poll_interval).await;
        }
    }

    pub async fn wait_for_task_with_options(
        &self,
        node: &str,
        upid: &str,
        options: &requests::WaitTaskOptions,
    ) -> Result<TaskStatus, PveError> {
        self.wait_for_task(node, upid, options.poll_interval, options.timeout)
            .await
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

    async fn send<T>(
        &self,
        method: Method,
        path: &str,
        query: Option<&PveParams>,
        form: Option<&PveParams>,
    ) -> Result<T, PveError>
    where
        T: DeserializeOwned,
    {
        let url = self.url(path)?;
        let mut request = self.http.request(method.clone(), url);

        if let Some(query) = query
            && !query.is_empty()
        {
            request = request.query(&query.0);
        }

        request = self.apply_auth(request, &method)?;

        if let Some(form) = form {
            request = request.form(&form.0);
        }

        self.execute(request).await
    }

    async fn send_multipart<T>(
        &self,
        method: Method,
        path: &str,
        form: multipart::Form,
    ) -> Result<T, PveError>
    where
        T: DeserializeOwned,
    {
        let url = self.url(path)?;
        let request = self.apply_auth(self.http.request(method.clone(), url), &method)?;
        let request = request.multipart(form);
        self.execute(request).await
    }

    async fn execute<T>(&self, request: RequestBuilder) -> Result<T, PveError>
    where
        T: DeserializeOwned,
    {
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(PveError::ApiStatus {
                status: status.as_u16(),
                body,
            });
        }

        let payload: ApiEnvelope<T> = serde_json::from_str(&body)?;
        Ok(payload.data)
    }

    fn apply_auth(
        &self,
        request: RequestBuilder,
        method: &Method,
    ) -> Result<RequestBuilder, PveError> {
        match &self.auth {
            Auth::None => Ok(request),
            Auth::ApiToken(token) => {
                let value = format!("PVEAPIToken={token}");
                Ok(request.header(
                    AUTHORIZATION,
                    HeaderValue::from_str(&value).map_err(|_| {
                        PveError::InvalidArgument("invalid api token header value".to_string())
                    })?,
                ))
            }
            Auth::Ticket { ticket, csrf } => {
                let mut request = request.header(COOKIE, format!("PVEAuthCookie={ticket}"));

                let is_write = matches!(
                    *method,
                    Method::POST | Method::PUT | Method::DELETE | Method::PATCH
                );

                if is_write {
                    let csrf = csrf.as_deref().ok_or(PveError::MissingCsrfToken)?;
                    request = request.header("CSRFPreventionToken", csrf);
                }

                Ok(request)
            }
        }
    }

    fn url(&self, path: &str) -> Result<Url, PveError> {
        let normalized = normalize_api_path(path);
        self.base_url
            .join(normalized.trim_start_matches('/'))
            .map_err(|_| PveError::InvalidBaseUrl(format!("unable to join path: {normalized}")))
    }
}

fn normalize_api_path(path: &str) -> String {
    if path.starts_with("/api2/json") {
        return path.to_string();
    }

    if path.starts_with('/') {
        format!("/api2/json{path}")
    } else {
        format!("/api2/json/{path}")
    }
}

fn enc(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

fn build_base_url(host: &str, port: u16, https: bool) -> Result<Url, PveError> {
    let mut host = host.trim().to_string();
    if host.starts_with("https://") || host.starts_with("http://") {
        let parsed = Url::parse(&host).map_err(|_| {
            PveError::InvalidBaseUrl(
                "invalid host URL, expected a hostname or IP without path/port".to_string(),
            )
        })?;
        if parsed.port().is_some() {
            return Err(PveError::InvalidBaseUrl(
                "host must not include port, use ClientOption::port()".to_string(),
            ));
        }
        if parsed.path() != "/" {
            return Err(PveError::InvalidBaseUrl(
                "host must not include path, use API methods with relative paths".to_string(),
            ));
        }
        if parsed.query().is_some() || parsed.fragment().is_some() {
            return Err(PveError::InvalidBaseUrl(
                "host must not include query or fragment".to_string(),
            ));
        }
        if !parsed.username().is_empty() || parsed.password().is_some() {
            return Err(PveError::InvalidBaseUrl(
                "host must not include credentials".to_string(),
            ));
        }
        host = parsed.host_str().unwrap_or_default().to_string();
    }

    host = host.trim_end_matches('/').to_string();
    if host.contains('/') {
        return Err(PveError::InvalidBaseUrl(
            "host must not include path, use ClientOption::host(\"pve.example.com\")".to_string(),
        ));
    }
    if host.contains('?') || host.contains('#') {
        return Err(PveError::InvalidBaseUrl(
            "host must not include query or fragment".to_string(),
        ));
    }
    if host.contains('@') {
        return Err(PveError::InvalidBaseUrl(
            "host must not include credentials".to_string(),
        ));
    }
    if host.contains("]:") || (host.matches(':').count() == 1 && !host.starts_with('[')) {
        return Err(PveError::InvalidBaseUrl(
            "host must not include port, use ClientOption::port()".to_string(),
        ));
    }

    if host.matches(':').count() > 1 && !host.starts_with('[') && !host.ends_with(']') {
        host = format!("[{host}]");
    }
    if host.is_empty() {
        return Err(PveError::InvalidBaseUrl("host is empty".to_string()));
    }

    let scheme = if https { "https" } else { "http" };
    let base = format!("{scheme}://{host}:{port}/");
    Url::parse(&base).map_err(|_| PveError::InvalidBaseUrl(base))
}

fn build_http_client(
    insecure_tls: bool,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
) -> Result<reqwest::Client, PveError> {
    let mut builder = reqwest::Client::builder().danger_accept_invalid_certs(insecure_tls);
    if let Some(timeout) = timeout {
        builder = builder.timeout(timeout);
    }
    if let Some(connect_timeout) = connect_timeout {
        builder = builder.connect_timeout(connect_timeout);
    }
    builder.build().map_err(PveError::from)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{Auth, PveClient, build_base_url, normalize_api_path};
    use crate::client_option::{ClientAuth, ClientOption};
    use crate::params::PveParams;

    #[test]
    fn normalize_path_adds_api_prefix() {
        assert_eq!(normalize_api_path("/nodes"), "/api2/json/nodes");
        assert_eq!(normalize_api_path("nodes"), "/api2/json/nodes");
        assert_eq!(
            normalize_api_path("/api2/json/version"),
            "/api2/json/version"
        );
    }

    #[test]
    fn build_base_url_from_host_and_port() {
        let url = build_base_url("pve.example.com", 8006, true).expect("must parse");
        assert_eq!(url.as_str(), "https://pve.example.com:8006/");
    }

    #[test]
    fn build_base_url_rejects_embedded_port() {
        let err = build_base_url("pve.example.com:8006", 8006, true).expect_err("must fail");
        assert!(err.to_string().contains("use ClientOption::port()"));
    }

    #[test]
    fn build_base_url_rejects_embedded_path() {
        let err = build_base_url("pve.example.com/api2/json", 8006, true).expect_err("must fail");
        assert!(err.to_string().contains("must not include path"));
    }

    #[test]
    fn build_base_url_supports_ipv6_without_brackets() {
        let url = build_base_url("2001:db8::1", 8006, true).expect("must parse");
        assert_eq!(url.as_str(), "https://[2001:db8::1]:8006/");
    }

    #[tokio::test]
    async fn client_option_chain_builds_client() {
        let client = ClientOption::new("pve.example.com")
            .port(8443)
            .https(false)
            .insecure_tls(false)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(3))
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        assert_eq!(
            client.url("/version").expect("url").as_str(),
            "http://pve.example.com:8443/api2/json/version"
        );
        assert!(matches!(client.auth(), Auth::ApiToken(_)));
    }

    #[tokio::test]
    async fn client_option_all_builds_client() {
        let options = ClientOption::all("pve.example.com", 9443, true, true, ClientAuth::None);
        let client = PveClient::from_option(options).await.expect("must build");
        assert_eq!(
            client.url("/nodes").expect("url").as_str(),
            "https://pve.example.com:9443/api2/json/nodes"
        );
    }

    #[tokio::test]
    async fn client_option_all_with_timeouts_builds_client() {
        let options = ClientOption::all_with_timeouts(
            "pve.example.com",
            9443,
            true,
            true,
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(3)),
            ClientAuth::None,
        );
        let client = PveClient::from_option(options).await.expect("must build");
        assert_eq!(
            client.url("/nodes").expect("url").as_str(),
            "https://pve.example.com:9443/api2/json/nodes"
        );
    }

    #[tokio::test]
    async fn client_option_api_token_partial_builds_token() {
        let client = ClientOption::new("pve.example.com")
            .api_token_partial("root", "pam", "ci", "secret")
            .build()
            .await
            .expect("must build");
        match client.auth() {
            Auth::ApiToken(token) => assert_eq!(token, "root@pam!ci=secret"),
            _ => panic!("expected api token auth"),
        }
    }

    #[tokio::test]
    async fn client_option_api_token_rejects_invalid_format() {
        let err = ClientOption::new("pve.example.com")
            .api_token("invalid-token")
            .build()
            .await
            .expect_err("must fail");
        assert!(err.to_string().contains("PVE_API_TOKEN format invalid"));
    }

    #[test]
    fn pve_params_handles_bool() {
        let mut params = PveParams::new();
        params.insert_bool("full", true);
        params.insert_bool("onboot", false);

        assert_eq!(params.get("full"), Some("1"));
        assert_eq!(params.get("onboot"), Some("0"));
    }
}
