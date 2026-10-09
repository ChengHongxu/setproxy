# setproxy

<div align="center">

一个轻量、跨平台的命令行代理设置工具，支持在当前 shell 会话中快速配置 HTTP/HTTPS/SOCKS5 代理。

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

## ✨ 特性

- 🚀 **直接生效**: `setproxy 127.0.0.1 7890` 无需 `eval`，直接在当前 shell 中设置代理
- 🔄 **跨平台**: 支持 Linux、macOS、WSL2
- 🎯 **WSL 优化**: 自动检测 WSL2 宿主机 IP，无需手动输入
- 📊 **状态查看**: 实时查看当前代理配置
- 🧹 **一键清除**: 快速清除代理设置
- 📦 **零依赖**: 编译为单一静态二进制文件

## 🏗️ 设计原理

`setproxy` 由两部分组成：

| 组件 | 描述 |
|------|------|
| `_setproxy` | Rust 编译的二进制文件，负责计算并输出结构化结果 |
| `setproxy` | Shell function，调用 `_setproxy` 并执行 `export`/`unset` |

这样设计的原因：**子进程无法修改父进程的环境变量**，这是 UNIX 的基本限制。shell function 运行在当前 shell 进程中，因此可以直接 `export`。

## 📦 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/setproxy.git
cd setproxy

# 编译
pixi install
pixi run cargo build --release

# 安装 binary
cp target/release/_setproxy ~/.local/bin/

# 在 .zshrc 中加载 shell function
echo 'source ~/path/to/setproxy/shell/setproxy.zsh' >> ~/.zshrc
source ~/.zshrc
```

**或者**直接把 `shell/setproxy.zsh` 的内容复制到 `~/.zshrc`，更简洁。

## 🚀 快速开始

source 之后，直接用，无需 `eval`：

```bash
# 设置代理
setproxy 127.0.0.1 7890

# WSL2 自动检测宿主机 IP
setproxy wsl

# 指定 WSL 端口（默认 8090）
setproxy wsl 7890

# 查看状态
setproxy status

# 清除代理
setproxy clear
```

## 📖 命令说明

```
setproxy [HOST] [PORT]     设置代理
setproxy wsl [PORT]        WSL2 自动检测（默认端口 8090）
setproxy clear             清除代理
setproxy status            查看当前代理状态
```

设置时会同时配置：
- `http_proxy` / `https_proxy` → `http://host:port`
- `all_proxy` → `socks5://host:port`

## 🔧 dotfiles 集成

将 `_setproxy` 放到 `~/dotfiles/bin/`，`shell/setproxy.zsh` 放到 `~/dotfiles/shell/`，在 `.zshrc` 中 source：

```bash
# ~/.zshrc
source "$DOTFILES_DIR/shell/setproxy.zsh"
```

chezmoi 同步后，新设备上运行 `pixi run init` 自动完成安装。

## 🛠️ 开发

```bash
# 运行测试
pixi run cargo test

# 构建 release
pixi run cargo build --release

# 项目结构
src/
├── main.rs       # 入口
├── cli.rs        # CLI 参数
└── proxy.rs      # 代理逻辑（输出结构化文本）
shell/
├── setproxy.zsh  # Zsh function
└── setproxy.bash # Bash function
```

## 📄 许可证

MIT — 详见 [LICENSE](LICENSE)
