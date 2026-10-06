# setproxy

<div align="center">

一个轻量、跨平台的命令行代理设置工具，支持在当前 shell 会话中快速配置 HTTP/HTTPS/SOCKS5 代理。

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

## ✨ 特性

- 🚀 **快速设置**: 一条命令即可在当前 shell 中设置代理
- 🔄 **跨平台**: 支持 Linux、macOS、WSL2
- 🎯 **WSL 优化**: 自动检测 WSL2 宿主机 IP，无需手动输入
- 📊 **状态查看**: 实时查看当前代理配置
- 🧹 **一键清除**: 快速清除代理设置
- 💡 **智能输出**: 自动检测管道模式，适配 `eval` 使用

## 📦 安装

### 使用 Pixi（推荐）

```bash
# 克隆仓库
git clone https://github.com/yourusername/setproxy.git
cd setproxy

# 使用 Pixi 安装依赖并编译
pixi install
pixi run cargo build --release

# 将编译好的二进制文件移动到 PATH 中
cp target/release/setproxy ~/.local/bin/
```

### 手动编译

```bash
# 确保已安装 Rust 1.80+
git clone https://github.com/yourusername/setproxy.git
cd setproxy
cargo build --release
cp target/release/setproxy ~/.local/bin/
```

## 🚀 快速开始

### 设置代理

```bash
# 基本用法
eval "$(setproxy 127.0.0.1 7890)"

# WSL2 自动检测宿主机 IP
eval "$(setproxy wsl)"

# 或指定 WSL 端口（默认 8090）
eval "$(setproxy wsl 7890)"
```

### 查看状态

```bash
setproxy status
```

### 清除代理

```bash
eval "$(setproxy clear)"
```

## 📖 使用说明

### 命令语法

```
setproxy [OPTIONS] [HOST] [PORT]
setproxy [COMMAND]

Commands:
  clear    清除代理设置
  status   显示当前代理状态
  wsl      WSL2 自动检测模式
  help     显示帮助信息

Arguments:
  [HOST]   代理服务器地址（如 192.168.1.1）
  [PORT]   代理服务器端口（如 8080）

Options:
  -h, --help     显示帮助信息
  -V, --version  显示版本信息
```

### 使用场景

#### 场景 1: 开发环境代理

```bash
# 启动本地代理工具（如 Clash）
# 然后在终端设置代理
eval "$(setproxy 127.0.0.1 7890)"

# 验证代理生效
curl -I https://www.google.com
```

#### 场景 2: WSL2 使用 Windows 代理

```bash
# WSL2 自动检测 Windows 宿主机 IP
eval "$(setproxy wsl)"

# 测试网络连接
curl -I https://www.google.com
```

#### 场景 3: 临时任务后清理

```bash
# 设置代理
eval "$(setproxy 192.168.1.1 8080)"

# 执行需要代理的任务
git clone https://github.com/some/repo.git

# 清除代理
eval "$(setproxy clear)"
```

### Shell 函数封装（可选）

为了更方便使用，可以在 `~/.zshrc` 或 `~/.bashrc` 中添加：

```bash
# 设置代理
proxy() {
    if [[ $# -eq 0 ]]; then
        # 无参数，使用 WSL 模式
        eval "$(setproxy wsl)"
    else
        eval "$(setproxy $@)"
    fi
}

# 清除代理
noproxy() {
    eval "$(setproxy clear)"
}

# 代理状态
proxystatus() {
    setproxy status
}
```

然后就可以简单使用：

```bash
proxy                    # WSL 模式
proxy 127.0.0.1 7890    # 指定地址
noproxy                  # 清除代理
proxystatus             # 查看状态
```

## 🔧 工作原理

`setproxy` 通过以下方式工作：

1. **智能输出检测**: 
   - 交互模式（直接运行）：显示彩色提示和使用说明
   - 管道模式（通过 `eval` 调用）：只输出纯净的 shell 命令

2. **环境变量设置**: 设置以下环境变量
   - `http_proxy`: HTTP 代理地址
   - `https_proxy`: HTTPS 代理地址  
   - `all_proxy`: SOCKS5 代理地址（用于其他协议）

3. **WSL2 自动检测**: 
   - 检测 `WSL_DISTRO_NAME` 环境变量确认 WSL 环境
   - 通过 `ip route` 命令获取默认网关（即 Windows 宿主机 IP）

## 🛠️ 开发

### 运行测试

```bash
pixi run cargo test
```

### 代码检查

```bash
pixi run cargo clippy
pixi run cargo fmt
```

### 项目结构

```
setproxy/
├── src/
│   ├── main.rs       # 主程序入口
│   ├── cli.rs        # CLI 参数定义
│   └── proxy.rs      # 代理设置逻辑
├── tests/
│   └── integration_test.rs  # 集成测试
├── Cargo.toml        # Rust 项目配置
├── pixi.toml         # Pixi 环境配置
└── README.md         # 项目文档
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- [clap](https://github.com/clap-rs/clap) - 强大的命令行参数解析
- [colored](https://github.com/mackwic/colored) - 终端彩色输出
- [anyhow](https://github.com/dtolnay/anyhow) - 优雅的错误处理

## 📞 联系方式

如有问题或建议，请通过以下方式联系：

- 提交 [Issue](https://github.com/yourusername/setproxy/issues)
- 发起 [Discussion](https://github.com/yourusername/setproxy/discussions)

---

<div align="center">

⭐ 如果这个项目对你有帮助，请给一个 Star！

</div>
