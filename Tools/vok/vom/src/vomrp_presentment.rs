// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM presentment - Tier 3 output record (VOSMM-entity.adoc "Seating
//! Validators", quoin vosmp_presentment). A seating validator's formal
//! report of one minting violation the census observed directly: the
//! violating inscription(s), the declaration sites, and the violated rule
//! cited to its CMK/MCM home (never restated here). Per VOr_m7w a
//! presentment over a fallible gate (sprue, rivet) is advisory, never an
//! assertion.

use crate::vomrs_signet::vomrs_Site;

/// One seating-validator finding.
pub struct vomrp_Presentment {
    pub inscriptions: Vec<String>,
    pub sites: Vec<vomrs_Site>,
    pub detail: String,
    pub rule: &'static str,
    pub advisory: bool,
}

impl vomrp_Presentment {
    /// Render one presentment block, operator-facing.
    pub fn vomrp_render(&self) -> String {
        let mut out = String::new();
        let stance = if self.advisory { "advisory" } else { "presentment" };
        out.push_str(&format!(
            "[{stance}] {}\n",
            self.inscriptions.join(", ")
        ));
        out.push_str(&format!("  rule: {}\n", self.rule));
        out.push_str(&format!("  {}\n", self.detail));
        for site in &self.sites {
            out.push_str(&format!("  site: {site}\n"));
        }
        out
    }
}

/// Render a whole presentment set, one block per finding, blank-line separated.
pub fn vomrp_render_all(presentments: &[vomrp_Presentment]) -> String {
    let mut out = String::new();
    for p in presentments {
        out.push_str(&p.vomrp_render());
        out.push('\n');
    }
    out
}

// eof
