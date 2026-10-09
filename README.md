# setproxy

一个轻量、跨平台的代理设置工具。**直接使用，无需 `eval`。**

## 工作原理

`setproxy` 由两部分组成：
- `_setproxy`：Rust 二进制，负责计算输出结构化结果（可跨平台编译）
- `setproxy`：Shell function（bash/zsh/PowerShell），source 后直接调用，可 export 环境变量

这样设计是因为子进程无法修改父进程环境变量（UNIX 限制），这也是 nvm、conda、pyenv 等所有同类工具的标准做法。

## 安装

### Linux / macOS
```bash
curl -fsSL https://raw.githubusercontent.com/<your-username>/setproxy/main/install.sh | sh
```

### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/<your-username>/setproxy/main/install.ps1 | iex
```

安装后重启终端或运行：
- **Linux/macOS**: `source ~/.zshrc` (或 `~/.bashrc`)
- **Windows**: `. $PROFILE`

### 手动安装
从 [Releases](https://github.com/<your-username>/setproxy/releases) 下载对应平台的二进制文件。

## 用法

```bash
setproxy 127.0.0.1 7890    # 设置代理
setproxy wsl               # WSL2 自动检测宿主机 IP
setproxy wsl 7890          # WSL2 指定端口
setproxy status            # 查看状态
setproxy clear             # 清除代理
```

设置后生效的环境变量：
- `http_proxy` / `https_proxy` → `http://host:port`
- `all_proxy` → `socks5://host:port`

详细用法见 [docs/usage.md](docs/usage.md)。

## 从源码构建

```bash
git clone https://github.com/<your-username>/setproxy
cd setproxy
cargo build --release
cp target/release/_setproxy ~/.local/bin/
```

## 许可证

MIT
