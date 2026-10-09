# setproxy 使用指南

## 命令详解

### 设置代理

```bash
setproxy <host> <port>
```

示例：
```bash
setproxy 127.0.0.1 7890
setproxy 192.168.1.100 8080
```

设置后，以下环境变量会在当前 shell 中生效：
- `http_proxy=http://host:port`
- `https_proxy=http://host:port`
- `all_proxy=socks5://host:port`

### WSL2 模式

在 WSL2 中自动检测 Windows 宿主机 IP：

```bash
setproxy wsl           # 使用默认端口 8090
setproxy wsl 7890      # 指定端口
```

### 查看状态

```bash
setproxy status
```

输出当前 `http_proxy`、`https_proxy`、`all_proxy` 的值。

### 清除代理

```bash
setproxy clear
```

unset 所有代理环境变量。

## 使用场景

### Docker 拉取镜像

```bash
setproxy 127.0.0.1 7890
docker pull ubuntu:latest
setproxy clear
```

### git clone

```bash
setproxy wsl
git clone https://github.com/torvalds/linux.git
setproxy clear
```

### pip 安装

```bash
setproxy 127.0.0.1 7890
pip install tensorflow torch
setproxy clear
```

### cargo build

```bash
setproxy 127.0.0.1 7890
cargo build
setproxy clear
```

## Shell 集成示例

在 `~/.zshrc` 或 `~/.bashrc` 中添加便捷别名：

```bash
alias px='setproxy 127.0.0.1 7890'
alias nopx='setproxy clear'
alias pxwsl='setproxy wsl'
```

## 故障排查

**代理未生效**

```bash
setproxy status   # 确认变量已设置
```

确保已 source shell function：
```bash
source ~/.zshrc
```

**WSL 检测失败**

手动指定宿主机 IP：
```bash
setproxy 172.20.0.1 7890
```

**某些工具不识别小写环境变量**

```bash
export HTTP_PROXY=$http_proxy
export HTTPS_PROXY=$https_proxy
```
