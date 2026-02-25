# pve-sdk-rs 功能补齐路线图（对齐 PVE 官方模块）

> 状态同步（2026-02-25）：已按当前实现更新，`README.md` 中声明的 Access/Datacenter/raw 能力均已落地。

## 目标

将当前 SDK 从“核心常用运维能力”提升到“可覆盖 PVE 官方主要模块”。

## 当前能力（已覆盖）

- 认证：API Token / Ticket / 用户名密码换票据
- 集群与节点基础：version、nodes、cluster status/resources/nextid、node tasks/network/status
- QEMU：列表、创建、配置、启停重启、快照、克隆、迁移
- LXC：列表、创建、配置、启停重启、快照、迁移
- Access 管理面：users/groups/roles/ACL/token 管理（含 ACL 参数预校验）
- Datacenter：config 读写
- 存储与备份：storage 查询、上传、删除卷、vzdump
- 异步任务：task status/log + wait helper
- 客户端扩展：`client.raw()` fallback（`raw_get/raw_post/raw_put/raw_delete`）

## 主要缺口（按官方模块维度）

- 网络/安全：Firewall 全量对象与规则管理
- 集群高级能力：HA、Replication、Cluster 管理操作
- 平台扩展模块：Ceph、SDN、Subscription/证书等
- Datacenter 扩展：除 config 外的更多全局能力（如更细粒度策略项）
- 深水区能力：QEMU/LXC 高级参数与更多运维接口的 typed 覆盖

## 分阶段计划

## P0（已完成）

- 新增 `raw_get/raw_post/raw_put/raw_delete` 公共方法（返回 `serde_json::Value`）
- 补齐 Access 基础：
  - users/groups/roles/ACL 查询与增删改
  - API token 管理（创建、禁用、删除）
- 补齐 Datacenter 常用接口：
  - datacenter config（读/改）
  - grouped API 入口（`client.datacenter()`）
- 提供 grouped API + 兼容平铺 API 并行模式，文档/examples 同步更新

验收标准：
- 常见“权限配置 + 自动化账号发放 + token 生命周期”可全程通过 SDK 完成。

## P1（下一阶段，2~3 个迭代）

- QEMU/LXC 深水区能力：
  - 更多配置项（磁盘扩容、模板相关、CPU/NUMA/高级参数）
  - 常见运维接口补齐（含更完整 query/body）
- 存储模块增强：
  - 更多 content 类型、导入/导出相关操作
- Task 体验增强：
  - 可选重试策略（仅幂等接口）
  - 日志抓取与失败诊断辅助

验收标准：
- VM/CT 全生命周期（创建→调整→迁移→备份）不需绕过 raw 接口。

## P2（第三阶段，按需扩展）

- Firewall 模块
- HA 模块
- Replication 模块
- Ceph 模块
- SDN 模块
- Subscription/证书等平台能力

验收标准：
- 官方主要章节均有对应 SDK 入口（至少 raw + typed 其一，核心链路 typed）。

## 工程与发布要求（每阶段都执行）

- 代码：`cargo fmt/check/test/clippy -- -D warnings` 全通过
- 文档：同步更新 `README.md`、`pve_sdk_docs/*`、`examples/*`
- 兼容性：优先增量扩展，避免破坏现有 `ClientOption`/`ClientAuth` 语义
- 示例：每新增模块至少 1 个最小可运行示例

## 近期执行建议（下一步）

1. 优先补 Firewall 与 HA 两个高频“平台级”模块（至少先给 raw + 最小 typed）
2. 扩展 QEMU/LXC 高级参数的 typed 覆盖并补对应示例
3. 在现有门禁上持续补充 mock 集成测试（401/5xx/任务异常路径）
