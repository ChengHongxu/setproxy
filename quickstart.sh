#!/bin/bash

# setproxy 快速安装和测试脚本
# 使用方法: ./quickstart.sh

set -e

echo "======================================"
echo "setproxy 快速开始"
echo "======================================"
echo ""

# 检查是否在项目目录
if [[ ! -f "Cargo.toml" ]]; then
    echo "✗ 错误: 请在 setproxy 项目根目录运行此脚本"
    exit 1
fi

# 步骤 1: 检查 Pixi
echo "步骤 1/5: 检查 Pixi 环境..."
if ! command -v pixi &> /dev/null; then
    echo "✗ Pixi 未安装，请先安装 Pixi:"
    echo "  curl -fsSL https://pixi.sh/install.sh | bash"
    exit 1
fi
echo "✓ Pixi 已安装"
echo ""

# 步骤 2: 安装依赖
echo "步骤 2/5: 安装 Pixi 环境和依赖..."
pixi install
echo "✓ 依赖安装完成"
echo ""

# 步骤 3: 编译 Release 版本
echo "步骤 3/5: 编译 Release 版本..."
pixi run build-release
echo "✓ 编译完成"
echo ""

# 步骤 4: 运行测试
echo "步骤 4/5: 运行测试..."
pixi run test
echo "✓ 所有测试通过"
echo ""

# 步骤 5: 安装到 PATH
echo "步骤 5/5: 安装到 ~/.local/bin/..."
mkdir -p ~/.local/bin
cp target/release/setproxy ~/.local/bin/
chmod +x ~/.local/bin/setproxy
echo "✓ 安装完成"
echo ""

# 验证安装
echo "======================================"
echo "验证安装..."
echo "======================================"
if command -v setproxy &> /dev/null; then
    echo "✓ setproxy 已成功安装到 PATH"
    echo ""
    setproxy --version
    echo ""
else
    echo "⚠ setproxy 未在 PATH 中找到"
    echo "请确保 ~/.local/bin 在你的 PATH 中:"
    echo '  export PATH="$HOME/.local/bin:$PATH"'
    echo ""
    echo "或者直接使用完整路径:"
    echo "  ~/.local/bin/setproxy"
    echo ""
fi

# 使用示例
echo "======================================"
echo "快速使用示例"
echo "======================================"
echo ""
echo "1. 设置代理:"
echo '   eval "$(setproxy 127.0.0.1 7890)"'
echo ""
echo "2. WSL2 自动检测 (如果在 WSL 环境):"
echo '   eval "$(setproxy wsl)"'
echo ""
echo "3. 查看状态:"
echo "   setproxy status"
echo ""
echo "4. 清除代理:"
echo '   eval "$(setproxy clear)"'
echo ""
echo "======================================"
echo "添加 Shell 函数 (可选)"
echo "======================================"
echo ""
echo "将以下内容添加到 ~/.zshrc 或 ~/.bashrc:"
echo ""
cat << 'SHELL_FUNC'
# setproxy 快捷函数
proxy() {
    if [[ $# -eq 0 ]]; then
        eval "$(setproxy wsl 2>/dev/null || setproxy 127.0.0.1 7890)"
    else
        eval "$(setproxy $@)"
    fi
}

noproxy() {
    eval "$(setproxy clear)"
}

proxystatus() {
    setproxy status
}
SHELL_FUNC
echo ""
echo "然后运行: source ~/.zshrc"
echo ""
echo "======================================"
echo "🎉 安装完成！开始使用 setproxy 吧！"
echo "======================================"
echo ""
echo "详细文档:"
echo "  - README.md - 项目介绍"
echo "  - USAGE.md - 详细使用指南"
echo "  - PROJECT_COMPLETE.md - 完整项目总结"
echo ""
