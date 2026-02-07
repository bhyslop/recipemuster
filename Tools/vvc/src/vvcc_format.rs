// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Branded commit message formatting infrastructure
//!
//! Provides shared hallmark retrieval and message formatting for all branded commits
//! (jjb: for Job Jockey, vvb: for VOK builds, etc.)

use std::fs;
use std::process::Command;

/// Path to brand file containing vvbh_hallmark
pub const VVCC_BRAND_FILE_PATH: &str = ".vvk/vvbf_brand.json";

/// Path to registry file for fallback hallmark lookup
pub const VVCC_REGISTRY_PATH: &str = "Tools/vok/vov_veiled/vovr_registry.json";

/// JSON field name for hallmark in brand file
pub const VVCC_HALLMARK_FIELD: &str = "vvbh_hallmark";

/// Get hallmark for commit message versioning
///
/// Source logic:
/// 1. Try `.vvk/vvbf_brand.json` → if exists, use `vvbh_hallmark` (4 digits)
/// 2. If missing (Kit Forge) → read `Tools/vok/vov_veiled/vovr_registry.json`,
///    find max hallmark, get `git rev-parse --short HEAD`, format as `{hallmark}-{commit}`
pub fn vvcc_get_hallmark() -> String {
    // Try reading brand file
    if let Ok(brand_content) = fs::read_to_string(VVCC_BRAND_FILE_PATH) {
        if let Ok(brand_json) = serde_json::from_str::<serde_json::Value>(&brand_content) {
            if let Some(hallmark) = brand_json.get(VVCC_HALLMARK_FIELD) {
                if let Some(hallmark_str) = hallmark.as_str() {
                    return hallmark_str.to_string();
                }
            }
        }
    }

    // Fallback: Kit Forge mode (read registry + git HEAD)
    let mut max_hallmark = 0u32;
    if let Ok(registry_content) = fs::read_to_string(VVCC_REGISTRY_PATH) {
        if let Ok(registry_json) = serde_json::from_str::<serde_json::Value>(&registry_content) {
            if let Some(hallmarks) = registry_json.get("hallmarks").and_then(|v| v.as_object()) {
                for key in hallmarks.keys() {
                    if let Ok(num) = key.parse::<u32>() {
                        max_hallmark = max_hallmark.max(num);
                    }
                }
            }
        }
    }

    // Get short commit hash
    let git_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();

    let commit_hash = if let Ok(output) = git_output {
        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "0000000".to_string()
        }
    } else {
        "0000000".to_string()
    };

    format!("{:04}-{}", max_hallmark, commit_hash)
}

/// Format a branded commit message
///
/// Produces: `{brand}:{hallmark}:{identity}:{action}: {subject}`
/// with optional `\n\n{body}` appended.
///
/// Arguments:
/// - brand: Commit prefix (e.g., "jjb", "vvb")
/// - hallmark: Version identifier (e.g., "1011", "1011-a8c3738f")
/// - identity: Context identifier (e.g., "₢AWAAb", "₣AW", "" for none)
/// - action: Single-letter action code (e.g., "n", "B", "W")
/// - subject: Commit subject line
/// - body: Optional extended description
///
/// Format guarantees:
/// - Four colon-delimited fields always present
/// - Empty identity produces "brand:hallmark::action: subject"
/// - Body separated by double newline if present
/// - No trailing newlines when body is None
pub fn vvcc_format_branded(
    brand: &str,
    hallmark: &str,
    identity: &str,
    action: &str,
    subject: &str,
    body: Option<&str>,
) -> String {
    let mut message = format!("{}:{}:{}:{}: {}", brand, hallmark, identity, action, subject);

    if let Some(body_text) = body {
        message.push_str("\n\n");
        message.push_str(body_text);
    }

    message
}
