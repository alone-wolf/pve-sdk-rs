// Minimal env (API_TOKEN):
// export PVE_HOST=10.0.0.2
// export PVE_NODE=pve-node-1
// export PVE_AUTH_METHOD=API_TOKEN
// export PVE_API_TOKEN='root@pam!ci=token-secret'
// And one of: PVE_BACKUP_VMID=100 or PVE_BACKUP_ALL=1
// Run: cargo run --example vzdump_backup

use std::env;
use std::time::Duration;

use pve_sdk_rs::{
    ClientAuth, ClientOption, MailNotification, PveClient, VzdumpCompress, VzdumpMode,
    VzdumpRequest, WaitTaskOptions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = env_required("PVE_HOST")?;
    let port = env_u16("PVE_PORT", PveClient::DEFAULT_PORT);
    let node = env_required("PVE_NODE")?;
    let insecure_tls = env_bool("PVE_INSECURE_TLS", true);
    let auth = ClientAuth::from_env()?;

    let client = ClientOption::new(host)
        .port(port)
        .insecure_tls(insecure_tls)
        .auth(auth)
        .build()
        .await?;

    let storage = env::var("PVE_BACKUP_STORAGE").ok();
    let vmid = env::var("PVE_BACKUP_VMID").ok();

    if vmid.is_none() && env::var("PVE_BACKUP_ALL").is_err() {
        return Err("set PVE_BACKUP_VMID=<id> or PVE_BACKUP_ALL=1".into());
    }

    let req = VzdumpRequest {
        vmid,
        all: Some(env_bool("PVE_BACKUP_ALL", false)),
        mode: Some(VzdumpMode::Snapshot),
        storage,
        compress: Some(VzdumpCompress::Zstd),
        mailnotification: Some(MailNotification::Failure),
        mailto: env::var("PVE_BACKUP_MAILTO").ok(),
        ..VzdumpRequest::default()
    };

    let upid = client.vzdump_backup_with(&node, &req).await?;
    println!("backup task started: {upid}");

    let wait = WaitTaskOptions {
        poll_interval: Duration::from_secs(3),
        timeout: Some(Duration::from_secs(3600)),
    };
    let status = client
        .wait_for_task_with_options(&node, &upid, &wait)
        .await?;

    println!(
        "backup finished: status={} exitstatus={:?}",
        status.status, status.exitstatus
    );

    Ok(())
}

fn env_required(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    env::var(name).map_err(|_| format!("missing env var {name}").into())
}

fn env_bool(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(value) => matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"),
        Err(_) => default,
    }
}

fn env_u16(name: &str, default: u16) -> u16 {
    env::var(name)
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(default)
}
