# 01 - 快速开始

## 目标

5 分钟内完成：

- 初始化客户端
- 验证连接
- 调一个只读接口

## 前置要求

- Rust 工具链可用（`cargo --version`）
- 可访问 PVE 管理地址
- 至少准备一种认证方式（推荐 API Token）

## 安装

使用 GitHub 最新主分支：

```toml
[dependencies]
pve-sdk-rs = { git = "https://github.com/alone-wolf/pve-sdk-rs.git", branch = "main" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## 最小示例

```rust,no_run
use pve_sdk_rs::{ClientAuth, ClientOption};

# async fn run() -> Result<(), pve_sdk_rs::PveError> {
let auth = ClientAuth::ApiToken("root@pam!ci=token-secret".to_string());
let client = ClientOption::new("pve.example.com")
    .port(8006)
    .insecure_tls(true)
    .auth(auth)
    .build()
    .await?;

let version = client.connect_with_version().await?;
println!("connected: {}", version.version);
# Ok(())
# }
```

## 推荐下一步

- 看 `02-client-option-and-auth.md`：理解 `build().await` 的原因
- 看 `03-auth-methods-env.md`：把认证改成环境变量驱动
- 跑 `examples/list_all_guests.rs` 做真实验证
