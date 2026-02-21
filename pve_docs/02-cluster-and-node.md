# 02 - 集群与节点接口

## 1) 常用查询入口

- 版本：`GET /version`
- 节点列表：`GET /nodes`
- 集群状态：`GET /cluster/status`
- 集群资源总览：`GET /cluster/resources`

示例：

```bash
curl -k "https://<pve-host>:8006/api2/json/version" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 2) 节点维度接口

- 节点状态：`GET /nodes/{node}/status`
- 节点任务列表：`GET /nodes/{node}/tasks`
- 节点网络信息：`GET /nodes/{node}/network`

示例：

```bash
curl -k "https://<pve-host>:8006/api2/json/nodes/pve1/status" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 3) 建议的调用模式

- 首先调用 `/nodes` 获取 `node` 名称
- 再按 `node` 路由访问 VM/LXC/存储等子资源
- 若做全局面板，优先用 `/cluster/resources` 聚合展示

## 4) API Viewer 字段补充（`2026-02-20`）

### 核心查询接口

- `GET /version`
  - 参数：无
  - 返回字段：`console`, `release`, `repoid`, `version`
- `GET /nodes`
  - 参数：无
  - 列表项字段（常用）：`node`, `status`, `cpu`, `mem`, `maxmem`, `uptime`
- `GET /cluster/status`
  - 参数：无
  - 列表项字段（常用）：`type`, `name`, `id`, `nodeid`, `online`, `quorate`
- `GET /cluster/resources`
  - 可选参数：`type`
  - 列表项字段（常用）：`id`, `type`, `node`, `status`, `cpu`, `mem`, `maxmem`, `disk`, `maxdisk`, `vmid`

### 节点相关接口

- `GET /nodes/{node}/status`
  - 必填参数：`node`
  - 返回字段（常用）：`cpu`, `memory`, `loadavg`, `pveversion`, `current-kernel`
- `GET /nodes/{node}/tasks`
  - 必填参数：`node`
  - 常用过滤参数：`limit`, `start`, `since`, `until`, `statusfilter`, `typefilter`, `userfilter`, `vmid`
  - 列表项字段：`upid`, `type`, `status`, `starttime`, `endtime`, `user`
- `GET /nodes/{node}/network`
  - 必填参数：`node`
  - 可选参数：`type`
  - 列表项字段（常用）：`iface`, `type`, `active`, `autostart`, `address`, `cidr`, `gateway`, `mtu`

## 5) 典型查询流程（建议）

1. `GET /version`：确认 API 版本与环境信息
2. `GET /nodes`：拿到可用节点集合与在线状态
3. `GET /cluster/resources?type=vm`：拉取全局 VM/LXC 资源视图
4. `GET /nodes/{node}/status`：按节点补齐 CPU/内存/负载细节

示例：

```bash
curl -sk "https://<pve-host>:8006/api2/json/cluster/resources?type=vm" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 6) VMID 分配建议

- 获取下一个可用 VMID：`GET /cluster/nextid`
- 可选传入 `vmid` 参数做校验式检查（是否可用）

示例：

```bash
curl -sk "https://<pve-host>:8006/api2/json/cluster/nextid" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 7) 节点任务过滤示例

`GET /nodes/{node}/tasks` 支持较丰富过滤：

- 时间：`since`, `until`
- 分页：`start`, `limit`
- 来源：`source=archive|active|all`
- 条件：`statusfilter`, `typefilter`, `userfilter`, `vmid`, `errors`

示例：

```bash
curl -sk "https://<pve-host>:8006/api2/json/nodes/pve1/tasks?source=all&limit=100&errors=1" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```
