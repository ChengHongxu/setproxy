#!/usr/bin/env sh
set -e

REPO="ChengHongxu/setproxy"
INSTALL_DIR="${HOME}/.local/bin"

# Detect OS and arch
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${OS}" in
  linux)  OS_NAME="linux"  ;;
  darwin) OS_NAME="darwin" ;;
  *) echo "Unsupported OS: ${OS}"; exit 1 ;;
esac

case "${ARCH}" in
  x86_64|amd64)   ARCH_NAME="amd64" ;;
  aarch64|arm64)  ARCH_NAME="arm64" ;;
  *) echo "Unsupported arch: ${ARCH}"; exit 1 ;;
esac

ASSET="setproxy-${OS_NAME}-${ARCH_NAME}"
LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
URL="https://github.com/${REPO}/releases/download/${LATEST}/${ASSET}"

echo "Installing setproxy ${LATEST} (${OS_NAME}/${ARCH_NAME})..."
mkdir -p "${INSTALL_DIR}"
curl -fsSL "${URL}" -o "${INSTALL_DIR}/setproxy"
chmod +x "${INSTALL_DIR}/setproxy"

# Detect shell RC file
SHELL_NAME=""
SHELL_RC=""
case "${SHELL:-}" in
  *zsh)  SHELL_NAME="zsh";  SHELL_RC="${ZDOTDIR:-${HOME}}/.zshrc" ;;
  *bash) SHELL_NAME="bash"; SHELL_RC="${HOME}/.bashrc" ;;
esac

if [ -z "${SHELL_RC}" ]; then
  echo ""
  echo "Installed setproxy to ${INSTALL_DIR}/setproxy"
  echo "Unknown shell: add ${INSTALL_DIR} to PATH, then run: setproxy init <zsh|bash>"
  exit 0
fi

# Running this installer is the explicit opt-in to editing the RC file.
# PATH goes first: the line added by `setproxy init` needs it.
if ! grep -qF "${INSTALL_DIR}" "${SHELL_RC}" 2>/dev/null; then
  echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "${SHELL_RC}"
fi
"${INSTALL_DIR}/setproxy" init "${SHELL_NAME}"

echo ""
echo "Then:"
echo "  setproxy 127.0.0.1 7890"
echo "  setproxy wsl"
echo "  setproxy status"
echo "  setproxy clear"
