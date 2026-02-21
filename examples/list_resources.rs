// Minimal env (API_TOKEN):
// export PVE_HOST=10.0.0.2
// export PVE_PORT=8006
// export PVE_INSECURE_TLS=true
// export PVE_AUTH_METHOD=API_TOKEN
// export PVE_API_TOKEN='root@pam!ci=token-secret'
// Run: cargo run --example list_resources

use std::env;

use pve_sdk_rs::{ClientAuth, ClientOption, ClusterResourceType, ClusterResourcesQuery, PveClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let version = client.version().await?;
    println!("PVE version: {}", version.version);

    let nodes = client.nodes().await?;
    println!("nodes ({}):", nodes.len());
    for node in &nodes {
        println!("- {} status={:?}", node.node, node.status);
    }

    let query = ClusterResourcesQuery {
        resource_type: Some(ClusterResourceType::Vm),
    };
    let resources = client.cluster_resources_with(&query).await?;

    println!("\ncluster vm resources ({}):", resources.len());
    for item in resources.iter().take(10) {
        println!(
            "- id={} node={:?} vmid={:?} status={:?}",
            item.id, item.node, item.vmid, item.status
        );
    }

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
