#!/usr/bin/env bash
set -euo pipefail

# Oxygen uninstaller script
# Usage: ./uninstall.sh

BINARY_NAME="oxy"
INSTALL_DIR="${HOME}/.local/bin"

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

main() {
    echo
    log_info "🗑️  Oxygen Uninstaller"
    log_info "====================="
    echo
    
    local binary_path="${INSTALL_DIR}/${BINARY_NAME}"
    
    if [[ -f "$binary_path" ]]; then
        log_info "Removing binary from: $binary_path"
        rm -f "$binary_path"
        log_success "Binary removed successfully"
    else
        log_warning "Binary not found at: $binary_path"
    fi
    
    # Check if there are any oxygen-related files
    local found_files
    found_files=$(find "${INSTALL_DIR}" -name "*oxygen*" -o -name "*oxy*" 2>/dev/null || true)
    
    if [[ -n "$found_files" ]]; then
        log_info "Found other oxygen-related files:"
        echo "$found_files"
        echo
        read -p "Remove these files? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "$found_files" | xargs rm -f
            log_success "Additional files removed"
        fi
    fi
    
    echo
    log_success "Uninstallation completed!"
    log_info "You may want to remove ${INSTALL_DIR} from your PATH if no longer needed"
}

main "$@"