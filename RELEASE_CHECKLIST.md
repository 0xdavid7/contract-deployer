# Release Checklist

## 🚀 **How to Create a Release**

### **1. Prepare the Release**

```bash
# Make sure everything is committed
git status

# Update version in Cargo.toml if needed
vim Cargo.toml

# Create and push a tag
git tag v1.0.0
git push origin v1.0.0
```

### **2. GitHub Actions Will Automatically:**

- ✅ Build binaries for all supported platforms:
  - `x86_64-unknown-linux-gnu` (Linux x64)
  - `aarch64-unknown-linux-gnu` (Linux ARM64)
  - `x86_64-apple-darwin` (macOS Intel)
  - `aarch64-apple-darwin` (macOS Apple Silicon)

- ✅ Create release artifacts:
  - `contract-deployer-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`
  - `contract-deployer-v1.0.0-aarch64-unknown-linux-gnu.tar.gz`
  - `contract-deployer-v1.0.0-x86_64-apple-darwin.tar.gz`
  - `contract-deployer-v1.0.0-aarch64-apple-darwin.tar.gz`

- ✅ Generate SHA256 checksums for each binary

- ✅ Create GitHub release with all artifacts attached

## 📁 **What's Included in Each Release**

Each `.tar.gz` file contains:
```
contract-deployer-v1.0.0-{target}/
├── contract-deployer          # Main binary
├── examples/                  # Configuration examples
│   ├── basic-deploy.toml
│   ├── multi-chain.toml
│   └── development.toml
├── README.md                  # Main documentation
└── INSTALLATION.md           # Installation guide
```

## 🔧 **Installation Methods**

### **Automatic Install Script**
```bash
curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash
```

### **Manual Download**
Users can download from: `https://github.com/0xdavid7/contract-deployer/releases/latest`

## 📋 **Release Workflow**

1. **Developer creates tag**: `git tag v1.0.0 && git push origin v1.0.0`
2. **GitHub Actions triggers**: Builds all platform binaries
3. **Artifacts uploaded**: All binaries + checksums uploaded to release
4. **Users can install**: Via install script or manual download

## ✅ **Supported Platforms**

| Platform | Architecture | Target | Status |
|----------|-------------|---------|---------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` | ✅ |
| Linux | ARM64 | `aarch64-unknown-linux-gnu` | ✅ |
| macOS | Intel | `x86_64-apple-darwin` | ✅ |
| macOS | Apple Silicon | `aarch64-apple-darwin` | ✅ |
| Windows | x86_64 | `x86_64-pc-windows-msvc` | ❌ |

## 🎯 **Installation Location**

- **Install Path**: `~/.contract-deployer/bin/contract-deployer`
- **Examples**: `~/.contract-deployer/examples/`
- **PATH**: Automatically added to shell profile

## 🔍 **Testing a Release**

After creating a release, test the installation:

```bash
# Test install script
curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash

# Verify installation
contract-deployer --version

# Test with example
contract-deployer --config ~/.contract-deployer/examples/basic-deploy.toml --help
```

## 📝 **Release Notes Template**

```markdown
## What's Changed

### 🚀 New Features
- Feature 1
- Feature 2

### 🐛 Bug Fixes
- Fix 1
- Fix 2

### 📚 Documentation
- Updated installation guide
- Added new examples

### 🔧 Technical
- Updated dependencies
- Performance improvements

## Installation

### Quick Install (macOS/Linux)
```bash
curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash
```

### Manual Download
Download the appropriate binary for your platform from the assets below.

**Full Changelog**: https://github.com/0xdavid7/contract-deployer/compare/v0.9.0...v1.0.0
```

## 🚨 **Important Notes**

- **Only macOS and Linux** are supported
- **Windows support** is not included
- **Install script** puts binary in `~/.contract-deployer/bin/`
- **PATH management** is automatic
- **Examples included** in every release
- **Checksums provided** for verification

This simplified approach focuses on binary distribution without complex package management or Windows support.