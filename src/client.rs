use std::path::Path;
use std::time::Duration;

use reqwest::{Method, RequestBuilder, multipart};
use serde_json::Value;
use tokio::fs::File;
use tokio::time::{Instant, sleep};
use tokio_util::io::ReaderStream;
use url::Url;

use crate::client_option::{ClientAuth, ClientOption, validate_api_token_format};
pub use crate::core::auth::Auth;
use crate::core::auth::apply_auth;
use crate::core::transport::{
    build_base_url, build_http_client, enc, execute as transport_execute, join_api_url,
};
use crate::error::PveError;
use crate::models::{
    AccessAcl, AccessGroup, AccessRole, AccessUser, AccessUserToken, ClusterResource,
    ClusterStatusItem, DatacenterConfig, LxcStatus, LxcSummary, NetworkInterface,
    NodeStorageStatus, NodeSummary, NodeTask, QemuStatus, QemuVmSummary, SnapshotInfo,
    StorageContentItem, StorageIndexItem, TaskLogLine, TaskStatus, TicketInfo, VersionInfo,
};
use crate::params::PveParams;
use crate::requests;

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

    pub async fn access_users(&self) -> Result<Vec<AccessUser>, PveError> {
        self.send(Method::GET, "/access/users", None, None).await
    }

    pub async fn access_user(&self, userid: &str) -> Result<AccessUser, PveError> {
        let path = format!("/access/users/{}", enc(userid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn access_create_user(&self, params: &PveParams) -> Result<Value, PveError> {
        self.send(Method::POST, "/access/users", None, Some(params))
            .await
    }

    pub async fn access_create_user_with(
        &self,
        request: &requests::AccessCreateUserRequest,
    ) -> Result<Value, PveError> {
        let params = request.to_params();
        self.access_create_user(&params).await
    }

    pub async fn access_update_user(
        &self,
        userid: &str,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/access/users/{}", enc(userid));
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn access_update_user_with(
        &self,
        userid: &str,
        request: &requests::AccessUpdateUserRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_update_user(userid, &params).await
    }

    pub async fn access_delete_user(&self, userid: &str) -> Result<Value, PveError> {
        let path = format!("/access/users/{}", enc(userid));
        self.send(Method::DELETE, &path, None, None).await
    }

    pub async fn access_groups(&self) -> Result<Vec<AccessGroup>, PveError> {
        self.send(Method::GET, "/access/groups", None, None).await
    }

    pub async fn access_group(&self, groupid: &str) -> Result<AccessGroup, PveError> {
        let path = format!("/access/groups/{}", enc(groupid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn access_create_group(&self, params: &PveParams) -> Result<Value, PveError> {
        self.send(Method::POST, "/access/groups", None, Some(params))
            .await
    }

    pub async fn access_create_group_with(
        &self,
        request: &requests::AccessCreateGroupRequest,
    ) -> Result<Value, PveError> {
        let params = request.to_params();
        self.access_create_group(&params).await
    }

    pub async fn access_update_group(
        &self,
        groupid: &str,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/access/groups/{}", enc(groupid));
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn access_update_group_with(
        &self,
        groupid: &str,
        request: &requests::AccessUpdateGroupRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_update_group(groupid, &params).await
    }

    pub async fn access_delete_group(&self, groupid: &str) -> Result<Value, PveError> {
        let path = format!("/access/groups/{}", enc(groupid));
        self.send(Method::DELETE, &path, None, None).await
    }

    pub async fn access_roles(&self) -> Result<Vec<AccessRole>, PveError> {
        self.send(Method::GET, "/access/roles", None, None).await
    }

    pub async fn access_acl(
        &self,
        path: Option<&str>,
        exact: Option<bool>,
    ) -> Result<Vec<AccessAcl>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("path", path);
        if let Some(exact) = exact {
            query.insert_bool("exact", exact);
        }
        self.send(Method::GET, "/access/acl", Some(&query), None)
            .await
    }

    pub async fn access_acl_with(
        &self,
        query: &requests::AccessAclQuery,
    ) -> Result<Vec<AccessAcl>, PveError> {
        let params = query.to_params();
        self.send(Method::GET, "/access/acl", Some(&params), None)
            .await
    }

    pub async fn access_set_acl(&self, params: &PveParams) -> Result<(), PveError> {
        validate_acl_params(params)?;
        let _: Value = self
            .send(Method::PUT, "/access/acl", None, Some(params))
            .await?;
        Ok(())
    }

    pub async fn access_set_acl_with(
        &self,
        request: &requests::AccessSetAclRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_set_acl(&params).await
    }

    pub async fn access_delete_acl_with(
        &self,
        request: &requests::AccessDeleteAclRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_set_acl(&params).await
    }

    pub async fn access_user_tokens(&self, userid: &str) -> Result<Vec<AccessUserToken>, PveError> {
        let path = format!("/access/users/{}/token", enc(userid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn access_create_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &PveParams,
    ) -> Result<Value, PveError> {
        let path = format!("/access/users/{}/token", enc(userid));
        let mut body = params.clone();
        body.insert("tokenid", tokenid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn access_create_user_token_with(
        &self,
        userid: &str,
        request: &requests::AccessCreateTokenRequest,
    ) -> Result<Value, PveError> {
        let params = request.to_params();
        self.access_create_user_token(userid, &request.tokenid, &params)
            .await
    }

    pub async fn access_update_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/access/users/{}/token/{}", enc(userid), enc(tokenid));
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn access_update_user_token_with(
        &self,
        userid: &str,
        tokenid: &str,
        request: &requests::AccessUpdateTokenRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_update_user_token(userid, tokenid, &params)
            .await
    }

    pub async fn access_delete_user_token(
        &self,
        userid: &str,
        tokenid: &str,
    ) -> Result<Value, PveError> {
        let path = format!("/access/users/{}/token/{}", enc(userid), enc(tokenid));
        self.send(Method::DELETE, &path, None, None).await
    }

    pub async fn datacenter_config(&self) -> Result<DatacenterConfig, PveError> {
        self.send(Method::GET, "/cluster/options", None, None).await
    }

    pub async fn datacenter_update_config(&self, params: &PveParams) -> Result<(), PveError> {
        let _: Value = self
            .send(Method::PUT, "/cluster/options", None, Some(params))
            .await?;
        Ok(())
    }

    pub async fn datacenter_update_config_with(
        &self,
        request: &requests::DatacenterConfigUpdateRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.datacenter_update_config(&params).await
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

    pub async fn raw_json(
        &self,
        method: Method,
        path: &str,
        query: Option<&PveParams>,
        form: Option<&PveParams>,
    ) -> Result<Value, PveError> {
        self.send(method, path, query, form).await
    }

    pub async fn raw_get(&self, path: &str, query: Option<&PveParams>) -> Result<Value, PveError> {
        self.raw_json(Method::GET, path, query, None).await
    }

    pub async fn raw_post(&self, path: &str, form: Option<&PveParams>) -> Result<Value, PveError> {
        self.raw_json(Method::POST, path, None, form).await
    }

    pub async fn raw_put(&self, path: &str, form: Option<&PveParams>) -> Result<Value, PveError> {
        self.raw_json(Method::PUT, path, None, form).await
    }

    pub async fn raw_delete(
        &self,
        path: &str,
        query: Option<&PveParams>,
    ) -> Result<Value, PveError> {
        self.raw_json(Method::DELETE, path, query, None).await
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
        T: serde::de::DeserializeOwned,
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
        T: serde::de::DeserializeOwned,
    {
        let url = self.url(path)?;
        let request = self.apply_auth(self.http.request(method.clone(), url), &method)?;
        let request = request.multipart(form);
        self.execute(request).await
    }

    async fn execute<T>(&self, request: RequestBuilder) -> Result<T, PveError>
    where
        T: serde::de::DeserializeOwned,
    {
        transport_execute(request).await
    }

    fn apply_auth(
        &self,
        request: RequestBuilder,
        method: &Method,
    ) -> Result<RequestBuilder, PveError> {
        apply_auth(&self.auth, request, method)
    }

    fn url(&self, path: &str) -> Result<Url, PveError> {
        join_api_url(&self.base_url, path)
    }
}

fn validate_acl_params(params: &PveParams) -> Result<(), PveError> {
    fn has_non_empty(params: &PveParams, key: &str) -> bool {
        params.get(key).is_some_and(|v| !v.trim().is_empty())
    }

    if !has_non_empty(params, "path") {
        return Err(PveError::InvalidArgument(
            "access acl requires non-empty path".to_string(),
        ));
    }

    let delete_acl = params
        .get("delete")
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
    let has_target = has_non_empty(params, "users")
        || has_non_empty(params, "groups")
        || has_non_empty(params, "tokens");

    if delete_acl {
        let has_roles = has_non_empty(params, "roles");
        if !(has_roles || has_target) {
            return Err(PveError::InvalidArgument(
                "access acl delete requires at least one of roles/users/groups/tokens".to_string(),
            ));
        }
        return Ok(());
    }

    if !has_non_empty(params, "roles") {
        return Err(PveError::InvalidArgument(
            "access acl set requires non-empty roles".to_string(),
        ));
    }
    if !has_target {
        return Err(PveError::InvalidArgument(
            "access acl set requires at least one of users/groups/tokens".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use reqwest::Method;
    use reqwest::header::{AUTHORIZATION, COOKIE};
    use url::Url;

    use super::{Auth, PveClient};
    use crate::client_option::{ClientAuth, ClientOption};
    use crate::core::transport::{build_base_url, normalize_api_path};
    use crate::error::PveError;
    use crate::params::PveParams;
    use crate::requests;

    fn client_with_auth(auth: Auth) -> PveClient {
        PveClient {
            base_url: Url::parse("https://pve.example.com:8006/").expect("base url"),
            http: reqwest::Client::new(),
            timeout: None,
            connect_timeout: None,
            auth,
        }
    }

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
    fn apply_auth_sets_api_token_header() {
        let client = client_with_auth(Auth::ApiToken("root@pam!ci=secret".to_string()));
        let request = client.http.request(
            Method::GET,
            "https://pve.example.com:8006/api2/json/version",
        );
        let request = client
            .apply_auth(request, &Method::GET)
            .expect("must apply auth")
            .build()
            .expect("request");
        let header = request
            .headers()
            .get(AUTHORIZATION)
            .expect("authorization header")
            .to_str()
            .expect("utf8");
        assert_eq!(header, "PVEAPIToken=root@pam!ci=secret");
    }

    #[test]
    fn apply_auth_ticket_get_does_not_require_csrf() {
        let client = client_with_auth(Auth::Ticket {
            ticket: "PVE:ticket-value".to_string(),
            csrf: None,
        });
        let request = client.http.request(
            Method::GET,
            "https://pve.example.com:8006/api2/json/version",
        );
        let request = client
            .apply_auth(request, &Method::GET)
            .expect("must apply auth")
            .build()
            .expect("request");
        assert!(request.headers().get("CSRFPreventionToken").is_none());
        let cookie = request
            .headers()
            .get(COOKIE)
            .expect("cookie header")
            .to_str()
            .expect("utf8");
        assert_eq!(cookie, "PVEAuthCookie=PVE:ticket-value");
    }

    #[test]
    fn apply_auth_ticket_write_requires_csrf() {
        let client = client_with_auth(Auth::Ticket {
            ticket: "PVE:ticket-value".to_string(),
            csrf: None,
        });
        let request = client
            .http
            .request(Method::POST, "https://pve.example.com:8006/api2/json/nodes");
        let err = client
            .apply_auth(request, &Method::POST)
            .expect_err("must reject missing csrf");
        assert!(matches!(err, PveError::MissingCsrfToken));
    }

    #[test]
    fn apply_auth_ticket_write_sets_csrf_header() {
        let client = client_with_auth(Auth::Ticket {
            ticket: "PVE:ticket-value".to_string(),
            csrf: Some("csrf-token-value".to_string()),
        });
        let request = client
            .http
            .request(Method::POST, "https://pve.example.com:8006/api2/json/nodes");
        let request = client
            .apply_auth(request, &Method::POST)
            .expect("must apply auth")
            .build()
            .expect("request");
        let csrf = request
            .headers()
            .get("CSRFPreventionToken")
            .expect("csrf header")
            .to_str()
            .expect("utf8");
        assert_eq!(csrf, "csrf-token-value");
    }

    #[test]
    fn pve_params_handles_bool() {
        let mut params = PveParams::new();
        params.insert_bool("full", true);
        params.insert_bool("onboot", false);

        assert_eq!(params.get("full"), Some("1"));
        assert_eq!(params.get("onboot"), Some("0"));
    }

    #[tokio::test]
    async fn access_set_acl_rejects_missing_subject() {
        let client = ClientOption::new("pve.example.com")
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        let request = requests::AccessSetAclRequest::new("/vms", "PVEVMAdmin");
        let err = client
            .access_set_acl_with(&request)
            .await
            .expect_err("must reject missing subject");
        assert!(matches!(err, PveError::InvalidArgument(_)));
    }

    #[tokio::test]
    async fn access_delete_acl_rejects_missing_targets() {
        let client = ClientOption::new("pve.example.com")
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        let request = requests::AccessDeleteAclRequest::new("/vms");
        let err = client
            .access_delete_acl_with(&request)
            .await
            .expect_err("must reject missing target");
        assert!(matches!(err, PveError::InvalidArgument(_)));
    }
}
