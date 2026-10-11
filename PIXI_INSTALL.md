# 📦 通过 Pixi 安装 setproxy

## 快速安装

```bash
# 1. 添加 channel
pixi config append default-channels "https://chenghongxu.github.io/packages/conda" --global

# 2. 安装
pixi global install setproxy

# 3. 配置 shell function（重要！）
# 添加到 ~/.zshrc 或 ~/.bashrc：
setproxy() {
    eval "$(_setproxy "$@")"
}

# 4. 重新加载 shell 或运行
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

conda 包运行在子进程中，无法直接修改父 shell 的环境变量。通过 `eval` wrapper，我们可以让 `_setproxy` 的输出在当前 shell 中执行，从而修改环境变量。

## 卸载

```bash
pixi global uninstall setproxy
```

## 故障排查

### 找不到 _setproxy

确保 pixi 的 bin 目录在 PATH 中：

```bash
echo $PATH | grep .pixi
```

如果没有，添加到 shell 配置：

```bash
export PATH="$HOME/.pixi/bin:$PATH"
```

### setproxy 命令不存在

确保已添加 shell function 到 `~/.zshrc` 或 `~/.bashrc`，并重新加载。

### 代理未生效

检查环境变量：

```bash
env | grep -i proxy
```

确保使用了 function 而非直接运行二进制。

## 更多信息

- 项目主页: https://github.com/ChengHongxu/setproxy
- Issue 反馈: https://github.com/ChengHongxu/setproxy/issues
