# Pixi/Conda 打包实现计划

## 目标
让用户通过 pixi 安装 setproxy：
```bash
pixi config append channels "https://chenghongxu.github.io/packages" --global
pixi global install setproxy
```

## 架构改动

### 1. 重构为 eval wrapper 模式

**当前问题：** shell function 必须 source，conda 包无法自动配置

**解决方案：** 改用 eval wrapper（类似 direnv/ssh-agent）

**新架构：**
- `_setproxy` (Rust 二进制) - 输出可 eval 的 shell 命令
- `setproxy` (bash/zsh 脚本) - wrapper，调用 `_setproxy` 并 eval 结果

**代码修改：**
1. `src/proxy.rs` - 修改输出格式为可 eval 的命令
   ```rust
   // 之前: 输出 "SET http://... socks5://..."
   // 之后: 输出 shell 命令
   println!("export http_proxy=\"{}\"", url);
   println!("export https_proxy=\"{}\"", url);
   println!("echo \"✓ 代理已启用: {}\"", url);
   ```

2. 创建 `shell/setproxy-wrapper.sh`
   ```bash
   #!/usr/bin/env bash
   eval "$(_setproxy "$@")"
   ```

3. 创建 `shell/setproxy-wrapper.ps1`（Windows 版本）

### 2. 添加 rattler-build recipe

**文件：** `recipe/recipe.yaml`

```yaml
context:
  version: "0.1.0"
  
package:
  name: setproxy
  version: ${{ version }}

source:
  - url: https://github.com/ChengHongxu/setproxy/releases/download/v${{ version }}/_setproxy-linux-amd64
    sha256: <will-be-computed>
    file_name: _setproxy
    if: linux and x86_64
  
  - url: https://github.com/ChengHongxu/setproxy/releases/download/v${{ version }}/_setproxy-darwin-amd64
    sha256: <will-be-computed>
    file_name: _setproxy
    if: osx and x86_64
  
  # ... 其他平台

build:
  number: 0
  script:
    - if: unix
      then:
        - mkdir -p $PREFIX/bin
        - install -m 755 _setproxy $PREFIX/bin/_setproxy
        - install -m 755 shell/setproxy-wrapper.sh $PREFIX/bin/setproxy
    - if: win
      then:
        - mkdir -p %PREFIX%/bin
        - copy _setproxy.exe %PREFIX%/bin/_setproxy.exe
        - copy shell/setproxy-wrapper.ps1 %PREFIX%/bin/setproxy.ps1

requirements:
  run:
    - bash  # [unix]

tests:
  - script:
      - _setproxy --version
      - setproxy --help

about:
  homepage: https://github.com/ChengHongxu/setproxy
  license: MIT
  license_file: LICENSE
  summary: Cross-platform proxy environment variable manager
  description: |
    setproxy is a CLI tool to manage proxy environment variables
    with support for WSL2 auto-detection.
```

### 3. 修改 GitHub Actions workflow

**文件：** `.github/workflows/release.yml`

添加新的 job：构建 conda 包并上传到 packages 仓库

```yaml
conda-package:
  needs: release
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    
    - name: Install pixi
      uses: prefix-dev/setup-pixi@v0.8.1
      with:
        pixi-version: latest
    
    - name: Install rattler-build
      run: pixi global install rattler-build
    
    - name: Get version
      id: version
      run: echo "VERSION=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT
    
    - name: Download release assets
      # 下载所有平台的二进制
    
    - name: Build conda packages
      run: |
        for platform in linux-64 osx-64 osx-arm64 win-64; do
          rattler-build build --recipe recipe/recipe.yaml --target-platform $platform
        done
    
    - name: Trigger packages repo update
      uses: peter-evans/repository-dispatch@v3
      with:
        token: ${{ secrets.CHANNEL_DISPATCH_TOKEN }}
        repository: ChengHongxu/packages
        event-type: add-package
        client-payload: |
          {
            "package": "setproxy",
            "version": "${{ steps.version.outputs.VERSION }}",
            "artifacts": "${{ github.event.release.assets }}"
          }
```

### 4. 创建 packages 仓库

**仓库结构：**
```
packages/
├── .github/workflows/
│   └── update-channel.yml
├── linux-64/
│   └── repodata.json
├── osx-64/
├── osx-arm64/
├── win-64/
├── noarch/
└── README.md
```

**Workflow: update-channel.yml**
- 监听 repository_dispatch 事件
- 下载新的 .conda 包
- 运行 `rattler-index` 更新 repodata.json
- 提交并推送到 gh-pages 分支

## 实施步骤

### Phase 1: 重构 setproxy 代码（在 setproxy 仓库）
1. 修改 `src/proxy.rs` 输出格式
2. 创建 wrapper 脚本
3. 更新 `install.sh` 和 `install.ps1` 使用新的 wrapper
4. 测试确保功能正常

### Phase 2: 添加 conda 打包配置（在 setproxy 仓库）
1. 创建 `recipe/recipe.yaml`
2. 本地测试 rattler-build
3. 修改 `.github/workflows/release.yml`

### Phase 3: 创建 packages 仓库（新仓库）
1. 初始化仓库结构
2. 创建 update-channel.yml workflow
3. 配置 GitHub Pages
4. 添加 README 说明使用方法

### Phase 4: 配置 secrets 和测试
1. 创建 GitHub PAT
2. 在 setproxy 仓库添加 secret
3. 打测试 tag 验证整个流程

## 注意事项

1. **sha256 计算**：recipe 中的 sha256 需要在 release 后计算
2. **跨平台兼容性**：wrapper 脚本需要同时支持 bash/zsh/fish
3. **向后兼容**：保留原有的 install.sh，让用户可以选择安装方式
4. **文档更新**：README 需要说明两种安装方式

## 用户最终体验

**安装：**
```bash
pixi config append channels "https://chenghongxu.github.io/packages" --global
pixi global install setproxy
```

**使用：**
```bash
setproxy 127.0.0.1 7890
setproxy status
setproxy clear
```

完全不需要 source，开箱即用！
