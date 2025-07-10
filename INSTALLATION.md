# Installation Guide (v0.0.7)

## 📦 **Quick Install**

### **One-line installer (macOS/Linux)**

```bash
curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash
```

## 🖥️ **Platform Support**

- ✅ **Linux** (x86_64, ARM64)
- ✅ **macOS** (Intel, Apple Silicon)
- ❌ **Windows** (not supported yet)

## 🏠 **Installation Location**

The binary is installed to:
- **Location**: `~/.contract-deployer/bin/contract-deployer`
- **PATH**: Automatically added to your shell profile
- **Examples**: `~/.contract-deployer/examples/`

## 📋 **Manual Installation**

### **Linux (x86_64)**

```bash
# Download the latest release
VERSION=$(curl -s https://api.github.com/repos/0xdavid7/contract-deployer/releases/latest | grep tag_name | cut -d '"' -f 4)
curl -LO "https://github.com/0xdavid7/contract-deployer/releases/download/$VERSION/contract-deployer-$VERSION-x86_64-unknown-linux-gnu.tar.gz"

# Extract
tar -xzf "contract-deployer-$VERSION-x86_64-unknown-linux-gnu.tar.gz"

# Install
mkdir -p ~/.contract-deployer/bin
cp "contract-deployer-$VERSION-x86_64-unknown-linux-gnu/contract-deployer" ~/.contract-deployer/bin/
cp -r "contract-deployer-$VERSION-x86_64-unknown-linux-gnu/examples" ~/.contract-deployer/

# Add to PATH
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify
contract-deployer --version
```

### **Linux (ARM64)**

```bash
# Download ARM64 version
VERSION=$(curl -s https://api.github.com/repos/0xdavid7/contract-deployer/releases/latest | grep tag_name | cut -d '"' -f 4)
curl -LO "https://github.com/0xdavid7/contract-deployer/releases/download/$VERSION/contract-deployer-$VERSION-aarch64-unknown-linux-gnu.tar.gz"

# Extract and install
tar -xzf "contract-deployer-$VERSION-aarch64-unknown-linux-gnu.tar.gz"
mkdir -p ~/.contract-deployer/bin
cp "contract-deployer-$VERSION-aarch64-unknown-linux-gnu/contract-deployer" ~/.contract-deployer/bin/
cp -r "contract-deployer-$VERSION-aarch64-unknown-linux-gnu/examples" ~/.contract-deployer/

# Add to PATH
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify
contract-deployer --version
```

### **macOS (Intel)**

```bash
# Download for Intel Macs
VERSION=$(curl -s https://api.github.com/repos/0xdavid7/contract-deployer/releases/latest | grep tag_name | cut -d '"' -f 4)
curl -LO "https://github.com/0xdavid7/contract-deployer/releases/download/$VERSION/contract-deployer-$VERSION-x86_64-apple-darwin.tar.gz"

# Extract and install
tar -xzf "contract-deployer-$VERSION-x86_64-apple-darwin.tar.gz"
mkdir -p ~/.contract-deployer/bin
cp "contract-deployer-$VERSION-x86_64-apple-darwin/contract-deployer" ~/.contract-deployer/bin/
cp -r "contract-deployer-$VERSION-x86_64-apple-darwin/examples" ~/.contract-deployer/

# Add to PATH (works for both bash and zsh)
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.zshrc
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.bashrc
source ~/.zshrc

# Verify
contract-deployer --version
```

### **macOS (Apple Silicon)**

```bash
# Download for M1/M2/M3 Macs
VERSION=$(curl -s https://api.github.com/repos/0xdavid7/contract-deployer/releases/latest | grep tag_name | cut -d '"' -f 4)
curl -LO "https://github.com/0xdavid7/contract-deployer/releases/download/$VERSION/contract-deployer-$VERSION-aarch64-apple-darwin.tar.gz"

# Extract and install
tar -xzf "contract-deployer-$VERSION-aarch64-apple-darwin.tar.gz"
mkdir -p ~/.contract-deployer/bin
cp "contract-deployer-$VERSION-aarch64-apple-darwin/contract-deployer" ~/.contract-deployer/bin/
cp -r "contract-deployer-$VERSION-aarch64-apple-darwin/examples" ~/.contract-deployer/

# Add to PATH
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.zshrc
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.bashrc
source ~/.zshrc

# Verify
contract-deployer --version
```

## 🛠️ **Build from Source**

### **Prerequisites**
- **Rust** 1.70.0 or later
- **Git**

### **Build Steps**

```bash
# Clone repository
git clone https://github.com/0xdavid7/contract-deployer.git
cd contract-deployer

# Build release binary
cargo build --release

# Install to home directory
mkdir -p ~/.contract-deployer/bin
cp target/release/contract-deployer ~/.contract-deployer/bin/
cp -r examples ~/.contract-deployer/

# Add to PATH
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify
contract-deployer --version
```

## 🔄 **Updating**

### **Using Install Script**
```bash
# Reinstall latest version (overwrites existing)
curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash
```

### **Manual Update**
```bash
# Check current version
contract-deployer --version

# Download and install new version (replace x86_64-unknown-linux-gnu with your platform)
VERSION=$(curl -s https://api.github.com/repos/0xdavid7/contract-deployer/releases/latest | grep tag_name | cut -d '"' -f 4)
curl -LO "https://github.com/0xdavid7/contract-deployer/releases/download/$VERSION/contract-deployer-$VERSION-x86_64-unknown-linux-gnu.tar.gz"
tar -xzf "contract-deployer-$VERSION-x86_64-unknown-linux-gnu.tar.gz"
cp "contract-deployer-$VERSION-x86_64-unknown-linux-gnu/contract-deployer" ~/.contract-deployer/bin/

# Verify update
contract-deployer --version
```

## 🗑️ **Uninstall**

```bash
# Remove binary and examples
rm -rf ~/.contract-deployer

# Remove from PATH (edit your shell profile manually)
# Remove this line from ~/.bashrc or ~/.zshrc:
# export PATH="$HOME/.contract-deployer/bin:$PATH"
```

## 📁 **Directory Structure**

After installation:
```
~/.contract-deployer/
├── bin/
│   └── contract-deployer           # Main binary
├── examples/
│   ├── basic-deploy.toml          # Basic configuration
│   ├── multi-chain.toml           # Multi-chain setup
│   └── development.toml           # Local development
└── README.md                      # Documentation
```

## 🔧 **Troubleshooting**

### **Command Not Found**

```bash
# Check if binary exists
ls -la ~/.contract-deployer/bin/contract-deployer

# Check if PATH is set
echo $PATH | grep contract-deployer

# Manually add to PATH
export PATH="$HOME/.contract-deployer/bin:$PATH"

# Make permanent (choose your shell)
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.bashrc  # bash
echo 'export PATH="$HOME/.contract-deployer/bin:$PATH"' >> ~/.zshrc   # zsh

# Reload shell
source ~/.bashrc  # or source ~/.zshrc
```

### **Permission Denied**

```bash
# Fix permissions
chmod +x ~/.contract-deployer/bin/contract-deployer
```

### **Download Failed**

```bash
# Check internet connection
curl -I https://api.github.com/repos/0xdavid7/contract-deployer/releases/latest

# Try with wget if curl fails
wget https://github.com/0xdavid7/contract-deployer/releases/latest/download/...
```

### **Wrong Architecture**

```bash
# Check your system architecture
uname -m
# x86_64 → use x86_64 builds
# aarch64/arm64 → use aarch64 builds

# Check your OS
uname -s
# Linux → use linux builds
# Darwin → use darwin builds
```

## 📋 **Verification**

After installation, verify everything works:

```bash
# Check version
contract-deployer --version

# Check help
contract-deployer --help

# Check examples
ls ~/.contract-deployer/examples/

# Test with example config
contract-deployer --config ~/.contract-deployer/examples/basic-deploy.toml --help
```

## 🚀 **Quick Start**

1. **Copy example configuration**:
```bash
cp ~/.contract-deployer/examples/basic-deploy.toml ./deploy.toml
```

2. **Edit configuration**:
```bash
# Edit the config file with your settings
nano deploy.toml
```

3. **Set up environment variables**:
```bash
cat > .env << EOF
API_KEY_ETHERSCAN=your_etherscan_api_key
ALCHEMY_API_KEY=your_alchemy_api_key
KEYSTORE_ACCOUNT=your_keystore_account
KEYSTORE_PASSWORD=your_keystore_password
EOF
```

4. **Deploy your contract**:
```bash
contract-deployer --config deploy.toml
```

## 📚 **Next Steps**

- Read the [Configuration Guide](README.md#configuration-examples)
- Explore [Usage Examples](README.md#usage-examples)
- Learn about [Environment Variables](README.md#environment-variables)

## 💬 **Support**

- **Issues**: [GitHub Issues](https://github.com/0xdavid7/contract-deployer/issues)
- **Documentation**: [README.md](README.md)

---

**Need help?** Open an issue on GitHub!