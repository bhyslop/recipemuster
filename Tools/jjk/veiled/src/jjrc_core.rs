//! JJK Core - shared infrastructure
//!
//! Common utilities used across JJK modules.

use std::path::PathBuf;
use chrono::Local;

/// Default path to the Gallops JSON file
pub const DEFAULT_GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";

/// Get the default Gallops file path relative to repo root
pub fn default_gallops_path() -> PathBuf {
    PathBuf::from(DEFAULT_GALLOPS_PATH)
}

/// Generate timestamp in YYMMDD format
pub fn timestamp_date() -> String {
    Local::now().format("%y%m%d").to_string()
}

/// Generate timestamp in YYMMDD-HHMM format
pub fn timestamp_full() -> String {
    Local::now().format("%y%m%d-%H%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_path() {
        let path = default_gallops_path();
        assert!(path.to_str().unwrap().contains("jjg_gallops.json"));
    }
}
