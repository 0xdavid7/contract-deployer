# Configuration Examples Guide

This directory contains three example configuration files that demonstrate different deployment scenarios. Each file is designed for specific use cases and can be used as templates for your own projects.

## 📁 File Overview

| File | Use Case | Best For |
|------|----------|----------|
| `basic-deploy.toml` | Simple, single-network deployment | Most common deployments, beginners |
| `multi-chain.toml` | Cross-chain or multi-network deployment | Complex DeFi protocols, bridges |
| `development.toml` | Local development and testing | Development workflow, testing |

## 🚀 Quick Start

1. **Choose the right template** for your use case
2. **Copy the file** to your project directory
3. **Modify the values** to match your project
4. **Create the required `.env` files**
5. **Run the deployment**

## 📋 Basic Deploy (`basic-deploy.toml`)

### When to Use
- First-time smart contract deployment
- Simple ERC-20 tokens or NFT contracts
- Single network deployment
- Learning the deployment process

### Key Features
- ✅ Simple, minimal configuration
- ✅ Supports Ethereum mainnet, Sepolia, and localhost
- ✅ Standard environment variable setup
- ✅ Etherscan verification enabled
- ✅ Uses `bun install` for dependencies

### Setup Steps
```bash
# 1. Copy the template
cp examples/basic-deploy.toml my-deploy.toml

# 2. Create .env file
cat > .env << EOF
API_KEY_ETHERSCAN=your_etherscan_api_key
ALCHEMY_API_KEY=your_alchemy_api_key
KEYSTORE_ACCOUNT=your_keystore_account_name
KEYSTORE_PASSWORD=your_keystore_password
BROADCAST_ACCOUNT=0x_your_deployer_address
EOF

# 3. Deploy
contract-deployer --config my-deploy.toml --repo https://github.com/user/contract.git
```

### Customization
- Change `network = "mainnet"` to deploy to mainnet
- Modify `setup_command` if you use npm/yarn instead of bun
- Add custom RPC URLs in the `[networks.*]` sections

## 🌐 Multi-Chain Deploy (`multi-chain.toml`)

### When to Use
- Cross-chain protocols (bridges, DEXs)
- Multi-chain DeFi applications
- Projects targeting multiple Layer 2s
- Enterprise deployments

### Key Features
- ✅ 15+ pre-configured networks
- ✅ Supports Ethereum, Polygon, Arbitrum, Optimism, BSC, Avalanche, Base
- ✅ Network-specific environment files
- ✅ Complex setup commands with build steps
- ✅ Custom deployment variables

### Setup Steps
```bash
# 1. Copy the template
cp examples/multi-chain.toml bridge-deploy.toml

# 2. Create base .env file
cat > .env << EOF
API_KEY_ETHERSCAN=your_etherscan_api_key
ALCHEMY_API_KEY=your_alchemy_api_key
KEYSTORE_ACCOUNT=your_keystore_account_name
KEYSTORE_PASSWORD=your_keystore_password
BROADCAST_ACCOUNT=0x_your_deployer_address
EOF

# 3. Create network-specific files (optional)
cat > .env.mainnet << EOF
BRIDGE_VERSION=v2.1.0
MIN_CONFIRMATION_BLOCKS=12
EOF

cat > .env.polygon << EOF
BRIDGE_VERSION=v2.1.0
MIN_CONFIRMATION_BLOCKS=128
EOF

# 4. Deploy to specific network
# Edit config: network = "polygon"
contract-deployer --config bridge-deploy.toml --repo https://github.com/user/bridge.git
```

### Deployment Workflow
1. **Test on testnets first**: sepolia, mumbai, arbitrum_sepolia
2. **Verify functionality**: Run integration tests
3. **Deploy to mainnets**: mainnet, polygon, arbitrum, optimism
4. **Configure cross-chain**: Set up bridge connections

### Network Support
- **Ethereum**: mainnet, sepolia, goerli
- **Polygon**: polygon, polygon_mumbai
- **Arbitrum**: arbitrum, arbitrum_sepolia
- **Optimism**: optimism, optimism_sepolia
- **BSC**: bsc, bsc_testnet
- **Avalanche**: avalanche, avalanche_fuji
- **Base**: base, base_sepolia
- **Custom**: Easy to add new networks

## 🔧 Development Deploy (`development.toml`)

### When to Use
- Local development and testing
- Rapid iteration cycles
- Contract debugging
- CI/CD pipelines

### Key Features
- ✅ Local network optimized (anvil, hardhat, ganache)
- ✅ Pre-configured with anvil test accounts
- ✅ Fast deployment (no verification)
- ✅ Development-friendly defaults
- ✅ Multiple local network options

### Setup Steps
```bash
# 1. Start local network
anvil --host 0.0.0.0 --port 8545 --chain-id 31337

# 2. Copy the template
cp examples/development.toml dev-deploy.toml

# 3. Create .env.local (optional - defaults are provided)
cat > .env.local << EOF
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
MNEMONIC="test test test test test test test test test test test junk"
EOF

# 4. Deploy locally
contract-deployer --config dev-deploy.toml
```

### Development Workflow
```bash
# 1. Write contract
vim src/MyContract.sol

# 2. Deploy locally
contract-deployer --config dev-deploy.toml

# 3. Test functionality
cast call 0x_contract_address "balanceOf(address)" 0xf39Fd6...

# 4. Deploy to testnet
# Edit config: network = "sepolia_dev"
contract-deployer --config dev-deploy.toml --repo https://github.com/user/contract.git

# 5. Deploy to mainnet (use production config)
contract-deployer --config production-deploy.toml --repo https://github.com/user/contract.git
```

## 🔐 Environment Variables

### Required for All Configs
```bash
API_KEY_ETHERSCAN=your_etherscan_api_key
ALCHEMY_API_KEY=your_alchemy_api_key
KEYSTORE_ACCOUNT=your_keystore_account_name
KEYSTORE_PASSWORD=your_keystore_password
BROADCAST_ACCOUNT=0x_your_deployer_address
```

### Optional Overrides
```bash
# Custom RPC URLs
MAINNET_RPC_URL=https://custom-mainnet-rpc.com
POLYGON_RPC_URL=https://custom-polygon-rpc.com

# Gas settings
GAS_LIMIT=3000000
GAS_PRICE=20000000000

# Contract settings
DEPLOYMENT_SALT=0x1234567890abcdef
CONSTRUCTOR_ARGS=1000000,0x123...
```

## 🛠 Customization Tips

### 1. Adding New Networks
```toml
[networks.my_custom_network]
chain_id = 1337
rpc_url = "https://my-custom-rpc.com"
verify = false
```

### 2. Complex Setup Commands
```toml
[project]
setup_command = "npm install && npm run build && npm run test"
```

### 3. Environment File Precedence
```toml
[env]
# Later files override earlier ones
load_files = [".env", ".env.local", ".env.${NETWORK}"]
```

### 4. Network-Specific Variables
```toml
[env]
additional_vars = { 
    MAINNET_DEPLOY_BLOCK = "18500000",
    TESTNET_DEPLOY_BLOCK = "4500000"
}
```

## 🚨 Common Issues & Solutions

### Issue: Missing Environment Variables
**Error**: `Missing required environment variables: ALCHEMY_API_KEY`
**Solution**: Create `.env` file with all required variables

### Issue: Network Not Found
**Error**: `Network 'custom' not found in configuration`
**Solution**: Add network configuration to `[networks.custom]` section

### Issue: RPC Connection Failed
**Error**: `Failed to connect to RPC endpoint`
**Solution**: Check RPC URL and ensure network is accessible

### Issue: Verification Failed
**Error**: `Contract verification failed`
**Solution**: Ensure `API_KEY_ETHERSCAN` is correct and network supports verification

## 📚 Advanced Usage

### Using Multiple Configs
```bash
# Development
contract-deployer --config dev-deploy.toml

# Testing
contract-deployer --config test-deploy.toml --repo https://github.com/user/contract.git

# Production
contract-deployer --config prod-deploy.toml --repo https://github.com/user/contract.git
```

### Passing Extra Arguments
```bash
contract-deployer --config basic-deploy.toml \
  --args "--constructor-args" \
  --args "1000000000000000000" \
  --args "--gas-limit" \
  --args "3000000"
```

### Environment-Specific Deployment
```bash
# Set NETWORK environment variable
export NETWORK=mainnet
contract-deployer --config multi-chain.toml --repo https://github.com/user/contract.git
```

## 📖 Next Steps

1. **Copy** the appropriate template for your use case
2. **Modify** the configuration values
3. **Set up** your environment variables
4. **Test** on a testnet first
5. **Deploy** to mainnet when ready

For more advanced configuration options, check the main documentation or create custom configurations based on these examples.