//! JJK Core - shared infrastructure
//!
//! Common utilities used across JJK modules.

use std::path::PathBuf;

/// Default path to the Gallops JSON file
pub const DEFAULT_GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";

/// Get the default Gallops file path relative to repo root
pub fn default_gallops_path() -> PathBuf {
    PathBuf::from(DEFAULT_GALLOPS_PATH)
}

/// Generate timestamp in YYMMDD format
pub fn timestamp_date() -> String {
    let now = std::time::SystemTime::now();
    let datetime = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    // Simple conversion - will be replaced with proper chrono usage if needed
    let secs = datetime.as_secs();
    let days = secs / 86400;
    // Approximate calculation - good enough for stub
    let years = 1970 + (days / 365);
    let yy = (years % 100) as u8;
    format!("{:02}0101", yy) // Placeholder - implement properly later
}

/// Generate timestamp in YYMMDD-HHMM format
pub fn timestamp_full() -> String {
    format!("{}-0000", timestamp_date())
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
