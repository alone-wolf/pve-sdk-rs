# 03 - 环境变量认证模式

## 标准入口

使用：

```rust,no_run
use pve_sdk_rs::ClientAuth;
# fn run() -> Result<(), pve_sdk_rs::PveError> {
let _auth = ClientAuth::from_env()?;
# Ok(())
# }
```

它会读取 `PVE_AUTH_METHOD` 并按模式解析其它变量。

## 模式 1：API_TOKEN

```bash
PVE_AUTH_METHOD=API_TOKEN
PVE_API_TOKEN='root@pam!ci=token-secret'
```

要求 `PVE_API_TOKEN` 是完整格式：  
`<user>@<realm>!<tokenid>=<secret>`

## 模式 2：API_TOKEN_PARTIAL

```bash
PVE_AUTH_METHOD=API_TOKEN_PARTIAL
PVE_API_TOKEN_USER='root'
PVE_API_TOKEN_REALM='pam'
PVE_API_TOKEN_ID='ci'
PVE_API_TOKEN_SECRET='token-secret'
```

SDK 会自动组装成完整 token。

## 模式 3：USERNAME_PASSWORD

```bash
PVE_AUTH_METHOD=USERNAME_PASSWORD
PVE_USERNAME='root@pam'
PVE_PASSWORD='secret-password'
# 可选
# PVE_OTP='123456'
# PVE_REALM='pam'
# PVE_TFA_CHALLENGE='...'
```

## 说明

当前 `ClientAuth::from_env()` 固定读取 `PVE_*` 前缀。  
建议将不同环境（dev/staging/prod）放到不同 `.env` 文件中管理。
