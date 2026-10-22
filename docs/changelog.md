# Changelog

## [0.3.1] - 2026-10-22

### Changed

- 启动文件中的一行改为 `eval "$(setproxy hook zsh)"`：`init` 只负责写入（运行一次），`hook` 负责输出 shell 函数（每次开终端）。原来的 `setproxy init <shell> --print` 已移除
- 重新运行 `setproxy init` 会把之前写入的 `# setproxy` 下面那一行原地更新
- `setproxy wsl` 在 WSL mirrored 网络模式下使用 `127.0.0.1`，NAT 模式下仍使用默认网关

### Fixed

- 设置和清除代理时同时处理大写的 `HTTP_PROXY` / `HTTPS_PROXY` / `ALL_PROXY`。WSL `autoProxy` 会自动设置大写变量，之前 `setproxy clear` 后它们仍然残留

## [0.3.0] - 2026-10-21

### Changed

- 可执行文件从 `_setproxy` 改名为 `setproxy`，用户只接触 `setproxy` 一个命令
- 新增 `setproxy init [zsh|bash|pwsh]`：自动识别当前 shell（沿进程树查找，兼容 pixi trampoline），往启动文件追加一行；重复执行不会重复写入
- 未执行 `setproxy init`，或执行后未重开终端时，`setproxy` 会提示下一步，不修改任何文件
- shell 函数源码移到 `shell/init.sh`、`shell/init.ps1`，通过 `include_str!` 编译进二进制，由 `setproxy init <shell> --print` 输出
- 可执行文件的 stdout 只输出 shell 代码（`--syntax posix|powershell`），提示信息改为 stderr
- PowerShell 不再用正则解析 bash 输出，直接执行生成的 PowerShell 代码
- 安装脚本改为调用 `setproxy init`

### Removed

- 运行任意命令时自动修改 `~/.zshrc` / `~/.bashrc` 的逻辑
- `shell-init` 子命令，以及 `shell/` 下的 activate、wrapper、兜底脚本

### Fixed

- 代理地址会进入 eval 的代码，现在只接受主机名/IP 字符，拒绝 `$(...)` 等注入
- `setproxy wsl --help` 不再把帮助文本当作代码 eval

## [0.1.0] - 2026-10-06

### Added

- `_setproxy` Rust binary：set/clear/status/wsl 命令
- shell function（bash/zsh）：source 后直接调用，无需 `eval`
- WSL2 自动检测宿主机 IP
- 支持 Linux、macOS、Windows
- GitHub Actions CI/CD：多平台测试与自动发布
- 一键安装脚本 `install.sh`
