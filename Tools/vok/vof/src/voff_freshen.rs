// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! CLAUDE.md Managed Section Freshening
//!
//! Provides utilities for managing CLAUDE.md sections during kit install/uninstall.
//!
//! Marker format:
//! - `<!-- MANAGED:{TAG}:BEGIN -->` ... content ... `<!-- MANAGED:{TAG}:END -->`
//! - `<!-- MANAGED:{TAG}:UNINSTALLED -->` (placeholder preserving position)
//!
//! Rules:
//! - Content between BEGIN/END markers is replaced entirely on install
//! - User content outside markers is preserved
//! - UNINSTALLED markers expand back to BEGIN/END at same location
//! - Missing markers → append section at end of file

use std::collections::HashMap;

/// A managed section to be installed or updated.
#[derive(Debug, Clone)]
pub struct voff_ManagedSection {
    /// Tag identifier (e.g., "JJK", "CMK", "JJK_GALLOPS")
    pub tag: String,
    /// Content to place between markers (excluding the markers themselves)
    pub content: String,
}

/// Result of freshening a CLAUDE.md file.
#[derive(Debug)]
pub struct voff_FreshenResult {
    /// The freshened content
    pub content: String,
    /// Tags that were updated (had existing markers)
    pub updated: Vec<String>,
    /// Tags that were expanded from UNINSTALLED markers
    pub expanded: Vec<String>,
    /// Tags that were appended (no prior markers)
    pub appended: Vec<String>,
}

/// Freshen CLAUDE.md content by updating managed sections.
///
/// # Arguments
/// * `content` - Current CLAUDE.md content
/// * `sections` - Sections to install, in order (order determines append order)
///
/// # Returns
/// Result containing freshened content and change summary
pub fn voff_freshen(content: &str, sections: &[voff_ManagedSection]) -> voff_FreshenResult {
    let mut result = content.to_string();
    let mut updated = Vec::new();
    let mut expanded = Vec::new();
    let mut appended = Vec::new();

    for section in sections {
        let begin_marker = format!("<!-- MANAGED:{}:BEGIN -->", section.tag);
        let end_marker = format!("<!-- MANAGED:{}:END -->", section.tag);
        let uninstalled_marker = format!("<!-- MANAGED:{}:UNINSTALLED -->", section.tag);

        if let Some(replacement) = zvoff_replace_between_markers(&result, &begin_marker, &end_marker, &section.content) {
            // Found existing BEGIN/END markers - replace content
            result = replacement;
            updated.push(section.tag.clone());
        } else if let Some(expansion) = zvoff_expand_uninstalled(&result, &uninstalled_marker, &begin_marker, &end_marker, &section.content) {
            // Found UNINSTALLED marker - expand it
            result = expansion;
            expanded.push(section.tag.clone());
        } else {
            // No markers found - append at end
            result = zvoff_append_section(&result, &begin_marker, &end_marker, &section.content);
            appended.push(section.tag.clone());
        }
    }

    voff_FreshenResult {
        content: result,
        updated,
        expanded,
        appended,
    }
}

/// Collapse managed sections to UNINSTALLED markers for uninstall.
///
/// # Arguments
/// * `content` - Current CLAUDE.md content
/// * `tags` - Tags to collapse
///
/// # Returns
/// Content with sections collapsed to UNINSTALLED markers
pub fn voff_collapse(content: &str, tags: &[&str]) -> String {
    let mut result = content.to_string();

    for tag in tags {
        let begin_marker = format!("<!-- MANAGED:{}:BEGIN -->", tag);
        let end_marker = format!("<!-- MANAGED:{}:END -->", tag);
        let uninstalled_marker = format!("<!-- MANAGED:{}:UNINSTALLED -->", tag);

        if let Some(collapsed) = zvoff_collapse_to_uninstalled(&result, &begin_marker, &end_marker, &uninstalled_marker) {
            result = collapsed;
        }
    }

    result
}

/// Parse existing managed sections from CLAUDE.md content.
///
/// # Returns
/// Map of tag -> (start_line, end_line, is_uninstalled)
pub fn voff_parse_sections(content: &str) -> HashMap<String, (usize, usize, bool)> {
    let mut sections = HashMap::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // Check for UNINSTALLED marker
        if let Some(tag) = zvoff_parse_uninstalled_marker(line) {
            sections.insert(tag, (i, i, true));
            i += 1;
            continue;
        }

        // Check for BEGIN marker
        if let Some(tag) = zvoff_parse_begin_marker(line) {
            // Find matching END marker
            let mut j = i + 1;
            while j < lines.len() {
                if zvoff_parse_end_marker(lines[j]) == Some(tag.clone()) {
                    sections.insert(tag, (i, j, false));
                    break;
                }
                j += 1;
            }
            i = j + 1;
            continue;
        }

        i += 1;
    }

    sections
}

// =============================================================================
// Internal Functions (zvoff_*)
// =============================================================================

/// Replace content between BEGIN and END markers.
fn zvoff_replace_between_markers(content: &str, begin: &str, end: &str, new_content: &str) -> Option<String> {
    let begin_pos = content.find(begin)?;
    let end_pos = content.find(end)?;

    if end_pos <= begin_pos {
        return None;
    }

    let before = &content[..begin_pos + begin.len()];
    let after = &content[end_pos..];

    // Ensure proper newlines around content
    let formatted_content = if new_content.is_empty() {
        "\n".to_string()
    } else if new_content.starts_with('\n') {
        format!("{}\n", new_content)
    } else {
        format!("\n{}\n", new_content)
    };

    Some(format!("{}{}{}", before, formatted_content, after))
}

/// Expand an UNINSTALLED marker to full BEGIN/END markers with content.
fn zvoff_expand_uninstalled(content: &str, uninstalled: &str, begin: &str, end: &str, new_content: &str) -> Option<String> {
    let pos = content.find(uninstalled)?;

    let before = &content[..pos];
    let after = &content[pos + uninstalled.len()..];

    // Format the expanded section
    let formatted_content = if new_content.is_empty() {
        format!("{}\n{}", begin, end)
    } else if new_content.starts_with('\n') {
        format!("{}{}\n{}", begin, new_content, end)
    } else {
        format!("{}\n{}\n{}", begin, new_content, end)
    };

    Some(format!("{}{}{}", before, formatted_content, after))
}

/// Append a new section at the end of content.
fn zvoff_append_section(content: &str, begin: &str, end: &str, new_content: &str) -> String {
    let separator = if content.ends_with('\n') { "\n" } else { "\n\n" };

    let formatted_content = if new_content.is_empty() {
        format!("{}\n{}", begin, end)
    } else if new_content.starts_with('\n') {
        format!("{}{}\n{}", begin, new_content, end)
    } else {
        format!("{}\n{}\n{}", begin, new_content, end)
    };

    format!("{}{}{}\n", content, separator, formatted_content)
}

/// Collapse a section to an UNINSTALLED marker.
fn zvoff_collapse_to_uninstalled(content: &str, begin: &str, end: &str, uninstalled: &str) -> Option<String> {
    let begin_pos = content.find(begin)?;
    let end_pos = content.find(end)?;

    if end_pos <= begin_pos {
        return None;
    }

    let before = &content[..begin_pos];
    let after = &content[end_pos + end.len()..];

    Some(format!("{}{}{}", before, uninstalled, after))
}

/// Parse a BEGIN marker and extract the tag.
fn zvoff_parse_begin_marker(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("<!-- MANAGED:") && trimmed.ends_with(":BEGIN -->") {
        let inner = &trimmed[13..trimmed.len() - 10];
        Some(inner.to_string())
    } else {
        None
    }
}

/// Parse an END marker and extract the tag.
fn zvoff_parse_end_marker(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("<!-- MANAGED:") && trimmed.ends_with(":END -->") {
        let inner = &trimmed[13..trimmed.len() - 8];
        Some(inner.to_string())
    } else {
        None
    }
}

/// Parse an UNINSTALLED marker and extract the tag.
fn zvoff_parse_uninstalled_marker(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("<!-- MANAGED:") && trimmed.ends_with(":UNINSTALLED -->") {
        let inner = &trimmed[13..trimmed.len() - 16];
        Some(inner.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freshen_update_existing() {
        let content = r#"# My CLAUDE.md

Some user content here.

<!-- MANAGED:JJK:BEGIN -->
old content
<!-- MANAGED:JJK:END -->

More user content.
"#;

        let sections = vec![
            voff_ManagedSection {
                tag: "JJK".to_string(),
                content: "new JJK content".to_string(),
            },
        ];

        let result = voff_freshen(content, &sections);

        assert!(result.content.contains("new JJK content"));
        assert!(!result.content.contains("old content"));
        assert!(result.content.contains("Some user content here."));
        assert!(result.content.contains("More user content."));
        assert_eq!(result.updated, vec!["JJK"]);
        assert!(result.expanded.is_empty());
        assert!(result.appended.is_empty());
    }

    #[test]
    fn test_freshen_expand_uninstalled() {
        let content = r#"# My CLAUDE.md

<!-- MANAGED:JJK:UNINSTALLED -->

User content.
"#;

        let sections = vec![
            voff_ManagedSection {
                tag: "JJK".to_string(),
                content: "reinstalled content".to_string(),
            },
        ];

        let result = voff_freshen(content, &sections);

        assert!(result.content.contains("<!-- MANAGED:JJK:BEGIN -->"));
        assert!(result.content.contains("reinstalled content"));
        assert!(result.content.contains("<!-- MANAGED:JJK:END -->"));
        assert!(!result.content.contains("UNINSTALLED"));
        assert_eq!(result.expanded, vec!["JJK"]);
    }

    #[test]
    fn test_freshen_append_new() {
        let content = "# My CLAUDE.md\n\nExisting content.\n";

        let sections = vec![
            voff_ManagedSection {
                tag: "NEW_KIT".to_string(),
                content: "brand new section".to_string(),
            },
        ];

        let result = voff_freshen(content, &sections);

        assert!(result.content.contains("<!-- MANAGED:NEW_KIT:BEGIN -->"));
        assert!(result.content.contains("brand new section"));
        assert!(result.content.contains("<!-- MANAGED:NEW_KIT:END -->"));
        assert!(result.content.contains("Existing content."));
        assert_eq!(result.appended, vec!["NEW_KIT"]);
    }

    #[test]
    fn test_collapse() {
        let content = r#"# My CLAUDE.md

<!-- MANAGED:JJK:BEGIN -->
some content
<!-- MANAGED:JJK:END -->

User content.
"#;

        let result = voff_collapse(content, &["JJK"]);

        assert!(result.contains("<!-- MANAGED:JJK:UNINSTALLED -->"));
        assert!(!result.contains("<!-- MANAGED:JJK:BEGIN -->"));
        assert!(!result.contains("some content"));
        assert!(result.contains("User content."));
    }

    #[test]
    fn test_parse_sections() {
        let content = r#"# Header

<!-- MANAGED:JJK:BEGIN -->
content
<!-- MANAGED:JJK:END -->

<!-- MANAGED:CMK:UNINSTALLED -->

<!-- MANAGED:BUK:BEGIN -->
more content
<!-- MANAGED:BUK:END -->
"#;

        let sections = voff_parse_sections(content);

        assert_eq!(sections.len(), 3);
        assert_eq!(sections.get("JJK"), Some(&(2, 4, false)));
        assert_eq!(sections.get("CMK"), Some(&(6, 6, true)));
        assert_eq!(sections.get("BUK"), Some(&(8, 10, false)));
    }

    #[test]
    fn test_multiple_sections_preserve_order() {
        let content = r#"# CLAUDE.md

<!-- MANAGED:CMK:BEGIN -->
CMK content
<!-- MANAGED:CMK:END -->

User notes here.

<!-- MANAGED:JJK:BEGIN -->
JJK content
<!-- MANAGED:JJK:END -->
"#;

        let sections = vec![
            voff_ManagedSection {
                tag: "CMK".to_string(),
                content: "updated CMK".to_string(),
            },
            voff_ManagedSection {
                tag: "JJK".to_string(),
                content: "updated JJK".to_string(),
            },
        ];

        let result = voff_freshen(content, &sections);

        // Verify order preserved - CMK before JJK
        let cmk_pos = result.content.find("updated CMK").unwrap();
        let jjk_pos = result.content.find("updated JJK").unwrap();
        assert!(cmk_pos < jjk_pos);

        // Verify user content preserved
        assert!(result.content.contains("User notes here."));
    }
}
