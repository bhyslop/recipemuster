// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Hardcoded CLAUDE.md Managed Section Templates
//!
//! MVP implementation: templates are compiled into the binary.
//! Post-MVP (₢AAABK) will externalize these to parcel files.
//!
//! Each kit has a managed section that gets installed into target repo's CLAUDE.md.
//! Content should be minimal — just essential configuration, not full documentation.

use crate::voff_freshen::voff_ManagedSection;

/// BUK managed section — Bash Utilities Kit configuration
pub fn vofm_buk_section() -> voff_ManagedSection {
    voff_ManagedSection {
        tag: "BUK".to_string(),
        content: r#"
## Bash Utility Kit (BUK)

BUK provides tabtarget/launcher infrastructure for bash-based tooling.

**Key files:**
- `Tools/buk/buc_command.sh` — command utilities
- `Tools/buk/bud_dispatch.sh` — dispatch utilities
- `Tools/buk/buw_workbench.sh` — workbench formulary

**Tabtarget pattern:** `{colophon}.{frontispiece}[.{imprint}].sh`

For full documentation, see `Tools/buk/README.md`.
"#.to_string(),
    }
}

/// CMK managed section — Concept Model Kit configuration
pub fn vofm_cmk_section() -> voff_ManagedSection {
    voff_ManagedSection {
        tag: "CMK".to_string(),
        content: r#"
## Concept Model Kit (CMK)

CMK provides tooling for MCM-format concept model documents.

**Available commands:**
- `/cma-normalize` — Apply MCM normalization
- `/cma-validate` — Check links and annotations
- `/cma-render` — Transform to ClaudeMark format

**Key files:**
- `Tools/cmk/README.md` — Kit documentation

For MCM specification, see `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc`.
"#.to_string(),
    }
}

/// JJK managed section — Job Jockey Kit configuration
pub fn vofm_jjk_section() -> voff_ManagedSection {
    voff_ManagedSection {
        tag: "JJK".to_string(),
        content: r#"
## Job Jockey (JJK)

JJK provides project initiative tracking with heats and paces.

**Key commands:**
- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-wrap` — Mark pace complete
- `/jjc-pace-notch` — JJ-aware git commit

**Quick verbs:** mount, slate, wrap, bridle, notch, muster, groom, quarter

**Key files:**
- `Tools/jjk/jjw_workbench.sh` — Workbench
- `.claude/jjm/` — Heat and pace data

For full documentation, see `Tools/jjk/README.md`.
"#.to_string(),
    }
}

/// VVK managed section — Voce Viva Kit configuration
pub fn vofm_vvk_section() -> voff_ManagedSection {
    voff_ManagedSection {
        tag: "VVK".to_string(),
        content: r#"
## Voce Viva Kit (VVK)

VVK provides core infrastructure for Claude Code kits.

**Key commands:**
- `/vvc-commit` — Guarded git commit with size validation

**Key files:**
- `Tools/vvk/bin/vvx` — Core binary
- `.vvk/vvbf_brand.json` — Installation brand file

For installation/uninstallation, use `vvi_install.sh` and `vvu_uninstall.sh`.
"#.to_string(),
    }
}

/// Get all managed sections in installation order.
/// Order matters: sections are appended to CLAUDE.md in this order if not already present.
pub fn vofm_all_sections() -> Vec<voff_ManagedSection> {
    vec![
        vofm_buk_section(),
        vofm_cmk_section(),
        vofm_jjk_section(),
        vofm_vvk_section(),
    ]
}

/// Get managed section for a specific kit by kit ID.
pub fn vofm_section_for_kit(kit_id: &str) -> Option<voff_ManagedSection> {
    match kit_id {
        "buk" => Some(vofm_buk_section()),
        "cmk" => Some(vofm_cmk_section()),
        "jjk" => Some(vofm_jjk_section()),
        "vvk" => Some(vofm_vvk_section()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_sections_count() {
        let sections = vofm_all_sections();
        assert_eq!(sections.len(), 4);
    }

    #[test]
    fn test_section_tags() {
        let sections = vofm_all_sections();
        let tags: Vec<&str> = sections.iter().map(|s| s.tag.as_str()).collect();
        assert_eq!(tags, vec!["BUK", "CMK", "JJK", "VVK"]);
    }

    #[test]
    fn test_section_for_kit() {
        assert!(vofm_section_for_kit("buk").is_some());
        assert!(vofm_section_for_kit("jjk").is_some());
        assert!(vofm_section_for_kit("nonexistent").is_none());
    }

    #[test]
    fn test_section_content_not_empty() {
        for section in vofm_all_sections() {
            assert!(!section.content.is_empty(), "Section {} has empty content", section.tag);
        }
    }
}
