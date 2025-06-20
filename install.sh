#!/usr/bin/env bash
set -euo pipefail

# Oxygen installer script for Linux systems
# Usage: curl -sSL https://raw.githubusercontent.com/yourname/oxygen/main/install.sh | bash
# Or: wget -qO- https://raw.githubusercontent.com/yourname/oxygen/main/install.sh | bash

GITHUB_REPO="ghostkellz/oxygen"
BINARY_NAME="oxy"
INSTALL_DIR="${HOME}/.local/bin"
TEMP_DIR=$(mktemp -d)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Cleanup function
cleanup() {
    if [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

# Detect architecture and OS
detect_platform() {
    local arch
    local os
    
    case "$(uname -m)" in
        x86_64) arch="x86_64" ;;
        aarch64) arch="aarch64" ;;
        arm64) arch="aarch64" ;;
        *) 
            log_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
    
    case "$(uname -s)" in
        Linux) os="unknown-linux-gnu" ;;
        Darwin) os="apple-darwin" ;;
        *)
            log_error "Unsupported OS: $(uname -s)"
            exit 1
            ;;
    esac
    
    echo "${arch}-${os}"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install from precompiled binary (when releases are available)
install_from_release() {
    local platform="$1"
    local latest_version
    
    log_info "Fetching latest release information..."
    
    if command_exists curl; then
        latest_version=$(curl -sSL "https://api.github.com/repos/${GITHUB_REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)
    elif command_exists wget; then
        latest_version=$(wget -qO- "https://api.github.com/repos/${GITHUB_REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)
    else
        log_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi
    
    if [[ -z "$latest_version" ]]; then
        log_warning "Could not fetch latest version, falling back to build from source"
        return 1
    fi
    
    local download_url="https://github.com/${GITHUB_REPO}/releases/download/${latest_version}/oxygen-${latest_version}-${platform}.tar.gz"
    local archive_path="${TEMP_DIR}/oxygen.tar.gz"
    
    log_info "Downloading Oxygen ${latest_version} for ${platform}..."
    
    if command_exists curl; then
        if ! curl -sSL -o "$archive_path" "$download_url"; then
            log_warning "Download failed, falling back to build from source"
            return 1
        fi
    elif command_exists wget; then
        if ! wget -qO "$archive_path" "$download_url"; then
            log_warning "Download failed, falling back to build from source"
            return 1
        fi
    fi
    
    log_info "Extracting archive..."
    tar -xzf "$archive_path" -C "$TEMP_DIR"
    
    # Find the binary in the extracted files
    local binary_path
    binary_path=$(find "$TEMP_DIR" -name "$BINARY_NAME" -type f | head -1)
    
    if [[ -z "$binary_path" ]]; then
        log_error "Binary not found in archive"
        return 1
    fi
    
    # Install the binary
    mkdir -p "$INSTALL_DIR"
    cp "$binary_path" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    log_success "Oxygen installed successfully to ${INSTALL_DIR}/${BINARY_NAME}"
    return 0
}

# Build from source
build_from_source() {
    log_info "Building Oxygen from source..."
    
    # Check if Rust is installed
    if ! command_exists cargo; then
        log_error "Rust/Cargo not found. Please install Rust first:"
        log_info "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Check if git is installed
    if ! command_exists git; then
        log_error "Git not found. Please install git first."
        exit 1
    fi
    
    log_info "Cloning repository..."
    git clone "https://github.com/${GITHUB_REPO}.git" "$TEMP_DIR/oxygen"
    cd "$TEMP_DIR/oxygen"
    
    log_info "Building release binary..."
    
    # Check what binaries cargo thinks should be built
    log_info "Checking project metadata..."
    if command_exists cargo; then
        cargo metadata --no-deps --format-version 1 2>/dev/null | grep -o '"name":"[^"]*"' | grep -v "oxygen$" || true
    fi
    
    if ! cargo build --release; then
        log_error "Build failed"
        exit 1
    fi
    
    log_info "Build completed. Checking what was built..."
    
    # Check if any binaries were actually built
    log_info "Contents of target/release/:"
    ls -la target/release/ 2>/dev/null || true
    
    # Look for any files that might be our binary (exclude known non-binaries)
    log_info "Looking for potential binaries (excluding build artifacts)..."
    find target/release/ -maxdepth 1 -type f -executable ! -name "*.so" ! -name "*.d" ! -name ".*" 2>/dev/null | while read -r file; do
        echo "  Found: $file"
        file "$file" 2>/dev/null || true
    done
    
    # Install the binary
    mkdir -p "$INSTALL_DIR"
    
    # Find the release binary
    local binary_path
    log_info "Looking for binary in target/release/..."
    ls -la target/release/ || true
    
    # Look for the main binary (not build scripts or deps)
    if [[ -f "target/release/oxygen" ]]; then
        binary_path="target/release/oxygen"
        log_info "Found binary at: $binary_path"
    elif [[ -f "target/release/${BINARY_NAME}" ]]; then
        binary_path="target/release/${BINARY_NAME}"
        log_info "Found binary at: $binary_path"
    else
        # Try to find the main executable (exclude build scripts and deps)
        log_info "Searching for main executable..."
        local found_binary
        found_binary=$(find target/release/ -maxdepth 1 -type f -executable ! -name "build-*" ! -name "*.so" ! -name "deps" 2>/dev/null | head -1)
        if [[ -n "$found_binary" ]]; then
            binary_path="$found_binary"
            log_info "Found executable binary at: $binary_path"
        else
            log_error "No main executable binary found. The build may have failed."
            log_info "Expected binary locations:"
            log_info "  - target/release/oxygen"
            log_info "  - target/release/${BINARY_NAME}"
            log_info "Available files in target/release/:"
            ls -la target/release/ || true
            exit 1
        fi
    fi
    
    cp "$binary_path" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    log_success "Oxygen built and installed successfully to ${INSTALL_DIR}/${BINARY_NAME}"
}

# Check PATH and add install directory if needed
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log_warning "Install directory $INSTALL_DIR is not in your PATH"
        log_info "Add this line to your shell profile (.bashrc, .zshrc, etc.):"
        echo
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        echo
        log_info "Or run this command to add it to your current session:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        echo
    fi
}

# Main installation function
main() {
    echo
    log_info "🦀 Oxygen Installer for Linux"
    log_info "==============================="
    echo
    
    local platform
    platform=$(detect_platform)
    log_info "Detected platform: $platform"
    
    # Try to install from release first, fall back to building from source
    if ! install_from_release "$platform"; then
        log_info "Installing from source..."
        build_from_source
    fi
    
    # Verify installation
    if [[ -x "${INSTALL_DIR}/${BINARY_NAME}" ]]; then
        log_success "Installation completed!"
        echo
        log_info "Installed version:"
        "${INSTALL_DIR}/${BINARY_NAME}" --version
        echo
        check_path
        echo
        log_info "Get started with: ${BINARY_NAME} --help"
        log_info "View available templates: ${BINARY_NAME} init --list-templates"
        log_info "Check your environment: ${BINARY_NAME} doctor"
    else
        log_error "Installation verification failed"
        exit 1
    fi
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "Oxygen Installer"
        echo
        echo "Usage: $0 [OPTIONS]"
        echo
        echo "Options:"
        echo "  --help, -h          Show this help message"
        echo "  --install-dir DIR   Custom installation directory (default: ~/.local/bin)"
        echo "  --source            Force build from source"
        echo
        echo "Environment Variables:"
        echo "  INSTALL_DIR         Installation directory"
        echo "  GITHUB_REPO         GitHub repository (default: yourname/oxygen)"
        echo
        exit 0
        ;;
    --install-dir)
        if [[ -n "${2:-}" ]]; then
            INSTALL_DIR="$2"
            shift 2
        else
            log_error "--install-dir requires a directory argument"
            exit 1
        fi
        ;;
    --source)
        # Skip release download and build from source
        build_from_source
        check_path
        exit 0
        ;;
    --*)
        log_error "Unknown option: $1"
        log_info "Use --help for usage information"
        exit 1
        ;;
esac

# Run main installation
main