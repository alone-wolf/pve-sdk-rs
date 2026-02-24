# 02 - ClientOption 与 ClientAuth

## 核心模型

- `ClientOption`：只负责“配置”
- `PveClient`：只负责“调用 API”
- `ClientAuth`：认证描述（无认证、token、票据、用户名密码）

## 为什么 `build()` 是异步

`ClientOption::build().await` 需要兼容 `ClientAuth::Password`。  
该模式下会先请求 `/access/ticket`，拿到 ticket + csrf，再返回可用的 `PveClient`。

## ClientOption 常用字段

- `host`：PVE 主机名/IP（不带端口、不带路径）
- `port`：默认 `8006`
- `https`：默认 `true`
- `insecure_tls`：默认 `true`（开发方便，生产建议关掉）
- `timeout`：请求总超时（默认不限制）
- `connect_timeout`：连接超时（默认不限制）
- `auth`：`ClientAuth`

## ClientAuth 变体

- `None`
- `ApiToken(String)`：完整 token 字符串
- `ApiTokenPartial { user, realm, token_id, secret }`
- `Ticket { ticket, csrf }`
- `Password { username, password, otp, realm, tfa_challenge }`

## 推荐初始化方式

```rust,no_run
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let client = ClientOption::new("pve.example.com")
    .auth(ClientAuth::ApiToken("root@pam!ci=token-secret".to_string()))
    .build()
    .await?;
client.connect().await?;
# Ok(())
# }
```

如果你偏好一次性传全参，也可使用 `ClientOption::all_with_timeouts(...)`。

## 连接探活

- `connect().await`：只验证可达性
- `connect_with_version().await`：验证可达性并返回版本信息
