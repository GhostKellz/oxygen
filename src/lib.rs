use clap::Subcommand;

pub mod commands;
pub mod config;
pub mod utils;

#[derive(Subcommand)]
pub enum ToolchainAction {
    /// List installed toolchains
    List,
    /// Install a toolchain
    Install {
        /// Toolchain to install (stable, beta, nightly, or specific version)
        toolchain: String,
    },
    /// Set default toolchain
    Default {
        /// Toolchain to set as default
        toolchain: String,
    },
    /// Show active toolchain
    Show,
    /// Remove a toolchain
    Remove {
        /// Toolchain to remove
        toolchain: String,
    },
}

#[derive(Subcommand)]
pub enum DepsAction {
    /// Show dependency tree with vulnerabilities
    Tree,
    /// Check for outdated dependencies
    Outdated,
    /// Audit dependencies for security issues
    Audit,
    /// Show dependency licenses
    Licenses,
    /// Analyze dependency sizes
    Size,
}

#[derive(Subcommand)]
pub enum GpgAction {
    /// Sign a release or commit
    Sign {
        /// What to sign (commit, tag, or file)
        target: String,
    },
    /// Verify GPG signature
    Verify {
        /// What to verify
        target: String,
    },
    /// Setup GPG for Rust development
    Setup,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Basic sanity test
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_utils_format_duration() {
        use std::time::Duration;
        let duration = Duration::from_millis(1500);
        let formatted = utils::format_duration(duration);
        assert!(formatted.contains("1.5"));
    }

    #[test]
    fn test_utils_is_rust_project() {
        // This should be true since we're in a Rust project
        assert!(utils::is_rust_project());
    }
}