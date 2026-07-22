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
pub use crate::jjrv_validate::{jjrg_validate, jjrg_reconcile};

// Re-export utilities
pub use crate::jjru_util::{jjrg_capture_commit_sha, jjrg_make_tack, jjrg_read_stdin, jjrg_read_stdin_optional};

// Re-export operations
pub use crate::jjro_ops::{jjrg_nominate, jjrg_nominate_excise, jjrg_nominate_apply, jjrg_NominatePlan, jjrg_slate, jjrg_rail, jjrg_tally, jjrg_draft, jjrg_retire, jjrg_retire_excise, jjrg_retire_apply, jjrg_RetirePlan, jjrg_build_trophy_preview, jjrg_furlough, jjrg_curry_apply, jjrg_restring};

// Backwards compatibility: impl methods on jjrg_Gallops
impl jjrg_Gallops {
    /// Load Gallops from a file path (legacy API - prefer jjdr_load)
    pub fn jjrg_load(path: &Path) -> Result<Self, String> {
        use crate::jjri_io::jjdr_load;

        // Use validated load, then unwrap to plain Gallops
        jjdr_load(path).map(|vg| vg.into_inner())
    }

    /// Hark-load Gallops from bytes lifted from a prior git revision (read-only
    /// retrospective). Sibling of jjrg_load; see jjdr_hark / JJS0 `jjdr_hark`.
    pub fn jjrg_hark(bytes: &[u8]) -> Result<Self, String> {
        use crate::jjri_io::jjdr_hark;

        jjdr_hark(bytes).map(|vg| vg.into_inner())
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

    /// Restring - bulk draft multiple paces atomically
    pub fn jjrg_restring(&mut self, args: jjrg_RestringArgs) -> Result<jjrg_RestringResult, String> {
        jjrg_restring(self, args)
    }

    // -- Spec-governed primitives (Operation Taxonomy in JJS0) --

    /// Resolve a Coronet (display-form key, `₢`-prefixed) to the Firemark key of
    /// the Heat that harbours it — the paces-scan the flat-id model requires
    /// (JJS0 jjdt_coronet Resolution). A Coronet embeds no affiliation, so lookup
    /// scans heats' paces rather than inferring a heat from the identity;
    /// cross-heat uniqueness makes the hit unambiguous.
    pub fn jjrg_heat_key_of_coronet(&self, coronet_key: &str) -> Option<String> {
        self.heats.iter()
            .find(|(_, heat)| heat.paces.contains_key(coronet_key))
            .map(|(fm, _)| fm.clone())
    }

    /// Render a Coronet in its heat-qualified emission form (JJS0 jjdt_coronet
    /// "Display and ingest"): `₢` + the live heat Firemark characters + the
    /// interpunct `·` + the 5-character body, e.g. `₢Bc·CAAAB`. The qualifier is
    /// read from LIVE affiliation here, so a later relocate changes tomorrow's
    /// rendering, never the identity — the one emission helper the listing and
    /// emblem surfaces route through. Fail-soft: a Coronet no heat harbours (or a
    /// malformed key) renders bare `₢CAAAB`, so a display path never fabricates an
    /// affiliation it cannot prove. Accepts stored, bare, or already-qualified
    /// input — it is normalized to the bare body first.
    pub fn jjrg_qualify_coronet(&self, coronet: &str) -> String {
        use crate::jjrf_favor::{jjrf_bare, JJRF_CORONET_PREFIX, JJRF_CORONET_QUALIFIER};
        let body = jjrf_bare(coronet);
        let display_key = format!("{}{}", JJRF_CORONET_PREFIX, body);
        match self.jjrg_heat_key_of_coronet(&display_key) {
            Some(heat_key) => format!(
                "{}{}{}{}",
                JJRF_CORONET_PREFIX, jjrf_bare(&heat_key), JJRF_CORONET_QUALIFIER, body
            ),
            None => display_key,
        }
    }

    /// Resolve Pace — shared read primitive
    ///
    /// Navigate Gallops from a coronet to the target pace and its current tack state.
    /// Returns PaceContext with parsed identities and current state snapshot.
    pub fn jjrg_resolve_pace(&self, coronet: &str) -> Result<jjrg_PaceContext, String> {
        use crate::jjrf_favor::jjrf_Coronet;

        let parsed = jjrf_Coronet::jjrf_parse(coronet)
            .map_err(|e| format!("Invalid coronet: {}", e))?;
        let coronet_key = parsed.jjrf_display();
        // Resolve the harbouring heat by paces-scan — a Coronet embeds no
        // affiliation (JJS0 jjdt_coronet Resolution).
        let firemark_key = self.jjrg_heat_key_of_coronet(&coronet_key)
            .ok_or_else(|| format!("Pace '{}' not found", coronet_key))?;

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
            tier: current.tier,
            effort: current.effort,
            text: jjrg_lines_to_text(&current.text),
            silks: current.silks.clone(),
        })
    }

    /// Set Tack — shared write primitive
    ///
    /// Replace a pace's tack with the given one. A pace holds a single current tack;
    /// tack evolution lives in git (JJS0 Git-as-Journal), not in an in-JSON history.
    /// Takes a PaceContext (from resolve_pace) so the coronet is parsed exactly once.
    pub fn jjrg_set_tack(&mut self, ctx: &jjrg_PaceContext, tack: jjrg_Tack) -> Result<(), String> {
        let heat = self.heats.get_mut(&ctx.firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", ctx.firemark_key))?;
        let pace = heat.paces.get_mut(&ctx.coronet_key)
            .ok_or_else(|| format!("Pace '{}' not found", ctx.coronet_key))?;

        pace.tacks = vec![tack];
        Ok(())
    }

    /// Revise Docket — composed method
    ///
    /// Pure state transform: resolve_pace → update docket → set_tack (replace).
    /// Takes basis and ts from caller (procedure layer captures I/O).
    /// Returns PaceContext so calling procedure has firemark/silks for commit message.
    ///
    /// Revert trigger: a designation is void when its judgment inputs change —
    /// redocketing a bridled pace (single or mass form, both land here) reverts
    /// it to rough and wipes the tier and effort. Other states pass through with
    /// their designation provenance intact.
    pub fn jjrg_revise_docket(&mut self, coronet: &str, docket: &str, basis: &str, ts: &str) -> Result<jjrg_PaceContext, String> {
        if docket.is_empty() {
            return Err("docket text must not be empty".to_string());
        }

        let ctx = self.jjrg_resolve_pace(coronet)?;

        let was_bridled = ctx.state == jjrg_PaceState::Bridled;
        let tack = jjrg_Tack {
            ts: ts.to_string(),
            state: if was_bridled { jjrg_PaceState::Rough } else { ctx.state.clone() },
            tier: if was_bridled { None } else { ctx.tier },
            effort: if was_bridled { None } else { ctx.effort },
            text: jjrg_text_to_lines(docket),
            silks: ctx.silks.clone(),
            basis: basis.to_string(),
        };

        self.jjrg_set_tack(&ctx, tack)?;

        // Drift signal: every docket revision — single or mass reslate, both
        // funnel through here — bumps the pace's redocket counter. Deliberately
        // NOT in jjrg_set_tack: bridle and release replace the tack too but are
        // not docket revisions. The frozen original-intent fields are untouched.
        let heat = self.heats.get_mut(&ctx.firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", ctx.firemark_key))?;
        let pace = heat.paces.get_mut(&ctx.coronet_key)
            .ok_or_else(|| format!("Pace '{}' not found", ctx.coronet_key))?;
        pace.redocket_count += 1;

        Ok(ctx)
    }

    /// Bridle — composed method (designation half of jjx_apostille)
    ///
    /// Records that a frontier agent judged this pace ready to execute and
    /// ruled on the tier its execution needs: transitions rough → bridled with
    /// the designated execution tier and an optional effort on the tack. Every
    /// designable tier is first-class here — a frontier designation (opus,
    /// fable) is an ordinary outcome, not an anomaly. Only a rough pace may be
    /// bridled — the
    /// rough filter is the precondition, so a bridled pace must be released
    /// (or reverted by a docket edit) before re-designation.
    /// Pure transform: caller provides basis and ts. Returns the pre-mutation
    /// PaceContext for the commit message.
    pub fn jjrg_bridle(
        &mut self,
        coronet: &str,
        tier: jjrg_Tier,
        effort: Option<jjrg_Effort>,
        basis: &str,
        ts: &str,
    ) -> Result<jjrg_PaceContext, String> {
        let ctx = self.jjrg_resolve_pace(coronet)?;

        if ctx.state != jjrg_PaceState::Rough {
            return Err(format!(
                "only a rough pace may be bridled; '{}' is {}",
                ctx.coronet_key, ctx.state.jjrg_as_str()
            ));
        }

        let tack = jjrg_Tack {
            ts: ts.to_string(),
            state: jjrg_PaceState::Bridled,
            tier: Some(tier),
            effort,
            text: jjrg_text_to_lines(&ctx.text),
            silks: ctx.silks.clone(),
            basis: basis.to_string(),
        };

        self.jjrg_set_tack(&ctx, tack)?;

        Ok(ctx)
    }

    /// Release — composed method (un-bridle half of jjx_apostille)
    ///
    /// The deliberate frontier escalation path: bridled → rough with tier and
    /// effort wiped, returning the pace to judgment work. Only a bridled pace
    /// may be released. Pure transform: caller provides basis and ts. Returns
    /// the pre-mutation PaceContext for the commit message.
    pub fn jjrg_release(&mut self, coronet: &str, basis: &str, ts: &str) -> Result<jjrg_PaceContext, String> {
        let ctx = self.jjrg_resolve_pace(coronet)?;

        if ctx.state != jjrg_PaceState::Bridled {
            return Err(format!(
                "only a bridled pace may be released; '{}' is {}",
                ctx.coronet_key, ctx.state.jjrg_as_str()
            ));
        }

        let tack = jjrg_Tack {
            ts: ts.to_string(),
            state: jjrg_PaceState::Rough,
            tier: None,
            effort: None,
            text: jjrg_text_to_lines(&ctx.text),
            silks: ctx.silks.clone(),
            basis: basis.to_string(),
        };

        self.jjrg_set_tack(&ctx, tack)?;

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
            tier: None,
            effort: None,
            text: jjrg_text_to_lines(text),
            silks: silks.to_string(),
            basis: "0000000".to_string(),
        };
        let pace = jjrg_Pace {
            tacks: vec![tack],
            ..Default::default()
        };
        let mut paces = BTreeMap::new();
        paces.insert(coronet_key.clone(), pace);
        let heat = jjrg_Heat {
            silks: "test-heat".to_string(),
            creation_time: "260318".to_string(),
            status: jjrg_HeatStatus::Racing,
            order: vec![coronet_key],
            paces,
        };
        let mut heats = BTreeMap::new();
        heats.insert(firemark_key.clone(), heat);
        jjrg_Gallops {
            next_heat_seed: "AB".to_string(),
            next_pace_seed: "CAAAA".to_string(),
            heat_order: vec![firemark_key],
            heats,
            retention_since: None,
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
    }

    #[test]
    fn test_revise_docket_updates_text() {
        let mut gallops = make_test_gallops(jjrg_PaceState::Rough, "original docket", "test-pace");
        let ctx = gallops.jjrg_revise_docket("AAAAA", "updated docket", "abc1234", "20260318T130000Z").unwrap();

        // Context returns pre-mutation state
        assert_eq!(ctx.text, "original docket");

        // Gallops now holds the single replacement tack with caller-provided basis+ts
        let pace = gallops.heats["₣AA"].paces.get("₢AAAAA").unwrap();
        assert_eq!(pace.tacks.len(), 1);
        assert_eq!(pace.tacks[0].text, vec!["updated docket".to_string()]);
        assert_eq!(pace.tacks[0].state, jjrg_PaceState::Rough);
        assert_eq!(pace.tacks[0].basis, "abc1234");
        assert_eq!(pace.tacks[0].ts, "20260318T130000Z");
    }

    #[test]
    fn test_revise_docket_empty_text_rejected() {
        let mut gallops = make_test_gallops(jjrg_PaceState::Rough, "original", "test-pace");
        let result = gallops.jjrg_revise_docket("AAAAA", "", "abc1234", "20260318T130000Z");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not be empty"));
    }
}
