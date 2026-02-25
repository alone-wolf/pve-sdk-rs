use std::path::Path;
use std::time::Duration;

use reqwest::multipart;
use serde_json::Value;

use crate::client::PveClient;
use crate::error::PveError;
use crate::models::{
    AccessAcl, AccessGroup, AccessRole, AccessUser, AccessUserToken, ClusterResource,
    ClusterStatusItem, DatacenterConfig, LxcStatus, LxcSummary, NetworkInterface,
    NodeStorageStatus, NodeSummary, NodeTask, QemuStatus, QemuVmSummary, SnapshotInfo,
    StorageContentItem, StorageIndexItem, TaskLogLine, TaskStatus, TicketInfo,
};
use crate::params::PveParams;
use crate::requests;

pub struct AccessApi<'a> {
    client: &'a PveClient,
}

impl<'a> AccessApi<'a> {
    pub async fn ticket(
        &self,
        username: &str,
        password: &str,
        otp: Option<&str>,
        realm: Option<&str>,
        tfa_challenge: Option<&str>,
    ) -> Result<TicketInfo, PveError> {
        self.client
            .request_ticket(username, password, otp, realm, tfa_challenge)
            .await
    }

    pub async fn ticket_with(
        &self,
        request: &requests::TicketRequest,
    ) -> Result<TicketInfo, PveError> {
        self.client.request_ticket_with(request).await
    }

    pub async fn users(&self) -> Result<Vec<AccessUser>, PveError> {
        self.client.access_users().await
    }

    pub async fn user(&self, userid: &str) -> Result<AccessUser, PveError> {
        self.client.access_user(userid).await
    }

    pub async fn create_user(&self, params: &PveParams) -> Result<Value, PveError> {
        self.client.access_create_user(params).await
    }

    pub async fn create_user_with(
        &self,
        request: &requests::AccessCreateUserRequest,
    ) -> Result<Value, PveError> {
        self.client.access_create_user_with(request).await
    }

    pub async fn update_user(&self, userid: &str, params: &PveParams) -> Result<(), PveError> {
        self.client.access_update_user(userid, params).await
    }

    pub async fn update_user_with(
        &self,
        userid: &str,
        request: &requests::AccessUpdateUserRequest,
    ) -> Result<(), PveError> {
        self.client.access_update_user_with(userid, request).await
    }

    pub async fn delete_user(&self, userid: &str) -> Result<Value, PveError> {
        self.client.access_delete_user(userid).await
    }

    pub async fn groups(&self) -> Result<Vec<AccessGroup>, PveError> {
        self.client.access_groups().await
    }

    pub async fn group(&self, groupid: &str) -> Result<AccessGroup, PveError> {
        self.client.access_group(groupid).await
    }

    pub async fn create_group(&self, params: &PveParams) -> Result<Value, PveError> {
        self.client.access_create_group(params).await
    }

    pub async fn create_group_with(
        &self,
        request: &requests::AccessCreateGroupRequest,
    ) -> Result<Value, PveError> {
        self.client.access_create_group_with(request).await
    }

    pub async fn update_group(&self, groupid: &str, params: &PveParams) -> Result<(), PveError> {
        self.client.access_update_group(groupid, params).await
    }

    pub async fn update_group_with(
        &self,
        groupid: &str,
        request: &requests::AccessUpdateGroupRequest,
    ) -> Result<(), PveError> {
        self.client.access_update_group_with(groupid, request).await
    }

    pub async fn delete_group(&self, groupid: &str) -> Result<Value, PveError> {
        self.client.access_delete_group(groupid).await
    }

    pub async fn roles(&self) -> Result<Vec<AccessRole>, PveError> {
        self.client.access_roles().await
    }

    pub async fn acl(
        &self,
        path: Option<&str>,
        exact: Option<bool>,
    ) -> Result<Vec<AccessAcl>, PveError> {
        self.client.access_acl(path, exact).await
    }

    pub async fn acl_with(
        &self,
        query: &requests::AccessAclQuery,
    ) -> Result<Vec<AccessAcl>, PveError> {
        self.client.access_acl_with(query).await
    }

    pub async fn set_acl(&self, params: &PveParams) -> Result<(), PveError> {
        self.client.access_set_acl(params).await
    }

    pub async fn set_acl_with(
        &self,
        request: &requests::AccessSetAclRequest,
    ) -> Result<(), PveError> {
        self.client.access_set_acl_with(request).await
    }

    pub async fn delete_acl_with(
        &self,
        request: &requests::AccessDeleteAclRequest,
    ) -> Result<(), PveError> {
        self.client.access_delete_acl_with(request).await
    }

    pub async fn user_tokens(&self, userid: &str) -> Result<Vec<AccessUserToken>, PveError> {
        self.client.access_user_tokens(userid).await
    }

    pub async fn create_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &PveParams,
    ) -> Result<Value, PveError> {
        self.client
            .access_create_user_token(userid, tokenid, params)
            .await
    }

    pub async fn create_user_token_with(
        &self,
        userid: &str,
        request: &requests::AccessCreateTokenRequest,
    ) -> Result<Value, PveError> {
        self.client
            .access_create_user_token_with(userid, request)
            .await
    }

    pub async fn update_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &PveParams,
    ) -> Result<(), PveError> {
        self.client
            .access_update_user_token(userid, tokenid, params)
            .await
    }

    pub async fn update_user_token_with(
        &self,
        userid: &str,
        tokenid: &str,
        request: &requests::AccessUpdateTokenRequest,
    ) -> Result<(), PveError> {
        self.client
            .access_update_user_token_with(userid, tokenid, request)
            .await
    }

    pub async fn delete_user_token(&self, userid: &str, tokenid: &str) -> Result<Value, PveError> {
        self.client.access_delete_user_token(userid, tokenid).await
    }
}

pub struct ClusterApi<'a> {
    client: &'a PveClient,
}

impl<'a> ClusterApi<'a> {
    pub async fn resources(
        &self,
        resource_type: Option<&str>,
    ) -> Result<Vec<ClusterResource>, PveError> {
        self.client.cluster_resources(resource_type).await
    }

    pub async fn resources_with(
        &self,
        query: &requests::ClusterResourcesQuery,
    ) -> Result<Vec<ClusterResource>, PveError> {
        self.client.cluster_resources_with(query).await
    }

    pub async fn status(&self) -> Result<Vec<ClusterStatusItem>, PveError> {
        self.client.cluster_status().await
    }

    pub async fn next_id(&self, vmid: Option<u32>) -> Result<u32, PveError> {
        self.client.cluster_next_id(vmid).await
    }
}

pub struct NodeApi<'a> {
    client: &'a PveClient,
}

pub struct DatacenterApi<'a> {
    client: &'a PveClient,
}

impl<'a> DatacenterApi<'a> {
    pub async fn config(&self) -> Result<DatacenterConfig, PveError> {
        self.client.datacenter_config().await
    }

    pub async fn update_config(&self, params: &PveParams) -> Result<(), PveError> {
        self.client.datacenter_update_config(params).await
    }

    pub async fn update_config_with(
        &self,
        request: &requests::DatacenterConfigUpdateRequest,
    ) -> Result<(), PveError> {
        self.client.datacenter_update_config_with(request).await
    }
}

impl<'a> NodeApi<'a> {
    pub async fn list(&self) -> Result<Vec<NodeSummary>, PveError> {
        self.client.nodes().await
    }

    pub async fn status(&self, node: &str) -> Result<Value, PveError> {
        self.client.node_status(node).await
    }

    pub async fn tasks(&self, node: &str, query: &PveParams) -> Result<Vec<NodeTask>, PveError> {
        self.client.node_tasks(node, query).await
    }

    pub async fn tasks_with(
        &self,
        node: &str,
        query: &requests::NodeTasksQuery,
    ) -> Result<Vec<NodeTask>, PveError> {
        self.client.node_tasks_with(node, query).await
    }

    pub async fn network(
        &self,
        node: &str,
        interface_type: Option<&str>,
    ) -> Result<Vec<NetworkInterface>, PveError> {
        self.client.node_network(node, interface_type).await
    }

    pub async fn network_with(
        &self,
        node: &str,
        query: &requests::NodeNetworkQuery,
    ) -> Result<Vec<NetworkInterface>, PveError> {
        self.client.node_network_with(node, query).await
    }
}

pub struct QemuApi<'a> {
    client: &'a PveClient,
}

impl<'a> QemuApi<'a> {
    pub async fn list(
        &self,
        node: &str,
        full: Option<bool>,
    ) -> Result<Vec<QemuVmSummary>, PveError> {
        self.client.qemu_list(node, full).await
    }

    pub async fn create(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_create(node, vmid, params).await
    }

    pub async fn create_with(
        &self,
        node: &str,
        request: &requests::QemuCreateRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_create_with(node, request).await
    }

    pub async fn config(
        &self,
        node: &str,
        vmid: u32,
        current: Option<bool>,
        snapshot: Option<&str>,
    ) -> Result<Value, PveError> {
        self.client.qemu_config(node, vmid, current, snapshot).await
    }

    pub async fn config_with(
        &self,
        node: &str,
        vmid: u32,
        query: &requests::QemuConfigQuery,
    ) -> Result<Value, PveError> {
        self.client.qemu_config_with(node, vmid, query).await
    }

    pub async fn set_config_async(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_set_config_async(node, vmid, params).await
    }

    pub async fn set_config_async_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSetConfigRequest,
    ) -> Result<String, PveError> {
        self.client
            .qemu_set_config_async_with(node, vmid, request)
            .await
    }

    pub async fn set_config_sync(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<(), PveError> {
        self.client.qemu_set_config_sync(node, vmid, params).await
    }

    pub async fn set_config_sync_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSetConfigRequest,
    ) -> Result<(), PveError> {
        self.client
            .qemu_set_config_sync_with(node, vmid, request)
            .await
    }

    pub async fn status(&self, node: &str, vmid: u32) -> Result<QemuStatus, PveError> {
        self.client.qemu_status(node, vmid).await
    }

    pub async fn start(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_start(node, vmid, params).await
    }

    pub async fn start_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_start_with(node, vmid, request).await
    }

    pub async fn shutdown(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_shutdown(node, vmid, params).await
    }

    pub async fn shutdown_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_shutdown_with(node, vmid, request).await
    }

    pub async fn stop(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_stop(node, vmid, params).await
    }

    pub async fn stop_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_stop_with(node, vmid, request).await
    }

    pub async fn reboot(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_reboot(node, vmid, params).await
    }

    pub async fn reboot_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_reboot_with(node, vmid, request).await
    }

    pub async fn suspend(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_suspend(node, vmid, params).await
    }

    pub async fn suspend_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_suspend_with(node, vmid, request).await
    }

    pub async fn resume(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_resume(node, vmid, params).await
    }

    pub async fn resume_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuActionRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_resume_with(node, vmid, request).await
    }

    pub async fn snapshots(&self, node: &str, vmid: u32) -> Result<Vec<SnapshotInfo>, PveError> {
        self.client.qemu_snapshots(node, vmid).await
    }

    pub async fn snapshot_create(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client
            .qemu_snapshot_create(node, vmid, snapname, params)
            .await
    }

    pub async fn snapshot_create_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSnapshotCreateRequest,
    ) -> Result<String, PveError> {
        self.client
            .qemu_snapshot_create_with(node, vmid, request)
            .await
    }

    pub async fn snapshot_rollback(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client
            .qemu_snapshot_rollback(node, vmid, snapname, params)
            .await
    }

    pub async fn snapshot_rollback_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuSnapshotRollbackRequest,
    ) -> Result<String, PveError> {
        self.client
            .qemu_snapshot_rollback_with(node, vmid, request)
            .await
    }

    pub async fn clone(
        &self,
        node: &str,
        vmid: u32,
        newid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_clone(node, vmid, newid, params).await
    }

    pub async fn clone_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuCloneRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_clone_with(node, vmid, request).await
    }

    pub async fn migrate(
        &self,
        node: &str,
        vmid: u32,
        target: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.qemu_migrate(node, vmid, target, params).await
    }

    pub async fn migrate_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::QemuMigrateRequest,
    ) -> Result<String, PveError> {
        self.client.qemu_migrate_with(node, vmid, request).await
    }
}

pub struct LxcApi<'a> {
    client: &'a PveClient,
}

impl<'a> LxcApi<'a> {
    pub async fn list(&self, node: &str) -> Result<Vec<LxcSummary>, PveError> {
        self.client.lxc_list(node).await
    }

    pub async fn create(
        &self,
        node: &str,
        vmid: u32,
        ostemplate: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.lxc_create(node, vmid, ostemplate, params).await
    }

    pub async fn create_with(
        &self,
        node: &str,
        request: &requests::LxcCreateRequest,
    ) -> Result<String, PveError> {
        self.client.lxc_create_with(node, request).await
    }

    pub async fn config(
        &self,
        node: &str,
        vmid: u32,
        current: Option<bool>,
        snapshot: Option<&str>,
    ) -> Result<Value, PveError> {
        self.client.lxc_config(node, vmid, current, snapshot).await
    }

    pub async fn config_with(
        &self,
        node: &str,
        vmid: u32,
        query: &requests::LxcConfigQuery,
    ) -> Result<Value, PveError> {
        self.client.lxc_config_with(node, vmid, query).await
    }

    pub async fn set_config(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<(), PveError> {
        self.client.lxc_set_config(node, vmid, params).await
    }

    pub async fn set_config_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcSetConfigRequest,
    ) -> Result<(), PveError> {
        self.client.lxc_set_config_with(node, vmid, request).await
    }

    pub async fn status(&self, node: &str, vmid: u32) -> Result<LxcStatus, PveError> {
        self.client.lxc_status(node, vmid).await
    }

    pub async fn start(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.lxc_start(node, vmid, params).await
    }

    pub async fn start_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        self.client.lxc_start_with(node, vmid, request).await
    }

    pub async fn shutdown(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.lxc_shutdown(node, vmid, params).await
    }

    pub async fn shutdown_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        self.client.lxc_shutdown_with(node, vmid, request).await
    }

    pub async fn stop(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.lxc_stop(node, vmid, params).await
    }

    pub async fn stop_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        self.client.lxc_stop_with(node, vmid, request).await
    }

    pub async fn reboot(
        &self,
        node: &str,
        vmid: u32,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.lxc_reboot(node, vmid, params).await
    }

    pub async fn reboot_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcActionRequest,
    ) -> Result<String, PveError> {
        self.client.lxc_reboot_with(node, vmid, request).await
    }

    pub async fn snapshots(&self, node: &str, vmid: u32) -> Result<Vec<SnapshotInfo>, PveError> {
        self.client.lxc_snapshots(node, vmid).await
    }

    pub async fn snapshot_create(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client
            .lxc_snapshot_create(node, vmid, snapname, params)
            .await
    }

    pub async fn snapshot_create_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcSnapshotCreateRequest,
    ) -> Result<String, PveError> {
        self.client
            .lxc_snapshot_create_with(node, vmid, request)
            .await
    }

    pub async fn snapshot_rollback(
        &self,
        node: &str,
        vmid: u32,
        snapname: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client
            .lxc_snapshot_rollback(node, vmid, snapname, params)
            .await
    }

    pub async fn snapshot_rollback_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcSnapshotRollbackRequest,
    ) -> Result<String, PveError> {
        self.client
            .lxc_snapshot_rollback_with(node, vmid, request)
            .await
    }

    pub async fn migrate(
        &self,
        node: &str,
        vmid: u32,
        target: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client.lxc_migrate(node, vmid, target, params).await
    }

    pub async fn migrate_with(
        &self,
        node: &str,
        vmid: u32,
        request: &requests::LxcMigrateRequest,
    ) -> Result<String, PveError> {
        self.client.lxc_migrate_with(node, vmid, request).await
    }
}

pub struct StorageApi<'a> {
    client: &'a PveClient,
}

impl<'a> StorageApi<'a> {
    pub async fn index(
        &self,
        storage_type: Option<&str>,
    ) -> Result<Vec<StorageIndexItem>, PveError> {
        self.client.storage_index(storage_type).await
    }

    pub async fn node_storage(
        &self,
        node: &str,
        query: &PveParams,
    ) -> Result<Vec<NodeStorageStatus>, PveError> {
        self.client.node_storage(node, query).await
    }

    pub async fn node_storage_with(
        &self,
        node: &str,
        query: &requests::NodeStorageQuery,
    ) -> Result<Vec<NodeStorageStatus>, PveError> {
        self.client.node_storage_with(node, query).await
    }

    pub async fn content(
        &self,
        node: &str,
        storage: &str,
        query: &PveParams,
    ) -> Result<Vec<StorageContentItem>, PveError> {
        self.client.storage_content(node, storage, query).await
    }

    pub async fn content_with(
        &self,
        node: &str,
        storage: &str,
        query: &requests::StorageContentQuery,
    ) -> Result<Vec<StorageContentItem>, PveError> {
        self.client.storage_content_with(node, storage, query).await
    }

    pub async fn allocate_disk(
        &self,
        node: &str,
        storage: &str,
        vmid: u32,
        filename: &str,
        size: &str,
        params: &PveParams,
    ) -> Result<String, PveError> {
        self.client
            .storage_allocate_disk(node, storage, vmid, filename, size, params)
            .await
    }

    pub async fn allocate_disk_with(
        &self,
        node: &str,
        storage: &str,
        request: &requests::StorageAllocateDiskRequest,
    ) -> Result<String, PveError> {
        self.client
            .storage_allocate_disk_with(node, storage, request)
            .await
    }

    pub async fn upload_file(
        &self,
        node: &str,
        storage: &str,
        content: &str,
        file_path: impl AsRef<Path>,
        checksum: Option<&str>,
        checksum_algorithm: Option<&str>,
    ) -> Result<String, PveError> {
        self.client
            .storage_upload_file(
                node,
                storage,
                content,
                file_path,
                checksum,
                checksum_algorithm,
            )
            .await
    }

    pub async fn upload_form(
        &self,
        node: &str,
        storage: &str,
        form: multipart::Form,
    ) -> Result<String, PveError> {
        self.client.storage_upload_form(node, storage, form).await
    }

    pub async fn upload_with(
        &self,
        node: &str,
        storage: &str,
        request: &requests::StorageUploadRequest,
    ) -> Result<String, PveError> {
        self.client
            .storage_upload_with(node, storage, request)
            .await
    }

    pub async fn delete_volume(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        delay: Option<u32>,
    ) -> Result<String, PveError> {
        self.client
            .storage_delete_volume(node, storage, volume, delay)
            .await
    }

    pub async fn delete_volume_with(
        &self,
        node: &str,
        storage: &str,
        volume: &str,
        request: &requests::StorageDeleteVolumeRequest,
    ) -> Result<String, PveError> {
        self.client
            .storage_delete_volume_with(node, storage, volume, request)
            .await
    }
}

pub struct BackupApi<'a> {
    client: &'a PveClient,
}

impl<'a> BackupApi<'a> {
    pub async fn vzdump(&self, node: &str, params: &PveParams) -> Result<String, PveError> {
        self.client.vzdump_backup(node, params).await
    }

    pub async fn vzdump_with(
        &self,
        node: &str,
        request: &requests::VzdumpRequest,
    ) -> Result<String, PveError> {
        self.client.vzdump_backup_with(node, request).await
    }
}

pub struct TaskApi<'a> {
    client: &'a PveClient,
}

pub struct RawApi<'a> {
    client: &'a PveClient,
}

impl<'a> RawApi<'a> {
    pub async fn json(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&PveParams>,
        form: Option<&PveParams>,
    ) -> Result<Value, PveError> {
        self.client.raw_json(method, path, query, form).await
    }

    pub async fn get(&self, path: &str, query: Option<&PveParams>) -> Result<Value, PveError> {
        self.client.raw_get(path, query).await
    }

    pub async fn post(&self, path: &str, form: Option<&PveParams>) -> Result<Value, PveError> {
        self.client.raw_post(path, form).await
    }

    pub async fn put(&self, path: &str, form: Option<&PveParams>) -> Result<Value, PveError> {
        self.client.raw_put(path, form).await
    }

    pub async fn delete(&self, path: &str, query: Option<&PveParams>) -> Result<Value, PveError> {
        self.client.raw_delete(path, query).await
    }
}

impl<'a> TaskApi<'a> {
    pub async fn status(&self, node: &str, upid: &str) -> Result<TaskStatus, PveError> {
        self.client.task_status(node, upid).await
    }

    pub async fn log(
        &self,
        node: &str,
        upid: &str,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<TaskLogLine>, PveError> {
        self.client.task_log(node, upid, start, limit).await
    }

    pub async fn log_with(
        &self,
        node: &str,
        upid: &str,
        query: &requests::TaskLogQuery,
    ) -> Result<Vec<TaskLogLine>, PveError> {
        self.client.task_log_with(node, upid, query).await
    }

    pub async fn wait(
        &self,
        node: &str,
        upid: &str,
        poll_interval: Duration,
        timeout: Option<Duration>,
    ) -> Result<TaskStatus, PveError> {
        self.client
            .wait_for_task(node, upid, poll_interval, timeout)
            .await
    }

    pub async fn wait_with_options(
        &self,
        node: &str,
        upid: &str,
        options: &requests::WaitTaskOptions,
    ) -> Result<TaskStatus, PveError> {
        self.client
            .wait_for_task_with_options(node, upid, options)
            .await
    }
}

impl PveClient {
    pub fn access(&self) -> AccessApi<'_> {
        AccessApi { client: self }
    }

    pub fn cluster(&self) -> ClusterApi<'_> {
        ClusterApi { client: self }
    }

    pub fn datacenter(&self) -> DatacenterApi<'_> {
        DatacenterApi { client: self }
    }

    pub fn node(&self) -> NodeApi<'_> {
        NodeApi { client: self }
    }

    pub fn qemu(&self) -> QemuApi<'_> {
        QemuApi { client: self }
    }

    pub fn lxc(&self) -> LxcApi<'_> {
        LxcApi { client: self }
    }

    pub fn storage(&self) -> StorageApi<'_> {
        StorageApi { client: self }
    }

    pub fn backup(&self) -> BackupApi<'_> {
        BackupApi { client: self }
    }

    pub fn task(&self) -> TaskApi<'_> {
        TaskApi { client: self }
    }

    pub fn raw(&self) -> RawApi<'_> {
        RawApi { client: self }
    }
}

#[cfg(test)]
mod tests {
    use crate::client_option::ClientOption;
    use crate::requests;

    #[tokio::test]
    async fn grouped_accessors_are_available() {
        let client = ClientOption::new("pve.example.com")
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        let _ = client.access();
        let _ = client.cluster();
        let _ = client.datacenter();
        let _ = client.node();
        let _ = client.qemu();
        let _ = client.lxc();
        let _ = client.storage();
        let _ = client.backup();
        let _ = client.task();
        let _ = client.raw();
    }

    #[tokio::test]
    async fn access_api_set_acl_with_propagates_validation_error() {
        let client = ClientOption::new("pve.example.com")
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        let request = requests::AccessSetAclRequest::new("/vms", "PVEVMAdmin");
        let err = client
            .access()
            .set_acl_with(&request)
            .await
            .expect_err("must fail");
        assert!(
            err.to_string()
                .contains("requires at least one of users/groups/tokens")
        );
    }
}
