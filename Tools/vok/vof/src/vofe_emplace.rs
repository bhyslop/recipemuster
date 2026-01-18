// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVK Emplace/Vacate - Install and uninstall kit assets
//!
//! Full install/uninstall operations including git commits.
//! Bash scripts (vvi_install.sh, vvu_uninstall.sh) are thin bootstraps
//! that detect platform and exec the binary.
//!
//! Emplace behavior (nuclear install):
//! 1. Parse burc.env, resolve paths relative to burc.env location
//! 2. Verify git repo is clean
//! 3. Delete .vvk/ and kit directories (nuclear cleanup)
//! 4. Create .vvk/, copy brand file
//! 5. Copy kit directories, route commands/hooks
//! 6. Freshen CLAUDE.md with kit sections
//! 7. Commit installation
//!
//! Vacate behavior:
//! 1. Parse burc.env, verify git clean
//! 2. Read brand file from .vvk/ to get kit list
//! 3. Remove commands and hooks from .claude/
//! 4. Collapse CLAUDE.md sections to UNINSTALLED markers
//! 5. Delete kit directories
//! 6. Delete brand file and .vvk/
//! 7. Commit uninstallation

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::voff_freshen::{voff_freshen, voff_collapse, voff_ManagedSection};
use crate::vofc_registry::vofc_find_kit_by_id;

// =============================================================================
// Constants
// =============================================================================

/// Brand file JSON field tags (must match vofr_release.rs)
const VOFE_BRAND_HALLMARK: &str = "vvbh_hallmark";
const VOFE_BRAND_KITS: &str = "vvbk_kits";

// =============================================================================
// Git Operations
// =============================================================================

/// Check that git working tree is clean (no uncommitted changes).
fn zvofe_check_git_clean(project_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("Failed to run git status: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git status failed: {}", stderr.trim()));
    }

    if !output.stdout.is_empty() {
        return Err(
            "Target repo has uncommitted changes. Commit or stash before install.".to_string(),
        );
    }

    Ok(())
}

/// Check that path is a git repository.
fn zvofe_check_is_git_repo(project_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("Failed to run git rev-parse: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Target is not a git repository: {}",
            project_root.display()
        ));
    }

    Ok(())
}

/// Run git add -A and commit with message.
fn zvofe_git_commit(project_root: &Path, message: &str) -> Result<(), String> {
    // Stage all changes
    let output = Command::new("git")
        .args(["add", "-A"])
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("Failed to run git add: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git add failed: {}", stderr.trim()));
    }

    // Commit
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("Failed to run git commit: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // "nothing to commit" is not an error for us
        if stderr.contains("nothing to commit") {
            return Ok(());
        }
        return Err(format!("git commit failed: {}", stderr.trim()));
    }

    Ok(())
}

// =============================================================================
// Public Types
// =============================================================================

/// Arguments for emplace operation.
#[derive(Debug)]
pub struct vofe_EmplaceArgs {
    /// Path to extracted parcel directory
    pub parcel_dir: PathBuf,
    /// Path to target repo's burc.env file
    pub burc_path: PathBuf,
}

/// Result of emplace operation.
#[derive(Debug)]
pub struct vofe_EmplaceResult {
    /// Hallmark from brand file
    pub hallmark: u32,
    /// Kits that were installed
    pub kits_installed: Vec<String>,
    /// Total files copied
    pub files_copied: u32,
    /// Commands routed to .claude/commands/
    pub commands_routed: u32,
    /// Hooks routed to .claude/hooks/
    pub hooks_routed: u32,
    /// CLAUDE.md sections updated
    pub claude_sections_updated: Vec<String>,
}

/// Parsed BURC environment.
#[derive(Debug)]
pub struct vofe_BurcEnv {
    /// BURC_TOOLS_DIR - where kit directories go
    pub tools_dir: PathBuf,
    /// BURC_PROJECT_ROOT - project root for .claude/, CLAUDE.md, .vvk/
    pub project_root: PathBuf,
}

/// Arguments for vacate operation.
#[derive(Debug)]
pub struct vofe_VacateArgs {
    /// Path to target repo's burc.env file
    pub burc_path: PathBuf,
}

/// Result of vacate operation.
#[derive(Debug)]
pub struct vofe_VacateResult {
    /// Kits that were removed
    pub kits_removed: Vec<String>,
    /// Total files deleted
    pub files_deleted: u32,
    /// Commands removed from .claude/commands/
    pub commands_removed: u32,
    /// Hooks removed from .claude/hooks/
    pub hooks_removed: u32,
    /// CLAUDE.md sections collapsed
    pub claude_sections_collapsed: Vec<String>,
}

// =============================================================================
// Public Functions
// =============================================================================

/// Install kit assets from parcel to target repo.
/// Nuclear install: deletes existing installation before copying fresh.
///
/// # Arguments
/// * `args` - Emplace arguments (parcel dir and burc.env path)
///
/// # Returns
/// EmplaceResult with installation summary, or error message
pub fn vofe_emplace(args: &vofe_EmplaceArgs) -> Result<vofe_EmplaceResult, String> {
    eprintln!("emplace: installing kit assets...");
    eprintln!("  parcel: {}", args.parcel_dir.display());
    eprintln!("  burc: {}", args.burc_path.display());

    // 1. Parse BURC (resolves paths relative to burc.env location)
    let burc = zvofe_parse_burc(&args.burc_path)?;

    // 2. Validate git state
    zvofe_check_is_git_repo(&burc.project_root)?;
    zvofe_check_git_clean(&burc.project_root)?;

    // 3. Read brand identity
    let brand_path = args.parcel_dir.join("vvbf_brand.json");
    let (hallmark, kit_ids) = zvofe_read_brand(&brand_path)?;

    // 4. Nuclear cleanup - delete existing .vvk/ and kit directories
    let vvk_dir = burc.project_root.join(".vvk");
    if vvk_dir.exists() {
        fs::remove_dir_all(&vvk_dir)
            .map_err(|e| format!("Failed to remove .vvk directory: {}", e))?;
    }

    for kit_id in &kit_ids {
        let kit_dir = burc.tools_dir.join(kit_id);
        if kit_dir.exists() {
            fs::remove_dir_all(&kit_dir)
                .map_err(|e| format!("Failed to remove kit directory {}: {}", kit_id, e))?;
        }
    }

    // 5. Create .vvk/ and copy brand file
    fs::create_dir_all(&vvk_dir)
        .map_err(|e| format!("Failed to create .vvk directory: {}", e))?;

    let brand_dest = vvk_dir.join("vvbf_brand.json");
    fs::copy(&brand_path, &brand_dest)
        .map_err(|e| format!("Failed to copy brand file: {}", e))?;

    // 6. Copy kit assets and route special files
    let mut total_files = 0u32;
    let mut commands_routed = 0u32;
    let mut hooks_routed = 0u32;

    let kits_dir = args.parcel_dir.join("kits");
    for kit_id in &kit_ids {
        let kit_source = kits_dir.join(kit_id);
        let kit_dest = burc.tools_dir.join(kit_id);

        if !kit_source.exists() {
            return Err(format!("Kit not found in parcel: {}", kit_id));
        }

        let (files, cmds, hks) = zvofe_copy_kit(
            &kit_source,
            &kit_dest,
            &burc.project_root,
            kit_id,
        )?;

        total_files += files;
        commands_routed += cmds;
        hooks_routed += hks;
    }

    // 7. Freshen CLAUDE.md with kit sections
    let sections = zvofe_read_templates(&args.parcel_dir, &kit_ids)?;
    let section_tags: Vec<String> = sections.iter().map(|s| s.tag.clone()).collect();

    let claude_path = burc.project_root.join("CLAUDE.md");
    zvofe_freshen_claude(&claude_path, &sections)?;

    // 8. Commit installation
    let commit_msg = format!("VVK install: hallmark {}", hallmark);
    zvofe_git_commit(&burc.project_root, &commit_msg)?;

    eprintln!("emplace: success - {} files, {} commands, {} hooks", total_files, commands_routed, hooks_routed);

    Ok(vofe_EmplaceResult {
        hallmark,
        kits_installed: kit_ids,
        files_copied: total_files,
        commands_routed,
        hooks_routed,
        claude_sections_updated: section_tags,
    })
}

/// Remove kit assets from target repo.
///
/// # Arguments
/// * `args` - Vacate arguments (burc.env path)
///
/// # Returns
/// VacateResult with removal summary, or error message
pub fn vofe_vacate(args: &vofe_VacateArgs) -> Result<vofe_VacateResult, String> {
    eprintln!("vacate: removing kit assets...");
    eprintln!("  burc: {}", args.burc_path.display());

    // 1. Parse BURC
    let burc = zvofe_parse_burc(&args.burc_path)?;

    // 2. Validate git state
    zvofe_check_git_clean(&burc.project_root)?;

    // 3. Read brand file from .vvk/
    let brand_path = burc.project_root.join(".vvk").join("vvbf_brand.json");
    if !brand_path.exists() {
        return Err("No VVK installation found (.vvk/vvbf_brand.json missing)".to_string());
    }
    let (_hallmark, kit_ids) = zvofe_read_brand(&brand_path)?;

    // 4. Remove commands and hooks
    let mut total_files = 0u32;
    let mut commands_removed = 0u32;
    let mut hooks_removed = 0u32;

    let commands_dir = burc.project_root.join(".claude").join("commands");
    let hooks_dir = burc.project_root.join(".claude").join("hooks");

    for kit_id in &kit_ids {
        // Get cipher from kit_id (e.g., "jjk" -> "jj")
        let cipher = &kit_id[..kit_id.len().saturating_sub(1)];

        // Remove commands matching {cipher}c-*.md
        if commands_dir.exists() {
            commands_removed += zvofe_remove_matching_files(&commands_dir, cipher, true)?;
        }

        // Remove hooks matching {cipher}h_*
        if hooks_dir.exists() {
            hooks_removed += zvofe_remove_matching_files(&hooks_dir, cipher, false)?;
        }
    }

    // 5. Collapse CLAUDE.md sections (get tags from registry)
    let section_tags: Vec<String> = kit_ids
        .iter()
        .filter_map(|kit_id| vofc_find_kit_by_id(kit_id))
        .flat_map(|kit| kit.managed_sections.iter().map(|s| s.tag.to_string()))
        .collect();

    let claude_path = burc.project_root.join("CLAUDE.md");
    zvofe_collapse_claude(&claude_path, &section_tags)?;

    // 6. Delete kit directories
    for kit_id in &kit_ids {
        let kit_dir = burc.tools_dir.join(kit_id);
        if kit_dir.exists() {
            // Count files before deletion
            total_files += zvofe_count_files(&kit_dir)?;
            fs::remove_dir_all(&kit_dir)
                .map_err(|e| format!("Failed to remove kit directory {}: {}", kit_dir.display(), e))?;
        } else {
            eprintln!("  warning: kit directory not found: {}", kit_dir.display());
        }
    }

    // 7. Delete brand file and .vvk/
    fs::remove_file(&brand_path)
        .map_err(|e| format!("Failed to remove brand file: {}", e))?;

    let vvk_dir = burc.project_root.join(".vvk");
    if vvk_dir.exists() {
        // Remove .vvk/ directory
        let _ = fs::remove_dir(&vvk_dir);
    }

    // 8. Commit uninstallation
    zvofe_git_commit(&burc.project_root, "VVK uninstall")?;

    eprintln!("vacate: success - {} files, {} commands, {} hooks removed", total_files, commands_removed, hooks_removed);

    Ok(vofe_VacateResult {
        kits_removed: kit_ids,
        files_deleted: total_files,
        commands_removed,
        hooks_removed,
        claude_sections_collapsed: section_tags,
    })
}

// =============================================================================
// Internal Functions (zvofe_*)
// =============================================================================

/// Parse burc.env file to extract environment variables.
/// BURC_PROJECT_ROOT is the path from burc.env's location to the project root.
/// BURC_TOOLS_DIR is resolved relative to the resolved project_root (per BURC spec).
fn zvofe_parse_burc(path: &Path) -> Result<vofe_BurcEnv, String> {
    if !path.exists() {
        return Err(format!("burc.env not found: {}", path.display()));
    }

    // Get the parent directory of burc.env for path resolution
    let burc_parent = path
        .parent()
        .ok_or_else(|| "burc.env path has no parent directory".to_string())?;

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read burc.env: {}", e))?;

    let mut vars: HashMap<String, String> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE (handle quoted values)
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let mut value = line[eq_pos + 1..].trim();

            // Remove surrounding quotes if present
            if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                value = &value[1..value.len() - 1];
            }

            vars.insert(key.to_string(), value.to_string());
        }
    }

    let tools_dir_str = vars
        .get("BURC_TOOLS_DIR")
        .ok_or_else(|| "burc.env missing required variable: BURC_TOOLS_DIR".to_string())?;

    let project_root_str = vars
        .get("BURC_PROJECT_ROOT")
        .ok_or_else(|| "burc.env missing required variable: BURC_PROJECT_ROOT".to_string())?;

    // BURC_PROJECT_ROOT is the path from burc.env's location to project root
    let project_root = burc_parent.join(project_root_str);
    let project_root = project_root
        .canonicalize()
        .map_err(|e| format!("BURC_PROJECT_ROOT path invalid ({}): {}", project_root.display(), e))?;

    // Resolve tools_dir relative to project_root (per BURC spec)
    let tools_dir = project_root.join(tools_dir_str);
    let tools_dir = tools_dir
        .canonicalize()
        .map_err(|e| format!("BURC_TOOLS_DIR path invalid ({}): {}", tools_dir.display(), e))?;

    Ok(vofe_BurcEnv {
        tools_dir,
        project_root,
    })
}

/// Read brand file and extract hallmark and kit list.
fn zvofe_read_brand(path: &Path) -> Result<(u32, Vec<String>), String> {
    if !path.exists() {
        return Err(format!("Brand file not found: {}", path.display()));
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read brand file: {}", e))?;

    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse brand JSON: {}", e))?;

    let hallmark = json
        .get(VOFE_BRAND_HALLMARK)
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .ok_or_else(|| "Brand file missing hallmark".to_string())?;

    let kits = json
        .get(VOFE_BRAND_KITS)
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Brand file missing kits array".to_string())?;

    let kit_ids: Vec<String> = kits
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    if kit_ids.is_empty() {
        return Err("Brand file has empty kits array".to_string());
    }

    Ok((hallmark, kit_ids))
}

/// Copy a kit directory and route special files.
/// Returns (files_copied, commands_routed, hooks_routed).
fn zvofe_copy_kit(
    source: &Path,
    dest: &Path,
    project_root: &Path,
    kit_id: &str,
) -> Result<(u32, u32, u32), String> {
    let mut files = 0u32;
    let mut commands = 0u32;
    let mut hooks = 0u32;

    // Get cipher from kit_id (e.g., "jjk" -> "jj")
    let cipher = &kit_id[..kit_id.len().saturating_sub(1)];

    // Walk source directory
    let entries = zvofe_walk_dir(source)?;

    for entry in entries {
        let rel_path = entry
            .strip_prefix(source)
            .map_err(|e| format!("Path strip failed: {}", e))?;

        let file_name = rel_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Determine destination based on file type
        let dest_path = if zvofe_is_command_file(file_name, cipher) {
            // Route commands to .claude/commands/
            let commands_dir = project_root.join(".claude").join("commands");
            fs::create_dir_all(&commands_dir)
                .map_err(|e| format!("Failed to create commands dir: {}", e))?;
            commands += 1;
            commands_dir.join(file_name)
        } else if zvofe_is_hook_file(file_name, cipher) {
            // Route hooks to .claude/hooks/
            let hooks_dir = project_root.join(".claude").join("hooks");
            fs::create_dir_all(&hooks_dir)
                .map_err(|e| format!("Failed to create hooks dir: {}", e))?;
            hooks += 1;
            hooks_dir.join(file_name)
        } else {
            // Default: preserve relative path in kit directory
            dest.join(rel_path)
        };

        // Create parent directories
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create dir {}: {}", parent.display(), e))?;
        }

        // Copy file
        fs::copy(&entry, &dest_path)
            .map_err(|e| format!("Failed to copy {} to {}: {}", entry.display(), dest_path.display(), e))?;

        files += 1;
    }

    Ok((files, commands, hooks))
}

/// Walk directory recursively, returning all file paths.
fn zvofe_walk_dir(dir: &Path) -> Result<Vec<PathBuf>, String> {
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
            files.extend(zvofe_walk_dir(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

/// Check if file is a command file (matches {cipher}c-*.md pattern).
fn zvofe_is_command_file(file_name: &str, cipher: &str) -> bool {
    let prefix = format!("{}c-", cipher);
    file_name.starts_with(&prefix) && file_name.ends_with(".md")
}

/// Check if file is a hook file (matches {cipher}h_* pattern).
fn zvofe_is_hook_file(file_name: &str, cipher: &str) -> bool {
    let prefix = format!("{}h_", cipher);
    file_name.starts_with(&prefix)
}

/// Read managed section templates from parcel.
fn zvofe_read_templates(
    parcel_dir: &Path,
    kit_ids: &[String],
) -> Result<Vec<voff_ManagedSection>, String> {
    let mut sections = Vec::new();

    for kit_id in kit_ids {
        let kit = vofc_find_kit_by_id(kit_id)
            .ok_or_else(|| format!("Kit not in registry: {}", kit_id))?;

        let templates_dir = parcel_dir.join("kits").join(kit_id).join("templates");

        for section in kit.managed_sections {
            let template_path = templates_dir.join(section.template_path);
            if !template_path.exists() {
                return Err(format!(
                    "Template not found in parcel: {}",
                    template_path.display()
                ));
            }

            let content = fs::read_to_string(&template_path)
                .map_err(|e| format!("Failed to read template {}: {}", section.template_path, e))?;

            sections.push(voff_ManagedSection {
                tag: section.tag.to_string(),
                content,
            });
        }
    }

    Ok(sections)
}

/// Freshen CLAUDE.md with managed sections.
fn zvofe_freshen_claude(
    claude_path: &Path,
    sections: &[voff_ManagedSection],
) -> Result<(), String> {
    // Read existing content or start with empty
    let content = if claude_path.exists() {
        fs::read_to_string(claude_path)
            .map_err(|e| format!("Failed to read CLAUDE.md: {}", e))?
    } else {
        "# Claude Code Project Memory\n".to_string()
    };

    // Freshen with new sections
    let result = voff_freshen(&content, sections);

    // Write back
    fs::write(claude_path, &result.content)
        .map_err(|e| format!("Failed to write CLAUDE.md: {}", e))?;

    Ok(())
}

/// Collapse CLAUDE.md managed sections to UNINSTALLED markers.
fn zvofe_collapse_claude(claude_path: &Path, tags: &[String]) -> Result<(), String> {
    if !claude_path.exists() {
        return Ok(()); // Nothing to collapse
    }

    let content = fs::read_to_string(claude_path)
        .map_err(|e| format!("Failed to read CLAUDE.md: {}", e))?;

    let tag_refs: Vec<&str> = tags.iter().map(|s| s.as_str()).collect();
    let collapsed = voff_collapse(&content, &tag_refs);

    fs::write(claude_path, &collapsed)
        .map_err(|e| format!("Failed to write CLAUDE.md: {}", e))?;

    Ok(())
}

/// Remove files matching a pattern from a directory.
/// If is_command is true, matches {cipher}c-*.md; otherwise matches {cipher}h_*
/// Returns count of files removed.
fn zvofe_remove_matching_files(dir: &Path, cipher: &str, is_command: bool) -> Result<u32, String> {
    let mut count = 0u32;

    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

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
            zvofe_is_command_file(file_name, cipher)
        } else {
            zvofe_is_hook_file(file_name, cipher)
        };

        if matches {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed to remove {}: {}", path.display(), e))?;
            count += 1;
        }
    }

    Ok(count)
}

/// Count files in a directory recursively.
fn zvofe_count_files(dir: &Path) -> Result<u32, String> {
    let files = zvofe_walk_dir(dir)?;
    Ok(files.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_command_file() {
        assert!(zvofe_is_command_file("jjc-heat-mount.md", "jj"));
        assert!(zvofe_is_command_file("vvc-commit.md", "vv"));
        assert!(!zvofe_is_command_file("jjw_workbench.sh", "jj"));
        assert!(!zvofe_is_command_file("README.md", "jj"));
    }

    #[test]
    fn test_is_hook_file() {
        assert!(zvofe_is_hook_file("jjh_post_commit.sh", "jj"));
        assert!(zvofe_is_hook_file("vvh_pre_push", "vv"));
        assert!(!zvofe_is_hook_file("jjc-heat-mount.md", "jj"));
        assert!(!zvofe_is_hook_file("jjw_workbench.sh", "jj"));
    }

    #[test]
    fn test_parse_burc_simple() {
        use std::io::Write;
        let temp_dir = std::env::temp_dir().join("vofe_test_burc");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let burc_path = temp_dir.join("burc.env");
        let mut f = fs::File::create(&burc_path).unwrap();
        writeln!(f, "# Comment").unwrap();
        writeln!(f, "BURC_TOOLS_DIR=/path/to/tools").unwrap();
        writeln!(f, "BURC_PROJECT_ROOT=/path/to/project").unwrap();

        let result = zvofe_parse_burc(&burc_path).unwrap();
        assert_eq!(result.tools_dir, PathBuf::from("/path/to/tools"));
        assert_eq!(result.project_root, PathBuf::from("/path/to/project"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_parse_burc_quoted() {
        use std::io::Write;
        let temp_dir = std::env::temp_dir().join("vofe_test_burc_quoted");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let burc_path = temp_dir.join("burc.env");
        let mut f = fs::File::create(&burc_path).unwrap();
        writeln!(f, "BURC_TOOLS_DIR=\"/path/to/tools\"").unwrap();
        writeln!(f, "BURC_PROJECT_ROOT='/path/to/project'").unwrap();

        let result = zvofe_parse_burc(&burc_path).unwrap();
        assert_eq!(result.tools_dir, PathBuf::from("/path/to/tools"));
        assert_eq!(result.project_root, PathBuf::from("/path/to/project"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
