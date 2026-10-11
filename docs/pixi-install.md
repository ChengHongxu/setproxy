# 📦 通过 Pixi 安装 setproxy

## 快速安装

```bash
# 1. 添加 channel（保留 conda-forge）
pixi config set default-channels '["conda-forge", "https://chenghongxu.github.io/packages/conda"]' --global

# 2. 安装
pixi global install setproxy

# 3. 添加 shell function 到 ~/.zshrc 或 ~/.bashrc
setproxy() {
    eval "$(_setproxy "$@")"
}

# 4. 重新加载 shell
source ~/.zshrc  # 或 source ~/.bashrc
```

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

## 为什么需要 shell function？

由于 shell 的安全限制，子进程无法修改父进程的环境变量。通过添加一个简单的 function，我们用 `eval` 在当前 shell 执行 `_setproxy` 的输出，从而实现环境变量修改。

这是一次性配置，之后就可以一直使用。

## 关于 channel 配置

推荐使用：
```bash
pixi config set default-channels '["conda-forge", "https://chenghongxu.github.io/packages/conda"]' --global
```

这样可以：
- ✅ 保留 `conda-forge`，可以安装其他 conda 包
- ✅ 添加自定义 channel，可以安装 `setproxy`
- ✅ 优先级：自定义 channel > conda-forge

## 卸载

```bash
pixi global uninstall setproxy
```

然后从 shell 配置文件中删除 `setproxy` function。

## 故障排查

### 找不到 setproxy 命令

确保已添加 function 到 shell 配置并重新加载：
```bash
source ~/.zshrc  # 或 source ~/.bashrc
```

### 代理未生效

检查是否使用了 function：
```bash
type setproxy
# 应该显示: setproxy is a shell function
```

如果显示 `setproxy is /home/.../.pixi/bin/setproxy`，说明没有正确配置 function。

检查环境变量：
```bash
env | grep -i proxy
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
