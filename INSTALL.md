# Installation Guide

This guide covers various methods to install Oxygen on different Linux distributions and systems.

## 📦 Quick Install (Recommended)

### One-line installer (Linux/macOS)
```bash
curl -sSL https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh | bash
```

This script will:
- Detect your system architecture
- Try to download a precompiled binary (when available)
- Fall back to building from source if needed
- Install to `~/.local/bin/oxy`
- Guide you through PATH setup

### Custom install directory
```bash
curl -sSL https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh | bash -s -- --install-dir /usr/local/bin
```

### Force build from source
```bash
curl -sSL https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh | bash -s -- --source
```

## 🏗️ Distribution-Specific

### Arch Linux

#### Option 1: Using provided PKGBUILD
```bash
git clone https://github.com/ghostkellz/oxygen.git
cd oxygen
./build-arch-package.sh stable
sudo pacman -U oxygen-*.pkg.tar.*
```

#### Option 2: Git version (latest)
```bash
./build-arch-package.sh git
sudo pacman -U oxygen-git-*.pkg.tar.*
```

#### Option 3: Manual PKGBUILD
```bash
# Copy PKGBUILD to a build directory
mkdir -p ~/builds/oxygen
cp PKGBUILD ~/builds/oxygen/
cd ~/builds/oxygen
makepkg -si
```

### Ubuntu/Debian

#### Using the installer script (recommended)
```bash
curl -sSL https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh | bash
```

#### Manual installation
```bash
# Install Rust if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build and install
git clone https://github.com/ghostkellz/oxygen.git
cd oxygen
cargo build --release
sudo cp target/release/oxygen /usr/local/bin/oxy
```

### Fedora/CentOS/RHEL

#### Using the installer script
```bash
curl -sSL https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh | bash
```

#### Manual installation
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build and install
git clone https://github.com/ghostkellz/oxygen.git
cd oxygen
cargo build --release
sudo cp target/release/oxygen /usr/local/bin/oxy
```

## 🛠️ From Source

### Prerequisites
- Rust toolchain (1.70.0 or later)
- Git
- C compiler (gcc or clang)

### Installation steps
```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Option 1: Direct from git
cargo install --git https://github.com/ghostkellz/oxygen

# Option 2: Clone and build
git clone https://github.com/ghostkellz/oxygen.git
cd oxygen
cargo build --release

# Install the binary
sudo cp target/release/oxygen /usr/local/bin/oxy
# Or to user directory
mkdir -p ~/.local/bin
cp target/release/oxygen ~/.local/bin/oxy
```

### Development build
```bash
git clone https://github.com/ghostkellz/oxygen.git
cd oxygen
cargo build
./target/debug/oxygen --help
```

## 🔧 Post-Installation

### Verify installation
```bash
oxy --version
oxy doctor
```

### Add to PATH (if needed)
The installer usually handles this, but if needed:

```bash
# For ~/.local/bin installation
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.bashrc
source ~/.bashrc

# For zsh users
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.zshrc
source ~/.zshrc
```

### Shell completions (optional)
If you want shell completions, they can be generated using:
```bash
# This feature will be added in a future version
oxy completions bash > ~/.local/share/bash-completion/completions/oxy
oxy completions zsh > ~/.local/share/zsh/site-functions/_oxy
oxy completions fish > ~/.config/fish/completions/oxy.fish
```

## 🗑️ Uninstallation

### Remove the binary
```bash
# If installed to /usr/local/bin
sudo rm /usr/local/bin/oxy

# If installed to ~/.local/bin
rm ~/.local/bin/oxy

# Arch Linux package
sudo pacman -R oxygen
```

### Clean up configuration (optional)
```bash
# Remove any config files (if created in future versions)
rm -rf ~/.config/oxygen
rm -rf ~/.cache/oxygen
```

## 🆘 Troubleshooting

### Installation fails with "Rust not found"
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Permission denied errors
```bash
# Use ~/.local/bin instead of /usr/local/bin
curl -sSL https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh | bash -s -- --install-dir ~/.local/bin
```

### PATH issues
```bash
# Check if directory is in PATH
echo $PATH | grep -q "$HOME/.local/bin" && echo "In PATH" || echo "Not in PATH"

# Add to PATH
export PATH="$PATH:$HOME/.local/bin"
```

### Build failures
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Getting help
```bash
oxy doctor  # Diagnose environment issues
oxy --help  # Show all available commands
```

For more help, visit: https://github.com/ghostkellz/oxygen/issues