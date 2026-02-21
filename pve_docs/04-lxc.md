# 04 - LXC 容器接口

## 1) 列表与详情

- 列表：`GET /nodes/{node}/lxc`
- 配置：`GET /nodes/{node}/lxc/{vmid}/config`
- 运行状态：`GET /nodes/{node}/lxc/{vmid}/status/current`

## 2) 创建与修改

- 创建容器：`POST /nodes/{node}/lxc`
- 修改配置：`PUT /nodes/{node}/lxc/{vmid}/config`

常见参数（示例）：

- `vmid`, `hostname`
- `ostemplate`
- `memory`, `cores`
- `rootfs`, `net0`

## 3) 生命周期操作

- 开机：`POST /nodes/{node}/lxc/{vmid}/status/start`
- 关机：`POST /nodes/{node}/lxc/{vmid}/status/shutdown`
- 断电：`POST /nodes/{node}/lxc/{vmid}/status/stop`
- 重启：`POST /nodes/{node}/lxc/{vmid}/status/reboot`

## 4) 快照与迁移

- 快照列表/创建：`GET|POST /nodes/{node}/lxc/{vmid}/snapshot`
- 快照回滚：`POST /nodes/{node}/lxc/{vmid}/snapshot/{snapname}/rollback`
- 迁移：`POST /nodes/{node}/lxc/{vmid}/migrate`

> LXC 与 QEMU 的路由结构相似，SDK 设计时可抽象通用生命周期接口。

## 5) API Viewer 字段补充（`2026-02-20`）

### 创建与配置

- `POST /nodes/{node}/lxc`
  - 必填：`node`, `vmid`, `ostemplate`
  - 常用可选：`hostname`, `memory`, `cores`, `rootfs`, `net[n]`, `swap`, `onboot`, `unprivileged`
  - 返回：`string`（通常是任务 `UPID`）
- `GET /nodes/{node}/lxc/{vmid}/config`
  - 必填：`node`, `vmid`
  - 可选：`current`, `snapshot`
  - 返回（常用字段）：`hostname`, `memory`, `cores`, `rootfs`, `net[n]`, `mp[n]`, `swap`, `onboot`, `tags`
- `PUT /nodes/{node}/lxc/{vmid}/config`
  - 必填：`node`, `vmid`
  - 返回：`null`

### 运行状态与生命周期

- `GET /nodes/{node}/lxc/{vmid}/status/current`
  - 返回（常用字段）：`status`, `cpu`, `mem`, `maxmem`, `disk`, `maxdisk`, `netin`, `netout`, `uptime`
- `POST /status/start`
  - 可选：`debug`, `skiplock`
- `POST /status/shutdown`
  - 可选：`timeout`, `forceStop`
- `POST /status/stop`
  - 可选：`overrule-shutdown`, `skiplock`
- `POST /status/reboot`
  - 可选：`timeout`

以上生命周期 `POST` 接口均为：必填 `node`, `vmid`，返回 `string`（`UPID`）。

### 快照与迁移

- `POST /nodes/{node}/lxc/{vmid}/snapshot`
  - 必填：`node`, `vmid`, `snapname`
  - 可选：`description`
- `POST /nodes/{node}/lxc/{vmid}/snapshot/{snapname}/rollback`
  - 必填：`node`, `vmid`, `snapname`
  - 可选：`start`
- `POST /nodes/{node}/lxc/{vmid}/migrate`
  - 必填：`node`, `vmid`, `target`
  - 常用可选：`online`, `restart`, `target-storage`, `bwlimit`, `timeout`

以上接口返回均为 `string`（`UPID`）。

## 6) 参数写法示例（高频）

### 根盘 `rootfs`

```text
rootfs=local-lvm:8
```

### 网络参数 `net[n]`

```text
net0=name=eth0,bridge=vmbr0,ip=dhcp
```

### 挂载点 `mp[n]`

```text
mp0=local-lvm:20,mp=/data
```

## 7) 端到端流程示例（创建 -> 开机 -> 查询）

```bash
# 1) 创建
UPID_CREATE=$(curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/lxc" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -d "vmid=320" \
  -d "hostname=ct-320" \
  -d "ostemplate=local:vztmpl/debian-12-standard_12.0-1_amd64.tar.zst" \
  -d "memory=1024" \
  -d "cores=1" \
  -d "rootfs=local-lvm:8" \
  -d "net0=name=eth0,bridge=vmbr0,ip=dhcp" | jq -r '.data')

# 2) 开机
UPID_START=$(curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/lxc/320/status/start" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" | jq -r '.data')

# 3) 查询状态
curl -sk "https://<pve-host>:8006/api2/json/nodes/pve1/lxc/320/status/current" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 8) LXC 修改配置建议

- 修改配置：`PUT /nodes/{node}/lxc/{vmid}/config`
- 并发保护：建议带 `digest`
- 删除旧字段：使用 `delete`（逗号分隔）

示例：

```bash
curl -sk -X PUT "https://<pve-host>:8006/api2/json/nodes/pve1/lxc/320/config" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -d "memory=2048" \
  -d "swap=512"
```

## 9) 迁移注意点

- `POST /nodes/{node}/lxc/{vmid}/migrate`
  - 必填：`target`
  - 在线迁移：`online=1`
  - 重启迁移：`restart=1`（结合 `timeout`）
  - 存储映射：`target-storage`
