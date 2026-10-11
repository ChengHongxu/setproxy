# Pixi/Conda 打包实施完成总结

## ✅ 已完成的工作

### Phase 1: 重构代码为 eval wrapper 模式 ✓

**修改内容：**
- `src/proxy.rs` - 二进制现在输出 shell 命令（export/unset）而非解析格式
- `shell/setproxy.sh` - 简化为 `eval "$(_setproxy "$@")"`
- `shell/setproxy-wrapper.sh` - 独立 wrapper 脚本（conda 包用）
- `shell/setproxy-wrapper.ps1` - Windows 版本

**优势：**
- 用户体验不变：仍然 source 一次即可使用
- 为 conda 包做好准备：wrapper 可以独立工作
- 更简洁的实现：不需要复杂的输出解析

**测试：**
```bash
source ~/.local/share/setproxy/setproxy.sh
setproxy 127.0.0.1 7890  # ✓ 工作正常
setproxy status          # ✓ 工作正常
setproxy clear           # ✓ 工作正常
```

### Phase 2: 添加 conda 打包配置 ✓

**新增文件：**
- `recipe/recipe.yaml` - rattler-build 配置
- `recipe/README.md` - 本地测试说明
- `.github/workflows/release.yml` - 扩展 CI 添加 conda 包构建

**工作流程：**
```
打 tag (v0.2.0)
    ↓
构建多平台二进制 (GitHub Actions)
    ↓
创建 GitHub Release
    ↓
计算 sha256 checksums
    ↓
使用 rattler-build 构建 .conda 包
    ↓
触发 packages 仓库更新
```

### Phase 3: packages 仓库模板 ✓

**位置：** `~/Workspace/packages-repo-template.tar.gz`

**包含文件：**
```
packages-repo-template/
├── .github/workflows/update-channel.yml  # 自动更新 channel
├── .gitignore
├── README.md                              # 用户使用文档
├── SETUP.md                               # 你的设置指南
├── linux-64/repodata.json                 # 初始索引文件
├── osx-64/repodata.json
├── osx-arm64/repodata.json
├── win-64/repodata.json
└── noarch/repodata.json
```

---

## 📋 接下来你需要做的事

### 1. 创建 packages 仓库（5 分钟）

```bash
# 解压模板
cd ~/Workspace
tar xzf packages-repo-template.tar.gz
cd packages-repo-template

# 在 GitHub 创建仓库
# https://github.com/new
# 仓库名: packages
# 公开仓库，不要初始化 README

# 推送
git init
git add .
git commit -m "Initial commit: conda channel structure"
git remote add origin https://github.com/ChengHongxu/packages.git
git branch -M main
git push -u origin main
```

### 2. 启用 GitHub Pages（2 分钟）

1. 进入 https://github.com/ChengHongxu/packages/settings/pages
2. Source: Deploy from a branch
3. Branch: `gh-pages` / `(root)`
4. Save

**注意：** `gh-pages` 分支会在第一次添加包时自动创建。

### 3. 创建 GitHub Token（3 分钟）

1. https://github.com/settings/tokens/new
2. Note: "Conda channel dispatch"
3. Scopes: `repo` + `workflow`
4. Generate token
5. **复制 token**

### 4. 添加 Secret 到 setproxy（1 分钟）

1. https://github.com/ChengHongxu/setproxy/settings/secrets/actions
2. New repository secret
3. Name: `CHANNEL_DISPATCH_TOKEN`
4. Value: 粘贴刚才的 token
5. Add secret

### 5. 测试发布（10 分钟）

```bash
cd ~/Workspace/setproxy

# 创建新版本 tag
git tag v0.2.0
git push origin v0.2.0
```

**等待 CI 完成：**
1. setproxy 仓库：构建 → 发布 → 构建 conda 包 (约 5 分钟)
2. packages 仓库：接收通知 → 更新 channel (约 2 分钟)

**检查结果：**
- https://github.com/ChengHongxu/setproxy/releases/tag/v0.2.0
- https://github.com/ChengHongxu/packages (会创建 gh-pages 分支)
- https://chenghongxu.github.io/packages/conda/linux-64/repodata.json

### 6. 测试安装（2 分钟）

```bash
# 添加 channel
pixi config append channels "https://chenghongxu.github.io/packages/conda" --global

# 安装
pixi global install setproxy

# 测试
setproxy --help
source <(setproxy 127.0.0.1 7890)
env | grep proxy
```

---

## 🚀 多包管理器支持

新的 packages 仓库结构支持扩展到多种包管理器：

```
packages/
├── conda/           # Conda/Pixi channel
├── homebrew/        # Homebrew tap (future)
└── .github/workflows/
```

**优势：**
- ✅ 清晰的目录组织
- ✅ 易于扩展到其他包管理器（apt、yum、scoop 等）
- ✅ 每个管理器独立维护
- ✅ 用户可以选择喜欢的安装方式

**未来添加 Homebrew：**
```bash
brew tap chenghongxu/packages
brew install setproxy
```

---

## 🎯 最终用户体验

### 安装（两行命令）

```bash
pixi config append channels "https://chenghongxu.github.io/packages/conda" --global
pixi global install setproxy
```

### 使用（需要 source wrapper）

因为 conda 包无法修改父 shell 的环境变量，用户需要：

**方式 1：手动 source（推荐）**
```bash
# 添加到 .zshrc/.bashrc
source "$(dirname $(which _setproxy))/../share/setproxy/setproxy-wrapper.sh"

# 然后
setproxy 127.0.0.1 7890
```

**方式 2：定义 function**
```bash
# 添加到 .zshrc/.bashrc
setproxy() {
    eval "$(_setproxy "$@")"
}

# 然后
setproxy 127.0.0.1 7890
```

---

## 🔧 故障排查

### Workflow 失败："Resource not accessible"
- PAT 需要 `repo` 和 `workflow` 权限
- 重新创建 token 并更新 secret

### Workflow 失败："404 Not Found"
- 检查仓库名是否正确 `ChengHongxu/packages`
- 验证 `CHANNEL_DISPATCH_TOKEN` secret 已添加

### repodata.json 不更新
- 检查 gh-pages 分支是否存在
- 查看 packages 仓库的 Actions 日志
- 等待 GitHub Pages 部署（2-3 分钟）

### 包找不到
```bash
# 清除缓存
pixi clean cache

# 手动检查
curl https://chenghongxu.github.io/packages/linux-64/repodata.json
```

---

## 📚 文档位置

- **setproxy 仓库：**
  - `recipe/recipe.yaml` - conda 包配置
  - `recipe/README.md` - 本地测试说明
  - `.claude/plan.md` - 完整实施计划

- **packages 仓库模板：**
  - `~/Workspace/packages-repo-template.tar.gz`
  - `SETUP.md` - 详细设置指南
  - `README.md` - 用户文档

---

## 🚀 添加更多包

以后添加新包时，参考 setproxy 的结构：

1. 在包仓库添加 `recipe/recipe.yaml`
2. 修改 CI 构建 conda 包
3. 触发 `ChengHongxu/packages` 的 `repository_dispatch`
4. 自动更新 channel

一个 channel，所有包！

---

## ✨ 已推送的 commits

```
3effbcd feat: add conda/pixi packaging support
e7f45ca refactor: change to eval wrapper pattern for better conda packaging
90fa606 refactor: install shell function to separate file instead of inline
```

全部代码已推送到 main 分支。下一步按照上面的步骤创建 packages 仓库并测试即可！
