# 04 - 核心操作

## 连接与基本信息

```rust,no_run
# use pve_sdk_rs::{ClientAuth, ClientOption};
# async fn run() -> Result<(), pve_sdk_rs::PveError> {
# let client = ClientOption::new("pve.example.com")
#     .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
#     .build().await?;
let version = client.connect_with_version().await?;
let nodes = client.node().list().await?;
println!("version={} nodes={}", version.version, nodes.len());
# Ok(())
# }
```

## 列出所有 QEMU / LXC

```rust,no_run
# use pve_sdk_rs::{ClientAuth, ClientOption};
# async fn run() -> Result<(), pve_sdk_rs::PveError> {
# let client = ClientOption::new("pve.example.com")
#     .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
#     .build().await?;
for node in client.node().list().await? {
    let qemus = client.qemu().list(&node.node, Some(true)).await?;
    let lxcs = client.lxc().list(&node.node).await?;
    println!("node={} qemu={} lxc={}", node.node, qemus.len(), lxcs.len());
}
# Ok(())
# }
```

## 创建任务型请求（返回 UPID）

很多写操作返回 `UPID`，例如：

- `client.qemu().create_with`
- `client.qemu().start_with` / `client.qemu().stop_with`
- `client.backup().vzdump_with`

拿到 `UPID` 后建议使用 `client.task().wait_with_options`。

## 存储上传

- 使用 `client.storage().upload_with` 或 `client.storage().upload_file`
- 当前实现为流式上传，更适合大文件

## Access / Datacenter / Raw fallback

```rust,no_run
# use pve_sdk_rs::{ClientAuth, ClientOption, PveParams};
# async fn run() -> Result<(), pve_sdk_rs::PveError> {
# let client = ClientOption::new("pve.example.com")
#     .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
#     .build().await?;
let _users = client.access().users().await?;
let _dc = client.datacenter().config().await?;
let _raw = client.raw().get("/cluster/resources", Some(&PveParams::new())).await?;
# Ok(())
# }
```

Access 用户/组/ACL 管理（增删改）：

```rust,no_run
# use pve_sdk_rs::{ClientAuth, ClientOption};
# use pve_sdk_rs::types::access::{AccessCreateUserRequest, AccessSetAclRequest};
# async fn run() -> Result<(), pve_sdk_rs::PveError> {
# let client = ClientOption::new("pve.example.com")
#     .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
#     .build().await?;
let mut user = AccessCreateUserRequest::new("devops@pve");
user.password = Some("change-me".to_string());
client.access().create_user_with(&user).await?;

let mut acl = AccessSetAclRequest::new("/vms", "PVEVMAdmin");
acl.users = Some("devops@pve".to_string());
client.access().set_acl_with(&acl).await?;

client.access().delete_user("devops@pve").await?;
# Ok(())
# }
```

ACL 参数校验说明（SDK 会在本地先校验，再发请求）：

- `set_acl_with`：必须包含 `path`、`roles`，并且至少包含一个主体（`users/groups/tokens`）
- `delete_acl_with`：必须包含 `path`，并且至少包含一个目标（`roles/users/groups/tokens`）
- 校验失败会返回 `PveError::InvalidArgument`，便于在调用侧快速定位参数问题
