#!/bin/bash

# Contract Deployer Installer Script
# Install location: ~/.contract-deployer/bin/contract-deployer
# Usage: curl -fsSL https://raw.githubusercontent.com/0xdavid7/contract-deployer/main/install.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
REPO="0xdavid7/contract-deployer"
BINARY_NAME="contract-deployer"
INSTALL_DIR="$HOME/.contract-deployer"
BIN_DIR="$INSTALL_DIR/bin"

# Helper functions
log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect platform
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case $os in
        linux*)
            OS="linux"
            ;;
        darwin*)
            OS="darwin"
            ;;
        *)
            error "Unsupported operating system: $os. Only Linux and macOS are supported."
            ;;
    esac
    
    case $arch in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch. Only x86_64 and aarch64 are supported."
            ;;
    esac
    
    if [ "$OS" = "linux" ]; then
        TARGET="${ARCH}-unknown-linux-gnu"
    elif [ "$OS" = "darwin" ]; then
        TARGET="${ARCH}-apple-darwin"
    fi
    
    log "Detected platform: $OS-$ARCH (target: $TARGET)"
}

# Get latest release version
get_latest_version() {
    log "Fetching latest release version..."
    
    if command -v curl >/dev/null 2>&1; then
        VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        VERSION=$(wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        error "Neither curl nor wget is available. Please install one of them."
    fi
    
    if [ -z "$VERSION" ]; then
        error "Failed to get latest version"
    fi
    
    log "Latest version: $VERSION"
}

# Download and install binary
install_binary() {
    local filename="${BINARY_NAME}-${VERSION}-${TARGET}.tar.gz"
    local download_url="https://github.com/$REPO/releases/download/$VERSION/$filename"
    local temp_dir=$(mktemp -d)
    
    log "Downloading $filename..."
    
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$download_url" -o "$temp_dir/$filename"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$download_url" -O "$temp_dir/$filename"
    else
        error "Neither curl nor wget is available"
    fi
    
    log "Extracting archive..."
    tar -xzf "$temp_dir/$filename" -C "$temp_dir"
    
    local extracted_dir="$temp_dir/${BINARY_NAME}-${VERSION}-${TARGET}"
    local binary_path="$extracted_dir/$BINARY_NAME"
    
    if [ ! -f "$binary_path" ]; then
        error "Binary not found in archive"
    fi
    
    log "Installing to $INSTALL_DIR..."
    
    # Create installation directories
    mkdir -p "$BIN_DIR"
    mkdir -p "$INSTALL_DIR/examples"
    
    # Install binary
    cp "$binary_path" "$BIN_DIR/$BINARY_NAME"
    chmod +x "$BIN_DIR/$BINARY_NAME"
    
    # Copy examples and documentation
    if [ -d "$extracted_dir/examples" ]; then
        cp -r "$extracted_dir/examples/"* "$INSTALL_DIR/examples/"
        log "Examples copied to $INSTALL_DIR/examples"
    fi
    
    if [ -f "$extracted_dir/README.md" ]; then
        cp "$extracted_dir/README.md" "$INSTALL_DIR/"
    fi
    
    if [ -f "$extracted_dir/INSTALLATION.md" ]; then
        cp "$extracted_dir/INSTALLATION.md" "$INSTALL_DIR/"
    fi
    
    # Cleanup
    rm -rf "$temp_dir"
    
    log "Installation completed successfully!"
}

# Add to PATH
setup_path() {
    local shell_profile=""
    local current_shell=$(basename "$SHELL")
    
    case $current_shell in
        bash)
            shell_profile="$HOME/.bashrc"
            ;;
        zsh)
            shell_profile="$HOME/.zshrc"
            ;;
        fish)
            shell_profile="$HOME/.config/fish/config.fish"
            ;;
        *)
            shell_profile="$HOME/.profile"
            ;;
    esac
    
    # Check if PATH is already set
    if grep -q "$BIN_DIR" "$shell_profile" 2>/dev/null; then
        log "PATH already configured in $shell_profile"
        return
    fi
    
    log "Adding $BIN_DIR to PATH in $shell_profile"
    
    # Add to PATH
    if [ "$current_shell" = "fish" ]; then
        echo "set -gx PATH \$PATH $BIN_DIR" >> "$shell_profile"
    else
        echo "" >> "$shell_profile"
        echo "# Contract Deployer" >> "$shell_profile"
        echo "export PATH=\"\$HOME/.contract-deployer/bin:\$PATH\"" >> "$shell_profile"
    fi
    
    # Also add to current session
    export PATH="$BIN_DIR:$PATH"
    
    log "PATH configured. You may need to restart your terminal or run:"
    log "  source $shell_profile"
}

# Verify installation
verify_installation() {
    log "Verifying installation..."
    
    if [ -x "$BIN_DIR/$BINARY_NAME" ]; then
        log "✅ Binary installed successfully at $BIN_DIR/$BINARY_NAME"
        
        # Check if it's in PATH
        if command -v "$BINARY_NAME" >/dev/null 2>&1; then
            local version=$($BINARY_NAME --version 2>/dev/null || echo "unknown")
            log "✅ $BINARY_NAME is in PATH and working"
            log "Version: $version"
        else
            warn "⚠️  $BINARY_NAME is not in PATH yet"
            warn "Please restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
        fi
    else
        error "❌ Installation failed: binary not found"
    fi
}

# Check dependencies
check_dependencies() {
    log "Checking dependencies..."
    
    local missing_deps=()
    
    if ! command -v tar >/dev/null 2>&1; then
        missing_deps+=("tar")
    fi
    
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        missing_deps+=("curl or wget")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        error "Missing dependencies: ${missing_deps[*]}"
    fi
    
    log "✅ All dependencies satisfied"
}

# Main installation function
main() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║                    Contract Deployer Installer                ║"
    echo "║                                                               ║"
    echo "║  Installing to: ~/.contract-deployer/bin/                    ║"
    echo "║  Supported: Linux (x64/ARM64), macOS (Intel/Apple Silicon)  ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    check_dependencies
    detect_platform
    get_latest_version
    install_binary
    setup_path
    verify_installation
    
    echo ""
    echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
    echo "2. Copy example config: cp ~/.contract-deployer/examples/basic-deploy.toml ./deploy.toml"
    echo "3. Edit your configuration file"
    echo "4. Set up environment variables in .env file"
    echo "5. Deploy: contract-deployer --config deploy.toml"
    echo ""
    echo "Examples location: ~/.contract-deployer/examples/"
    echo "Documentation: ~/.contract-deployer/README.md"
    echo ""
    echo "For help: contract-deployer --help"
    echo "Issues: https://github.com/$REPO/issues"
}

# Run installer
main "$@"