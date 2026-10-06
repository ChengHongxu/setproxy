# setproxy 使用示例

本文档提供 `setproxy` 工具的详细使用示例和最佳实践。

## 📋 目录

- [基础用法](#基础用法)
- [进阶用法](#进阶用法)
- [Shell 集成](#shell-集成)
- [常见场景](#常见场景)
- [故障排查](#故障排查)

## 基础用法

### 1. 设置 HTTP 代理

最基本的用法是指定代理服务器的 IP 和端口：

```bash
# 设置代理到本地 7890 端口
eval "$(setproxy 127.0.0.1 7890)"

# 设置代理到局域网服务器
eval "$(setproxy 192.168.1.100 8080)"
```

**输出示例：**
```
✓ 代理已启用: http://127.0.0.1:7890
```

### 2. WSL2 自动检测

在 WSL2 环境中，可以自动检测 Windows 宿主机 IP：

```bash
# 使用默认端口 8090
eval "$(setproxy wsl)"

# 指定端口
eval "$(setproxy wsl 7890)"
```

**输出示例：**
```
✓ 代理已启用: http://192.168.10.1:8090
```

### 3. 查看代理状态

```bash
setproxy status
```

**输出示例：**
```
🌐 当前代理状态:

  http_proxy = http://127.0.0.1:7890
  https_proxy = http://127.0.0.1:7890
  all_proxy = socks5://127.0.0.1:7890
```

### 4. 清除代理

```bash
eval "$(setproxy clear)"
```

**输出示例：**
```
✓ 代理已清除
```

## 进阶用法

### 临时代理作用域

使用子 shell 限制代理作用域：

```bash
# 只在子 shell 中启用代理
(
    eval "$(setproxy 127.0.0.1 7890)"
    git clone https://github.com/some/repo.git
    npm install
)
# 退出子 shell 后，代理自动失效
```

### 条件代理设置

根据网络环境自动选择：

```bash
#!/bin/bash

# 检测是否在公司网络
if ping -c 1 -W 1 company-proxy.local &> /dev/null; then
    eval "$(setproxy company-proxy.local 8080)"
    echo "使用公司代理"
elif [[ -n "$WSL_DISTRO_NAME" ]]; then
    eval "$(setproxy wsl)"
    echo "使用 WSL 代理"
else
    echo "直连网络"
fi
```

### 代理链配合

```bash
# 设置主代理
eval "$(setproxy 127.0.0.1 7890)"

# 对特定命令使用不同代理
http_proxy=http://192.168.1.100:8080 curl https://example.com
```

## Shell 集成

### Zsh 配置

在 `~/.zshrc` 中添加：

```bash
# ========== 代理管理函数 ==========

# 设置代理
proxy() {
    if [[ $# -eq 0 ]]; then
        # 优先使用 WSL 模式
        if [[ -n "$WSL_DISTRO_NAME" ]]; then
            eval "$(setproxy wsl)"
        else
            eval "$(setproxy 127.0.0.1 7890)"
        fi
    elif [[ $1 == "wsl" ]]; then
        eval "$(setproxy wsl ${2:-8090})"
    else
        eval "$(setproxy $@)"
    fi
}

# 清除代理
noproxy() {
    eval "$(setproxy clear)"
}

# 查看代理状态
proxystatus() {
    setproxy status
}

# 测试代理连接
proxytest() {
    echo "测试代理连接..."
    if curl -I --connect-timeout 5 https://www.google.com &> /dev/null; then
        echo "✓ 代理连接正常"
    else
        echo "✗ 代理连接失败"
    fi
}

# 自动代理（根据环境）
autoproxy() {
    if [[ -n "$WSL_DISTRO_NAME" ]]; then
        proxy wsl
    else
        proxy 127.0.0.1 7890
    fi
}
```

**使用示例：**

```bash
proxy                 # 自动检测环境
proxy wsl             # WSL 模式
proxy 192.168.1.1 8080  # 指定地址
noproxy               # 清除
proxystatus           # 查看状态
proxytest             # 测试连接
autoproxy             # 自动配置
```

### Bash 配置

在 `~/.bashrc` 中添加类似配置（语法相同）。

### 持久化配置（不推荐）

如果需要在每个新 shell 中自动启用代理（通常不推荐）：

```bash
# ~/.zshrc
# 自动启用代理（仅在确实需要时使用）
if [[ -n "$WSL_DISTRO_NAME" ]]; then
    eval "$(setproxy wsl)" 2>/dev/null
fi
```

## 常见场景

### 场景 1: Docker 拉取镜像

```bash
# 设置代理
eval "$(setproxy 127.0.0.1 7890)"

# 拉取镜像
docker pull ubuntu:latest

# 清除代理（可选）
eval "$(setproxy clear)"
```

### 场景 2: Git 克隆大型仓库

```bash
# 临时使用代理克隆
eval "$(setproxy wsl)"
git clone https://github.com/torvalds/linux.git
eval "$(setproxy clear)"
```

### 场景 3: Python pip 安装

```bash
# 设置代理
eval "$(setproxy 127.0.0.1 7890)"

# 安装包
pip install tensorflow torch

# 清除代理
eval "$(setproxy clear)"
```

### 场景 4: Node.js npm/yarn

```bash
eval "$(setproxy 127.0.0.1 7890)"
npm install
# 或
yarn install
eval "$(setproxy clear)"
```

### 场景 5: Rust Cargo

```bash
eval "$(setproxy 127.0.0.1 7890)"
cargo build
eval "$(setproxy clear)"
```

### 场景 6: 开发环境启动脚本

创建 `start-dev.sh`：

```bash
#!/bin/bash

# 启动代理
eval "$(setproxy wsl)"

# 启动开发服务
docker-compose up -d
npm run dev

# 注意：脚本退出后代理在当前 shell 中仍然有效
# 如需清除，手动运行: eval "$(setproxy clear)"
```

## 故障排查

### 问题 1: 代理未生效

**症状：** 设置代理后网络请求仍然失败

**排查步骤：**

```bash
# 1. 检查代理状态
setproxy status

# 2. 检查代理服务是否运行
curl -I --proxy http://127.0.0.1:7890 https://www.google.com

# 3. 测试直连（绕过代理）
unset http_proxy https_proxy all_proxy
curl -I https://www.google.com
```

**解决方案：**
- 确保代理软件（Clash/V2Ray 等）正在运行
- 检查端口号是否正确
- 确认使用了 `eval` 命令

### 问题 2: WSL 模式检测失败

**症状：** `setproxy wsl` 报错找不到宿主机 IP

**排查步骤：**

```bash
# 检查是否在 WSL 环境
echo $WSL_DISTRO_NAME

# 手动获取网关 IP
ip route | grep default | awk '{print $3}'
```

**解决方案：**
- 如果输出为空，手动指定 IP：`eval "$(setproxy 192.168.x.x 8090)"`
- 检查 WSL2 网络配置

### 问题 3: 某些工具不识别代理

**症状：** 设置代理后某些工具仍然直连

**原因：** 某些工具使用大写环境变量 `HTTP_PROXY`

**解决方案：**

```bash
# 同时设置大写变量
export HTTP_PROXY=$http_proxy
export HTTPS_PROXY=$https_proxy
export ALL_PROXY=$all_proxy
```

### 问题 4: 忘记清除代理

**症状：** 关闭代理软件后网络无法访问

**解决方案：**

```bash
# 快速清除所有代理环境变量
eval "$(setproxy clear)"

# 或手动清除
unset http_proxy https_proxy all_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY
```

### 问题 5: eval 命令报错

**症状：** 运行 `eval "$(setproxy ...)"` 时出现语法错误

**检查：**

```bash
# 不使用 eval，直接查看输出
setproxy 127.0.0.1 7890

# 应该看到类似输出：
# export http_proxy="http://127.0.0.1:7890"
# export https_proxy="http://127.0.0.1:7890"
# export all_proxy="socks5://127.0.0.1:7890"
# echo "✓ 代理已启用: http://127.0.0.1:7890"
```

**解决方案：**
- 确保 `setproxy` 安装正确
- 检查 shell 版本（bash 4.0+, zsh 5.0+）

## 性能提示

### 减少重复设置

```bash
# 检查是否已设置代理
if [[ -z "$http_proxy" ]]; then
    eval "$(setproxy 127.0.0.1 7890)"
fi
```

### 批量命令

```bash
# 一次性执行多个需要代理的命令
eval "$(setproxy 127.0.0.1 7890)" && {
    git clone https://github.com/repo1.git
    git clone https://github.com/repo2.git
    npm install
    pip install -r requirements.txt
} && eval "$(setproxy clear)"
```

## 最佳实践

1. **按需启用**: 只在需要时启用代理，避免影响局域网访问
2. **用后即清**: 完成任务后清除代理，避免遗忘
3. **环境检测**: 使用 Shell 函数自动检测环境
4. **测试连接**: 设置后测试代理是否正常工作
5. **文档化**: 在项目 README 中说明代理需求

## 安全注意事项

⚠️ **重要提醒：**

- 不要在公共网络上使用未加密的代理
- 避免在脚本中硬编码代理密码
- 定期检查代理日志，确保无异常流量
- 使用 HTTPS 代理传输敏感数据

---

更多信息请参考主 [README.md](README.md)
