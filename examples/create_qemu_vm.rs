// Minimal env (API_TOKEN):
// export PVE_HOST=10.0.0.2
// export PVE_NODE=pve-node-1
// export PVE_AUTH_METHOD=API_TOKEN
// export PVE_API_TOKEN='root@pam!ci=token-secret'
// Optional: PVE_STORAGE, PVE_BRIDGE, PVE_MEMORY_MB, PVE_CORES, PVE_VMID
// Run: cargo run --example create_qemu_vm

use std::env;
use std::time::Duration;

use pve_sdk_rs::{
    ClientAuth, ClientOption, PveClient, QemuActionRequest, QemuCreateRequest, WaitTaskOptions,
};
mod common;
use common::{env_bool, env_required, env_u16, env_u32};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = env_required("PVE_HOST")?;
    let port = env_u16("PVE_PORT", PveClient::DEFAULT_PORT);
    let node = env_required("PVE_NODE")?;
    let insecure_tls = env_bool("PVE_INSECURE_TLS", true);
    let auth = ClientAuth::from_env()?;

    let storage = env::var("PVE_STORAGE").unwrap_or_else(|_| "local-lvm".to_string());
    let bridge = env::var("PVE_BRIDGE").unwrap_or_else(|_| "vmbr0".to_string());
    let disk_size_gb = env::var("PVE_DISK_GB").unwrap_or_else(|_| "32".to_string());
    let auto_start = env_bool("PVE_AUTO_START", true);

    let client = ClientOption::new(host)
        .port(port)
        .insecure_tls(insecure_tls)
        .auth(auth)
        .build()
        .await?;

    let vmid = match env::var("PVE_VMID") {
        Ok(value) => value.parse::<u32>()?,
        Err(_) => client.cluster_next_id(None).await?,
    };

    let mut create = QemuCreateRequest::new(vmid);
    create.name = Some(format!("vm-{vmid}"));
    create.memory = Some(env_u32("PVE_MEMORY_MB", 4096));
    create.cores = Some(env_u32("PVE_CORES", 2));
    create.net0 = Some(format!("virtio,bridge={bridge}"));
    create.scsi0 = Some(format!("{storage}:{disk_size_gb}"));

    println!("creating vm {vmid} on node {node}...");
    let create_upid = client.qemu_create_with(&node, &create).await?;

    let wait_create = WaitTaskOptions {
        poll_interval: Duration::from_secs(2),
        timeout: Some(Duration::from_secs(600)),
    };
    client
        .wait_for_task_with_options(&node, &create_upid, &wait_create)
        .await?;
    println!("create done, upid={create_upid}");

    if auto_start {
        println!("starting vm {vmid}...");
        let start = QemuActionRequest {
            timeout: Some(120),
            ..QemuActionRequest::default()
        };
        let start_upid = client.qemu_start_with(&node, vmid, &start).await?;

        let wait_start = WaitTaskOptions {
            poll_interval: Duration::from_secs(2),
            timeout: Some(Duration::from_secs(300)),
        };
        client
            .wait_for_task_with_options(&node, &start_upid, &wait_start)
            .await?;
        println!("start done, upid={start_upid}");
    }

    let status = client.qemu_status(&node, vmid).await?;
    println!("vm status: {:?}", status.status);

    Ok(())
}
