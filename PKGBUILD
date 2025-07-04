# Maintainer: Christopher Kelley <ckelley@ghostkellz.sh>
pkgname=oxygen
pkgver=0.2.0
pkgrel=1
pkgdesc="The essential Rust dev environment enhancer"
arch=('x86_64' 'aarch64')
url="https://github.com/ghostkellz/oxygen"
license=('MIT' 'Apache-2.0')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
optdepends=('rustup: for toolchain management'
           'cargo-audit: for dependency security auditing'
           'cargo-outdated: for checking outdated dependencies'
           'cargo-bloat: for dependency size analysis'
           'cargo-watch: for file watching capabilities'
           'gpg: for GPG signing and verification')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP') # Replace with actual checksum when available

prepare() {
    cd "$pkgname-$pkgver"
    # Generate/update Cargo.lock if needed
    cargo fetch --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-$pkgver"
    
    # Set build flags for optimized release
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    
    # Build with release optimizations (remove --frozen to allow lock file updates)
    cargo build --release --all-features
}

check() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    
    # Run tests
    cargo test --release --all-features
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
    
    # Install license if available
    if [[ -f LICENSE ]]; then
        install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    elif [[ -f LICENSE-MIT ]]; then
        install -Dm644 LICENSE-MIT "$pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT"
    fi
    
    # Install documentation if available
    if [[ -f README.md ]]; then
        install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    fi
    
    # Install shell completions if they exist
    if [[ -f "completions/oxy.bash" ]]; then
        install -Dm644 "completions/oxy.bash" "$pkgdir/usr/share/bash-completion/completions/oxy"
    fi
    
    if [[ -f "completions/oxy.zsh" ]]; then
        install -Dm644 "completions/oxy.zsh" "$pkgdir/usr/share/zsh/site-functions/_oxy"
    fi
    
    if [[ -f "completions/oxy.fish" ]]; then
        install -Dm644 "completions/oxy.fish" "$pkgdir/usr/share/fish/vendor_completions.d/oxy.fish"
    fi
    
    # Install man page if it exists
    if [[ -f "docs/oxy.1" ]]; then
        install -Dm644 "docs/oxy.1" "$pkgdir/usr/share/man/man1/oxy.1"
    fi
}