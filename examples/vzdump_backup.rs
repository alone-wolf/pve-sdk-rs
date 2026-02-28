// Minimal env (API_TOKEN):
// export PVE_HOST=10.0.0.2
// export PVE_NODE=pve-node-1
// export PVE_AUTH_METHOD=API_TOKEN
// export PVE_API_TOKEN='root@pam!ci=token-secret'
// And one of: PVE_BACKUP_VMID=100 or PVE_BACKUP_ALL=1
// Run: cargo run --example vzdump_backup

use std::time::Duration;

use pve_sdk_rs::types::backup::{MailNotification, VzdumpCompress, VzdumpMode, VzdumpRequest};
use pve_sdk_rs::types::task::WaitTaskOptions;
mod common;
use common::{build_client_from_env, env_bool, env_required};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = env_required("PVE_NODE")?;
    let client = build_client_from_env().await?;

    let storage = std::env::var("PVE_BACKUP_STORAGE").ok();
    let vmid = std::env::var("PVE_BACKUP_VMID").ok();

    if vmid.is_none() && std::env::var("PVE_BACKUP_ALL").is_err() {
        return Err("set PVE_BACKUP_VMID=<id> or PVE_BACKUP_ALL=1".into());
    }

    let req = VzdumpRequest {
        vmid,
        all: Some(env_bool("PVE_BACKUP_ALL", false)),
        mode: Some(VzdumpMode::Snapshot),
        storage,
        compress: Some(VzdumpCompress::Zstd),
        mailnotification: Some(MailNotification::Failure),
        mailto: std::env::var("PVE_BACKUP_MAILTO").ok(),
        ..VzdumpRequest::default()
    };

    let upid = client.backup().vzdump_with(&node, &req).await?;
    println!("backup task started: {upid}");

    let wait = WaitTaskOptions {
        poll_interval: Duration::from_secs(3),
        timeout: Some(Duration::from_secs(3600)),
    };
    let status = client.task().wait_with_options(&node, &upid, &wait).await?;

    println!(
        "backup finished: status={} exitstatus={:?}",
        status.status, status.exitstatus
    );

    Ok(())
}
