// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM cadastre - the one generated complete-census file (VOSMM-entity.adoc
//! "Report and Cadastre"). A pure projection of the frozen signet trie: a
//! do-not-hand-edit header naming the regenerate tabtarget (the
//! `rbtdgc_consts.rs` generated-file precedent), one section per registered
//! cipher, one row per seated declaration home in trie order, repo-root-
//! relative paths, no globs and no multi-file coalescing. Any hand edit is
//! overwritten by the next render run.
//!
//! The freshness half of the precedent rides the report: vomrc_freshness
//! compares the committed cadastre against the frozen census and surfaces a
//! mismatch as a presentment naming the regenerate tabtarget. The file's
//! content nowhere names the census tool (VOr_q4f name-hygiene at the
//! artifact); it describes itself only by what it is - a name census.

use std::collections::BTreeSet;
use std::path::Path;

use crate::vomrm_matricula::vomrm_Matricula;
use crate::vomrp_presentment::vomrp_Presentment;
use crate::vomrs_signet::vomrs_Site;

/// Repo-root-relative home of the generated cadastre file.
pub const VOMRC_CADASTRE_PATH: &str = "Tools/vok/vod_cadastre.md";

/// The tabtarget that regenerates the cadastre, named in its header.
pub const VOMRC_REGENERATE: &str = "tt/vow-mc.RenderCadastre.sh";

/// Render the whole cadastre, pure over the frozen census: header, one
/// section per registered cipher (registry order), one row per seated
/// declaration home (trie order; homes sorted, deduped per file, never
/// coalesced).
pub fn vomrc_render(census: &vomrm_Matricula) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "<!-- Generated file - do not hand-edit; the next render run overwrites it. Regenerate: {VOMRC_REGENERATE} -->\n"
    ));
    out.push_str("\n# Cadastre\n\n");
    out.push_str(
        "Every minted name seated in this repository, one row per declaration home, grouped by project prefix.\n",
    );

    let trie = census.vomrm_signet_trie();
    for cipher in vof::ALL_CIPHERS {
        out.push_str(&format!(
            "\n## {} ({})\n\n",
            cipher.project(),
            cipher.prefix()
        ));
        for signet in trie.vomrs_signets() {
            if zvomrc_cipher_of(signet).map(|c| c.prefix()) != Some(cipher.prefix()) {
                continue;
            }
            let mut homes: BTreeSet<&str> = BTreeSet::new();
            for (_, site) in trie.vomrs_sites(signet) {
                homes.insert(site.file.as_str());
            }
            for home in homes {
                out.push_str(&format!("- `{signet}` → `{home}`\n"));
            }
        }
    }
    out
}

/// The registered cipher a seated signet belongs to. Registry prefixes are
/// mutually prefix-free, so the match is unique when it exists.
pub(crate) fn zvomrc_cipher_of(signet: &str) -> Option<&'static vof::vofc_Cipher> {
    let lower = signet.to_ascii_lowercase();
    vof::ALL_CIPHERS.iter().find(|c| lower.starts_with(c.prefix()))
}

/// The report-side freshness gate: compare the committed cadastre against
/// the frozen census and surface a mismatch (or a missing file) as a
/// presentment naming the regenerate tabtarget. Read-only.
pub fn vomrc_freshness(census: &vomrm_Matricula, repo_root: &Path) -> Option<vomrp_Presentment> {
    let expected = vomrc_render(census);
    let on_disk = std::fs::read_to_string(repo_root.join(VOMRC_CADASTRE_PATH)).ok();
    if on_disk.as_deref() == Some(expected.as_str()) {
        return None;
    }
    let state = if on_disk.is_none() { "missing" } else { "stale" };
    Some(vomrp_Presentment {
        inscriptions: vec![VOMRC_CADASTRE_PATH.to_string()],
        sites: vec![vomrs_Site::vomrs_new(VOMRC_CADASTRE_PATH, 0)],
        detail: format!("cadastre {state} against the frozen census - regenerate: {VOMRC_REGENERATE}"),
        rule: "generated-file freshness (VOSMM 'Report and Cadastre')",
        advisory: false,
    })
}

// eof
