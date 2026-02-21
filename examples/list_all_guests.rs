// Minimal env (API_TOKEN) with `.env`:
// PVE_HOST=10.0.0.2
// PVE_PORT=8006
// PVE_INSECURE_TLS=true
// PVE_AUTH_METHOD=API_TOKEN
// PVE_API_TOKEN='root@pam!ci=token-secret'
// Run: cargo run --example list_all_guests

use std::env;

use dotenvy::dotenv;
use pve_sdk_rs::{ClientAuth, ClientOption, PveClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env_required("PVE_HOST")?;
    let port = env_u16("PVE_PORT", PveClient::DEFAULT_PORT);
    let insecure_tls = env_bool("PVE_INSECURE_TLS", true);
    let auth = ClientAuth::from_env()?;

    let client = ClientOption::new(host)
        .port(port)
        .insecure_tls(insecure_tls)
        .auth(auth)
        .build()
        .await?;

    client.connect().await?;

    let nodes = client.nodes().await?;
    if nodes.is_empty() {
        println!("no nodes found");
        return Ok(());
    }

    let mut total_qemu = 0usize;
    let mut total_lxc = 0usize;

    for node in nodes {
        let node_name = node.node;
        let qemus = client.qemu_list(&node_name, Some(true)).await?;
        let lxcs = client.lxc_list(&node_name).await?;

        println!("\nnode={node_name}");
        println!("  qemu count={}", qemus.len());
        for vm in &qemus {
            println!(
                "    [qemu] vmid={} name={} status={}",
                vm.vmid,
                vm.name.as_deref().unwrap_or("-"),
                vm.status.as_deref().unwrap_or("-")
            );
        }

        println!("  lxc count={}", lxcs.len());
        for ct in &lxcs {
            println!(
                "    [lxc]  vmid={} name={} status={}",
                ct.vmid,
                ct.name.as_deref().unwrap_or("-"),
                ct.status.as_deref().unwrap_or("-")
            );
        }

        total_qemu += qemus.len();
        total_lxc += lxcs.len();
    }

    println!(
        "\nall guests: qemu={} lxc={} total={}",
        total_qemu,
        total_lxc,
        total_qemu + total_lxc
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
