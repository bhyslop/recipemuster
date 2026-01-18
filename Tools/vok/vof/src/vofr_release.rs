// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVK Parcel Release Utilities
//!
//! Provides asset collection and branding for VVK parcel creation.
//!
//! - `vofr_collect`: Enumerate kit files, copy to staging (excludes vov_veiled/)
//! - `vofr_brand`: Compute super-SHA, allocate hallmark, write brand file

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::vofc_registry::{DISTRIBUTABLE_KITS, vofc_Kit, VOFC_COMMAND_SIGNET_SUFFIX, VOFC_HOOK_SIGNET_SUFFIX};

// =============================================================================
// Constants
// =============================================================================

/// Starting hallmark for new registries
const VOFR_HALLMARK_START: u32 = 1000;

/// Brand file JSON field tags (match VOS spec entity members)
const VOFR_BRAND_HALLMARK: &str = "vvbh_hallmark";
const VOFR_BRAND_DATE: &str = "vvbd_date";
const VOFR_BRAND_SHA: &str = "vvbs_sha";
const VOFR_BRAND_COMMIT: &str = "vvbc_commit";
const VOFR_BRAND_KITS: &str = "vvbk_kits";

/// Registry JSON field
const VOFR_REGISTRY_HALLMARKS: &str = "hallmarks";

// =============================================================================
// Public Types
// =============================================================================

/// Result of asset collection.
#[derive(Debug)]
pub struct vofr_CollectResult {
    /// Number of files copied per kit
    pub kit_counts: BTreeMap<String, u32>,
    /// Total files copied
    pub total_files: u32,
    /// Commands routed to commands/ directory
    pub commands_routed: u32,
}

/// Result of branding operation.
#[derive(Debug)]
pub struct vofr_BrandResult {
    /// Allocated hallmark
    pub hallmark: u32,
    /// Whether this was a new hallmark (vs reused)
    pub is_new: bool,
    /// Computed super-SHA
    pub super_sha: String,
}

/// Registry entry for a hallmark.
#[derive(Debug, Clone)]
pub struct vofr_RegistryEntry {
    pub date: String,
    pub sha: String,
}

// =============================================================================
// Public Functions
// =============================================================================

/// Collect kit assets to staging directory.
///
/// # Arguments
/// * `tools_dir` - Source Tools/ directory
/// * `staging_dir` - Target staging directory
/// * `install_script_path` - Path to vvi_install.sh to copy to staging root
/// * `managed_kits` - List of kit IDs to include (from BURC_MANAGED_KITS)
///
/// # Returns
/// CollectResult with file counts, or error message
pub fn vofr_collect(
    tools_dir: &Path,
    staging_dir: &Path,
    install_script_path: &Path,
    managed_kits: &[String],
) -> Result<vofr_CollectResult, String> {
    let mut kit_counts = BTreeMap::new();
    let mut total_files = 0u32;
    let mut commands_routed = 0u32;

    // Create staging directories
    let kits_dir = staging_dir.join("kits");
    fs::create_dir_all(&kits_dir)
        .map_err(|e| format!("Failed to create kits dir: {}", e))?;

    // Process only kits in managed_kits list
    for kit_id in managed_kits {
        let kit = DISTRIBUTABLE_KITS
            .iter()
            .find(|k| k.cipher.kit_id() == *kit_id)
            .ok_or_else(|| format!("Kit '{}' not found in registry", kit_id))?;

        let kit_source = tools_dir.join(kit_id);
        let kit_staging = kits_dir.join(kit_id);

        if !kit_source.exists() {
            return Err(format!("Kit source not found: {}", kit_source.display()));
        }

        let (count, cmd_count) = zvofr_collect_kit(&kit_source, &kit_staging, kit)?;
        kit_counts.insert(kit_id.clone(), count);
        total_files += count;
        commands_routed += cmd_count;

        // Copy managed section templates from vov_veiled/ to templates/
        let template_count = zvofr_copy_templates(&kit_source, &kit_staging, kit)?;
        total_files += template_count;

        // Collect Claude config assets (commands and hooks) from kit forge
        let cipher = kit.cipher.prefix();
        let (cmds, hks) = zvofr_collect_claude_assets(tools_dir, staging_dir, cipher)?;
        total_files += cmds + hks;
        commands_routed += cmds;
    }

    // Copy install script to staging root
    let install_dest = staging_dir.join("vvi_install.sh");
    fs::copy(install_script_path, &install_dest)
        .map_err(|e| format!("Failed to copy install script: {}", e))?;
    total_files += 1;

    Ok(vofr_CollectResult {
        kit_counts,
        total_files,
        commands_routed,
    })
}

/// Brand a staging directory with hallmark and metadata.
///
/// # Arguments
/// * `staging_dir` - Staging directory with collected assets
/// * `registry_path` - Path to vovr_registry.json
/// * `commit_sha` - Current git commit SHA
/// * `managed_kits` - List of kit IDs included (from BURC_MANAGED_KITS)
///
/// # Returns
/// BrandResult with hallmark info, or error message
pub fn vofr_brand(
    staging_dir: &Path,
    registry_path: &Path,
    commit_sha: &str,
    managed_kits: &[String],
) -> Result<vofr_BrandResult, String> {
    // Compute super-SHA of staging content
    let super_sha = zvofr_compute_super_sha(staging_dir)?;

    // Load or create registry
    let mut registry = zvofr_load_registry(registry_path)?;

    // Check if SHA already exists
    let (hallmark, is_new) = if let Some(existing) = zvofr_find_by_sha(&registry, &super_sha) {
        (existing, false)
    } else {
        let new_hallmark = zvofr_allocate_hallmark(&registry);
        (new_hallmark, true)
    };

    // Generate timestamp
    let date = zvofr_generate_date();

    // If new, update registry
    if is_new {
        registry.insert(hallmark, vofr_RegistryEntry {
            date: date.clone(),
            sha: super_sha.clone(),
        });
        zvofr_save_registry(registry_path, &registry)?;
    }

    // Write brand file with managed_kits as the kit list
    let brand_path = staging_dir.join("vvbf_brand.json");
    zvofr_write_brand_file(&brand_path, hallmark, &date, &super_sha, commit_sha, managed_kits)?;

    Ok(vofr_BrandResult {
        hallmark,
        is_new,
        super_sha,
    })
}

/// Load registry from JSON file.
pub fn vofr_load_registry(path: &Path) -> Result<BTreeMap<u32, vofr_RegistryEntry>, String> {
    zvofr_load_registry(path)
}

// =============================================================================
// Internal Functions (zvofr_*)
// =============================================================================

/// Copy managed section templates from vov_veiled/ to templates/.
fn zvofr_copy_templates(
    kit_source: &Path,
    kit_staging: &Path,
    kit: &vofc_Kit,
) -> Result<u32, String> {
    let mut count = 0u32;

    if kit.managed_sections.is_empty() {
        return Ok(0);
    }

    let veiled_dir = kit_source.join("vov_veiled");
    let templates_dir = kit_staging.join("templates");

    for section in kit.managed_sections {
        let template_source = veiled_dir.join(section.template_path);
        if !template_source.exists() {
            return Err(format!(
                "Template not found for kit {}: {}",
                kit.cipher.kit_id(),
                template_source.display()
            ));
        }

        fs::create_dir_all(&templates_dir)
            .map_err(|e| format!("Failed to create templates dir: {}", e))?;

        let template_dest = templates_dir.join(section.template_path);
        fs::copy(&template_source, &template_dest)
            .map_err(|e| format!("Failed to copy template {}: {}", section.template_path, e))?;

        count += 1;
    }

    Ok(count)
}

/// Collect a single kit's assets.
fn zvofr_collect_kit(
    source: &Path,
    staging: &Path,
    kit: &vofc_Kit,
) -> Result<(u32, u32), String> {
    let mut count = 0u32;
    let mut cmd_count = 0u32;
    let cipher = kit.cipher.prefix();

    // Walk source directory
    let entries = zvofr_walk_dir(source)?;

    for entry in entries {
        let rel_path = entry.strip_prefix(source)
            .map_err(|e| format!("Path strip failed: {}", e))?;

        // Skip vov_veiled/ directory
        if zvofr_is_veiled_path(rel_path) {
            continue;
        }

        // Determine destination
        let file_name = rel_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let dest = if zvofr_is_command_file(file_name, cipher) {
            // Route commands to commands/ subdirectory
            cmd_count += 1;
            let commands_dir = staging.join("commands");
            fs::create_dir_all(&commands_dir)
                .map_err(|e| format!("Failed to create commands dir: {}", e))?;
            commands_dir.join(file_name)
        } else {
            // Default: preserve relative path
            staging.join(rel_path)
        };

        // Create parent directories
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create dir {}: {}", parent.display(), e))?;
        }

        // Copy file
        fs::copy(&entry, &dest)
            .map_err(|e| format!("Failed to copy {} to {}: {}", entry.display(), dest.display(), e))?;

        count += 1;
    }

    Ok((count, cmd_count))
}

/// Collect Claude config assets for a single kit.
/// Scans .claude/commands/ and .claude/hooks/ in kit forge for cipher-matched files.
/// Returns (commands_collected, hooks_collected).
fn zvofr_collect_claude_assets(
    kit_forge: &Path,
    staging: &Path,
    cipher: &str,
) -> Result<(u32, u32), String> {
    let mut commands_count = 0u32;
    let mut hooks_count = 0u32;

    // Collect commands from .claude/commands/
    let commands_source = kit_forge.join(".claude").join("commands");
    if commands_source.exists() {
        let commands_dest = staging.join("claude").join("commands");
        fs::create_dir_all(&commands_dest)
            .map_err(|e| format!("Failed to create claude/commands dir: {}", e))?;

        commands_count = zvofr_copy_matching_files(&commands_source, &commands_dest, cipher, true)?;
    }

    // Collect hooks from .claude/hooks/
    let hooks_source = kit_forge.join(".claude").join("hooks");
    if hooks_source.exists() {
        let hooks_dest = staging.join("claude").join("hooks");
        fs::create_dir_all(&hooks_dest)
            .map_err(|e| format!("Failed to create claude/hooks dir: {}", e))?;

        hooks_count = zvofr_copy_matching_files(&hooks_source, &hooks_dest, cipher, false)?;
    }

    Ok((commands_count, hooks_count))
}

/// Copy files matching cipher pattern from source to dest.
/// is_command determines whether to match commands (c-) or hooks (h-).
/// Returns count of files copied.
fn zvofr_copy_matching_files(
    source_dir: &Path,
    dest_dir: &Path,
    cipher: &str,
    is_command: bool,
) -> Result<u32, String> {
    let mut count = 0u32;

    let entries = fs::read_dir(source_dir)
        .map_err(|e| format!("Failed to read directory {}: {}", source_dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Dir entry error: {}", e))?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let matches = if is_command {
            zvofr_is_command_file(file_name, cipher)
        } else {
            zvofr_is_hook_file(file_name, cipher)
        };

        if matches {
            let dest_path = dest_dir.join(file_name);
            fs::copy(&path, &dest_path)
                .map_err(|e| format!("Failed to copy {} to {}: {}", path.display(), dest_path.display(), e))?;
            count += 1;
        }
    }

    Ok(count)
}

/// Walk directory recursively, returning all file paths.
fn zvofr_walk_dir(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();

    if !dir.is_dir() {
        return Ok(files);
    }

    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read dir {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Dir entry error: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(zvofr_walk_dir(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

/// Check if path is within vov_veiled/ directory.
fn zvofr_is_veiled_path(rel_path: &Path) -> bool {
    for component in rel_path.components() {
        if let std::path::Component::Normal(name) = component {
            if name == "vov_veiled" {
                return true;
            }
        }
    }
    false
}

/// Check if file is a command file (matches {cipher}c-*.md pattern).
fn zvofr_is_command_file(file_name: &str, cipher: &str) -> bool {
    let prefix = format!("{}{}", cipher, VOFC_COMMAND_SIGNET_SUFFIX);
    file_name.starts_with(&prefix) && file_name.ends_with(".md")
}

/// Check if file is a hook file (matches {cipher}h-*.md pattern).
fn zvofr_is_hook_file(file_name: &str, cipher: &str) -> bool {
    let prefix = format!("{}{}", cipher, VOFC_HOOK_SIGNET_SUFFIX);
    file_name.starts_with(&prefix) && file_name.ends_with(".md")
}

/// Compute order-independent super-SHA of staging directory.
fn zvofr_compute_super_sha(staging_dir: &Path) -> Result<String, String> {
    use std::collections::BTreeSet;

    let mut file_hashes: BTreeSet<String> = BTreeSet::new();

    let files = zvofr_walk_dir(staging_dir)?;

    for file_path in files {
        let rel_path = file_path.strip_prefix(staging_dir)
            .map_err(|e| format!("Strip prefix failed: {}", e))?;

        // Skip brand file itself
        if rel_path == Path::new("vvbf_brand.json") {
            continue;
        }

        // Hash: path + content
        let path_str = rel_path.to_string_lossy();
        let content = fs::read(&file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

        let hash = zvofr_sha256_bytes(format!("{}:{}", path_str, zvofr_sha256_bytes(&content)).as_bytes());
        file_hashes.insert(hash);
    }

    // Combine all hashes
    let combined = file_hashes.into_iter().collect::<Vec<_>>().join("\n");
    Ok(zvofr_sha256_bytes(combined.as_bytes()))
}

/// Compute SHA256 of bytes, return hex string.
fn zvofr_sha256_bytes(data: &[u8]) -> String {
    // Simple SHA256 implementation using standard library
    // In production, would use sha2 crate, but avoiding extra dependencies
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = Command::new("shasum")
        .args(["-a", "256"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn shasum");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(data).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read shasum output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.split_whitespace().next().unwrap_or("").to_string()
}

/// Load registry from JSON file.
fn zvofr_load_registry(path: &Path) -> Result<BTreeMap<u32, vofr_RegistryEntry>, String> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read registry: {}", e))?;

    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse registry JSON: {}", e))?;

    let hallmarks = json.get(VOFR_REGISTRY_HALLMARKS)
        .and_then(|v| v.as_object())
        .ok_or_else(|| "Registry missing 'hallmarks' object".to_string())?;

    let mut registry = BTreeMap::new();

    for (key, value) in hallmarks {
        let hallmark: u32 = key.parse()
            .map_err(|e| format!("Invalid hallmark key '{}': {}", key, e))?;

        let date = value.get("date")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Hallmark {} missing 'date'", hallmark))?
            .to_string();

        let sha = value.get("sha")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Hallmark {} missing 'sha'", hallmark))?
            .to_string();

        registry.insert(hallmark, vofr_RegistryEntry { date, sha });
    }

    Ok(registry)
}

/// Save registry to JSON file.
fn zvofr_save_registry(path: &Path, registry: &BTreeMap<u32, vofr_RegistryEntry>) -> Result<(), String> {
    let mut hallmarks = serde_json::Map::new();

    for (hallmark, entry) in registry {
        let mut entry_obj = serde_json::Map::new();
        entry_obj.insert("date".to_string(), serde_json::Value::String(entry.date.clone()));
        entry_obj.insert("sha".to_string(), serde_json::Value::String(entry.sha.clone()));
        hallmarks.insert(hallmark.to_string(), serde_json::Value::Object(entry_obj));
    }

    let mut root = serde_json::Map::new();
    root.insert(VOFR_REGISTRY_HALLMARKS.to_string(), serde_json::Value::Object(hallmarks));

    let json = serde_json::to_string_pretty(&serde_json::Value::Object(root))
        .map_err(|e| format!("Failed to serialize registry: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write registry: {}", e))?;

    Ok(())
}

/// Find hallmark by SHA.
fn zvofr_find_by_sha(registry: &BTreeMap<u32, vofr_RegistryEntry>, sha: &str) -> Option<u32> {
    for (hallmark, entry) in registry {
        if entry.sha == sha {
            return Some(*hallmark);
        }
    }
    None
}

/// Allocate next sequential hallmark.
fn zvofr_allocate_hallmark(registry: &BTreeMap<u32, vofr_RegistryEntry>) -> u32 {
    registry.keys().max().map(|m| m + 1).unwrap_or(VOFR_HALLMARK_START)
}

/// Generate date string in YYMMDD-HHMM format.
fn zvofr_generate_date() -> String {
    use std::process::Command;

    let output = Command::new("date")
        .args(["+%y%m%d-%H%M"])
        .output()
        .expect("Failed to run date command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Write brand file to staging.
fn zvofr_write_brand_file(
    path: &Path,
    hallmark: u32,
    date: &str,
    sha: &str,
    commit: &str,
    kits: &[String],
) -> Result<(), String> {
    let mut root = serde_json::Map::new();

    root.insert(VOFR_BRAND_HALLMARK.to_string(), serde_json::Value::Number(hallmark.into()));
    root.insert(VOFR_BRAND_DATE.to_string(), serde_json::Value::String(date.to_string()));
    root.insert(VOFR_BRAND_SHA.to_string(), serde_json::Value::String(sha.to_string()));
    root.insert(VOFR_BRAND_COMMIT.to_string(), serde_json::Value::String(commit.to_string()));

    let kit_values: Vec<serde_json::Value> = kits
        .iter()
        .map(|k| serde_json::Value::String(k.clone()))
        .collect();
    root.insert(VOFR_BRAND_KITS.to_string(), serde_json::Value::Array(kit_values));

    let json = serde_json::to_string_pretty(&serde_json::Value::Object(root))
        .map_err(|e| format!("Failed to serialize brand file: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write brand file: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vofr_is_command_file_test() {
        assert!(zvofr_is_command_file("jjc-heat-mount.md", "jj"));
        assert!(zvofr_is_command_file("vvc-commit.md", "vv"));
        assert!(!zvofr_is_command_file("jju_utility.sh", "jj"));
        assert!(!zvofr_is_command_file("README.md", "jj"));
        assert!(!zvofr_is_command_file("jjc-heat-mount.txt", "jj"));
    }

    #[test]
    fn vofr_is_veiled_path_test() {
        assert!(zvofr_is_veiled_path(Path::new("vov_veiled/test.rs")));
        assert!(zvofr_is_veiled_path(Path::new("foo/vov_veiled/bar.rs")));
        assert!(!zvofr_is_veiled_path(Path::new("commands/jjc-test.md")));
        assert!(!zvofr_is_veiled_path(Path::new("buc_command.sh")));
    }

    #[test]
    fn vofr_hallmark_allocation_test() {
        let empty: BTreeMap<u32, vofr_RegistryEntry> = BTreeMap::new();
        assert_eq!(zvofr_allocate_hallmark(&empty), VOFR_HALLMARK_START);

        let mut registry = BTreeMap::new();
        registry.insert(1000, vofr_RegistryEntry {
            date: "260117-1400".to_string(),
            sha: "abc123".to_string(),
        });
        assert_eq!(zvofr_allocate_hallmark(&registry), 1001);

        registry.insert(1005, vofr_RegistryEntry {
            date: "260117-1500".to_string(),
            sha: "def456".to_string(),
        });
        assert_eq!(zvofr_allocate_hallmark(&registry), 1006);
    }
}
