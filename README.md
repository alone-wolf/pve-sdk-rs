# pve-sdk-rs

Async Rust SDK for the Proxmox VE (`/api2/json`) API.

## Highlights

- API token and ticket auth support
- Username/password login support (auto ticket exchange on build)
- Typed request builders for common VM/LXC/storage operations
- Access 管理面基础能力（users/groups/ACL/token）与 Datacenter config
- Task polling helpers for async PVE jobs (UPID)
- Multipart upload support for storage endpoints
- Raw endpoint fallback (`client.raw()`) for not-yet-typed APIs

## Documentation

- SDK usage docs: `pve_sdk_docs/README.md`
- Raw PVE API references: `pve_docs/README.md`

## Install

From GitHub:

```toml
[dependencies]
pve-sdk-rs = { git = "https://github.com/alone-wolf/pve-sdk-rs.git", branch = "main" }
```

## Quick Start

```rust,no_run
use pve_sdk_rs::{ClientOption, PveClient};

# async fn run() -> Result<PveClient, pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .port(8006)
    .https(true)
    .insecure_tls(true)
    .api_token("root@pam!ci=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
    .build()
    .await?;
# Ok(client)
# }
```

You can explicitly verify connectivity first:

```rust,no_run
use pve_sdk_rs::ClientOption;

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .api_token("root@pam!ci=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
    .build()
    .await?;
client.connect().await?;
# Ok(())
# }
```

If you also want the server version during connect:

```rust,no_run
use pve_sdk_rs::ClientOption;

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .api_token("root@pam!ci=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
    .build()
    .await?;
let version = client.connect_with_version().await?;
println!("Connected to PVE {}", version.version);
# Ok(())
# }
```

Or build and connect in one step:

```rust,no_run
use pve_sdk_rs::ClientOption;

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let _client = ClientOption::new("pve.example.com")
    .api_token("root@pam!ci=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
    .build_and_connect()
    .await?;
# Ok(())
# }
```

Grouped API style (recommended for discoverability):

```rust,no_run
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
    .build()
    .await?;

let _nodes = client.node().list().await?;
let _vms = client.qemu().list("pve1", Some(true)).await?;
let _dc = client.datacenter().config().await?;
let _raw = client.raw().get("/cluster/resources", None).await?;
# Ok(())
# }
```

ACL write validation rules (preflight in SDK):

- `set_acl_with`: requires `path`, `roles`, and at least one target in `users/groups/tokens`
- `delete_acl_with`: requires `path`, and at least one target in `roles/users/groups/tokens`
- invalid payload returns `PveError::InvalidArgument` before sending request

Domain-grouped type namespace is also available:

```rust,no_run
use pve_sdk_rs::types;

let _create = types::qemu::QemuCreateRequest::new(220);
let _tasks = types::node::NodeTasksQuery::default();
```

Full-parameter initializer form:

```rust,no_run
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let _client = ClientOption::all(
    "pve.example.com",
    8006,
    true,
    true,
    ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()),
)
.build()
.await?;
# Ok(())
# }
```

Full-parameter initializer with timeout controls:

```rust,no_run
use std::time::Duration;
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let _client = ClientOption::all_with_timeouts(
    "pve.example.com",
    8006,
    true,
    true,
    Some(Duration::from_secs(30)),
    Some(Duration::from_secs(5)),
    ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()),
)
.build()
.await?;
# Ok(())
# }
```

API token partial form (input token parts separately):

```rust,no_run
use pve_sdk_rs::ClientOption;

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let _client = ClientOption::new("pve.example.com")
    .api_token_partial("root", "pam", "ci", "token-secret")
    .build()
    .await?;
# Ok(())
# }
```

Username/password login:

```rust,no_run
use pve_sdk_rs::ClientOption;

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .password("root@pam", "secret-password")
    .build()
    .await?;
client.connect().await?;
# Ok(())
# }
```

Load auth from environment (`ClientAuth::from_env`):

```rust,no_run
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let auth = ClientAuth::from_env()?;
let client = ClientOption::new("pve.example.com")
    .auth(auth)
    .build()
    .await?;
client.connect().await?;
# Ok(())
# }
```

Env layout by `PVE_AUTH_METHOD`:

```bash
# API_TOKEN
PVE_AUTH_METHOD=API_TOKEN
PVE_API_TOKEN='root@pam!ci=token-secret'

# API_TOKEN_PARTIAL
PVE_AUTH_METHOD=API_TOKEN_PARTIAL
PVE_API_TOKEN_USER='root'
PVE_API_TOKEN_REALM='pam'
PVE_API_TOKEN_ID='ci'
PVE_API_TOKEN_SECRET='token-secret'

# USERNAME_PASSWORD
PVE_AUTH_METHOD=USERNAME_PASSWORD
PVE_USERNAME='root@pam'
PVE_PASSWORD='secret-password'
# Optional:
# PVE_OTP='123456'
# PVE_REALM='pam'
# PVE_TFA_CHALLENGE='...'
```

By default, `ClientOption::new`:

- uses `https`
- uses port `8006`
- skips TLS certificate validation
- has no HTTP timeout limits
- expects `host` to be hostname/IP only; set port via `.port(...)`

You can override TLS verification:

```rust,no_run
# use pve_sdk_rs::ClientOption;
# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .api_token("token")
    .insecure_tls(false)
    .build()
    .await?;
# Ok(())
# }
```

You can also set request/connect timeouts:

```rust,no_run
use std::time::Duration;
use pve_sdk_rs::ClientOption;

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let _client = ClientOption::new("pve.example.com")
    .api_token("root@pam!ci=token-secret")
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(5))
    .build()
    .await?;
# Ok(())
# }
```

Examples are available in `examples/` and all use `ClientAuth::from_env()`.

`list_all_guests` (supports `.env` via `dotenvy`):

```bash
# .env
PVE_HOST=10.0.0.2
PVE_PORT=8006
PVE_INSECURE_TLS=true
PVE_AUTH_METHOD=API_TOKEN
PVE_API_TOKEN='root@pam!ci=token-secret'

cargo run --example list_all_guests
```

`list_resources`:

```bash
export PVE_HOST=10.0.0.2
export PVE_PORT=8006
export PVE_INSECURE_TLS=true
export PVE_AUTH_METHOD=API_TOKEN
export PVE_API_TOKEN='root@pam!ci=token-secret'

cargo run --example list_resources
```

`create_qemu_vm`:

```bash
export PVE_HOST=10.0.0.2
export PVE_NODE=pve-node-1
export PVE_AUTH_METHOD=API_TOKEN
export PVE_API_TOKEN='root@pam!ci=token-secret'
# Optional: PVE_STORAGE, PVE_BRIDGE, PVE_MEMORY_MB, PVE_CORES, PVE_VMID

cargo run --example create_qemu_vm
```

`vzdump_backup`:

```bash
export PVE_HOST=10.0.0.2
export PVE_NODE=pve-node-1
export PVE_AUTH_METHOD=API_TOKEN
export PVE_API_TOKEN='root@pam!ci=token-secret'
# And one of: PVE_BACKUP_VMID=100 or PVE_BACKUP_ALL=1

cargo run --example vzdump_backup
```

More docs:

- SDK guides: `pve_sdk_docs/README.md`
- Raw PVE API references: `pve_docs/README.md`
