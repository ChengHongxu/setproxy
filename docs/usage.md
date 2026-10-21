# 使用指南

## 快速开始

### 设置代理

```bash
# 基本用法
setproxy 127.0.0.1 7890

# WSL2 自动检测宿主机 IP（默认端口 7890）
setproxy wsl

# WSL2 自定义端口
setproxy wsl 10809
```

### 查看状态

```bash
setproxy status
```

输出示例：
```
当前代理状态:
✓ http_proxy = http://127.0.0.1:7890
✓ https_proxy = http://127.0.0.1:7890
✓ all_proxy = socks5://127.0.0.1:7890
```

### 清除代理

```bash
setproxy clear
```

## 平台差异

### Linux / macOS

一键安装（会自动执行 `setproxy init`）：
```bash
curl -fsSL https://raw.githubusercontent.com/ChengHongxu/setproxy/main/install.sh | sh
source ~/.zshrc  # 或 ~/.bashrc
```

其他方式安装后，执行一次 `setproxy init` 并重开终端。

### Windows (PowerShell)

一键安装：
```powershell
iwr -useb https://raw.githubusercontent.com/ChengHongxu/setproxy/main/install.ps1 | iex
. $PROFILE
```

**注意**：Windows 上环境变量只对当前 PowerShell 会话生效，不影响系统全局代理设置。

### WSL2

在 WSL2 中，`setproxy wsl` 会自动检测宿主机 IP（通过 `/etc/resolv.conf`）。

## 实际使用场景

### 场景 1：开发时临时启用代理

```bash
# 启用代理访问 GitHub
setproxy 127.0.0.1 7890
git clone https://github.com/...

# 完成后清除
setproxy clear
```

### 场景 2：Docker 构建使用代理

```bash
# 设置代理
setproxy 127.0.0.1 7890

# Docker 会继承环境变量
docker build --build-arg http_proxy --build-arg https_proxy .
```

### 场景 3：WSL2 中使用宿主机代理

```bash
# 自动检测 Windows 宿主机 IP
setproxy wsl

# 现在可以访问外网
curl https://www.google.com
```

### 场景 4：apt/yum 使用代理

```bash
# 设置代理
setproxy 127.0.0.1 7890

# apt 会自动使用 http_proxy 环境变量
sudo apt update

# 完成后清除
setproxy clear
```

## 环境变量说明

`setproxy` 设置以下环境变量：

| 变量名 | 值 | 用途 |
|--------|-----|------|
| `http_proxy` | `http://host:port` | HTTP 请求代理 |
| `https_proxy` | `http://host:port` | HTTPS 请求代理 |
| `all_proxy` | `socks5://host:port` | SOCKS5 通用代理 |

**注意**：
- 大多数工具（curl、wget、git、apt）会自动识别这些变量
- 某些工具可能需要额外配置（如 Maven、Gradle）
- Windows 上这些变量不影响系统 IE 代理设置

## 故障排查

### 代理不生效

1. **检查环境变量是否已设置**：
   ```bash
   setproxy status
   ```

2. **确认代理服务器正在运行**：
   ```bash
   curl -x http://127.0.0.1:7890 https://www.google.com
   ```

3. **某些工具需要额外配置**：
   ```bash
   # Git 可能需要单独配置
   git config --global http.proxy http://127.0.0.1:7890
   ```

### WSL2 检测失败

如果 `setproxy wsl` 无法自动检测宿主机 IP：

1. **手动查看 IP**：
   ```bash
   cat /etc/resolv.conf
   ```

2. **手动指定**：
   ```bash
   setproxy <宿主机IP> 7890
   ```

### Windows 权限问题

如果 `install.ps1` 被阻止执行：

1. **临时允许脚本执行**：
   ```powershell
   Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
   iwr -useb https://raw.githubusercontent.com/ChengHongxu/setproxy/main/install.ps1 | iex
   ```

2. **或使用手动安装方式**

## 卸载

### Linux / macOS

```bash
# 删除二进制
rm ~/.local/bin/setproxy

# 从 shell 配置中移除（手动编辑）
vim ~/.zshrc  # 删除 # setproxy 和 eval "$(setproxy init zsh --print)" 两行
```

### Windows

```powershell
# 删除安装目录
Remove-Item -Recurse "$env:LOCALAPPDATA\setproxy"

# 从 PATH 中移除（手动在系统设置中操作）

# 从 PowerShell profile 中移除（手动编辑）
notepad $PROFILE  # 删除 # setproxy 和 setproxy init powershell --print 两行
```

## 高级用法

### 与其他工具集成

**在脚本中使用**：
```bash
#!/bin/bash
setproxy 127.0.0.1 7890
npm install
setproxy clear
```

**在 Makefile 中使用**：
```makefile
install:
	setproxy 127.0.0.1 7890 && npm install && setproxy clear
```

**在 CI/CD 中使用**：
```yaml
# .github/workflows/ci.yml
- name: Setup proxy
  run: |
    curl -fsSL https://raw.githubusercontent.com/ChengHongxu/setproxy/main/install.sh | sh
    source ~/.zshrc
    setproxy 127.0.0.1 7890
```

### 创建别名

在 `~/.zshrc` 或 `~/.bashrc` 中：
```bash
alias sp='setproxy 127.0.0.1 7890'
alias spw='setproxy wsl'
alias spc='setproxy clear'
```

## 更多帮助

- [GitHub Issues](https://github.com/ChengHongxu/setproxy/issues)
- [贡献指南](contributing.md)
- [变更日志](changelog.md)
