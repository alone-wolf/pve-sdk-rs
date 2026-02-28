# 05 - 任务与错误处理

## 任务轮询

写接口通常返回 `UPID`，建议模式：

1. 发起任务拿 `UPID`
2. `client.task().wait_with_options` 轮询
3. 对失败任务读取 `client.task().log_with`

```rust,no_run
# use std::time::Duration;
# use pve_sdk_rs::{ClientAuth, ClientOption};
# use pve_sdk_rs::types::task::WaitTaskOptions;
# async fn run() -> Result<(), pve_sdk_rs::PveError> {
# let client = ClientOption::new("pve.example.com")
#     .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
#     .build().await?;
# let upid = String::from("UPID:...");
let status = client
    .task()
    .wait_with_options(
        "pve1",
        &upid,
        &WaitTaskOptions {
            poll_interval: Duration::from_secs(2),
            timeout: Some(Duration::from_secs(600)),
        },
    )
    .await?;
println!("done: {:?}", status.exitstatus);
# Ok(())
# }
```

## 常见错误类型

- `InvalidBaseUrl`：host/port/scheme 拼接异常
- `InvalidArgument`：参数无效、认证字段缺失等
- `Http`：网络层失败
- `ApiStatus { status, body }`：PVE 返回非 2xx（401/403/5xx）
- `MissingCsrfToken`：ticket 写请求缺少 csrf
- `TaskFailed` / `TaskTimeout`：异步任务失败或超时

## 建议的错误处理策略

- `401/403`：先检查认证字符串与 ACL
- `5xx`：可做有限重试（注意幂等性）
- `TaskFailed`：抓取 `task_log` 辅助定位
