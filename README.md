# pve-sdk-rs

Async Rust SDK for Proxmox VE (`/api2/json`).

## Install

```toml
[dependencies]
pve-sdk-rs = { git = "https://github.com/alone-wolf/pve-sdk-rs.git", branch = "main" }
```

## Quick Start

```rust,no_run
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
    .build()
    .await?;

client.connect().await?;
let nodes = client.node().list().await?;
println!("nodes={}", nodes.len());
# Ok(())
# }
```

## Auth From Env

```rust,no_run
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let auth = ClientAuth::from_env()?;
let client = ClientOption::new("pve.example.com")
    .auth(auth)
    .build()
    .await?;
# Ok(())
# }
```

Environment variables are documented in:
- `pve_sdk_docs/03-auth-methods-env.md`

## Typed Requests

Use domain-grouped types from `types::*`:

```rust,no_run
use pve_sdk_rs::types;

let create = types::qemu::QemuCreateRequest::new(220);
let query = types::cluster::ClusterResourcesQuery {
    resource_type: Some(types::cluster::ClusterResourceType::Vm),
};
# let _ = (create, query);
```

## Examples

```bash
cargo run --example list_all_guests
cargo run --example list_resources
cargo run --example create_qemu_vm
cargo run --example vzdump_backup
cargo run --example access_acl_manage
```

All examples load `.env` via `dotenvy` and use `ClientAuth::from_env()`.

## Documentation

- SDK guides: `pve_sdk_docs/README.md`
- Raw PVE API notes: `pve_docs/README.md`
