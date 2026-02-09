// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Probe - Claude Code environment probe
//!
//! Discovers model IDs and platform information by spawning minimal claude invocations.
//! See Tools/vok/vov_veiled/VOSRP-probe.adoc for specification.

use chrono::{DateTime, Local, Utc};
use regex::Regex;
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

/// Brand prefix for VVC commits
pub const VVCC_BRAND_PREFIX: &str = "vvb";

/// Action code for invitatory commits
pub const VVCP_ACTION_INVITATORY: &str = "i";

/// Officium token for invitatory commit subjects
pub const VVCP_OFFICIUM_TOKEN: &str = "OFFICIUM";

/// Gap threshold in seconds for officium detection (1 hour).
/// For manual testing, lower to 60 and rebuild; restore after.
pub const VVCP_OFFICIUM_GAP_SECS: u64 = 3600;

/// Raw probe output file for haiku
const VVCP_RAW_HAIKU_FILE: &str = "vvcp_raw_haiku.txt";

/// Raw probe output file for sonnet
const VVCP_RAW_SONNET_FILE: &str = "vvcp_raw_sonnet.txt";

/// Raw probe output file for opus
const VVCP_RAW_OPUS_FILE: &str = "vvcp_raw_opus.txt";

/// XML element name for haiku model ID
const VVCP_ELEMENT_HAIKU: &str = "vvpxh_haiku";

/// XML element name for sonnet model ID
const VVCP_ELEMENT_SONNET: &str = "vvpxs_sonnet";

/// XML element name for opus model ID
const VVCP_ELEMENT_OPUS: &str = "vvpxo_opus";

/// Environment variable for BURD_TEMP_DIR
const VVCP_BURD_TEMP_DIR_VAR: &str = "BURD_TEMP_DIR";

/// Probe Claude Code environment for model IDs and platform information.
///
/// Spawns haiku and sonnet probes in parallel, then asks opus to interpret
/// the raw outputs and self-report. Returns a 5-line string in "key: value" format.
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
    // Spawn haiku and sonnet probes in parallel to get raw output
    let haiku_future = probe_model_tier_raw("haiku");
    let sonnet_future = probe_model_tier_raw("sonnet");

    // Wait for both probes
    let (haiku_raw, sonnet_raw) = tokio::join!(haiku_future, sonnet_future);

    // Get BURD_TEMP_DIR from environment
    let temp_dir = std::env::var(VVCP_BURD_TEMP_DIR_VAR).ok();

    // Write raw haiku and sonnet outputs to files if BURD_TEMP_DIR is set
    if let Some(ref dir) = temp_dir {
        let _ = write_raw_output(dir, VVCP_RAW_HAIKU_FILE, &haiku_raw).await;
        let _ = write_raw_output(dir, VVCP_RAW_SONNET_FILE, &sonnet_raw).await;
    }

    // Build opus prompt from constants
    let prompt = format!(
        "Report your own model ID. Then extract the Claude model ID from each raw output below.\n\
        \n\
        <raw_haiku>\n\
        {}\n\
        </raw_haiku>\n\
        \n\
        <raw_sonnet>\n\
        {}\n\
        </raw_sonnet>\n\
        \n\
        Respond with exactly:\n\
        <{}>[opus model ID]</{}>\n\
        <{}>[haiku model ID]</{}>\n\
        <{}>[sonnet model ID]</{}>",
        haiku_raw,
        sonnet_raw,
        VVCP_ELEMENT_OPUS,
        VVCP_ELEMENT_OPUS,
        VVCP_ELEMENT_HAIKU,
        VVCP_ELEMENT_HAIKU,
        VVCP_ELEMENT_SONNET,
        VVCP_ELEMENT_SONNET
    );

    // Invoke opus with the prompt (no --system-prompt flag)
    let opus_output = Command::new("claude")
        .args(["-p", "--model", "opus", "--no-session-persistence", "--", &prompt])
        .output()
        .await;

    let opus_raw = match opus_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        _ => String::new(),
    };

    // Write raw opus response if BURD_TEMP_DIR is set
    if let Some(ref dir) = temp_dir {
        let _ = write_raw_output(dir, VVCP_RAW_OPUS_FILE, &opus_raw).await;
    }

    // Parse opus response with regexes
    let haiku_id = extract_xml_element(&opus_raw, VVCP_ELEMENT_HAIKU);
    let sonnet_id = extract_xml_element(&opus_raw, VVCP_ELEMENT_SONNET);
    let opus_id = extract_xml_element(&opus_raw, VVCP_ELEMENT_OPUS);

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
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).to_string(),
        _ => String::new(),
    }
}

/// Write raw probe output to file
async fn write_raw_output(dir: &str, filename: &str, content: &str) -> Result<(), String> {
    let mut path = PathBuf::from(dir);
    path.push(filename);

    fs::write(&path, content)
        .await
        .map_err(|e| format!("Failed to write {}: {}", filename, e))
}

/// Extract content from XML element using regex
/// Returns "unavailable" if element not found or empty
fn extract_xml_element(text: &str, element: &str) -> String {
    let pattern = format!(r"<{}>(.*?)</{}>", regex::escape(element), regex::escape(element));
    let re = match Regex::new(&pattern) {
        Ok(re) => re,
        Err(_) => return "unavailable".to_string(),
    };

    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unavailable".to_string())
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

    #[test]
    fn test_extract_xml_element_clean() {
        // Test with clean XML
        let xml = "<vvpxh_haiku>claude-3-5-haiku-20241022</vvpxh_haiku>";
        let result = extract_xml_element(xml, VVCP_ELEMENT_HAIKU);
        assert_eq!(result, "claude-3-5-haiku-20241022");

        let xml = "<vvpxs_sonnet>claude-sonnet-4-20250514</vvpxs_sonnet>";
        let result = extract_xml_element(xml, VVCP_ELEMENT_SONNET);
        assert_eq!(result, "claude-sonnet-4-20250514");

        let xml = "<vvpxo_opus>claude-opus-4-5-20251101</vvpxo_opus>";
        let result = extract_xml_element(xml, VVCP_ELEMENT_OPUS);
        assert_eq!(result, "claude-opus-4-5-20251101");
    }

    #[test]
    fn test_extract_xml_element_missing() {
        // Test with missing element returns unavailable
        let xml = "<vvpxs_sonnet>claude-sonnet-4-20250514</vvpxs_sonnet>";
        let result = extract_xml_element(xml, VVCP_ELEMENT_HAIKU);
        assert_eq!(result, "unavailable");
    }

    #[test]
    fn test_extract_xml_element_empty() {
        // Test with empty element returns unavailable
        let xml = "<vvpxh_haiku></vvpxh_haiku>";
        let result = extract_xml_element(xml, VVCP_ELEMENT_HAIKU);
        assert_eq!(result, "unavailable");

        // Test with whitespace-only element
        let xml = "<vvpxh_haiku>   </vvpxh_haiku>";
        let result = extract_xml_element(xml, VVCP_ELEMENT_HAIKU);
        assert_eq!(result, "unavailable");
    }

    #[test]
    fn test_extract_xml_element_with_extra_text() {
        // Test with extra text around XML still extracts correctly
        let xml = "Here is the information you requested:\n\
                   <vvpxh_haiku>claude-3-5-haiku-20241022</vvpxh_haiku>\n\
                   <vvpxs_sonnet>claude-sonnet-4-20250514</vvpxs_sonnet>\n\
                   <vvpxo_opus>claude-opus-4-5-20251101</vvpxo_opus>\n\
                   I hope this helps!";

        let result = extract_xml_element(xml, VVCP_ELEMENT_HAIKU);
        assert_eq!(result, "claude-3-5-haiku-20241022");

        let result = extract_xml_element(xml, VVCP_ELEMENT_SONNET);
        assert_eq!(result, "claude-sonnet-4-20250514");

        let result = extract_xml_element(xml, VVCP_ELEMENT_OPUS);
        assert_eq!(result, "claude-opus-4-5-20251101");
    }

    #[test]
    fn test_extract_xml_element_with_whitespace() {
        // Test that trimming works inside elements
        let xml = "<vvpxh_haiku>  claude-3-5-haiku-20241022  </vvpxh_haiku>";
        let result = extract_xml_element(xml, VVCP_ELEMENT_HAIKU);
        assert_eq!(result, "claude-3-5-haiku-20241022");
    }
}
