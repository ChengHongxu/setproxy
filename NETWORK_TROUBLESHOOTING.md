# 网络问题解决方案

## 🔴 当前问题

编译 Rust 项目时遇到网络超时：
```
warning: spurious network error: [28] Timeout was reached
```

这是因为 crates.io 和 GitHub 在某些地区访问较慢。

## ✅ 解决方案

### 方案 1：配置 Cargo 使用国内镜像（推荐）

创建或编辑 `~/.cargo/config.toml`：

```bash
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

[registries.ustc]
index = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

[net]
git-fetch-with-cli = true
EOF
```

### 方案 2：使用清华镜像

```bash
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[net]
git-fetch-with-cli = true
EOF
```

### 方案 3：使用字节跳动镜像

```bash
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
EOF
```

## 🚀 应用配置后重新编译

```bash
cd ~/Workspace/setproxy

# 清理之前的编译缓存（可选）
pixi run cargo clean

# 重新编译
pixi run cargo build --release
```

## 💡 如果你已经有代理

如果你有可用的代理服务器，可以：

### 临时使用代理（本次编译）

```bash
# 设置代理环境变量
export http_proxy="http://127.0.0.1:7890"
export https_proxy="http://127.0.0.1:7890"

# 编译
cd ~/Workspace/setproxy
pixi run cargo build --release
```

### 永久配置 Git 代理

```bash
# HTTP 代理
git config --global http.proxy http://127.0.0.1:7890
git config --global https.proxy http://127.0.0.1:7890

# SOCKS5 代理
git config --global http.proxy socks5://127.0.0.1:1080
git config --global https.proxy socks5://127.0.0.1:1080
```

## 📝 推荐步骤

1. **配置 Cargo 镜像**（选择方案 1、2 或 3）
2. **重新编译项目**
3. **编译成功后，你的 setproxy 工具就可以用了！**

之后就可以方便地切换代理，不再需要每次手动设置环境变量。

## 🎯 编译成功后的使用

```bash
# 假设你编译成功并安装了 setproxy
# 以后切换代理就很方便了：

# 设置代理
eval "$(setproxy 127.0.0.1 7890)"

# 编译其他 Rust 项目
cargo build

# 清除代理
eval "$(setproxy clear)"
```

## 🔍 检查配置是否生效

```bash
# 查看 Cargo 配置
cat ~/.cargo/config.toml

# 测试网络
curl -I https://crates.io
```

## 📞 其他可能的问题

### DNS 问题
如果是 DNS 解析问题，可以配置：

```bash
# 临时使用 Google DNS
export CARGO_HTTP_CAINFO=/etc/ssl/certs/ca-certificates.crt

# 或在 ~/.cargo/config.toml 中添加
[http]
timeout = 600
```

### WSL2 网络问题
如果在 WSL2 中，可能需要：

```bash
# 检查 DNS
cat /etc/resolv.conf

# 如果 DNS 有问题，手动设置
sudo bash -c 'echo "nameserver 8.8.8.8" > /etc/resolv.conf'
```

---

**建议**：先尝试方案 1（USTC 镜像），通常最稳定快速。
