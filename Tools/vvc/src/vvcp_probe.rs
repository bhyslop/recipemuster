// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Probe - Claude Code environment probe
//!
//! Discovers model IDs and platform information by spawning minimal claude invocations.
//! See Tools/vok/vov_veiled/VOSRP-probe.adoc for specification.

use chrono::{DateTime, Local, Utc};
use tokio::process::Command;

/// Brand prefix for VVC commits
pub const VVCC_BRAND_PREFIX: &str = "vvb";

/// Action code for invitatory commits
pub const VVCP_ACTION_INVITATORY: &str = "i";

/// Officium token for invitatory commit subjects
pub const VVCP_OFFICIUM_TOKEN: &str = "OFFICIUM";

/// Gap threshold in seconds for officium detection (1 hour)
pub const VVCP_OFFICIUM_GAP_SECS: u64 = 3600;

/// Probe Claude Code environment for model IDs and platform information.
///
/// Spawns three parallel claude invocations to discover actual model versions.
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
    // Spawn three parallel claude invocations to probe model IDs
    let haiku_future = probe_model_tier("haiku");
    let sonnet_future = probe_model_tier("sonnet");
    let opus_future = probe_model_tier("opus");

    // Wait for all three probes in parallel
    let (haiku_id, sonnet_id, opus_id) = tokio::join!(haiku_future, sonnet_future, opus_future);

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
async fn probe_model_tier(tier: &str) -> String {
    let output = Command::new("claude")
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
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unavailable".to_string(),
    }
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

/// Check if an officium invitatory is needed
///
/// Searches git log for most recent invitatory commit (vvb:...:i:) and checks
/// if the time gap exceeds VVCP_OFFICIUM_GAP_SECS (1 hour).
///
/// Returns true if no invitatory found OR gap exceeds threshold.
pub(crate) async fn zvvcp_needs_officium() -> bool {
    // Build grep pattern from constants
    let pattern = format!("^{}:.*:{}:", VVCC_BRAND_PREFIX, VVCP_ACTION_INVITATORY);

    // Search git log for most recent invitatory commit
    let output = Command::new("git")
        .args([
            "log",
            "--all",
            &format!("--grep={}", pattern),
            "--format=%ai",
            "-1",
        ])
        .output()
        .await;

    let timestamp_str = match output {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let trimmed = output_str.trim();
            if trimmed.is_empty() {
                // No invitatory commit found
                return true;
            }
            trimmed.to_string()
        }
        _ => {
            // Git command failed or no commits found
            return true;
        }
    };

    // Parse timestamp (format: "2026-02-07 14:30:00 -0800")
    let parsed_time = match DateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S %z") {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            // Parse error - assume gap needed
            return true;
        }
    };

    let now = Utc::now();
    let gap_secs = (now - parsed_time).num_seconds();

    gap_secs.abs() as u64 > VVCP_OFFICIUM_GAP_SECS
}

/// Create an invitatory commit to open a new officium
///
/// Checks if invitatory is needed via zvvcp_needs_officium().
/// If not needed, prints "Officium current" and returns Ok.
/// Otherwise, probes for model IDs, gets hallmark, and creates
/// an empty branded commit with OFFICIUM timestamp.
pub async fn vvcp_invitatory() -> Result<(), String> {
    // Check if invitatory is needed
    if !zvvcp_needs_officium().await {
        println!("Officium current");
        return Ok(());
    }

    // Probe for model IDs
    let probe_data = vvcp_probe().await?;

    // Get hallmark
    let hallmark = crate::vvcc_get_hallmark();

    // Generate timestamp in YYMMDD-HHMM format
    let now = Local::now();
    let timestamp = now.format("%y%m%d-%H%M").to_string();

    // Format subject
    let subject = format!("{} {}", VVCP_OFFICIUM_TOKEN, timestamp);

    // Format commit message
    let message = crate::vvcc_format_branded(
        VVCC_BRAND_PREFIX,
        &hallmark,
        "",
        VVCP_ACTION_INVITATORY,
        &subject,
        Some(&probe_data),
    );

    // Create commit arguments
    let commit_args = crate::vvcc_CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
        size_limit: crate::VVCG_SIZE_LIMIT,
        warn_limit: crate::VVCG_WARN_LIMIT,
    };

    // Create empty commit
    let hash = match crate::vvcc_CommitLock::vvcc_acquire() {
        Ok(lock) => lock.vvcc_commit(&commit_args)?,
        Err(e) => return Err(format!("Failed to acquire lock: {}", e)),
    };

    println!("Officium: {} ({})", hash, timestamp);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
