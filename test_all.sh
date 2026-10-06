#!/bin/bash

set -e

echo "========================================="
echo "setproxy 完整功能测试"
echo "========================================="
echo ""

BINARY="./target/release/setproxy"

# 测试 1: 帮助信息
echo "✓ 测试 1: 帮助信息"
$BINARY --help | head -n 5
echo ""

# 测试 2: 版本信息
echo "✓ 测试 2: 版本信息"
$BINARY --version
echo ""

# 测试 3: 设置代理（不使用 eval，查看输出）
echo "✓ 测试 3: 设置代理（查看生成的脚本）"
$BINARY 127.0.0.1 7890
echo ""

# 测试 4: 实际设置代理并验证
echo "✓ 测试 4: 实际设置代理并验证"
eval "$($BINARY 127.0.0.1 7890)"
echo "  http_proxy = $http_proxy"
echo "  https_proxy = $https_proxy"
echo "  all_proxy = $all_proxy"
echo ""

# 测试 5: 查看状态
echo "✓ 测试 5: 查看代理状态"
$BINARY status
echo ""

# 测试 6: 清除代理（不使用 eval，查看输出）
echo "✓ 测试 6: 清除代理（查看生成的脚本）"
$BINARY clear
echo ""

# 测试 7: 实际清除代理并验证
echo "✓ 测试 7: 实际清除代理并验证"
eval "$($BINARY clear)"
echo "  http_proxy = ${http_proxy:-未设置}"
echo "  https_proxy = ${https_proxy:-未设置}"
echo "  all_proxy = ${all_proxy:-未设置}"
echo ""

# 测试 8: WSL 模式
echo "✓ 测试 8: WSL 模式"
if [[ -n "$WSL_DISTRO_NAME" ]]; then
    echo "  检测到 WSL 环境: $WSL_DISTRO_NAME"
    $BINARY wsl
    echo ""
    echo "  实际设置 WSL 代理..."
    eval "$($BINARY wsl)"
    $BINARY status
else
    echo "  非 WSL 环境，跳过此测试"
fi
echo ""

# 测试 9: 错误处理
echo "✓ 测试 9: 错误处理（缺少参数）"
$BINARY 2>&1 | head -n 3 || true
echo ""

echo "========================================="
echo "所有测试完成！"
echo "========================================="
