# pve-sdk-rs 文档中心

这套文档面向 SDK 使用者，重点讲 `ClientOption + ClientAuth`。

如果你想看 Proxmox VE 原始 REST API 字段细节，请看仓库里的 `pve_docs/`。

## 阅读顺序

1. `01-getting-started.md`：先跑通最小示例
2. `02-client-option-and-auth.md`：理解客户端初始化模型
3. `03-auth-methods-env.md`：按环境变量切换认证方式
4. `04-core-operations.md`：常用资源操作
5. `05-task-and-error-handling.md`：任务轮询与错误处理
6. `06-examples-guide.md`：快速使用现有 examples
7. `07-best-practices.md`：上线前建议
8. `08-migration-notes.md`：老代码迁移
9. `09-faq.md`：常见问题

## 快速导航

- 只想先连上：`01` + `03`
- 只想用 `.env` 管理认证：`03`
- 只想看如何查 VM/LXC：`04` + `06`
- 遇到 401/任务失败：`05` + `09`
