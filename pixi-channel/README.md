# pixi channel 支持

## 通过 pixi 安装

```bash
pixi global install --channel https://prefix.dev/<your-username>/setproxy _setproxy
```

安装后需要手动加载 shell function：

```bash
curl -fsSL https://raw.githubusercontent.com/<your-username>/setproxy/main/shell/setproxy.sh >> ~/.zshrc
source ~/.zshrc
```

## 发布 pixi channel

如需自行发布 pixi channel，需要：

1. 在 [prefix.dev](https://prefix.dev) 注册账号
2. 创建 channel
3. 使用 `rattler-build` 构建 conda 包
4. 使用 `pixi publish` 推送到 channel

实际 conda package 的构建超出本项目范围。推荐直接使用 [Releases](https://github.com/<your-username>/setproxy/releases) 下载预编译二进制。
