// Read-only coverage example for pve-sdk-rs grouped APIs.
//
// Required env:
//   PVE_HOST
//   PVE_AUTH_METHOD
//   (auth fields required by PVE_AUTH_METHOD)
//
// Optional env for deeper reads:
//   PVE_NODE
//   PVE_READ_USERID
//   PVE_READ_GROUPID
//   PVE_QEMU_VMID
//   PVE_LXC_VMID
//   PVE_STORAGE
//   PVE_UPID
//
// Run:
//   cargo run --example read_all_features

use std::env;

use pve_sdk_rs::PveParams;
use pve_sdk_rs::types;
use pve_sdk_rs::types::access::AccessAclQuery;
use pve_sdk_rs::types::cluster::{ClusterResourceType, ClusterResourcesQuery};
use pve_sdk_rs::types::lxc::LxcConfigQuery;
use pve_sdk_rs::types::node::{NodeNetworkQuery, NodeTasksQuery, TaskSource};
use pve_sdk_rs::types::qemu::QemuConfigQuery;
use pve_sdk_rs::types::storage::{NodeStorageQuery, StorageContentQuery};
use pve_sdk_rs::types::task::TaskLogQuery;

mod common;
use common::build_client_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client_from_env().await?;

    let version = client.connect_with_version().await?;
    println!("connected: version={}", version.version);

    // --- Access ---
    let users = client.access().users().await?;
    let groups = client.access().groups().await?;
    let roles = client.access().roles().await?;
    let acl = client.access().acl(None, None).await?;
    let acl_with = client
        .access()
        .acl_with(&AccessAclQuery {
            path: None,
            exact: Some(false),
        })
        .await?;
    println!(
        "access: users={} groups={} roles={} acl={} acl_with={}",
        users.len(),
        groups.len(),
        roles.len(),
        acl.len(),
        acl_with.len()
    );

    if let Ok(userid) = env::var("PVE_READ_USERID") {
        let user = client.access().user(&userid).await?;
        let tokens = client.access().user_tokens(&userid).await?;
        println!(
            "access user detail: userid={} enable={:?} tokens={}",
            user.userid,
            user.enable,
            tokens.len()
        );
    }

    if let Ok(groupid) = env::var("PVE_READ_GROUPID") {
        let group = client.access().group(&groupid).await?;
        println!("access group detail: groupid={}", group.groupid);
    }

    // --- Cluster / Datacenter ---
    let cluster_status = client.cluster().status().await?;
    let cluster_resources = client.cluster().resources(None).await?;
    let cluster_resources_with = client
        .cluster()
        .resources_with(&ClusterResourcesQuery {
            resource_type: Some(ClusterResourceType::Vm),
        })
        .await?;
    let next_id = client.cluster().next_id(None).await?;
    let _dc_cfg = client.datacenter().config().await?;
    println!(
        "cluster: status={} resources={} resources_with={} next_id={}",
        cluster_status.len(),
        cluster_resources.len(),
        cluster_resources_with.len(),
        next_id
    );

    // --- Node reads ---
    let nodes = client.node().list().await?;
    println!("nodes={}", nodes.len());
    if nodes.is_empty() {
        println!("no nodes found, stopping node/vm/storage/task reads");
        return Ok(());
    }

    let node = env::var("PVE_NODE")
        .ok()
        .unwrap_or_else(|| nodes[0].node.clone());
    let node_status = client.node().status(&node).await?;
    println!(
        "node selected: {} status_keys={}",
        node,
        node_status.as_object().map(|o| o.len()).unwrap_or_default()
    );

    let mut node_task_params = PveParams::new();
    node_task_params.insert("limit", "20");
    let node_tasks = client.node().tasks(&node, &node_task_params).await?;
    let node_tasks_with = client
        .node()
        .tasks_with(
            &node,
            &NodeTasksQuery {
                limit: Some(20),
                source: Some(TaskSource::All),
                ..NodeTasksQuery::default()
            },
        )
        .await?;
    println!(
        "node tasks: plain={} with={}",
        node_tasks.len(),
        node_tasks_with.len()
    );

    let node_network = client.node().network(&node, None).await?;
    let node_network_with = client
        .node()
        .network_with(&node, &NodeNetworkQuery::default())
        .await?;
    println!(
        "node network: plain={} with={}",
        node_network.len(),
        node_network_with.len()
    );

    // --- QEMU reads ---
    let qemus = client.qemu().list(&node, Some(true)).await?;
    println!("qemu list={}", qemus.len());
    let qemu_vmid = env::var("PVE_QEMU_VMID")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .or_else(|| qemus.first().map(|v| v.vmid));
    if let Some(vmid) = qemu_vmid {
        let qemu_cfg = client.qemu().config(&node, vmid, None, None).await?;
        let qemu_cfg_with = client
            .qemu()
            .config_with(&node, vmid, &QemuConfigQuery::default())
            .await?;
        let qemu_status = client.qemu().status(&node, vmid).await?;
        let qemu_snaps = client.qemu().snapshots(&node, vmid).await?;
        println!(
            "qemu vmid={} cfg_keys={} cfg_with_keys={} status={:?} snaps={}",
            vmid,
            qemu_cfg.as_object().map(|o| o.len()).unwrap_or_default(),
            qemu_cfg_with
                .as_object()
                .map(|o| o.len())
                .unwrap_or_default(),
            qemu_status.status,
            qemu_snaps.len()
        );
    }

    // --- LXC reads ---
    let lxcs = client.lxc().list(&node).await?;
    println!("lxc list={}", lxcs.len());
    let lxc_vmid = env::var("PVE_LXC_VMID")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .or_else(|| lxcs.first().map(|v| v.vmid));
    if let Some(vmid) = lxc_vmid {
        let lxc_cfg = client.lxc().config(&node, vmid, None, None).await?;
        let lxc_cfg_with = client
            .lxc()
            .config_with(&node, vmid, &LxcConfigQuery::default())
            .await?;
        let lxc_status = client.lxc().status(&node, vmid).await?;
        let lxc_snaps = client.lxc().snapshots(&node, vmid).await?;
        println!(
            "lxc vmid={} cfg_keys={} cfg_with_keys={} status={:?} snaps={}",
            vmid,
            lxc_cfg.as_object().map(|o| o.len()).unwrap_or_default(),
            lxc_cfg_with
                .as_object()
                .map(|o| o.len())
                .unwrap_or_default(),
            lxc_status.status,
            lxc_snaps.len()
        );
    }

    // --- Storage reads ---
    let storage_index = client.storage().index(None).await?;
    println!("storage index={}", storage_index.len());

    let node_storage = client
        .storage()
        .node_storage(&node, &PveParams::new())
        .await?;
    let node_storage_with = client
        .storage()
        .node_storage_with(&node, &NodeStorageQuery::default())
        .await?;
    println!(
        "node storage: plain={} with={}",
        node_storage.len(),
        node_storage_with.len()
    );

    let storage = env::var("PVE_STORAGE")
        .ok()
        .or_else(|| node_storage.first().map(|s| s.storage.clone()))
        .or_else(|| storage_index.first().map(|s| s.storage.clone()));
    if let Some(storage) = storage {
        let content_plain = client
            .storage()
            .content(&node, &storage, &PveParams::new())
            .await?;
        let content_with = client
            .storage()
            .content_with(&node, &storage, &StorageContentQuery::default())
            .await?;
        println!(
            "storage content: storage={} plain={} with={}",
            storage,
            content_plain.len(),
            content_with.len()
        );
    }

    // --- Task reads (requires existing UPID) ---
    if let Ok(upid) = env::var("PVE_UPID") {
        let t_status = client.task().status(&node, &upid).await?;
        let t_log = client.task().log(&node, &upid, Some(0), Some(20)).await?;
        let t_log_with = client
            .task()
            .log_with(
                &node,
                &upid,
                &TaskLogQuery {
                    start: Some(0),
                    limit: Some(20),
                },
            )
            .await?;
        println!(
            "task upid={} status={} log={} log_with={}",
            upid,
            t_status.status,
            t_log.len(),
            t_log_with.len()
        );
    } else {
        println!("skip task status/log: set PVE_UPID to test");
    }

    // --- Raw read fallback ---
    let mut raw_query = PveParams::new();
    raw_query.insert("type", ClusterResourceType::Vm.as_str());
    let raw_resources = client
        .raw()
        .get("/cluster/resources", Some(&raw_query))
        .await?;
    println!(
        "raw get /cluster/resources?type=vm keys={}",
        raw_resources
            .as_array()
            .map(|a| a.len())
            .or_else(|| raw_resources.as_object().map(|o| o.len()))
            .unwrap_or_default()
    );

    let _ = types::task::WaitTaskOptions::default();
    println!("read_all_features completed");
    Ok(())
}
