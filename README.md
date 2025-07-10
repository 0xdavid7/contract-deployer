# Contract Deployer

A Rust-based CLI tool for deploying smart contracts with TOML configuration files, making deployments consistent, reproducible, and secure across different networks.

[![Release](https://img.shields.io/github/v/release/0xdavid7/contract-deployer)](https://github.com/0xdavid7/contract-deployer/releases)
[![License](https://img.shields.io/github/license/0xdavid7/contract-deployer)](LICENSE)
[![CI](https://github.com/0xdavid7/contract-deployer/actions/workflows/ci.yml/badge.svg)](https://github.com/0xdavid7/contract-deployer/actions/workflows/ci.yml)
[![Issues](https://img.shields.io/github/issues/0xdavid7/contract-deployer)](https://img.shields.io/github/issues/0xdavid7/contract-deployer)

## ✨ **Features**

- 🔧 **TOML Configuration** - Clean, readable configuration format
- 🌍 **Multi-Network Support** - Deploy to mainnet, testnets, or local networks
- 🔐 **Secure Authentication** - Keystore or private key support
- 📦 **Repository Cloning** - Deploy directly from Git repositories
- 🚀 **Real-time Output** - See deployment progress as it happens
- 🔄 **Environment Management** - Multiple `.env` files with variable expansion
- ✅ **Automatic Verification** - Built-in Etherscan verification
- 🛠️ **Flexible Setup** - Custom dependency installation commands

## 🚀 **Quick Start**

### **Installation**

```bash
# One-line install (macOS/Linux)
curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash
```

### **Basic Usage**

1. **Create configuration file**:
```bash
cp ~/.contract-deployer/examples/basic-deploy.toml ./deploy.toml
```

2. **Set up environment variables**:
```bash
cat > .env << EOF
API_KEY_ETHERSCAN=your_etherscan_api_key
ALCHEMY_API_KEY=your_alchemy_api_key
KEYSTORE_ACCOUNT=your_keystore_account
KEYSTORE_PASSWORD=your_keystore_password
EOF
```

3. **Deploy your contract**:
```bash
contract-deployer --config deploy.toml
```

## 📋 **Configuration**

### **Basic Configuration**

```toml
[project]
name = "my-smart-contract"
script = "Deploy"                    # Will look for script/Deploy.s.sol
network = "sepolia"                  # Default network
setup_command = "bun install"       # Dependencies installation
repo = "https://github.com/user/contract.git"  # Optional: Git repository

[env]
load_files = [".env"]               # Environment files to load

[networks.sepolia]
chain_id = 11155111
rpc_url = "https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
verify = true

[networks.mainnet]
chain_id = 1
rpc_url = "https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
verify = true

[networks.localhost]
chain_id = 31337
rpc_url = "http://localhost:8545"
verify = false
```

### **Environment Variables**

```bash
# Required
API_KEY_ETHERSCAN=your_etherscan_api_key
ALCHEMY_API_KEY=your_alchemy_api_key

# Authentication (choose one method)
# Method 1: Keystore (recommended)
KEYSTORE_ACCOUNT=your_keystore_account
KEYSTORE_PASSWORD=your_keystore_password

# Method 2: Private key (development)
PRIVATE_KEY=0x_your_private_key
```

## 🌍 **Supported Networks**

### **Ethereum**
- **Mainnet** - Production Ethereum network
- **Sepolia** - Primary Ethereum testnet
- **Localhost** - Local development (Anvil, Hardhat)

### **Layer 2**
- **Polygon** - Polygon PoS chain
- **Arbitrum** - Arbitrum One
- **Optimism** - Optimism mainnet
- **Base** - Coinbase's L2

### **Other Chains**
- **BSC** - Binance Smart Chain
- **Avalanche** - Avalanche C-Chain

*See [examples/multi-chain.toml](examples/multi-chain.toml) for complete network configurations.*

## 💡 **Usage Examples**

### **Deploy from Git Repository**
```bash
# Repository specified in config
contract-deployer --config deploy.toml

# With custom arguments
contract-deployer --config deploy.toml --args "--value" --args "1000000"
```

### **Deploy from Local Directory**
```bash
# No repo specified in config = deploy from current directory
contract-deployer --config local-deploy.toml
```

### **Multi-Chain Deployment**
```bash
# Deploy to multiple networks
contract-deployer --config mainnet-deploy.toml
contract-deployer --config polygon-deploy.toml
contract-deployer --config arbitrum-deploy.toml
```

### **Development Workflow**
```bash
# Start local network
anvil

# Deploy to local network
contract-deployer --config examples/development.toml

# Test and iterate
forge test

# Deploy to testnet
contract-deployer --config testnet-deploy.toml

# Deploy to mainnet
contract-deployer --config mainnet-deploy.toml
```

## 📁 **Configuration Examples**

### **Basic Deployment** ([examples/basic-deploy.toml](examples/basic-deploy.toml))
Simple configuration for deploying to Ethereum testnets.

### **Multi-Chain Setup** ([examples/multi-chain.toml](examples/multi-chain.toml))
Comprehensive setup supporting 15+ networks including Ethereum, Polygon, Arbitrum, Optimism, BSC, and more.

### **Development Setup** ([examples/development.toml](examples/development.toml))
Optimized for local development with Anvil, including pre-configured test accounts and fast deployment.

## 🔐 **Authentication Methods**

### **Keystore (Recommended for Production)**
```bash
# Create keystore
cast wallet import my-account --interactive

# Use in config
KEYSTORE_ACCOUNT=my-account
KEYSTORE_PASSWORD=secure_password
```

### **Private Key (Good for Development)**
```bash
# Use private key directly
PRIVATE_KEY=
```

## 🛠️ **Advanced Features**

### **Variable Expansion**
```toml
[networks.custom]
rpc_url = "https://rpc.${NETWORK_NAME}.example.com/${API_KEY}"
```

### **Multiple Environment Files**
```toml
[env]
load_files = [".env", ".env.local", ".env.${NETWORK}"]
```

### **Custom Setup Commands**
```toml
[project]
setup_command = "npm install && npm run build && forge build"
```

### **Network-Specific Variables**
```toml
[env.additional_vars]
DEPLOYMENT_SALT = "0x1234567890abcdef"
MIN_CONFIRMATION_BLOCKS = "12"
```

## 🚨 **Common Issues & Solutions**

### **Authentication Failed**
```bash
# Check keystore exists
cast wallet list

# Verify keystore address
cast wallet address --account your_account

# Test authentication
cast wallet address --account your_account --password your_password
```

### **Network Connection Failed**
```bash
# Test RPC endpoint
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' \
  https://eth-sepolia.g.alchemy.com/v2/YOUR_API_KEY
```

### **Contract Verification Failed**
```bash
# Check Etherscan API key
curl "https://api.etherscan.io/api?module=account&action=balance&address=0x1234&apikey=YOUR_KEY"

# Ensure network supports verification
verify = true  # in network config
```

### **Command Not Found**
```bash
# Check installation
ls ~/.contract-deployer/bin/contract-deployer

# Check PATH
echo $PATH | grep contract-deployer

# Reload shell
source ~/.bashrc  # or ~/.zshrc
```

## 🔄 **Migration from Foundry Scripts**

### **Before (Foundry)**
```bash
forge script script/Deploy.s.sol \
  --chain-id 11155111 \
  --rpc-url sepolia \
  --broadcast \
  --verify \
  --account my-account \
  --password my-password
```

### **After (Contract Deployer)**
```toml
# deploy.toml
[project]
name = "my-contract"
script = "Deploy"
network = "sepolia"
setup_command = "forge build"

[networks.sepolia]
chain_id = 11155111
rpc_url = "https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
verify = true
```

```bash
contract-deployer --config deploy.toml
```

## 📦 **Installation**

### **Supported Platforms**
- ✅ **Linux** (x86_64, ARM64)
- ✅ **macOS** (Intel, Apple Silicon)
- ❌ **Windows** (not supported)

### **Quick Install**
```bash
curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash
```

### **Manual Installation**
See [INSTALLATION.md](INSTALLATION.md) for detailed installation instructions.

### **Build from Source**
```bash
git clone https://github.com/0xdavid7/contract-deployer.git
cd contract-deployer
cargo build --release
cp target/release/contract-deployer ~/.local/bin/
```

## 🧪 **Development**

### **Prerequisites**
- Rust 1.70.0+
- Git
- Foundry (forge, cast, anvil)

### **Setup**
```bash
git clone https://github.com/0xdavid7/contract-deployer.git
cd contract-deployer
cargo build
```

### **Testing**
```bash
cargo test
```

### **Running Examples**
```bash
# Start local network
anvil

# Test with development config
cargo run -- --config examples/development.toml
```

## 📚 **Architecture**

```
contract-deployer/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # Command line interface
│   ├── config.rs         # TOML configuration
│   ├── environment.rs    # Environment management
│   └── deployer.rs       # Core deployment logic
├── examples/             # Configuration examples
└── .github/workflows/    # CI/CD pipeline
```

### **Key Components**

- **Configuration Parser** - Handles TOML parsing and validation
- **Environment Manager** - Loads and expands environment variables
- **Git Integration** - Clones repositories with SSH/HTTPS authentication
- **Forge Integration** - Executes forge scripts with proper authentication
- **Network Manager** - Handles multi-network deployments

## 🤝 **Contributing**

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Commit** your changes: `git commit -m 'Add amazing feature'`
4. **Push** to the branch: `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### **Development Guidelines**
- Follow Rust best practices
- Add tests for new features
- Update documentation
- Ensure CI passes

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 **Acknowledgments**

- **Foundry** - The underlying deployment infrastructure
- **Rust Community** - For excellent tooling and libraries
- **Ethereum Ecosystem** - For the standards and infrastructure

## 📞 **Support**

- **Documentation**: This README and [INSTALLATION.md](INSTALLATION.md)
- **Issues**: [GitHub Issues](https://github.com/0xdavid7/contract-deployer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/0xdavid7/contract-deployer/discussions)

## 🗺️ **Roadmap**

- [ ] **Windows Support** - Add Windows binaries and installer
- [ ] **Package Managers** - Homebrew, Chocolatey, AUR packages
- [ ] **Interactive Mode** - CLI wizard for configuration creation
- [ ] **Deployment History** - Track and manage past deployments
- [ ] **Multi-sig Support** - Enhanced support for multi-signature deployments
- [ ] **Plugin System** - Extensible architecture for custom deployment workflows

---


*Deploy smart contracts with confidence!*