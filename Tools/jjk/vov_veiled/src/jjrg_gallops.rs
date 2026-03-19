// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops JSON operations
//!
//! Implements read/write operations on the Gallops JSON store.
//! All operations are atomic (write to temp, then rename).
//!
//! This module re-exports the public API from the new modular structure.

use std::path::Path;

// Re-export types
pub use crate::jjrt_types::*;

// Re-export validation
pub use crate::jjrv_validate::jjrg_validate;

// Re-export utilities
pub use crate::jjru_util::{jjrg_capture_commit_sha, jjrg_make_tack, jjrg_read_stdin, jjrg_read_stdin_optional};

// Re-export operations
pub use crate::jjro_ops::{jjrg_nominate, jjrg_slate, jjrg_rail, jjrg_tally, jjrg_draft, jjrg_retire, jjrg_build_trophy_preview, jjrg_furlough, jjrg_curry, jjrg_garland, jjrg_restring};

// Backwards compatibility: impl methods on jjrg_Gallops
impl jjrg_Gallops {
    /// Load Gallops from a file path (legacy API - prefer jjdr_load)
    pub fn jjrg_load(path: &Path) -> Result<Self, String> {
        use crate::jjri_io::jjdr_load;

        // Use validated load, then unwrap to plain Gallops
        jjdr_load(path).map(|vg| vg.into_inner())
    }

    /// Save Gallops to a file path (legacy API - prefer jjdr_save)
    pub fn jjrg_save(&self, path: &Path) -> Result<(), String> {
        use crate::jjri_io::jjdr_save;

        jjdr_save(self, path)
    }

    /// Validate the Gallops structure
    pub fn jjrg_validate(&self) -> Result<(), Vec<String>> {
        jjrg_validate(self)
    }

    /// Nominate a new Heat
    pub fn jjrg_nominate(&mut self, args: jjrg_NominateArgs, base_path: &Path) -> Result<jjrg_NominateResult, String> {
        jjrg_nominate(self, args, base_path)
    }

    /// Slate a new Pace
    pub fn jjrg_slate(&mut self, args: jjrg_SlateArgs) -> Result<jjrg_SlateResult, String> {
        jjrg_slate(self, args)
    }

    /// Rail - reorder Paces within a Heat
    pub fn jjrg_rail(&mut self, args: jjrg_RailArgs) -> Result<Vec<String>, String> {
        jjrg_rail(self, args)
    }

    /// Tally - add a new Tack to a Pace
    pub fn jjrg_tally(&mut self, args: jjrg_TallyArgs) -> Result<(), String> {
        jjrg_tally(self, args)
    }

    /// Draft - move a Pace from one Heat to another
    pub fn jjrg_draft(&mut self, args: jjrg_DraftArgs) -> Result<jjrg_DraftResult, String> {
        jjrg_draft(self, args)
    }

    /// Retire a Heat
    pub fn jjrg_retire(
        &mut self,
        args: jjrg_RetireArgs,
        base_path: &Path,
        steeplechase: &[crate::jjrs_steeplechase::jjrs_SteeplechaseEntry],
    ) -> Result<jjrg_RetireResult, String> {
        jjrg_retire(self, args, base_path, steeplechase)
    }

    /// Build trophy markdown preview (dry-run)
    pub fn jjrg_build_trophy_preview(
        &self,
        firemark: &str,
        paddock_content: &str,
        today: &str,
        steeplechase: &[crate::jjrs_steeplechase::jjrs_SteeplechaseEntry],
    ) -> Result<String, String> {
        jjrg_build_trophy_preview(self, firemark, paddock_content, today, steeplechase)
    }

    /// Furlough a Heat - change status or rename
    pub fn jjrg_furlough(&mut self, args: jjrg_FurloughArgs) -> Result<(), String> {
        jjrg_furlough(self, args)
    }

    /// Garland a Heat - celebrate completion and create continuation
    pub fn jjrg_garland(&mut self, args: jjrg_GarlandArgs, base_path: &Path) -> Result<jjrg_GarlandResult, String> {
        jjrg_garland(self, args, base_path)
    }

    /// Restring - bulk draft multiple paces atomically
    pub fn jjrg_restring(&mut self, args: jjrg_RestringArgs) -> Result<jjrg_RestringResult, String> {
        jjrg_restring(self, args)
    }

    // -- Spec-governed primitives (Operation Taxonomy in JJS0) --

    /// Resolve Pace — shared read primitive
    ///
    /// Navigate Gallops from a coronet to the target pace and its current tack state.
    /// Returns PaceContext with parsed identities and current state snapshot.
    pub fn jjrg_resolve_pace(&self, coronet: &str) -> Result<jjrg_PaceContext, String> {
        use crate::jjrf_favor::jjrf_Coronet as Coronet;

        let parsed = Coronet::jjrf_parse(coronet)
            .map_err(|e| format!("Invalid coronet: {}", e))?;
        let coronet_key = parsed.jjrf_display();
        let firemark = parsed.jjrf_parent_firemark();
        let firemark_key = firemark.jjrf_display();

        let heat = self.heats.get(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;
        let pace = heat.paces.get(&coronet_key)
            .ok_or_else(|| format!("Pace '{}' not found", coronet_key))?;
        let current = pace.tacks.first()
            .ok_or_else(|| "Pace has no tacks (should never happen)".to_string())?;

        Ok(jjrg_PaceContext {
            coronet_key,
            firemark_key,
            state: current.state.clone(),
            text: current.text.clone(),
            silks: current.silks.clone(),
            direction: current.direction.clone(),
        })
    }

    /// Prepend Tack — shared write primitive
    ///
    /// Insert a new tack at position zero of a pace's tack history.
    pub fn jjrg_prepend_tack(&mut self, coronet: &str, tack: jjrg_Tack) -> Result<(), String> {
        use crate::jjrf_favor::jjrf_Coronet as Coronet;

        let parsed = Coronet::jjrf_parse(coronet)
            .map_err(|e| format!("Invalid coronet: {}", e))?;
        let coronet_key = parsed.jjrf_display();
        let firemark = parsed.jjrf_parent_firemark();
        let firemark_key = firemark.jjrf_display();

        let heat = self.heats.get_mut(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;
        let pace = heat.paces.get_mut(&coronet_key)
            .ok_or_else(|| format!("Pace '{}' not found", coronet_key))?;

        pace.tacks.insert(0, tack);
        Ok(())
    }

    /// Revise Docket — composed method
    ///
    /// Pure state transform: resolve_pace → update docket → prepend_tack.
    /// Takes basis and ts from caller (procedure layer captures I/O).
    /// Returns PaceContext so calling procedure has firemark/silks for commit message.
    pub fn jjrg_revise_docket(&mut self, coronet: &str, docket: &str, basis: &str, ts: &str) -> Result<jjrg_PaceContext, String> {
        if docket.is_empty() {
            return Err("docket text must not be empty".to_string());
        }

        let ctx = self.jjrg_resolve_pace(coronet)?;

        let tack = jjrg_Tack {
            ts: ts.to_string(),
            state: ctx.state.clone(),
            text: docket.to_string(),
            silks: ctx.silks.clone(),
            basis: basis.to_string(),
            direction: None,
        };

        self.jjrg_prepend_tack(coronet, tack)?;

        Ok(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// Build a minimal Gallops from stack immediates — no disk, no git.
    /// Keys use display format (with ₣/₢ prefixes) matching real Gallops structure.
    fn make_test_gallops(state: jjrg_PaceState, text: &str, silks: &str) -> jjrg_Gallops {
        let firemark_key = "₣AA".to_string();
        let coronet_key = "₢AAAAA".to_string();
        let tack = jjrg_Tack {
            ts: "20260318T120000Z".to_string(),
            state,
            text: text.to_string(),
            silks: silks.to_string(),
            basis: "0000000".to_string(),
            direction: None,
        };
        let pace = jjrg_Pace {
            tacks: vec![tack],
        };
        let mut paces = BTreeMap::new();
        paces.insert(coronet_key.clone(), pace);
        let heat = jjrg_Heat {
            silks: "test-heat".to_string(),
            creation_time: "260318".to_string(),
            status: jjrg_HeatStatus::Racing,
            order: vec![coronet_key],
            next_pace_seed: "AAB".to_string(),
            paddock_file: ".claude/jjm/jjp_test.md".to_string(),
            paces,
        };
        let mut heats = BTreeMap::new();
        heats.insert(firemark_key.clone(), heat);
        jjrg_Gallops {
            schema_version: Some(4),
            next_heat_seed: "AB".to_string(),
            heat_order: vec![firemark_key],
            heats,
        }
    }

    #[test]
    fn test_resolve_pace_returns_context() {
        let gallops = make_test_gallops(jjrg_PaceState::Rough, "original docket", "test-pace");
        let ctx = gallops.jjrg_resolve_pace("AAAAA").unwrap();
        assert_eq!(ctx.firemark_key, "₣AA");
        assert_eq!(ctx.coronet_key, "₢AAAAA");
        assert_eq!(ctx.state, jjrg_PaceState::Rough);
        assert_eq!(ctx.text, "original docket");
        assert_eq!(ctx.silks, "test-pace");
        assert_eq!(ctx.direction, None);
    }

    #[test]
    fn test_revise_docket_updates_text() {
        let mut gallops = make_test_gallops(jjrg_PaceState::Rough, "original docket", "test-pace");
        let ctx = gallops.jjrg_revise_docket("AAAAA", "updated docket", "abc1234", "20260318T130000Z").unwrap();

        // Context returns pre-mutation state
        assert_eq!(ctx.text, "original docket");

        // Gallops now has new tack prepended with caller-provided basis+ts
        let pace = gallops.heats["₣AA"].paces.get("₢AAAAA").unwrap();
        assert_eq!(pace.tacks.len(), 2);
        assert_eq!(pace.tacks[0].text, "updated docket");
        assert_eq!(pace.tacks[0].state, jjrg_PaceState::Rough);
        assert_eq!(pace.tacks[0].basis, "abc1234");
        assert_eq!(pace.tacks[0].ts, "20260318T130000Z");
        assert_eq!(pace.tacks[1].text, "original docket");
    }

    #[test]
    fn test_revise_docket_empty_text_rejected() {
        let mut gallops = make_test_gallops(jjrg_PaceState::Rough, "original", "test-pace");
        let result = gallops.jjrg_revise_docket("AAAAA", "", "abc1234", "20260318T130000Z");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not be empty"));
    }
}
