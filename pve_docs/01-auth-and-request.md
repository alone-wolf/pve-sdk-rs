# 01 - 认证与请求格式

## 1) API 入口与响应结构

- 入口：`https://<pve-host>:8006/api2/json`
- 响应结构通常为：

```json
{
  "data": {"...": "..."}
}
```

## 2) Ticket 登录（用户名/密码）

接口：`POST /access/ticket`

常见参数：

- `username`：例如 `root@pam`
- `password`

返回中重点字段：

- `ticket`：用于 `PVEAuthCookie`
- `CSRFPreventionToken`：写操作时放到请求头

示例：

```bash
curl -k -X POST "https://<pve-host>:8006/api2/json/access/ticket" \
  -d "username=root@pam" \
  --data-urlencode "password=<your-password>"
```

后续请求头示例：

```bash
-H "Cookie: PVEAuthCookie=<ticket>" \
-H "CSRFPreventionToken: <csrf-token>"
```

> 说明：只读 `GET` 请求通常不要求 `CSRFPreventionToken`；写请求建议始终带上。

## 3) API Token（推荐自动化）

Header 格式：

```text
Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>
```

示例：

```bash
curl -k "https://<pve-host>:8006/api2/json/nodes" \
  -H "Authorization: PVEAPIToken=root@pam!ci=<token-secret>"
```

Token 场景通常不需要 CSRF 头，适合 CI/CD、服务账号。

## 4) TLS 与安全建议

- 生产环境不要长期使用 `-k`（忽略证书校验）
- 为自动化账号最小化权限（ACL）
- 给 Token 设置独立权限与轮转策略

## 5) API Viewer 字段补充（`2026-02-20`）

### `POST /access/ticket`

- 必填参数：`username`, `password`
- 常用可选：`otp`, `tfa-challenge`, `realm`, `new-format`
- 返回字段：`ticket`, `CSRFPreventionToken`, `username`, `clustername`

### Header 使用建议

- Cookie 登录链路：
  - `Cookie: PVEAuthCookie=<ticket>`
  - 写请求加 `CSRFPreventionToken`
- Token 登录链路：
  - `Authorization: PVEAPIToken=...`
  - 通常不需要 `CSRFPreventionToken`

## 6) 请求编码与 Content-Type

- 常规参数提交：通常使用 `application/x-www-form-urlencoded`（`curl -d`）
- 文件上传接口：使用 `multipart/form-data`（例如 `/storage/.../upload`）
- 只读查询参数：使用 URL Query（例如 `?type=vm&limit=100`）

## 7) 认证模板（可直接复用）

### 7.1 Ticket + Cookie

```bash
# 1) 登录，提取 ticket/csrf
AUTH_JSON=$(curl -sk -X POST "https://<pve-host>:8006/api2/json/access/ticket" \
  -d "username=<user>@<realm>" \
  --data-urlencode "password=<password>")

TICKET=$(echo "$AUTH_JSON" | jq -r '.data.ticket')
CSRF=$(echo "$AUTH_JSON" | jq -r '.data.CSRFPreventionToken')

# 2) GET 示例
curl -sk "https://<pve-host>:8006/api2/json/nodes" \
  -H "Cookie: PVEAuthCookie=$TICKET"

# 3) POST 示例
curl -sk -X POST "https://<pve-host>:8006/api2/json/nodes/<node>/qemu/<vmid>/status/start" \
  -H "Cookie: PVEAuthCookie=$TICKET" \
  -H "CSRFPreventionToken: $CSRF"
```

### 7.2 API Token

```bash
curl -sk "https://<pve-host>:8006/api2/json/nodes" \
  -H "Authorization: PVEAPIToken=<user>@<realm>!<tokenid>=<token-secret>"
```

## 8) 鉴权失败处理建议

- `401 Unauthorized`
  - 检查用户名/密码、Token Secret、Token 是否已删除或禁用
  - Ticket 模式下重新调用 `POST /access/ticket`
- `403 Forbidden`
  - 认证通过但权限不足，检查 ACL（路径权限、角色、继承）

## 9) SDK 层推荐抽象

- `AuthProvider`
  - `TicketAuthProvider`：维护 `PVEAuthCookie + CSRF`
  - `TokenAuthProvider`：固定 `Authorization` 头
- `RequestSigner`
  - 根据 HTTP Method 自动决定是否注入 `CSRFPreventionToken`
- `AuthRefresh`
  - 当接口返回 `401` 时触发一次重登并重试（仅幂等请求自动重试）
