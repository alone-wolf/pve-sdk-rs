// Minimal env:
// PVE_HOST=10.0.0.2
// PVE_AUTH_METHOD=API_TOKEN
// PVE_API_TOKEN='root@pam!ci=token-secret'
// Run: cargo run --example list_all_guests

mod common;
use common::build_client_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client_from_env().await?;

    client.connect().await?;

    let nodes = client.node().list().await?;
    if nodes.is_empty() {
        println!("no nodes found");
        return Ok(());
    }

    let mut total_qemu = 0usize;
    let mut total_lxc = 0usize;

    for node in nodes {
        let node_name = node.node;
        let qemus = client.qemu().list(&node_name, Some(true)).await?;
        let lxcs = client.lxc().list(&node_name).await?;

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
