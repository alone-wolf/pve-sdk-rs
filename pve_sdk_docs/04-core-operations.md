# 04 - 核心操作

## 连接与基本信息

```rust,no_run
# use pve_sdk_rs::{ClientAuth, ClientOption};
# async fn run() -> Result<(), pve_sdk_rs::PveError> {
# let client = ClientOption::new("pve.example.com")
#     .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
#     .build().await?;
let version = client.connect_with_version().await?;
let nodes = client.nodes().await?;
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
for node in client.nodes().await? {
    let qemus = client.qemu_list(&node.node, Some(true)).await?;
    let lxcs = client.lxc_list(&node.node).await?;
    println!("node={} qemu={} lxc={}", node.node, qemus.len(), lxcs.len());
}
# Ok(())
# }
```

## 创建任务型请求（返回 UPID）

很多写操作返回 `UPID`，例如：

- `qemu_create_with`
- `qemu_start_with` / `qemu_stop_with`
- `vzdump_backup_with`

拿到 `UPID` 后建议使用 `wait_for_task_with_options`。

## 存储上传

- 使用 `storage_upload_with` 或 `storage_upload_file`
- 当前实现为流式上传，更适合大文件
