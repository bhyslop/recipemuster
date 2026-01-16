// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK Core - shared infrastructure
//!
//! Common utilities used across JJK modules.

#![allow(non_camel_case_types)]

use std::path::PathBuf;
use chrono::Local;

/// Default path to the Gallops JSON file
pub const JJRC_DEFAULT_GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";

/// Get the default Gallops file path relative to repo root
pub fn jjrc_default_gallops_path() -> PathBuf {
    PathBuf::from(JJRC_DEFAULT_GALLOPS_PATH)
}

/// Generate timestamp in YYMMDD format
pub fn jjrc_timestamp_date() -> String {
    Local::now().format("%y%m%d").to_string()
}

/// Generate timestamp in YYMMDD-HHMM format
pub fn jjrc_timestamp_full() -> String {
    Local::now().format("%y%m%d-%H%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_path() {
        let path = jjrc_default_gallops_path();
        assert!(path.to_str().unwrap().contains("jjg_gallops.json"));
    }
}
