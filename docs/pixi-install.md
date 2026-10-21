# 通过 Pixi 安装 setproxy

## 快速安装

```bash
# 1. 添加 channel（追加，不影响已有 channels）
pixi config append default-channels "https://chenghongxu.github.io/packages/conda" --global

# 2. 安装
pixi global install setproxy

# 3. 接入当前 shell，然后重开终端
setproxy init
```

`setproxy init` 会识别你当前所在的 shell（zsh / bash / PowerShell），往对应的启动文件追加两行：

```bash
# setproxy
eval "$(setproxy init zsh --print)"
```

也可以指定 shell：`setproxy init zsh`、`setproxy init bash`、`setproxy init pwsh`。重复执行不会重复写入。

## 使用

```bash
setproxy 127.0.0.1 7890   # 设置代理
setproxy wsl              # WSL2 自动检测宿主机 IP
setproxy status           # 查看状态
setproxy clear            # 清除代理
```

## 为什么需要 `setproxy init`

子进程无法修改父进程的环境变量，所以修改当前 shell 环境的工具（conda、pyenv、nvm、zoxide、mise）都需要在启动文件里加载一个 shell 函数。

pixi 和 brew 都不会在安装时替包修改 dotfiles：pixi 默认不执行包的 post-link 脚本，`pixi global` 的激活脚本也只作用于 trampoline 内部，进不了交互 shell。所以这一步由你执行一次 `setproxy init` 完成，setproxy 不会在其他命令里偷偷写入。

没执行过 `setproxy init` 就运行 `setproxy 127.0.0.1 7890`，会提示先运行 `setproxy init`；执行过但还没重开终端，会提示 `source` 哪个文件。

## 从 0.2.x 升级

0.2.x 的可执行文件叫 `_setproxy`，并会自动往 `~/.zshrc` / `~/.bashrc` 写入：

```bash
# >>> setproxy initialized <<<
eval "$(_setproxy shell-init)"
```

0.3.0 起 `_setproxy` 不再存在，这一行在新终端里会报 `command not found: _setproxy`。升级后执行一次：

```bash
setproxy init
```

它会删除上面这些旧行，写入新的配置。

## 卸载

```bash
pixi global uninstall setproxy
```

然后从 shell 启动文件中删除 `# setproxy` 和下面的 `setproxy init ... --print` 那一行。

## 故障排查

### 代理未生效

确认使用的是 shell 函数：

```bash
type setproxy
# 应该显示: setproxy is a shell function
```

如果显示 `setproxy is /home/.../.pixi/bin/setproxy`，说明还没执行 `setproxy init`，或者执行后还没重开终端。直接运行 `setproxy 127.0.0.1 7890` 会告诉你是哪一种。

### pixi 找不到包

清除缓存后重试：

```bash
rm -rf ~/.cache/rattler/cache/repodata/
pixi global install setproxy
```

## 更多信息

- 项目主页: https://github.com/ChengHongxu/setproxy
- Issue 反馈: https://github.com/ChengHongxu/setproxy/issues
