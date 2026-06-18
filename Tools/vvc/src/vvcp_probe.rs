// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Probe - Claude Code environment probe
//!
//! Discovers model IDs and platform information by spawning minimal claude invocations.
//! See Tools/vok/vov_veiled/VOSRP-probe.adoc for specification.

use chrono::{DateTime, Utc};
use tokio::process::Command;

/// Probe Claude Code environment for model IDs and platform information.
///
/// Spawns haiku, sonnet, and opus probes in parallel via `claude -p`.
/// Returns a 5-line string in "key: value" format.
///
/// # Returns
///
/// Five-line string:
/// ```text
/// haiku: claude-3-5-haiku-20241022
/// sonnet: claude-sonnet-4-20250514
/// opus: claude-opus-4-5-20251101
/// host: macbook-pro.local
/// platform: darwin-arm64
/// ```
///
/// On probe failure for a tier, returns "unavailable" for that tier.
pub async fn vvcp_probe() -> Result<String, String> {
    let haiku_future = probe_model_tier_raw("haiku");
    let sonnet_future = probe_model_tier_raw("sonnet");
    let opus_future = probe_model_tier_raw("opus");
    let (haiku_raw, sonnet_raw, opus_raw) = tokio::join!(haiku_future, sonnet_future, opus_future);

    // Extract the canonical model-ID token from each reply (see zvvcp_extract_model_id); a chatty
    // or refusing reply must not pollute the output or the jjx_open invitatory commit body.
    let haiku_id = zvvcp_extract_model_id(&haiku_raw).unwrap_or_else(|| "unavailable".to_string());
    let sonnet_id = zvvcp_extract_model_id(&sonnet_raw).unwrap_or_else(|| "unavailable".to_string());
    let opus_id = zvvcp_extract_model_id(&opus_raw).unwrap_or_else(|| "unavailable".to_string());

    // Collect hostname
    let hostname = get_hostname().await;

    // Collect platform (OS and architecture)
    let platform = get_platform().await;

    // Format as 5-line string
    let result = format!(
        "haiku: {}\nsonnet: {}\nopus: {}\nhost: {}\nplatform: {}",
        haiku_id, sonnet_id, opus_id, hostname, platform
    );

    Ok(result)
}

/// Probe a single model tier by invoking claude with minimal prompt
/// Returns raw stdout without any trimming or parsing
async fn probe_model_tier_raw(tier: &str) -> String {
    let output = Command::from(crate::vvce_claude_command())
        .args([
            "-p",
            "--model",
            tier,
            "--no-session-persistence",
            "--",
            "Report your exact model ID string only",
        ])
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).to_string(),
        _ => String::new(),
    }
}

/// Extract the canonical Claude model-ID token from a probe reply, ignoring surrounding prose.
///
/// The model is prompted for a bare ID, but a chatty or refusing reply embeds the ID in prose
/// (or omits it). Trusting the raw reply pollutes the jjx_open invitatory commit body and breaks
/// the fixed 5-line probe-output contract, so we extract the surveyed signature and treat anything
/// else as unavailable.
///
/// Every released Claude model ID is `claude-` followed by lowercase alphanumerics, hyphens, and
/// (rarely) dots, optionally trailed by a bracketed context-window suffix such as `[1m]`. This
/// holds across the full history — `claude-2.1`, `claude-3-5-haiku-20241022`, `claude-opus-4-8`,
/// `claude-opus-4-8[1m]`, `claude-fable-5`. Pattern references:
///   https://platform.claude.com/docs/en/about-claude/models/model-ids-and-versions
///   https://tygartmedia.com/claude-api-model/
///   https://claudefa.st/blog/models
fn zvvcp_extract_model_id(raw: &str) -> Option<String> {
    let start = raw.find("claude-")?;
    let bytes = raw.as_bytes();
    let mut end = start;
    while end < bytes.len() {
        let c = bytes[end];
        if c.is_ascii_alphanumeric() || c == b'-' || c == b'.' {
            end += 1;
        } else {
            break;
        }
    }
    // A real ID never ends in a dot, so drop a trailing sentence period ("...claude-opus-4-8.");
    // claude-2.1 keeps its dot because it ends in a digit.
    while end > start && bytes[end - 1] == b'.' {
        end -= 1;
    }
    // Optional bracketed context-window suffix, e.g. the `[1m]` on `claude-opus-4-8[1m]`.
    if raw[end..].starts_with('[') {
        if let Some(close) = raw[end..].find(']') {
            end += close + 1;
        }
    }
    Some(raw[start..end].to_string())
}

/// Get hostname via std::env or hostname command
async fn get_hostname() -> String {
    // Try environment variable first
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        return hostname;
    }

    // Fall back to hostname command
    let output = Command::new("hostname").output().await;

    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    }
}

/// Get platform information (OS and architecture)
async fn get_platform() -> String {
    // Spawn uname -s and uname -m in parallel
    let os_future = Command::new("uname").arg("-s").output();
    let arch_future = Command::new("uname").arg("-m").output();

    let (os_result, arch_result) = tokio::join!(os_future, arch_future);

    let os = match os_result {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_lowercase()
        }
        _ => "unknown".into(),
    };

    let arch = match arch_result {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    };

    format!("{}-{}", os, arch)
}

/// Brand prefix for VVC commits
const ZVVCP_BRAND_PREFIX: &str = "vvb";

/// Action code for invitatory commits
pub const VVCP_ACTION_INVITATORY: &str = "i";

/// Gap threshold in seconds for officium detection (1 hour).
const ZVVCP_OFFICIUM_GAP_SECS: u64 = 3600;

/// Check if an officium invitatory is needed.
///
/// Searches git log for most recent invitatory commit (vvb:...:i:) and checks
/// if the time gap exceeds the threshold (1 hour).
/// Returns true if no invitatory found OR gap exceeds threshold.
async fn zvvcp_needs_officium() -> bool {
    let pattern = format!("^{}:.*:{}:", ZVVCP_BRAND_PREFIX, VVCP_ACTION_INVITATORY);

    let output = Command::from(crate::vvce_git_command(&[
            "log",
            "--all",
            &format!("--grep={}", pattern),
            "--format=%ai",
            "-1",
        ]))
        .output()
        .await;

    let timestamp_str = match output {
        Ok(output) if output.status.success() => {
            let s = String::from_utf8_lossy(&output.stdout);
            let trimmed = s.trim().to_string();
            if trimmed.is_empty() { return true; }
            trimmed
        }
        _ => return true,
    };

    match DateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S %z") {
        Ok(dt) => {
            let gap = (Utc::now() - dt.with_timezone(&Utc)).num_seconds();
            gap.unsigned_abs() > ZVVCP_OFFICIUM_GAP_SECS
        }
        Err(_) => true,
    }
}

/// Create an invitatory commit to open a new officium.
///
/// Checks gap threshold first — no-ops if a recent invitatory exists (within 1 hour).
/// Otherwise probes for model IDs and creates an empty branded commit.
pub async fn vvcp_invitatory() -> Result<(), String> {
    if !zvvcp_needs_officium().await {
        return Ok(());
    }

    let probe_data = vvcp_probe().await?;
    let brand = crate::vvcc_get_brand();
    let now = chrono::Local::now();
    let timestamp = now.format("%y%m%d-%H%M").to_string();
    let subject = format!("OFFICIUM {}", timestamp);

    let message = crate::vvcc_format_branded(
        ZVVCP_BRAND_PREFIX,
        &brand,
        "",
        VVCP_ACTION_INVITATORY,
        &subject,
        Some(&probe_data),
    );

    let commit_args = crate::vvcc_CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
        size_limit: crate::VVCG_SIZE_LIMIT,
        warn_limit: crate::VVCG_WARN_LIMIT,
    };

    let mut output = crate::vvco_Output::buffer();
    match crate::vvcc_CommitLock::vvcc_acquire() {
        Ok(lock) => { lock.vvcc_commit(&commit_args, &mut output)?; }
        Err(e) => return Err(format!("invitatory lock: {}", e)),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_model_id() {
        // Bare ID.
        assert_eq!(
            zvvcp_extract_model_id("claude-opus-4-8"),
            Some("claude-opus-4-8".to_string())
        );
        // Context-window bracket suffix is preserved.
        assert_eq!(
            zvvcp_extract_model_id("claude-opus-4-8[1m]"),
            Some("claude-opus-4-8[1m]".to_string())
        );
        // Extracted from a refusing/chatty reply (the shape that flaked the suite).
        assert_eq!(
            zvvcp_extract_model_id(
                "I will not read any project files for this.\n\nclaude-opus-4-8"
            ),
            Some("claude-opus-4-8".to_string())
        );
        // Trailing sentence period dropped; embedded dotted version kept.
        assert_eq!(
            zvvcp_extract_model_id("My model ID is claude-opus-4-8."),
            Some("claude-opus-4-8".to_string())
        );
        assert_eq!(
            zvvcp_extract_model_id("claude-2.1"),
            Some("claude-2.1".to_string())
        );
        // Dated historical ID.
        assert_eq!(
            zvvcp_extract_model_id("claude-3-5-haiku-20241022"),
            Some("claude-3-5-haiku-20241022".to_string())
        );
        // No model ID present.
        assert_eq!(zvvcp_extract_model_id("I cannot help with that."), None);
    }

    #[tokio::test]
    async fn test_probe_output_format() {
        // This test verifies the output format structure
        // Actual model IDs may be "unavailable" in test environments
        let result = vvcp_probe().await;

        assert!(result.is_ok());
        let output = result.unwrap();

        // Should have exactly 5 lines
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 5, "Expected 5 lines, got: {}", output);

        // Verify line format (key: value)
        assert!(lines[0].starts_with("haiku: "), "Line 0: {}", lines[0]);
        assert!(lines[1].starts_with("sonnet: "), "Line 1: {}", lines[1]);
        assert!(lines[2].starts_with("opus: "), "Line 2: {}", lines[2]);
        assert!(lines[3].starts_with("host: "), "Line 3: {}", lines[3]);
        assert!(lines[4].starts_with("platform: "), "Line 4: {}", lines[4]);

        // Verify each line has content after the colon
        for (i, line) in lines.iter().enumerate() {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            assert_eq!(parts.len(), 2, "Line {} missing colon separator: {}", i, line);
            assert!(!parts[1].is_empty(), "Line {} has empty value: {}", i, line);
        }
    }

    #[tokio::test]
    async fn test_get_hostname() {
        let hostname = get_hostname().await;
        assert!(!hostname.is_empty());
        assert_ne!(hostname, "unknown");
    }

    #[tokio::test]
    async fn test_get_platform() {
        let platform = get_platform().await;
        assert!(platform.contains('-'), "Platform should be OS-ARCH format");
        assert!(!platform.starts_with("unknown-"), "OS should be detected");
    }

}
