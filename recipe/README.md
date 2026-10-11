# Conda Package Recipe

This directory contains the rattler-build recipe for packaging setproxy as a conda package.

## Prerequisites

```bash
pixi global install rattler-build
```

## Local Testing

### 1. Create checksum files

After creating a GitHub release, download the binaries and compute their sha256:

```bash
mkdir -p recipe/checksums
cd recipe/checksums

# For each platform
sha256sum _setproxy-linux-amd64 | cut -d' ' -f1 > linux-amd64.sha256
sha256sum _setproxy-linux-arm64 | cut -d' ' -f1 > linux-arm64.sha256
sha256sum _setproxy-darwin-amd64 | cut -d' ' -f1 > darwin-amd64.sha256
sha256sum _setproxy-darwin-arm64 | cut -d' ' -f1 > darwin-arm64.sha256
sha256sum _setproxy-windows-amd64.exe | cut -d' ' -f1 > windows-amd64.sha256
```

### 2. Build the package

```bash
# Build for current platform
rattler-build build --recipe recipe/recipe.yaml

# Build for specific platform
rattler-build build --recipe recipe/recipe.yaml --target-platform linux-64
rattler-build build --recipe recipe/recipe.yaml --target-platform osx-64
rattler-build build --recipe recipe/recipe.yaml --target-platform osx-arm64
rattler-build build --recipe recipe/recipe.yaml --target-platform win-64
```

### 3. Test the package locally

```bash
# Install from local build
pixi global install ./output/linux-64/setproxy-*.conda

# Test
setproxy --help
source <(setproxy 127.0.0.1 7890)
```

## CI/CD Integration

The GitHub Actions workflow will:
1. Build binaries for all platforms
2. Create GitHub release
3. Compute sha256 checksums
4. Build conda packages
5. Upload to packages repository
6. Trigger repodata update

## Package Structure

```
$PREFIX/
├── bin/
│   ├── _setproxy          # Rust binary
│   └── setproxy           # Wrapper script (sourced by user)
```

## Usage After Installation

```bash
# Add channel (one-time)
pixi config append channels "https://chenghongxu.github.io/packages" --global

# Install
pixi global install setproxy

# Use
setproxy 127.0.0.1 7890
setproxy status
setproxy clear
```
