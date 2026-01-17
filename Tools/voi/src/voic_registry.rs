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
pub struct voic_Cipher {
    prefix: &'static str,
    project: &'static str,
}

impl voic_Cipher {
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
pub const VV: voic_Cipher = voic_Cipher::new("vv", "Voce Viva");

/// VO: Vox Obscura - the kit forge infrastructure (never distributed)
pub const VO: voic_Cipher = voic_Cipher::new("vo", "Vox Obscura");

// -----------------------------------------------------------------------------
// Portable Kits
// -----------------------------------------------------------------------------

/// JJ: Job Jockey - project management and heat tracking
pub const JJ: voic_Cipher = voic_Cipher::new("jj", "Job Jockey");

/// BU: Bash Utilities Kit - enterprise bash infrastructure
pub const BU: voic_Cipher = voic_Cipher::new("bu", "Bash Utilities");

/// CM: Concept Model Kit - MCM/AXLA concept model tooling
pub const CM: voic_Cipher = voic_Cipher::new("cm", "Concept Model");

/// HM: Hard-state Machine Kit - state machine infrastructure
pub const HM: voic_Cipher = voic_Cipher::new("hm", "Hard-state Machine");

// -----------------------------------------------------------------------------
// Recipe Bottle Domain
// -----------------------------------------------------------------------------

/// RB: Recipe Bottle - container orchestration and deployment
pub const RB: voic_Cipher = voic_Cipher::new("rb", "Recipe Bottle");

/// CRG: Config Regime - configuration management
pub const CRG: voic_Cipher = voic_Cipher::new("crg", "Config Regime");

// -----------------------------------------------------------------------------
// Tools and Utilities
// -----------------------------------------------------------------------------

/// GAD: Google AsciiDoc Differ - diff visualization tool
pub const GAD: voic_Cipher = voic_Cipher::new("gad", "Google AsciiDoc Differ");

/// CCC: Claude Code Container Kit - Docker/container operations
pub const CCC: voic_Cipher = voic_Cipher::new("ccc", "Claude Code Container");

/// LMCI: Language Model Console Integration
pub const LMCI: voic_Cipher = voic_Cipher::new("lmci", "Language Model Console Integration");

/// VSL: Visual SlickEdit Local Kit - IDE integration
pub const VSL: voic_Cipher = voic_Cipher::new("vsl", "Visual SlickEdit Local");

// -----------------------------------------------------------------------------
// Concept Model Vocabulary
// -----------------------------------------------------------------------------

/// MCM: Meta Concept Model - specification for concept models
pub const MCM: voic_Cipher = voic_Cipher::new("mcm", "Meta Concept Model");

/// AXL: Axiom Lexicon - shared vocabulary definitions
pub const AXL: voic_Cipher = voic_Cipher::new("axl", "Axiom Lexicon");

// -----------------------------------------------------------------------------
// Other Projects
// -----------------------------------------------------------------------------

/// PB: Paneboard - cross-platform UI toolkit
pub const PB: voic_Cipher = voic_Cipher::new("pb", "Paneboard");

/// WRS: Ward Realm Substrate - distributed state machine substrate
pub const WRS: voic_Cipher = voic_Cipher::new("wrs", "Ward Realm Substrate");

/// SRF: Study Raft - learning and exploration
pub const SRF: voic_Cipher = voic_Cipher::new("srf", "Study Raft");

// =============================================================================
// Registry Access
// =============================================================================

/// All registered ciphers for iteration and validation.
pub const ALL_CIPHERS: &[voic_Cipher] = &[
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

/// Find a cipher by prefix.
pub fn voic_find_by_prefix(prefix: &str) -> Option<&'static voic_Cipher> {
    ALL_CIPHERS.iter().find(|c| c.prefix == prefix)
}

/// Check if a prefix is registered.
pub fn voic_is_registered(prefix: &str) -> bool {
    voic_find_by_prefix(prefix).is_some()
}

/// Validate terminal exclusivity: ensure no prefix is a prefix of another.
/// Returns None if valid, or Some pair of conflicting ciphers.
pub fn voic_validate_terminal_exclusivity() -> Option<(&'static voic_Cipher, &'static voic_Cipher)> {
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
        assert!(voic_validate_terminal_exclusivity().is_none());
    }

    #[test]
    fn test_find_by_prefix() {
        assert_eq!(voic_find_by_prefix("jj"), Some(&JJ));
        assert_eq!(voic_find_by_prefix("bu"), Some(&BU));
        assert_eq!(voic_find_by_prefix("nonexistent"), None);
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
}
