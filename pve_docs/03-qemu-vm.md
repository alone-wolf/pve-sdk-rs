# 03 - QEMU/KVM 虚拟机接口

## 1) 列表与详情

- 列表：`GET /nodes/{node}/qemu`
- 配置：`GET /nodes/{node}/qemu/{vmid}/config`
- 运行状态：`GET /nodes/{node}/qemu/{vmid}/status/current`

## 2) 创建与修改

- 创建 VM：`POST /nodes/{node}/qemu`
- 修改配置（同步）：`PUT /nodes/{node}/qemu/{vmid}/config`
- 修改配置（异步）：`POST /nodes/{node}/qemu/{vmid}/config`

常见参数（示例）：

- `vmid`, `name`
- `memory`, `cores`, `sockets`
- `net0`
- `scsi0` / `virtio0`（磁盘）

创建示例（最小示意，参数请按实际模板完善）：

```bash
curl -k -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/qemu" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -d "vmid=120" \
  -d "name=demo-vm" \
  -d "memory=4096" \
  -d "cores=2"
```

## 3) 生命周期操作

- 开机：`POST /nodes/{node}/qemu/{vmid}/status/start`
- 关机：`POST /nodes/{node}/qemu/{vmid}/status/shutdown`
- 断电：`POST /nodes/{node}/qemu/{vmid}/status/stop`
- 重启：`POST /nodes/{node}/qemu/{vmid}/status/reboot`
- 挂起：`POST /nodes/{node}/qemu/{vmid}/status/suspend`
- 恢复：`POST /nodes/{node}/qemu/{vmid}/status/resume`

## 4) 常见高级动作

- 快照列表/创建：`GET|POST /nodes/{node}/qemu/{vmid}/snapshot`
- 快照回滚：`POST /nodes/{node}/qemu/{vmid}/snapshot/{snapname}/rollback`
- 克隆：`POST /nodes/{node}/qemu/{vmid}/clone`
- 迁移：`POST /nodes/{node}/qemu/{vmid}/migrate`

> 大多数写操作返回 `UPID`，需在任务接口轮询完成状态。

## 5) API Viewer 字段补充（`2026-02-20`）

### 创建与配置

- `POST /nodes/{node}/qemu`
  - 必填：`node`, `vmid`
  - 常用可选：`name`, `memory`, `cores`, `sockets`, `cpu`, `net[n]`, `scsi[n]`, `virtio[n]`, `ostype`, `bios`, `agent`
  - 返回：`string`（通常是任务 `UPID`）
- `GET /nodes/{node}/qemu/{vmid}/config`
  - 必填：`node`, `vmid`
  - 可选：`current`, `snapshot`
  - 返回（常用字段）：`name`, `memory`, `cores`, `sockets`, `cpu`, `net[n]`, `scsi[n]`, `virtio[n]`, `boot`, `agent`, `tags`
- `PUT /nodes/{node}/qemu/{vmid}/config`
  - 必填：`node`, `vmid`
  - 返回：`null`
- `POST /nodes/{node}/qemu/{vmid}/config`
  - 必填：`node`, `vmid`
  - 返回：`string`（异步任务，适合 hotplug/存储变更）

### 运行状态与生命周期

- `GET /nodes/{node}/qemu/{vmid}/status/current`
  - 返回（常用字段）：`status`, `qmpstatus`, `cpu`, `mem`, `maxmem`, `netin`, `netout`, `diskread`, `diskwrite`, `uptime`
- `POST /status/start`
  - 可选：`skiplock`, `timeout`, `migration_network`, `targetstorage`
- `POST /status/shutdown`
  - 可选：`timeout`, `forceStop`, `skiplock`
- `POST /status/stop`
  - 可选：`timeout`, `overrule-shutdown`, `skiplock`
- `POST /status/reboot`
  - 可选：`timeout`
- `POST /status/suspend`
  - 可选：`todisk`, `statestorage`, `skiplock`
- `POST /status/resume`
  - 可选：`nocheck`, `skiplock`

以上生命周期 `POST` 接口均为：必填 `node`, `vmid`，返回 `string`（`UPID`）。

### 快照 / 克隆 / 迁移

- `POST /nodes/{node}/qemu/{vmid}/snapshot`
  - 必填：`node`, `vmid`, `snapname`
  - 可选：`description`, `vmstate`
- `POST /nodes/{node}/qemu/{vmid}/snapshot/{snapname}/rollback`
  - 必填：`node`, `vmid`, `snapname`
  - 可选：`start`
- `POST /nodes/{node}/qemu/{vmid}/clone`
  - 必填：`node`, `vmid`, `newid`
  - 常用可选：`name`, `target`, `storage`, `full`, `pool`, `snapname`
- `POST /nodes/{node}/qemu/{vmid}/migrate`
  - 必填：`node`, `vmid`, `target`
  - 常用可选：`online`, `with-local-disks`, `targetstorage`, `migration_network`, `bwlimit`

以上接口返回均为 `string`（`UPID`）。

## 6) 参数写法示例（高频）

### 网络参数 `net[n]`

```text
net0=virtio,bridge=vmbr0,firewall=1
```

### 磁盘参数 `scsi[n]` / `virtio[n]`

```text
scsi0=local-lvm:32
virtio0=local-lvm:64
```

说明：`STORAGE_ID:SIZE_IN_GiB` 可直接触发新卷分配。

### 引导顺序 `boot`

```text
boot=order=scsi0;net0
```

## 7) `POST /config` 与 `PUT /config` 的选择

- `PUT /nodes/{node}/qemu/{vmid}/config`
  - 同步接口，返回 `null`
  - 适合简单、即时可完成的配置变更
- `POST /nodes/{node}/qemu/{vmid}/config`
  - 异步接口，返回 `UPID`
  - 更适合 hotplug、磁盘分配、可能耗时的变更

实务上建议优先支持 `POST /config`，并统一走任务轮询。

## 8) 端到端流程示例（创建 -> 开机 -> 查询）

```bash
# 1) 创建
UPID_CREATE=$(curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/qemu" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -d "vmid=220" \
  -d "name=app-220" \
  -d "memory=4096" \
  -d "cores=2" \
  -d "net0=virtio,bridge=vmbr0" \
  -d "scsi0=local-lvm:32" | jq -r '.data')

# 2) 轮询创建任务
curl -sk "https://<pve-host>:8006/api2/json/nodes/pve1/tasks/$UPID_CREATE/status" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"

# 3) 开机
UPID_START=$(curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/qemu/220/status/start" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" | jq -r '.data')

# 4) 查询运行态
curl -sk "https://<pve-host>:8006/api2/json/nodes/pve1/qemu/220/status/current" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 9) 迁移与克隆注意点

- 迁移 `POST /migrate`
  - 必填 `target`
  - 在线迁移常用 `online=1`
  - 本地盘参与迁移时注意 `with-local-disks`
- 克隆 `POST /clone`
  - 必填 `newid`
  - 模板克隆常结合 `full` 与 `storage`
  - 跨节点克隆时 `target` 受共享存储约束
