// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Branded commit message formatting infrastructure
//!
//! Provides shared brand retrieval and message formatting for all branded commits
//! (jjb: for Job Jockey, vvb: for VOK builds, etc.)

use std::fs;

/// Path to brand file containing vvbh_brand
pub const VVCC_BRAND_FILE_PATH: &str = ".vvk/vvbf_brand.json";

/// Path to registry file for fallback brand lookup
pub const VVCC_REGISTRY_PATH: &str = "Tools/vok/vov_veiled/vovr_registry.json";

/// JSON field name for brand in brand file
pub const VVCC_BRAND_FIELD: &str = "vvbf_brand";

/// JSON field name for brands map in registry file
pub const VVCC_REGISTRY_BRANDS_FIELD: &str = "vovr_brands";

/// Get brand for commit message versioning
///
/// Source logic:
/// 1. Try `.vvk/vvbf_brand.json` → if exists, use `vvbh_brand` (4 digits)
/// 2. If missing (Kit Forge) → read `Tools/vok/vov_veiled/vovr_registry.json`,
///    find max brand, get `git rev-parse --short HEAD`, format as `{brand}-{commit}`
pub fn vvcc_get_brand() -> String {
    // Try reading brand file
    if let Ok(brand_content) = fs::read_to_string(VVCC_BRAND_FILE_PATH) {
        if let Ok(brand_json) = serde_json::from_str::<serde_json::Value>(&brand_content) {
            if let Some(brand) = brand_json.get(VVCC_BRAND_FIELD) {
                if let Some(brand_str) = brand.as_str() {
                    return brand_str.to_string();
                }
            }
        }
    }

    // Fallback: Kit Forge mode (read registry + git HEAD)
    let mut max_brand = 0u32;
    if let Ok(registry_content) = fs::read_to_string(VVCC_REGISTRY_PATH) {
        if let Ok(registry_json) = serde_json::from_str::<serde_json::Value>(&registry_content) {
            if let Some(brands) = registry_json.get(VVCC_REGISTRY_BRANDS_FIELD).and_then(|v| v.as_object()) {
                for key in brands.keys() {
                    if let Ok(num) = key.parse::<u32>() {
                        max_brand = max_brand.max(num);
                    }
                }
            }
        }
    }

    // Get short commit hash
    let git_output = crate::vvce_git_command(&["rev-parse", "--short", "HEAD"])
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

    format!("{:04}-{}", max_brand, commit_hash)
}

/// Format a branded commit message
///
/// Produces: `{prefix}:{brand}:{identity}:{action}: {subject}`
/// with optional `\n\n{body}` appended.
///
/// Arguments:
/// - prefix: Commit prefix (e.g., "jjb", "vvb")
/// - brand: Version identifier (e.g., "1011", "1011-a8c3738f")
/// - identity: Context identifier (e.g., "₢AWAAb", "₣AW", "" for none)
/// - action: Single-letter action code (e.g., "n", "B", "W")
/// - subject: Commit subject line
/// - body: Optional extended description
///
/// Format guarantees:
/// - Four colon-delimited fields always present
/// - Empty identity produces "prefix:brand::action: subject"
/// - Body separated by double newline if present
/// - No trailing newlines when body is None
pub fn vvcc_format_branded(
    prefix: &str,
    brand: &str,
    identity: &str,
    action: &str,
    subject: &str,
    body: Option<&str>,
) -> String {
    let mut message = format!("{}:{}:{}:{}: {}", prefix, brand, identity, action, subject);

    if let Some(body_text) = body {
        message.push_str("\n\n");
        message.push_str(body_text);
    }

    message
}
