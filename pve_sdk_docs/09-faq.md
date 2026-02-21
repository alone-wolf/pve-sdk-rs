# 09 - FAQ

## Q1: 设置了 `PVE_API_TOKEN` 还是 401？

先检查：

- 格式是否完整：`user@realm!tokenid=secret`
- token 是否有目标资源 ACL
- token 是否被禁用/过期

## Q2: API Token 需要再传用户名吗？

不需要单独传用户名参数。  
用户名已经包含在 token 格式的 `user@realm` 部分里。

## Q3: 为什么 `build()` 变成异步？

因为 `ClientAuth::Password` 需要在构建时请求 `/access/ticket`。

## Q4: `connect()` 和第一次 API 调用有什么区别？

- `connect()`：显式探活，尽早暴露认证/网络问题
- 直接调用业务接口：也能工作，但失败更晚出现

## Q5: `PVE_HOST` 该填什么？

建议填主机名/IP，不带路径。  
端口由 `PVE_PORT` 管理（默认 8006）。

## Q6: 生产要不要 `PVE_INSECURE_TLS=true`？

不建议。  
生产建议用受信证书并设置 `.insecure_tls(false)`。
