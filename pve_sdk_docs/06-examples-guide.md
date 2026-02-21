# 06 - Examples 指南

## 运行方式

```bash
cargo run --example <example_name>
```

## list_all_guests

- 文件：`examples/list_all_guests.rs`
- 作用：遍历所有节点并列出 QEMU/LXC
- 必需变量：
  - `PVE_HOST`
  - `PVE_AUTH_METHOD`
- 认证相关变量：见 `03-auth-methods-env.md`
- 可选：
  - `PVE_PORT`（默认 8006）
  - `PVE_INSECURE_TLS`（默认 true）

## list_resources

- 文件：`examples/list_resources.rs`
- 作用：查询版本、节点、集群 VM 资源
- 必需变量：
  - `PVE_HOST`
  - `PVE_AUTH_METHOD`
- 认证相关变量：见 `03-auth-methods-env.md`
- 可选：
  - `PVE_PORT`
  - `PVE_INSECURE_TLS`

## create_qemu_vm

- 文件：`examples/create_qemu_vm.rs`
- 作用：创建 VM 并按需自动启动
- 必需变量：
  - `PVE_HOST`
  - `PVE_AUTH_METHOD`
  - `PVE_NODE`
- 认证相关变量：见 `03-auth-methods-env.md`
- 常用可选：
  - `PVE_VMID`
  - `PVE_STORAGE`
  - `PVE_BRIDGE`
  - `PVE_MEMORY_MB`
  - `PVE_CORES`

## vzdump_backup

- 文件：`examples/vzdump_backup.rs`
- 作用：触发 vzdump 备份并等待结束
- 必需变量：
  - `PVE_HOST`
  - `PVE_AUTH_METHOD`
  - `PVE_NODE`
- 认证相关变量：见 `03-auth-methods-env.md`
- 你必须二选一：
  - `PVE_BACKUP_VMID`
  - `PVE_BACKUP_ALL=1`
