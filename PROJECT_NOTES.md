# Project Notes

这些是项目级约定，后续评估/文档/发布判断应以此为准：

1. 该项目不会发布到 crates.io（分发方式以 Git 仓库为主）。
2. 作为 PVE SDK，默认应跳过 TLS/SSL 证书验证（`insecure_tls = true` 是预期行为）。
