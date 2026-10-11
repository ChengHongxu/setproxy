#!/usr/bin/env sh
set -e

REPO="ChengHongxu/setproxy"
INSTALL_DIR="${HOME}/.local/bin"
INSTALL_SHARE_DIR="${HOME}/.local/share/setproxy"
SHELL_FUNC_URL="https://raw.githubusercontent.com/${REPO}/main/shell/setproxy.sh"

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

ASSET="_setproxy-${OS_NAME}-${ARCH_NAME}"
LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
URL="https://github.com/${REPO}/releases/download/${LATEST}/${ASSET}"

echo "Installing _setproxy ${LATEST} (${OS_NAME}/${ARCH_NAME})..."
mkdir -p "${INSTALL_DIR}"
curl -fsSL "${URL}" -o "${INSTALL_DIR}/_setproxy"
chmod +x "${INSTALL_DIR}/_setproxy"

# Add to PATH if needed (add to shell rc)
SHELL_RC=""
if echo "${SHELL}" | grep -q "zsh"; then
  SHELL_RC="${HOME}/.zshrc"
elif echo "${SHELL}" | grep -q "bash"; then
  SHELL_RC="${HOME}/.bashrc"
fi

# Inject PATH if not already present
if [ -n "${SHELL_RC}" ]; then
  if ! grep -q "${INSTALL_DIR}" "${SHELL_RC}" 2>/dev/null; then
    echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "${SHELL_RC}"
  fi
fi

# Install shell function to shared directory
mkdir -p "${INSTALL_SHARE_DIR}"
curl -fsSL "${SHELL_FUNC_URL}" -o "${INSTALL_SHARE_DIR}/setproxy.sh"

# Add source line to shell RC
SOURCE_LINE="source \"${INSTALL_SHARE_DIR}/setproxy.sh\""
if [ -n "${SHELL_RC}" ]; then
  if ! grep -q "source.*setproxy.sh" "${SHELL_RC}" 2>/dev/null; then
    echo "" >> "${SHELL_RC}"
    echo "# setproxy - source shell function" >> "${SHELL_RC}"
    echo "${SOURCE_LINE}" >> "${SHELL_RC}"
    echo "Shell function source line added to ${SHELL_RC}"
  else
    echo "Shell function source line already present"
  fi
else
  echo "Unknown shell, skipping shell RC modification"
fi

echo ""
echo "Installed _setproxy to ${INSTALL_DIR}/_setproxy"
echo ""
echo "Run the following to start using setproxy:"
echo "  source ${SHELL_RC:-~/.zshrc}"
echo ""
echo "Then:"
echo "  setproxy 127.0.0.1 7890"
echo "  setproxy wsl"
echo "  setproxy status"
echo "  setproxy clear"
