# 05 - 存储与备份接口

## 1) 存储查询

- 集群存储：`GET /storage`
- 节点存储：`GET /nodes/{node}/storage`
- 存储内容：`GET /nodes/{node}/storage/{storage}/content`

示例：

```bash
curl -k "https://<pve-host>:8006/api2/json/nodes/pve1/storage/local/content" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 2) 存储内容管理

- 上传内容（如 ISO/模板）：`POST /nodes/{node}/storage/{storage}/upload`
- 分配磁盘镜像：`POST /nodes/{node}/storage/{storage}/content`
- 删除卷/内容：`DELETE /nodes/{node}/storage/{storage}/content/{volume}`

## 3) 备份（vzdump）

- 发起备份：`POST /nodes/{node}/vzdump`

常见参数（示例）：

- `vmid`（可单个或多个）
- `mode`（常见 `snapshot`）
- `storage`
- `compress`

示例：

```bash
curl -k -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/vzdump" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -d "vmid=120" \
  -d "mode=snapshot" \
  -d "storage=backup-nfs"
```

> 备份任务同样返回 `UPID`，请在任务接口跟踪进度。

## 4) API Viewer 字段补充（`2026-02-20`）

### 存储查询

- `GET /storage`
  - 可选：`type`
  - 列表项字段：`storage`
- `GET /nodes/{node}/storage`
  - 必填：`node`
  - 可选：`content`, `enabled`, `format`, `storage`, `target`
  - 列表项字段（常用）：`storage`, `type`, `active`, `enabled`, `used`, `avail`, `total`, `shared`, `content`
- `GET /nodes/{node}/storage/{storage}/content`
  - 必填：`node`, `storage`
  - 可选：`content`, `vmid`
  - 列表项字段（常用）：`volid`, `format`, `size`, `used`, `vmid`, `ctime`, `notes`

### 存储内容管理

- `POST /nodes/{node}/storage/{storage}/content`
  - 必填：`node`, `storage`, `vmid`, `filename`, `size`
  - 可选：`format`
  - 返回：`string`（`UPID`）
- `POST /nodes/{node}/storage/{storage}/upload`
  - 必填：`node`, `storage`, `content`, `filename`
  - 可选：`checksum`, `checksum-algorithm`, `tmpfilename`
  - 返回：`string`（`UPID`）
- `DELETE /nodes/{node}/storage/{storage}/content/{volume}`
  - 路径必填：`node`, `storage`, `volume`
  - 可选：`delay`
  - 返回：`string`（`UPID`）

### 备份（vzdump）

- `POST /nodes/{node}/vzdump`
  - 常用参数：`vmid` 或 `all`, `mode`, `storage`, `compress`, `mailnotification`, `mailto`, `notes-template`
  - 返回：`string`（`UPID`）
- 实务建议：
  - 单机备份常用 `vmid=<id>`
  - 全量任务常用 `all=1`
  - 不要同时混用冲突参数（以 API Viewer 当前版本约束为准）

## 5) 上传与卷分配细节

### `POST /nodes/{node}/storage/{storage}/upload`

- `content` 枚举：`iso` / `vztmpl` / `import`
- 校验参数：`checksum` + `checksum-algorithm`（`md5`/`sha1`/`sha224`/`sha256`/`sha384`/`sha512`）

示例（上传 ISO）：

```bash
curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/storage/local/upload" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -F "content=iso" \
  -F "filename=@./debian-12.5.0-amd64-netinst.iso"
```

### `POST /nodes/{node}/storage/{storage}/content`

- 必填：`filename`, `size`, `vmid`
- 可选：`format`
- `format` 枚举：`raw` / `qcow2` / `subvol` / `vmdk`

示例（为 VM 分配卷）：

```bash
curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/storage/local-lvm/content" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -d "vmid=220" \
  -d "filename=vm-220-disk-1" \
  -d "size=32G" \
  -d "format=raw"
```

## 6) 备份参数建议（vzdump）

- 关键参数：
  - 范围：`vmid` 或 `all=1`
  - 模式：`mode=snapshot|suspend|stop`
  - 压缩：`compress=0|1|gzip|lzo|zstd`
  - 目标：`storage=<backup-storage>`
- 通知参数（兼容字段）：`mailnotification`, `mailto`

示例（单机快照备份）：

```bash
curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/pve1/vzdump" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>" \
  -d "vmid=220" \
  -d "mode=snapshot" \
  -d "compress=zstd" \
  -d "storage=backup-nfs"
```

## 7) 删除卷的安全建议

- 删除前先 `GET /nodes/{node}/storage/{storage}/content` 确认 `volid`
- 删除调用：`DELETE /nodes/{node}/storage/{storage}/content/{volume}`
- 可选 `delay` 用于延迟删除

示例：

```bash
curl -sk -X DELETE "https://<pve-host>:8006/api2/json/nodes/pve1/storage/local-lvm/content/vm-220-disk-1" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```
