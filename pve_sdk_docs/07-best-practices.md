# 07 - 最佳实践

## 1) 生产环境优先安全配置

- 把 `.insecure_tls(false)` 作为生产默认
- 使用可信 CA 或受信证书

## 2) API Token 最小权限

- 为不同场景创建不同 token
- 按资源路径分配最小 ACL
- 定期轮转 secret

## 3) 用户名密码仅用于必要场景

- 自动化任务优先 API Token
- 若使用 `USERNAME_PASSWORD`，请妥善保管密码和 OTP

## 4) 任务接口必须带超时

- 调 `wait_for_task_with_options` 时设置 `timeout`
- 超时后拉 `task_log`，不要盲目无限重试

## 5) 配置 HTTP 超时

- 推荐设置 `.connect_timeout(...)` 和 `.timeout(...)`
- 避免网络异常时请求长期挂起

## 6) 日志避免泄露密钥

- 不打印完整 `PVE_API_TOKEN`
- 不把 `.env` 提交到仓库

## 7) 统一认证入口

- 推荐统一使用 `ClientAuth::from_env()`
- 让部署环境只改变量，不改代码

## 8) ACL 写操作先做参数自检

- `AccessSetAclRequest` 需要 `path + roles + (users/groups/tokens 至少一个)`
- `AccessDeleteAclRequest` 需要 `path + (roles/users/groups/tokens 至少一个)`
- 这类错误优先在本地修正，避免把无效 ACL 请求发到服务端
