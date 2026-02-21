# PVE API 使用文档（索引）

这组文档用于快速上手 Proxmox VE API（`/api2/json`）。

## 文档清单

- `01-auth-and-request.md`：认证、请求格式、鉴权头。
- `02-cluster-and-node.md`：集群与节点查询接口。
- `03-qemu-vm.md`：QEMU/KVM 虚拟机常见操作。
- `04-lxc.md`：LXC 容器常见操作。
- `05-storage-and-backup.md`：存储与备份相关接口。
- `06-tasks-and-errors.md`：异步任务、日志和错误处理。

## 使用约定

- Base URL：`https://<pve-host>:8006/api2/json`
- 读请求通常用 `GET`，写操作通常用 `POST`/`PUT`/`DELETE`
- 大部分写操作返回 `UPID`（异步任务 ID），需要轮询任务状态
- 具体参数、可选值、版本差异请以 API Viewer 为准：
  - `https://pve.proxmox.com/pve-docs/api-viewer/`

## 本次补充范围

本目录已在原有内容上补充了：

- 各核心路径的必填参数、常用可选参数
- 常见返回字段（尤其是列表项字段）
- `POST` 返回 `UPID` 的任务追踪落地方式

字段依据：`api-viewer/apidata.js`（`proxmox/pve-docs`，拉取时间：`2026-02-20`）。

## 建议阅读顺序

1. 先看认证与请求格式（`01`）
2. 再看资源查询（`02`）
3. 按场景看 VM/LXC/存储（`03~05`）
4. 最后补齐任务与错误处理（`06`）

## 高频场景导航

- 登录与鉴权
  - Ticket/Cookie：见 `01-auth-and-request.md`
  - API Token：见 `01-auth-and-request.md`
- 资源发现
  - 集群与节点：`02-cluster-and-node.md`
  - VM/LXC 列表：`03-qemu-vm.md` / `04-lxc.md`
- 生命周期管理
  - VM 开关机/快照：`03-qemu-vm.md`
  - LXC 开关机/快照：`04-lxc.md`
- 数据与备份
  - 存储上传/卷管理：`05-storage-and-backup.md`
  - vzdump 备份：`05-storage-and-backup.md`
- 任务追踪
  - `UPID` 轮询与失败诊断：`06-tasks-and-errors.md`

## 参数风格说明

- `node`：节点名（如 `pve1`）
- `vmid`：虚拟机/容器 ID（整数）
- `storage`：存储 ID（如 `local-lvm`、`local`、`nfs-backup`）
- `snapname`：快照名称（字符串）
- `UPID`：任务唯一 ID（多数写操作返回）

## 维护建议

- PVE 升级后优先核对以下路径：
  - `/access/ticket`
  - `/nodes/{node}/qemu` 与 `/nodes/{node}/lxc`
  - `/nodes/{node}/storage/{storage}/upload`
  - `/nodes/{node}/vzdump`
- 若发现参数变化，优先更新各文档中的“API Viewer 字段补充”章节。
