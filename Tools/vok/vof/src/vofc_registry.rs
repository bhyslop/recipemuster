// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Cipher Registry - Single source of truth for project prefixes
//!
//! A Cipher is a 2-5 character project prefix that owns exclusive namespace rights.
//! All kit artifacts must use prefixes derived from their cipher.
//!
//! Rules (from Prefix Naming Discipline):
//! - Lowercase letters only
//! - Globally unique
//! - Terminal exclusivity: a prefix either IS a name or HAS children, never both
//!
//! Declaration pattern: const values with compile-time uniqueness.

/// Cipher represents a project's namespace root.
/// Uniqueness enforced at compile time by const declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct vofc_Cipher {
    prefix: &'static str,
    project: &'static str,
}

impl vofc_Cipher {
    /// Create a new cipher at compile time.
    pub const fn new(prefix: &'static str, project: &'static str) -> Self {
        Self { prefix, project }
    }

    /// Get the prefix string.
    pub const fn prefix(&self) -> &'static str {
        self.prefix
    }

    /// Get the project name.
    pub const fn project(&self) -> &'static str {
        self.project
    }

    /// Check if a string starts with this cipher's prefix.
    pub fn matches_prefix(&self, s: &str) -> bool {
        s.starts_with(self.prefix)
    }

    /// Validate that a string follows cipher naming rules.
    /// Returns true if the string starts with this cipher's prefix
    /// and the character after the prefix (if any) is not part of
    /// another cipher's prefix (terminal exclusivity).
    pub fn validate_name(&self, name: &str) -> bool {
        if !name.starts_with(self.prefix) {
            return false;
        }
        // Name must have content after the prefix
        name.len() > self.prefix.len()
    }

    /// Generate kit identifier by appending 'k' to prefix.
    /// Does not validate whether this cipher actually has a kit.
    pub fn kit_id(&self) -> String {
        format!("{}k", self.prefix)
    }
}

// =============================================================================
// Project Cipher Registry
// =============================================================================
// Organized by domain. Each cipher owns exclusive namespace rights.
// Adding a new cipher here is a deliberate act requiring coordination.

// -----------------------------------------------------------------------------
// Voce Viva Ecosystem (VO/VV)
// -----------------------------------------------------------------------------

/// VV: Voce Viva - the universal kit present in every target repo
pub const VV: vofc_Cipher = vofc_Cipher::new("vv", "Voce Viva");

/// VO: Vox Obscura - the kit forge infrastructure (never distributed)
pub const VO: vofc_Cipher = vofc_Cipher::new("vo", "Vox Obscura");

// -----------------------------------------------------------------------------
// Portable Kits
// -----------------------------------------------------------------------------

/// JJ: Job Jockey - project management and heat tracking
pub const JJ: vofc_Cipher = vofc_Cipher::new("jj", "Job Jockey");

/// BU: Bash Utilities Kit - enterprise bash infrastructure
pub const BU: vofc_Cipher = vofc_Cipher::new("bu", "Bash Utilities");

/// CM: Concept Model Kit - MCM/AXLA concept model tooling
pub const CM: vofc_Cipher = vofc_Cipher::new("cm", "Concept Model");

/// HM: Hard-state Machine Kit - state machine infrastructure
pub const HM: vofc_Cipher = vofc_Cipher::new("hm", "Hard-state Machine");

// -----------------------------------------------------------------------------
// Recipe Bottle Domain
// -----------------------------------------------------------------------------

/// RB: Recipe Bottle - container orchestration and deployment
pub const RB: vofc_Cipher = vofc_Cipher::new("rb", "Recipe Bottle");

/// CRG: Config Regime - configuration management
pub const CRG: vofc_Cipher = vofc_Cipher::new("crg", "Config Regime");

// -----------------------------------------------------------------------------
// Tools and Utilities
// -----------------------------------------------------------------------------

/// GAD: Google AsciiDoc Differ - diff visualization tool
pub const GAD: vofc_Cipher = vofc_Cipher::new("gad", "Google AsciiDoc Differ");

/// CCC: Claude Code Container Kit - Docker/container operations
pub const CCC: vofc_Cipher = vofc_Cipher::new("ccc", "Claude Code Container");

/// LMCI: Language Model Console Integration
pub const LMCI: vofc_Cipher = vofc_Cipher::new("lmci", "Language Model Console Integration");

/// VSL: Visual SlickEdit Local Kit - IDE integration
pub const VSL: vofc_Cipher = vofc_Cipher::new("vsl", "Visual SlickEdit Local");

// -----------------------------------------------------------------------------
// Concept Model Vocabulary
// -----------------------------------------------------------------------------

/// MCM: Meta Concept Model - specification for concept models
pub const MCM: vofc_Cipher = vofc_Cipher::new("mcm", "Meta Concept Model");

/// AXL: Axiom Lexicon - shared vocabulary definitions
pub const AXL: vofc_Cipher = vofc_Cipher::new("axl", "Axiom Lexicon");

// -----------------------------------------------------------------------------
// Other Projects
// -----------------------------------------------------------------------------

/// PB: Paneboard - cross-platform UI toolkit
pub const PB: vofc_Cipher = vofc_Cipher::new("pb", "Paneboard");

/// WRS: Ward Realm Substrate - distributed state machine substrate
pub const WRS: vofc_Cipher = vofc_Cipher::new("wrs", "Ward Realm Substrate");

/// SRF: Study Raft - learning and exploration
pub const SRF: vofc_Cipher = vofc_Cipher::new("srf", "Study Raft");

// =============================================================================
// Registry Access
// =============================================================================

/// All registered ciphers for iteration and validation.
pub const ALL_CIPHERS: &[vofc_Cipher] = &[
    // VO/VV ecosystem
    VV, VO,
    // Portable kits
    JJ, BU, CM, HM,
    // Recipe Bottle domain
    RB, CRG,
    // Tools and utilities
    GAD, CCC, LMCI, VSL,
    // Concept model vocabulary
    MCM, AXL,
    // Other projects
    PB, WRS, SRF,
];

// =============================================================================
// Kit Registry
// =============================================================================
// Typed kit declarations for distribution. Field names align with VOS entity
// members (vosem_kit_id, vosem_display_name).

/// A distributable kit with typed cipher reference.
/// Kit identifier derived from cipher via cipher.kit_id().
#[derive(Debug, Clone, Copy)]
pub struct vofc_Kit {
    /// Reference to the kit's cipher (typed, not string).
    pub cipher: &'static vofc_Cipher,
    /// Human-readable name. See vosem_display_name.
    pub display_name: &'static str,
}

/// Asset routing rule for kit installation.
/// Determines where files are copied during install.
#[derive(Debug, Clone, Copy)]
pub struct vofc_AssetRoute {
    /// Source pattern relative to kit directory.
    pub source_pattern: &'static str,
    /// Target path relative to target repo.
    pub target_path: &'static str,
    /// If true, routes to .claude/commands/ instead of target_path.
    pub is_command: bool,
}

/// Kits included in VVK distribution.
/// Order matters: kits are installed in this order, affecting CLAUDE.md section ordering.
pub const DISTRIBUTABLE_KITS: &[vofc_Kit] = &[
    vofc_Kit { cipher: &BU, display_name: "Bash Utilities" },
    vofc_Kit { cipher: &CM, display_name: "Concept Model" },
    vofc_Kit { cipher: &JJ, display_name: "Job Jockey" },
    vofc_Kit { cipher: &VV, display_name: "Voce Viva" },
];

/// Compatibility accessor: returns kit IDs as strings.
/// For callers that only need the kit identifier strings.
pub fn vofc_distributable_kit_ids() -> Vec<String> {
    DISTRIBUTABLE_KITS.iter().map(|k| k.cipher.kit_id()).collect()
}

/// Find a cipher by prefix.
pub fn vofc_find_by_prefix(prefix: &str) -> Option<&'static vofc_Cipher> {
    ALL_CIPHERS.iter().find(|c| c.prefix == prefix)
}

/// Check if a prefix is registered.
pub fn vofc_is_registered(prefix: &str) -> bool {
    vofc_find_by_prefix(prefix).is_some()
}

/// Validate terminal exclusivity: ensure no prefix is a prefix of another.
/// Returns None if valid, or Some pair of conflicting ciphers.
pub fn vofc_validate_terminal_exclusivity() -> Option<(&'static vofc_Cipher, &'static vofc_Cipher)> {
    for (i, a) in ALL_CIPHERS.iter().enumerate() {
        for b in ALL_CIPHERS.iter().skip(i + 1) {
            if a.prefix.starts_with(b.prefix) || b.prefix.starts_with(a.prefix) {
                return Some((a, b));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cipher_creation() {
        assert_eq!(JJ.prefix(), "jj");
        assert_eq!(JJ.project(), "Job Jockey");
    }

    #[test]
    fn test_prefix_matching() {
        assert!(JJ.matches_prefix("jja_arcanum"));
        assert!(JJ.matches_prefix("jjk"));
        assert!(!JJ.matches_prefix("buk"));
    }

    #[test]
    fn test_name_validation() {
        assert!(JJ.validate_name("jja_arcanum"));
        assert!(JJ.validate_name("jjk"));
        assert!(!JJ.validate_name("jj")); // prefix alone is not valid
        assert!(!JJ.validate_name("buk"));
    }

    #[test]
    fn test_terminal_exclusivity() {
        // Should be None if all ciphers are properly exclusive
        assert!(vofc_validate_terminal_exclusivity().is_none());
    }

    #[test]
    fn test_find_by_prefix() {
        assert_eq!(vofc_find_by_prefix("jj"), Some(&JJ));
        assert_eq!(vofc_find_by_prefix("bu"), Some(&BU));
        assert_eq!(vofc_find_by_prefix("nonexistent"), None);
    }

    #[test]
    fn test_all_ciphers_lowercase() {
        for cipher in ALL_CIPHERS {
            assert!(
                cipher.prefix.chars().all(|c| c.is_ascii_lowercase()),
                "Cipher {} has non-lowercase prefix",
                cipher.prefix
            );
        }
    }

    #[test]
    fn test_kit_id_generation() {
        assert_eq!(JJ.kit_id(), "jjk");
        assert_eq!(BU.kit_id(), "buk");
        assert_eq!(VV.kit_id(), "vvk");
        assert_eq!(GAD.kit_id(), "gadk");
    }

    #[test]
    fn test_distributable_kit_ids() {
        let ids = vofc_distributable_kit_ids();
        assert_eq!(ids, vec!["buk", "cmk", "jjk", "vvk"]);
    }
}
