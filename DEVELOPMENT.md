# Setproxy 开发指南

## 🏗️ 项目结构

```
setproxy/
├── Cargo.toml              # Rust 项目配置
├── pixi.toml               # Pixi 开发环境配置
├── src/
│   ├── main.rs             # 程序入口
│   ├── cli.rs              # 命令行参数解析
│   └── proxy.rs            # 核心代理逻辑
├── tests/                  # 集成测试
├── .github/workflows/      # CI/CD 配置
│   ├── ci.yml             # 持续集成
│   └── release.yml        # 自动发布
├── README.md               # 项目说明
├── LICENSE                 # MIT 许可证
└── DEVELOPMENT.md          # 本文档
```

## 🚀 快速开始

### 方式 1：使用 Pixi（推荐）

```bash
cd ~/Workspace/setproxy

# 安装开发环境（Rust + Git）
pixi install

# 激活环境
pixi shell

# 构建项目
cargo build

# 运行测试
cargo test

# 运行程序
cargo run -- 192.168.1.1 8080
```

### 方式 2：使用系统 Rust

```bash
# 如果没有 Rust，先安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 构建和测试
cargo build
cargo test
```

## 📝 开发工作流

### 1. 代码格式化

```bash
# 格式化代码
cargo fmt

# 检查格式（CI 会运行）
cargo fmt -- --check
```

### 2. 代码检查

```bash
# 运行 Clippy 静态分析
cargo clippy

# 严格模式（不允许任何警告）
cargo clippy -- -D warnings
```

### 3. 运行测试

```bash
# 运行所有测试
cargo test

# 显示测试输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_proxy_config

# 生成测试覆盖率报告
cargo tarpaulin --out Html
```

### 4. 构建 Release 版本

```bash
# 优化构建
cargo build --release

# 构建后的二进制文件位于
./target/release/setproxy

# 查看大小
ls -lh target/release/setproxy

# 查看依赖
ldd target/release/setproxy  # Linux
otool -L target/release/setproxy  # macOS
```

## 🔧 使用 Pixi 任务

Pixi 提供了预定义的任务简化开发：

```bash
# 查看所有可用任务
pixi task list

# 运行任务
pixi run build              # 构建项目
pixi run build-release      # 构建 release 版本
pixi run test               # 运行测试
pixi run fmt                # 格式化代码
pixi run clippy             # 代码检查
pixi run run                # 运行程序
pixi run check              # 格式化 + 检查 + 测试（一键验证）
```

## 🐛 调试

### 使用 Rust 调试器

```bash
# 安装 rust-gdb 或 rust-lldb
rustup component add rust-gdb  # Linux
rustup component add rust-lldb # macOS

# 构建 debug 版本
cargo build

# 使用 GDB 调试
rust-gdb target/debug/setproxy

# 使用 LLDB 调试
rust-lldb target/debug/setproxy
```

### 使用 dbg! 宏

```rust
// 在代码中添加调试输出
let config = ProxyConfig::new(host, port);
dbg!(&config);  // 打印调试信息
```

### 环境变量调试

```bash
# 启用 Rust backtrace
RUST_BACKTRACE=1 cargo run -- 192.168.1.1 8080

# 完整 backtrace
RUST_BACKTRACE=full cargo run -- 192.168.1.1 8080
```

## 🧪 测试策略

### 单元测试

在每个模块中编写单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(1 + 1, 2);
    }
}
```

### 集成测试

在 `tests/` 目录创建集成测试：

```rust
// tests/integration_test.rs
use assert_cmd::Command;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("setproxy").unwrap();
    cmd.arg("--help").assert().success();
}
```

### 测试覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html --output-dir coverage
```

## 🎯 性能分析

### 编译大小优化

Cargo.toml 中已配置优化：

```toml
[profile.release]
opt-level = "z"      # 优化体积
lto = true           # 链接时优化
codegen-units = 1    # 更好的优化
strip = true         # 移除调试符号
panic = "abort"      # 减小二进制体积
```

### 进一步压缩

```bash
# 使用 UPX 压缩（可选）
upx --best --lzma target/release/setproxy

# 对比大小
ls -lh target/release/setproxy
```

## 🔨 交叉编译

### 安装交叉编译工具

```bash
# 添加目标平台
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# 安装 cross（推荐）
cargo install cross
```

### 编译不同平台

```bash
# Linux x64
cargo build --release --target x86_64-unknown-linux-gnu

# Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-gnu
```

### 一键构建所有平台

```bash
# 使用 Pixi 任务（需要配置 cross）
pixi run build-all
```

## 📦 发布流程

### 1. 更新版本号

编辑 `Cargo.toml`：

```toml
[package]
version = "0.2.0"  # 更新版本
```

### 2. 更新 CHANGELOG

创建 `CHANGELOG.md` 记录更新内容：

```markdown
## [0.2.0] - 2024-10-06

### Added
- 新功能：WSL2 自动检测

### Changed
- 改进：更好的错误提示

### Fixed
- 修复：某个 bug
```

### 3. 提交并打标签

```bash
# 提交更改
git add .
git commit -m "chore: bump version to 0.2.0"

# 创建标签
git tag -a v0.2.0 -m "Release v0.2.0"

# 推送
git push origin main
git push origin v0.2.0
```

### 4. 自动发布

推送标签后，GitHub Actions 会自动：
- 运行测试
- 构建所有平台的二进制文件
- 创建 GitHub Release
- 上传编译产物
- 发布到 crates.io（如果配置了 token）

## 🔐 CI/CD 配置

### GitHub Secrets

需要在 GitHub 仓库设置以下 Secrets：

1. **GITHUB_TOKEN**（自动提供，无需配置）
   - 用于创建 Release 和上传资源

2. **CARGO_TOKEN**（可选）
   - 用于发布到 crates.io
   - 获取：https://crates.io/settings/tokens
   - Settings → Secrets → New repository secret
   - Name: `CARGO_TOKEN`
   - Value: 你的 crates.io token

### CI 工作流

- **ci.yml**：每次 push 和 PR 时运行
  - 多平台测试（Linux、macOS、Windows）
  - 代码检查（clippy）
  - 格式检查（rustfmt）

- **release.yml**：创建标签时触发
  - 构建多平台二进制文件
  - 创建 GitHub Release
  - 上传编译产物

## 🎨 代码风格指南

### 命名规范

```rust
// 函数和变量：snake_case
fn set_proxy() {}
let proxy_config = ProxyConfig::new();

// 类型和 trait：PascalCase
struct ProxyConfig {}
trait ProxyHandler {}

// 常量：SCREAMING_SNAKE_CASE
const DEFAULT_PORT: u16 = 8080;
```

### 错误处理

```rust
// 使用 anyhow::Result
use anyhow::{Context, Result};

fn do_something() -> Result<()> {
    something_that_might_fail()
        .context("Failed to do something")?;
    Ok(())
}
```

### 文档注释

```rust
/// 设置代理环境变量
///
/// # Arguments
///
/// * `config` - 代理配置
///
/// # Examples
///
/// ```
/// let config = ProxyConfig::new("192.168.1.1".to_string(), 8080);
/// set_proxy(&config)?;
/// ```
pub fn set_proxy(config: &ProxyConfig) -> Result<()> {
    // ...
}
```

## 🤝 贡献指南

### 提交 Pull Request

1. Fork 仓库
2. 创建功能分支：`git checkout -b feature/my-feature`
3. 确保代码通过所有检查：`pixi run check`
4. 提交更改：`git commit -m "feat: add my feature"`
5. 推送到分支：`git push origin feature/my-feature`
6. 创建 Pull Request

### 提交信息规范

使用 Conventional Commits：

- `feat:` - 新功能
- `fix:` - Bug 修复
- `docs:` - 文档更新
- `style:` - 代码格式
- `refactor:` - 重构
- `test:` - 测试
- `chore:` - 构建/工具更改

示例：
```
feat: add WSL2 auto-detection
fix: handle empty host input
docs: update installation instructions
```

## 📚 学习资源

### Rust 学习

- [The Rust Book](https://doc.rust-lang.org/book/) - 官方教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 示例学习
- [Rustlings](https://github.com/rust-lang/rustlings) - 练习题

### CLI 开发

- [clap 文档](https://docs.rs/clap/) - 命令行解析
- [Command Line Applications in Rust](https://rust-cli.github.io/book/) - CLI 开发指南

### 其他资源

- [Rust 标准库](https://doc.rust-lang.org/std/)
- [docs.rs](https://docs.rs/) - Rust 包文档
- [crates.io](https://crates.io/) - Rust 包仓库

## 🐞 常见问题

### Q: 编译失败，提示找不到 cargo

**A:** 确保已安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Q: cross 编译失败

**A:** 使用 Docker 模式的 cross：

```bash
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

### Q: Windows 编译失败

**A:** 在 Windows 上，使用 MSVC 工具链：

```bash
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

### Q: 如何减小二进制文件大小？

**A:** 已在 Cargo.toml 配置优化，还可以：

```bash
# 移除调试符号
strip target/release/setproxy

# 使用 UPX 压缩
upx --best target/release/setproxy
```

---

**Happy Coding! 🦀**
