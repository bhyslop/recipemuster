// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Probe - Claude Code environment probe
//!
//! Discovers model IDs and platform information by spawning minimal claude invocations.
//! See Tools/vok/vov_veiled/VOSRP-probe.adoc for specification.

use tokio::process::Command;

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
