# 08 - 迁移说明

## 背景

SDK 已统一为 `ClientOption` 初始化，减少多个构造入口造成的维护成本。

## 迁移原则

- 原来直接在 `PveClient` 里传 host/port/auth 的方式，迁移到 `ClientOption`
- 构建客户端统一改成异步：`build().await`

## 常见迁移映射

- 旧：`PveClient::with_api_token(...)`
- 新：`ClientOption::new(...).api_token(...).build().await`

- 旧：`PveClient::with_ticket(...)`
- 新：`ClientOption::new(...).ticket(...).build().await`

- 旧：`PveClient::login_with_password...`
- 新：`ClientOption::new(...).password(...).build().await`

- 旧：多处分支手写 env 解析
- 新：`ClientAuth::from_env()`

## 一次性重构建议

1. 先把初始化入口统一到一个函数
2. 替换为 `ClientOption` 链式配置
3. 最后把认证收敛到 `ClientAuth::from_env()`
