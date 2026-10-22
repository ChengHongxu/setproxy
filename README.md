# setproxy

一个轻量、跨平台的代理设置工具。

## 工作原理

子进程无法修改父进程的环境变量，所以 `setproxy` 由两部分组成：
- `setproxy` 可执行文件（Rust）：计算代理地址，输出对应 shell 的代码（`export ...` 或 `$env:... = ...`）
- `setproxy` shell 函数：`setproxy init` 把它接入 shell 启动文件，之后在当前 shell 中执行上面的代码

函数和可执行文件同名，函数优先于 PATH，所以平时用到的始终是函数。这与 zoxide、starship、mise 的做法一致：安装只负责放好可执行文件，shell 接入由你执行一次 `setproxy init` 完成，平时运行任何命令都不会修改启动文件。

## 安装

### 方式 1: Pixi/Conda (推荐)

```bash
# 添加 channel（追加，不影响其他 channels）
pixi config append default-channels "https://chenghongxu.github.io/packages/conda" --global

# 安装
pixi global install setproxy

# 接入当前 shell（自动识别 zsh / bash / PowerShell），然后重开终端
setproxy init
```

详见 [docs/pixi-install.md](docs/pixi-install.md)。

### 方式 2: 一键安装脚本

#### Linux / macOS
```bash
curl -fsSL https://raw.githubusercontent.com/ChengHongxu/setproxy/main/install.sh | sh
```

#### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/ChengHongxu/setproxy/main/install.ps1 | iex
```

安装脚本会自动执行 `setproxy init`（运行安装脚本即视为同意修改启动文件）。安装后重启终端或运行：
- **Linux/macOS**: `source ~/.zshrc` (或 `~/.bashrc`)
- **Windows**: `. $PROFILE`

### 方式 3: 手动安装
从 [Releases](https://github.com/ChengHongxu/setproxy/releases) 下载对应平台的二进制文件，命名为 `setproxy`，放到 PATH 中，再运行 `setproxy init`。

## 用法

```bash
setproxy init              # 首次使用：接入 shell（也可指定 zsh / bash / pwsh）
setproxy 127.0.0.1 7890    # 设置代理
setproxy wsl               # WSL2 自动检测宿主机 IP
setproxy wsl 7890          # WSL2 指定端口
setproxy status            # 查看状态
setproxy clear             # 清除代理
```

没有执行过 `setproxy init`，或者执行后还没重开终端，`setproxy` 会提示下一步该做什么，不会修改任何文件。

设置后生效的环境变量：
- `http_proxy` / `https_proxy` → `http://host:port`
- `all_proxy` → `socks5://host:port`
- 以及对应的大写版本 `HTTP_PROXY` / `HTTPS_PROXY` / `ALL_PROXY`（Windows 环境变量不区分大小写，只设一套）

`setproxy clear` 会把大小写两套一起清除，`no_proxy` 不受影响。

详细用法见 [docs/usage.md](docs/usage.md)。

## 从源码构建

```bash
git clone https://github.com/ChengHongxu/setproxy
cd setproxy
cargo build --release
cp target/release/setproxy ~/.local/bin/
setproxy init
```

## 许可证

MIT
