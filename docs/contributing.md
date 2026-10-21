# 开发者指南

## 环境要求

- Rust stable（通过 [rustup](https://rustup.rs) 安装）

## 常用命令

```bash
cargo build            # 调试构建
cargo build --release  # 发布构建（输出 target/release/setproxy）
cargo test             # 运行测试
cargo clippy           # 静态分析
cargo fmt              # 格式化
```

## 项目结构

```
setproxy/
├── src/
│   ├── main.rs      # 入口，命令分发
│   ├── cli.rs       # 命令行参数定义（clap）
│   ├── proxy.rs     # 核心逻辑：set/clear/status/wsl，按 --syntax 输出代码
│   └── shell.rs     # `init`：识别 shell、写启动文件、迁移旧配置；`--print` 原样输出 shell/init.*
├── shell/
│   ├── init.sh      # bash/zsh shell 函数（setproxy init zsh|bash --print 输出）
│   └── init.ps1     # PowerShell 函数（setproxy init powershell --print 输出）
├── docs/            # 文档
├── install.sh       # 一键安装脚本
├── Cargo.toml
└── .github/workflows/
    ├── ci.yml       # PR/push 测试
    └── release.yml  # tag 触发发布
```

## shell 函数与可执行文件

两者同名。函数通过 `command setproxy`（PowerShell 中为 `Get-Command -CommandType Application`）调用可执行文件，并传入隐藏参数 `--syntax posix|powershell`。可执行文件收到 `--syntax` 才输出代码；没收到说明是用户直接运行（还没接入 shell），这时只在 stderr 打印提示并以 1 退出。

## 提交规范

使用 Conventional Commits：

```
feat: 新功能
fix: bug 修复
docs: 文档更新
chore: 构建/工具链
test: 测试相关
```

## 发布流程

1. 更新 `Cargo.toml` 中的版本号
2. 更新 `docs/changelog.md`
3. 提交并打 tag：`git tag v0.2.0`
4. 推送 tag：`git push origin v0.2.0`
5. GitHub Actions 自动构建并创建 Release
