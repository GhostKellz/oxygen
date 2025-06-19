#!/usr/bin/env bash
set -euo pipefail

# Script to build Arch Linux packages for Oxygen
# Usage: ./build-arch-package.sh [stable|git]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_TYPE="${1:-stable}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# Check if running on Arch Linux or derivative
check_arch_linux() {
    if ! command -v makepkg >/dev/null 2>&1; then
        log_error "makepkg not found. This script requires Arch Linux or an Arch-based distribution."
        exit 1
    fi
    
    if ! command -v pacman >/dev/null 2>&1; then
        log_error "pacman not found. This script requires Arch Linux or an Arch-based distribution."
        exit 1
    fi
}

# Install build dependencies
install_build_deps() {
    log_info "Checking build dependencies..."
    
    local deps=("rust" "cargo" "base-devel")
    local missing_deps=()
    
    for dep in "${deps[@]}"; do
        if ! pacman -Qi "$dep" >/dev/null 2>&1; then
            missing_deps+=("$dep")
        fi
    done
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_warning "Missing dependencies: ${missing_deps[*]}"
        log_info "Installing missing dependencies..."
        sudo pacman -S --needed "${missing_deps[@]}"
    else
        log_success "All build dependencies are installed"
    fi
}

# Build stable package
build_stable() {
    log_info "Building stable package..."
    
    local build_dir="$SCRIPT_DIR/pkg-build-stable"
    rm -rf "$build_dir"
    mkdir -p "$build_dir"
    
    # Copy PKGBUILD
    cp "$SCRIPT_DIR/PKGBUILD" "$build_dir/"
    cd "$build_dir"
    
    # Update checksums
    log_info "Updating checksums..."
    updpkgsums
    
    # Build package
    log_info "Building package with makepkg..."
    makepkg -sf --noconfirm
    
    log_success "Stable package built successfully!"
    
    # List generated packages
    local packages
    packages=(*.pkg.tar.*)
    if [[ ${#packages[@]} -gt 0 && -f "${packages[0]}" ]]; then
        log_info "Generated packages:"
        for pkg in "${packages[@]}"; do
            echo "  $pkg"
        done
        
        log_info "Install with: sudo pacman -U ${packages[0]}"
    fi
}

# Build git package
build_git() {
    log_info "Building git package..."
    
    local build_dir="$SCRIPT_DIR/pkg-build-git"
    rm -rf "$build_dir"
    mkdir -p "$build_dir"
    
    # Copy PKGBUILD-git as PKGBUILD
    cp "$SCRIPT_DIR/PKGBUILD-git" "$build_dir/PKGBUILD"
    cd "$build_dir"
    
    # Build package
    log_info "Building package with makepkg..."
    makepkg -sf --noconfirm
    
    log_success "Git package built successfully!"
    
    # List generated packages
    local packages
    packages=(*.pkg.tar.*)
    if [[ ${#packages[@]} -gt 0 && -f "${packages[0]}" ]]; then
        log_info "Generated packages:"
        for pkg in "${packages[@]}"; do
            echo "  $pkg"
        done
        
        log_info "Install with: sudo pacman -U ${packages[0]}"
    fi
}

# Main function
main() {
    echo
    log_info "📦 Oxygen Arch Linux Package Builder"
    log_info "===================================="
    echo
    
    check_arch_linux
    install_build_deps
    
    case "$BUILD_TYPE" in
        stable)
            build_stable
            ;;
        git)
            build_git
            ;;
        both)
            build_stable
            echo
            build_git
            ;;
        *)
            log_error "Invalid build type: $BUILD_TYPE"
            log_info "Usage: $0 [stable|git|both]"
            exit 1
            ;;
    esac
    
    echo
    log_success "Package build completed!"
    echo
    log_info "💡 Tips:"
    log_info "  • Install: sudo pacman -U <package-file>"
    log_info "  • Remove: sudo pacman -R oxygen"
    log_info "  • Verify: oxy --version"
}

# Handle help flag
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    echo "Oxygen Arch Linux Package Builder"
    echo
    echo "Usage: $0 [BUILD_TYPE]"
    echo
    echo "Build Types:"
    echo "  stable    Build from latest release (default)"
    echo "  git       Build from latest git commit"
    echo "  both      Build both stable and git packages"
    echo
    echo "Requirements:"
    echo "  • Arch Linux or Arch-based distribution"
    echo "  • makepkg and pacman"
    echo "  • Internet connection for downloading dependencies"
    echo
    exit 0
fi

main