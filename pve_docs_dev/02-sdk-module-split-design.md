# pve-sdk-rs 功能拆分与组合改进设计（基于 PVE 官方模块）

> 状态同步（2026-02-25）：本文最初是重构设计稿；其中 `client.raw()`、grouped API 与 Access/Datacenter 基础能力已落地。

## 1. 背景与问题

当前 SDK 将大部分能力集中在 `src/client.rs`，导致：

- 文件体积过大，阅读和定位成本高
- 新增接口时容易在同一文件产生冲突
- 模块边界不清晰，难以按 PVE 官方功能域演进
- 测试粒度偏粗，难做按域回归
- 使用体验上“方法平铺过长”，自动补全噪声较高

目标是把当前“单体 Client 方法集合”重构为“按官方模块拆分 + 可组合调用”的结构。

---

## 2. 设计目标

1. **按官方功能域拆分**：与 `pve_docs/01~06` 对齐，并为后续 Access/Firewall/HA/SDN/Ceph 预留位。
2. **保持兼容**：短期不破坏现有 `PveClient` 入口与行为。
3. **提升可扩展性**：新增模块时做到“低耦合、低改动”。
4. **提升易用性**：支持分组调用（如 `client.qemu().start(...)`），减少平铺方法噪声。
5. **提升测试质量**：按模块组织单测与集成测试。

---

## 3. 模块拆分基线（对齐官方文档）

基于当前已有的文档模块，定义第一层 API 域：

- `access`：认证票据 + users/groups/roles/ACL/token 管理
- `cluster`：`/cluster/*`
- `node`：`/nodes/{node}/*` 中通用节点能力
- `qemu`：`/nodes/{node}/qemu/*`
- `lxc`：`/nodes/{node}/lxc/*`
- `storage`：`/storage` 与 `/nodes/{node}/storage/*`
- `backup`：`/nodes/{node}/vzdump`
- `task`：`/nodes/{node}/tasks/*` 与等待器

后续增量模块（P1/P2）：

- `firewall`
- `ha`
- `replication`
- `sdn`
- `ceph`
- `subscription`

---

## 4. 目标代码组织

建议从“单文件平铺”调整为以下结构：

```text
src/
  lib.rs
  error.rs

  core/
    mod.rs
    transport.rs        # send/send_multipart/execute/url/路径规范化
    auth.rs             # Auth、header 注入、ticket/token 逻辑
    client_inner.rs     # host/port/timeout/http 持有与共享状态

  client/
    mod.rs              # PveClient 对外 façade
    option.rs           # ClientOption / ClientAuth
    raw.rs              # raw_get/post/put/delete fallback

  api/
    mod.rs
    access.rs
    cluster.rs
    node.rs
    qemu.rs
    lxc.rs
    storage.rs
    backup.rs
    task.rs

  types/
    mod.rs
    common.rs
    access.rs
    cluster.rs
    node.rs
    qemu.rs
    lxc.rs
    storage.rs
    backup.rs
    task.rs
```

说明：

- 当前 `requests.rs` / `models.rs` 建议逐步迁入 `types/*`（允许过渡期保留 re-export）。
- `core/*` 只做基础能力，不包含具体业务 endpoint。

---

## 5. 组合式 API 设计

### 5.1 对外入口

保留主入口 `PveClient`，新增按域访问器：

```rust
let client = ClientOption::new("pve.example.com").auth(auth).build().await?;

let nodes = client.cluster().nodes().await?;
let qemus = client.qemu().list("pve1", Some(true)).await?;
let upid = client.qemu().start("pve1", 220, &req).await?;
let status = client.task().wait("pve1", &upid, &wait_opts).await?;
```

### 5.2 访问器实现方式

每个域对象仅持有对 `ClientInner` 的借用或 `Arc` 引用：

- `ClusterApi<'a> { inner: &'a ClientInner }`
- `QemuApi<'a> { inner: &'a ClientInner }`
- `TaskApi<'a> { inner: &'a ClientInner }`

优点：

- 无重复持有连接配置
- 无额外网络状态复制
- 模块间可复用统一 transport/auth 逻辑

### 5.3 兼容策略

在过渡期保留旧方法：

- 旧：`client.qemu_start_with(...)`
- 新：`client.qemu().start_with(...)`

旧方法内部仅做转发，后续可标记 `#[deprecated]`（非强制）。

---

## 6. 核心抽象边界

### 6.1 Transport 层（跨模块共享）

提供统一接口：

- `send_json(method, path, query, form)`
- `send_multipart(method, path, form)`
- `raw_json(method, path, query, form) -> serde_json::Value`

并集中处理：

- URL 拼接
- `data` envelope 解包
- 状态码错误包装
- 鉴权头注入

### 6.2 Auth 层

集中在 `core/auth.rs`：

- `Auth::ApiToken` / `Auth::Ticket`
- 写请求 CSRF 判定
- ticket 请求与刷新入口

### 6.3 类型层

每个模块定义各自 request/response，减少超大 `requests.rs/models.rs`。

---

## 7. 面向“官方全功能覆盖”的扩展机制

为避免每次缺接口都改主 Client，建议引入两层策略：

1. **typed first**：核心高频接口提供强类型 API。
2. **raw fallback**：所有未封装接口可先走 `client.raw().get/post/...`。

这样可以保证：

- 官方新接口出现时，SDK 立即“可达”
- 后续再逐步补齐 typed 体验，不阻塞业务

---

## 8. 迁移计划（低风险分期）

## Phase 0：仅重排，不改行为

- 拆出 `core/transport`、`core/auth`
- `src/client.rs` 仅保留 façade 和转发
- 全量测试保持通过

## Phase 1：引入组合式入口

- 增加 `client.cluster()/qemu()/lxc()/storage()/task()`
- 保留旧平铺 API（内部转发）
- 文档新增“新旧调用对照”

## Phase 2：类型分域

- 将 `requests/models` 分拆进 `types/*`
- 旧类型路径 re-export 保持兼容

## Phase 3：补齐缺口模块

- 优先 `firewall/ha/sdn/ceph`，并继续补深各域 typed 覆盖
- 每个模块：typed + example + docs + tests

---

## 9. 测试与质量门禁

- 单元测试按域存放：`api/<domain>.rs` 内部 tests
- 关键跨域路径保留集成测试：认证、任务等待、上传
- 发布门禁不变：
  - `cargo fmt --all -- --check`
  - `cargo check --all-targets`
  - `cargo test`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo check --examples`

---

## 10. 文档与示例同步策略

每次模块迁移/新增后同步：

- `README.md`：入口与最小示例
- `pve_sdk_docs/02-*`：ClientOption + 组合式入口
- `pve_sdk_docs/04/05/06`：按模块更新 API 示例
- `examples/`：优先展示 `client.<domain>()` 风格

---

## 11. 风险与规避

- **风险：兼容性破坏**
  - 规避：旧 API 保留转发至少一个大版本
- **风险：拆分过程中行为漂移**
  - 规避：Phase 0 仅重排，先不改签名与语义
- **风险：类型迁移导致导出路径变化**
  - 规避：在 `lib.rs` 做兼容 re-export

---

## 12. 最小改造包状态

已完成：

1. `core/transport.rs` + `core/auth.rs` 拆分
2. `client.raw()` fallback
3. grouped API 入口（含 Access/Datacenter/Raw）
4. 兼容平铺 API + 文档示例同步

下一步建议：

1. 继续拆分 `src/client.rs` / `src/client_api.rs` 以降低单文件复杂度
2. 优先补 Firewall/HA 模块的最小 typed 覆盖
