# 📦 通过 Pixi 安装 setproxy

## 快速安装

```bash
# 1. 添加 channel（追加，不影响已有 channels）
pixi config append default-channels "https://chenghongxu.github.io/packages/conda" --global

# 2. 安装
pixi global install setproxy

# 3. 重开终端即可使用 ✓
```

首次运行任意 `_setproxy` 命令时，会自动检测 shell 集成是否已配置：
- 未配置时：自动将 `eval "$(_setproxy shell-init)"` 写入 `~/.zshrc` 或 `~/.bashrc`
- 已配置时：静默跳过，不重复写入

重开终端后 `setproxy` 即可直接使用。

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

推荐使用追加方式，不影响已有 channels：
```bash
pixi config append default-channels "https://chenghongxu.github.io/packages/conda" --global
```

这样可以：
- ✅ 保留 `conda-forge` 及其他已有 channels
- ✅ 添加自定义 channel，可以安装 `setproxy`
- ✅ 不会覆盖现有配置

## 卸载

```bash
pixi global uninstall setproxy
```

然后从 shell 配置文件中删除 `setproxy` function。

## 故障排查

### 找不到 setproxy 命令

首次运行任意命令会自动配置，例如：
```bash
_setproxy status
```
然后重开终端即可。

### 代理未生效

检查是否使用了 function：
```bash
type setproxy
# 应该显示: setproxy is a shell function
```

如果显示 `setproxy is /home/.../.pixi/bin/setproxy`，说明 shell function 还未加载。运行：
```bash
_setproxy status  # 触发自动配置
```
然后重开终端。

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
