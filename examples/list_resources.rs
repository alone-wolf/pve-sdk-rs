// Minimal env:
// export PVE_HOST=10.0.0.2
// export PVE_AUTH_METHOD=API_TOKEN
// export PVE_API_TOKEN='root@pam!ci=token-secret'
// Run: cargo run --example list_resources

use pve_sdk_rs::types::cluster::{ClusterResourceType, ClusterResourcesQuery};
mod common;
use common::build_client_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client_from_env().await?;

    let version = client.connect_with_version().await?;
    println!("PVE version: {}", version.version);

    let nodes = client.node().list().await?;
    println!("nodes ({}):", nodes.len());
    for node in &nodes {
        println!("- {} status={:?}", node.node, node.status);
    }

    let query = ClusterResourcesQuery {
        resource_type: Some(ClusterResourceType::Vm),
    };
    let resources = client.cluster().resources_with(&query).await?;

    println!("\ncluster vm resources ({}):", resources.len());
    for item in resources.iter().take(10) {
        println!(
            "- id={} node={:?} vmid={:?} status={:?}",
            item.id, item.node, item.vmid, item.status
        );
    }

    Ok(())
}
