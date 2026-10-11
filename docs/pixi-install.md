# 📦 通过 Pixi 安装 setproxy

## 快速安装

```bash
# 1. 添加 channel（保留 conda-forge）
pixi config set default-channels '["conda-forge", "https://chenghongxu.github.io/packages/conda"]' --global

# 2. 安装
pixi global install setproxy

# 3. 重新加载 shell 或打开新终端
source ~/.zshrc  # 或 source ~/.bashrc
```

✨ **就这样！** 安装后 `setproxy` 命令立即可用，无需手动配置。

## 使用

```bash
# 设置代理
setproxy 127.0.0.1 7890

# WSL2 自动检测
setproxy wsl

# 查看状态
setproxy status

# 清除代理
setproxy clear
```

## 工作原理

Pixi 安装时会自动：
1. 安装 `_setproxy` 二进制
2. 安装 `setproxy` wrapper 脚本
3. 配置 conda activation script，在环境激活时定义 `setproxy` function

这样你就可以直接使用 `setproxy` 命令，无需手动配置。

## 关于 channel 配置

推荐使用：
```bash
pixi config set default-channels '["conda-forge", "https://chenghongxu.github.io/packages/conda"]' --global
```

这样可以：
- ✅ 保留 `conda-forge`，可以安装其他 conda 包
- ✅ 添加自定义 channel，可以安装 `setproxy`

**不推荐**使用 `append`：
```bash
# ❌ 会覆盖而不是追加
pixi config append default-channels "https://chenghongxu.github.io/packages/conda" --global
```

## 卸载

```bash
pixi global uninstall setproxy
```

## 故障排查

### 找不到 setproxy 命令

重新加载 shell：
```bash
source ~/.zshrc  # 或 source ~/.bashrc
```

或打开新终端窗口。

### 代理未生效

检查环境变量：
```bash
env | grep -i proxy
```

如果 `setproxy` 命令存在但环境变量未设置，尝试：
```bash
# 手动 source wrapper
source $(which setproxy)
setproxy 127.0.0.1 7890
```

### pixi 找不到包

清除缓存后重试：
```bash
rm -rf ~/.cache/rattler/cache/repodata/
pixi global install setproxy
```

## 更多信息

- 项目主页: https://github.com/ChengHongxu/setproxy
- Issue 反馈: https://github.com/ChengHongxu/setproxy/issues
