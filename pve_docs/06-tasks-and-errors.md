# 06 - 任务跟踪与错误处理

## 1) 异步任务模型

PVE 的多数写操作会立即返回一个 `UPID`，真正执行在后台任务中完成。

常用接口：

- 查询任务状态：`GET /nodes/{node}/tasks/{upid}/status`
- 查询任务日志：`GET /nodes/{node}/tasks/{upid}/log`

## 2) 轮询模式（推荐）

1. 调用写接口，拿到 `UPID`
2. 每 1~3 秒轮询 `/status`
3. 当任务结束后检查：
   - `status`（是否 `stopped`）
   - `exitstatus`（是否 `OK`）
4. 若失败，读取 `/log` 定位原因

## 3) 常见错误分类

- `401 Unauthorized`：未认证或凭据失效
- `403 Forbidden`：权限不足（ACL/Token 权限不够）
- `5xx`：节点内部异常、参数冲突、底层命令失败等

## 4) SDK 侧建议

- 将 `UPID` 封装为统一任务句柄
- 标准化重试策略（仅对幂等读取或可安全重试场景）
- 在错误对象中保留：HTTP 状态码、PVE 原始错误、UPID、任务日志摘要

## 5) API Viewer 字段补充（`2026-02-20`）

### 节点任务列表

- `GET /nodes/{node}/tasks`
  - 必填：`node`
  - 常用过滤：`limit`, `start`, `since`, `until`, `statusfilter`, `typefilter`, `userfilter`, `vmid`, `errors`
  - 列表项字段：`upid`, `type`, `status`, `starttime`, `endtime`, `user`, `node`

### 单任务状态与日志

- `GET /nodes/{node}/tasks/{upid}/status`
  - 必填：`node`, `upid`
  - 返回字段：`status`, `exitstatus`, `starttime`, `pid`, `type`, `user`
- `GET /nodes/{node}/tasks/{upid}/log`
  - 必填：`node`, `upid`
  - 可选：`start`, `limit`, `download`
  - 列表项字段：`n`（行号）, `t`（日志文本）

### 错误处理落地建议

- `exitstatus != OK` 时，优先读取 `/log` 并附带最后 N 行日志
- 对返回 `UPID` 的接口统一走任务轮询，不要只看发起请求是否 `200`
- 对可重试操作，建议把 `UPID` 与请求参数一起记录，便于幂等重放/排障

## 6) Bash 轮询模板（可直接复用）

```bash
# 入参: NODE, UPID, AUTH_HEADER
NODE="pve1"
UPID="<upid>"
AUTH_HEADER="Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"

while true; do
  STATUS_JSON=$(curl -sk "https://<pve-host>:8006/api2/json/nodes/${NODE}/tasks/${UPID}/status" -H "$AUTH_HEADER")
  STATUS=$(echo "$STATUS_JSON" | jq -r '.data.status')
  EXITSTATUS=$(echo "$STATUS_JSON" | jq -r '.data.exitstatus // empty')

  if [ "$STATUS" = "stopped" ]; then
    echo "task done: exitstatus=${EXITSTATUS}"
    if [ "$EXITSTATUS" != "OK" ]; then
      echo "task failed, tail log:"
      curl -sk "https://<pve-host>:8006/api2/json/nodes/${NODE}/tasks/${UPID}/log?start=0&limit=200" -H "$AUTH_HEADER"
      exit 1
    fi
    break
  fi

  sleep 2
done
```

## 7) Rust SDK 轮询抽象建议

```rust
pub struct TaskResult {
    pub upid: String,
    pub status: String,
    pub exitstatus: Option<String>,
}

pub async fn wait_task(client: &PveClient, node: &str, upid: &str) -> anyhow::Result<TaskResult> {
    loop {
        let resp = client.get_task_status(node, upid).await?;
        if resp.status == "stopped" {
            return Ok(TaskResult {
                upid: upid.to_string(),
                status: resp.status,
                exitstatus: resp.exitstatus,
            });
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
```

## 8) 错误分层模型（推荐）

- `TransportError`
  - DNS/TLS/超时/连接失败
- `HttpError`
  - 非 2xx 响应，保留 `status_code` 与响应体
- `ApiError`
  - 2xx 但 `UPID` 最终失败（`exitstatus != OK`）
- `DecodeError`
  - JSON 结构变化或字段反序列化失败

## 9) 观测与审计建议

- 日志最小字段：`request_id`, `node`, `path`, `method`, `vmid`, `upid`, `status_code`, `exitstatus`
- 对写操作记录完整参数快照（注意脱敏）
- 对失败任务留存日志尾部（如最后 200 行）用于定位
