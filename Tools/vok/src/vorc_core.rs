// core.rs - Shared infrastructure for vvr
//
// Common utilities used across VOK and kit-specific Rust code.

use std::env;
use std::path::PathBuf;

/// Get the directory where vvr binary is located
pub fn binary_dir() -> Option<PathBuf> {
    env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf()))
}

/// Detect current platform in VOK naming convention
pub fn platform() -> &'static str {
    match (env::consts::OS, env::consts::ARCH) {
        ("macos", "aarch64") => "darwin-arm64",
        ("macos", "x86_64") => "darwin-x86_64",
        ("linux", "x86_64") => "linux-x86_64",
        ("linux", "aarch64") => "linux-aarch64",
        ("windows", "x86_64") => "windows-x86_64",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_not_unknown() {
        // On any supported platform, we should get a known value
        let p = platform();
        assert!(
            ["darwin-arm64", "darwin-x86_64", "linux-x86_64", "linux-aarch64", "windows-x86_64"]
                .contains(&p),
            "Unexpected platform: {}",
            p
        );
    }
}
