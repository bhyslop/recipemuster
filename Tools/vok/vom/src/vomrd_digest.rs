// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM digest audit - the read-only staleness audit of the hand-curated
//! `claude-*-acronyms.md` digests (VOSMM-entity.adoc "Report and Cadastre").
//! The digests stay hand-owned: selecting the important rows out of every
//! declaration home is editorial judgment, which VOr_m7w bars from
//! mechanization. This audit reads each curated row as a validation subject,
//! never round-tripping it: a cipher-prefixed row head with no seated
//! declaration home is a dead-row presentment (advisory - a home the
//! allowlist cannot see, a diagram source say, reads as dead without being
//! so); a non-cipher head (a guide acronym, a foreign tree) is out of
//! jurisdiction and skipped.
//!
//! Veiled digests are skipped whole: their homes are veiled paths the census
//! walk excludes, so every row would read falsely dead.

use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::vomrm_matricula::vomrm_Matricula;
use crate::vomrp_presentment::vomrp_Presentment;
use crate::vomrs_signet::vomrs_Site;

// Curated digest row: `- **HEAD** → ...` (the uniform row shape the digests
// carry). The head is the audit subject; the backticked home spans decide
// jurisdiction only (veiled-home skip), never liveness.
static ZVOMRD_ROW: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*-\s+\*\*([A-Za-z0-9_]+)\*\*").unwrap());

// Backticked home spans on a digest row.
static ZVOMRD_HOME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"`([^`]+)`").unwrap());

/// Whether a tracked path is a curated acronym digest this audit reads.
/// Veiled digests are excluded (their homes sit outside census jurisdiction).
pub fn vomrd_is_digest(rel_path: &Path) -> bool {
    if vof::vofr_is_veiled_path(rel_path) {
        return false;
    }
    rel_path
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with("claude-") && n.ends_with("-acronyms.md"))
}

/// Gather the digest corpus from the repo, read-only - the same
/// walk-then-read shape as the census seat, decoupled so the audit itself
/// can be exercised against planted corpora.
pub fn vomrd_gather(repo_root: &Path) -> Result<Vec<(String, String)>, String> {
    let tracked = vof::vofr_git_tracked_files(repo_root)?;
    let mut corpus = Vec::new();
    for rel_path in &tracked {
        if !vomrd_is_digest(rel_path) {
            continue;
        }
        let Some(path_str) = rel_path.to_str() else {
            continue;
        };
        let Ok(content) = std::fs::read_to_string(repo_root.join(rel_path)) else {
            continue;
        };
        corpus.push((path_str.to_string(), content));
    }
    Ok(corpus)
}

/// Audit every curated digest row against the frozen census: one dead-row
/// presentment per cipher-prefixed head with no seated declaration home.
/// Pure over the census and the handed corpus; mutates nothing.
pub fn vomrd_audit(
    census: &vomrm_Matricula,
    corpus: &[(String, String)],
) -> Vec<vomrp_Presentment> {
    let mut out = Vec::new();
    for (path, content) in corpus {
        for (line_no, line) in content.lines().enumerate() {
            let Some(cap) = ZVOMRD_ROW.captures(line) else {
                continue;
            };
            let head = &cap[1];
            if crate::vomrc_cadastre::zvomrc_cipher_of(head).is_none() {
                continue;
            }
            if zvomrd_row_homes_all_veiled(line) {
                continue;
            }
            if zvomrd_is_seated(census, head) {
                continue;
            }
            out.push(vomrp_Presentment {
                inscriptions: vec![head.to_string()],
                sites: vec![vomrs_Site::vomrs_new(path.clone(), line_no + 1)],
                detail: format!(
                    "curated digest row head `{head}` has no seated declaration home"
                ),
                rule: "index-of-record audit (VOSMM 'Report and Cadastre')",
                advisory: true,
            });
        }
    }
    out
}

// A non-veiled digest may still curate rows whose homes sit in a veiled
// tree, or in the studbook (a sibling repo the census walk never reaches);
// those homes are outside census jurisdiction exactly as veiled digests are,
// so auditing them would read falsely dead. Skipped only when every
// backticked home span on the row is out-of-jurisdiction - a mixed or
// path-free row stays in jurisdiction.
fn zvomrd_row_homes_all_veiled(line: &str) -> bool {
    let mut saw_home = false;
    for cap in ZVOMRD_HOME.captures_iter(line) {
        saw_home = true;
        if !zvomrd_home_out_of_jurisdiction(&cap[1]) {
            return false;
        }
    }
    saw_home
}

// A home span is out of census jurisdiction when it is veiled, or when it
// anchors into the studbook - a sibling repo the census walk excludes just
// as it excludes veiled trees.
fn zvomrd_home_out_of_jurisdiction(home: &str) -> bool {
    vof::vofr_is_veiled_path(Path::new(home)) || zvomrd_is_studbook_home(home)
}

// Whether a backticked home span anchors into `jjqs_studbook/`.
fn zvomrd_is_studbook_home(home: &str) -> bool {
    Path::new(home)
        .components()
        .any(|c| c.as_os_str() == "jjqs_studbook")
}

// A row head is live when some seated signet matches it case-folded - equal,
// or extending it (a family head like RBFCV is announced live by any seated
// `rbfcv_*` member). Lenient by design: under-flagging beats a false dead
// verdict on an advisory channel.
fn zvomrd_is_seated(census: &vomrm_Matricula, head: &str) -> bool {
    let head_lower = head.to_ascii_lowercase();
    census
        .vomrm_signet_trie()
        .vomrs_signets()
        .any(|s| s.to_ascii_lowercase().starts_with(&head_lower))
}

// eof
