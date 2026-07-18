// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — perambulation: the ship/withhold judgment over every tracked path.
//
// Named for the manorial walking of the bounds: the periodic act that fixes,
// stretch by stretch, what lies within the estate and what lies without. A
// perambulation that does not close is not a perambulation — which is exactly
// the invariant the totality functions below hold it to.
//
// THE ONE MATCHER. This module is the single implementation of the judgment
// (RBSHC "The cut, and the single matcher"): the cut materializes from it, the
// sweep enforces it, and the crate's self-proofs prove it. No second matcher,
// in any language, may live beside it.
//
// The matcher: a row is a literal path PREFIX; the longest matching row wins,
// so precedence is a property of the rows themselves, never of their order,
// and no row can be silently shadowed by an earlier one. One rule spans all
// three grains — a tree (trailing slash), a stem (to the discriminating char),
// one file (its whole path). No glob syntax, deliberately: a glob's precedence
// is either order-dependent or ambiguous, and both are how a withheld path
// ends up shipped.
//
// The grain is a judgment in itself. A tree row promises the tree is UNIFORM —
// anything added under it later may ship (or must not) without further
// thought. Where that promise is false the tree is judged at a finer grain,
// and the finer rows win by construction.
//
// Pure over its inputs: every function below is a function of the table and a
// path list, so the proofs run on planted lists without cloning anything. The
// tracked set is captured by the caller (the cut), from git, never from a
// list — any other source of truth is a second copy waiting to drift.

use std::fmt;

/// A row's verdict. Typed, so no malformed disposition can exist at all —
/// the bash table had to gate this at enrollment; the enum is the gate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum rbthdr_Disposition {
    /// The path is delivered: materialized into the candidate from committed
    /// bytes, and expected in the candidate's object graph.
    Ship,
    /// The path stays behind: never materialized, and — the assertion that
    /// matters — never present in the candidate's object graph at all, at any
    /// depth of its history. A withheld path found in a candidate is a leak.
    Withhold,
}

impl fmt::Display for rbthdr_Disposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            rbthdr_Disposition::Ship => write!(f, "ship"),
            rbthdr_Disposition::Withhold => write!(f, "withhold"),
        }
    }
}

use rbthdr_Disposition::{Ship, Withhold};

/// The perambulation — one row per judgment, the longest matching prefix wins.
///
/// The prose beside each group argues the judgments that need an argument, so
/// the table itself stays a bare table.
pub const RBTHDR_ROWS: &[(&str, rbthdr_Disposition)] = &[
    // THE VEILED HALVES. Each kit's closed record, finer than the kit rows
    // beneath them so the judgment holds even where the whole kit ships. A NEW
    // kit's veiled half is deliberately not covered: it lands unjudged and
    // reddens. That is correct — a kit's exposure is a ruling, never a default.
    ("Tools/buk/vov_veiled/", Withhold),
    ("Tools/cmk/vov_veiled/", Withhold),
    ("Tools/gad/vov_veiled/", Withhold),
    ("Tools/jjk/vov_veiled/", Withhold),
    ("Tools/rbk/vov_veiled/", Withhold),
    ("Tools/vok/vov_veiled/", Withhold),
    // THE DELIVERED KITS. BUK and RBK ship whole apart from their veiled
    // halves: the .sh surface, the shellcheck config, the READMEs, the
    // agent-context markdown, the theurge crate, the in-pool python step
    // bodies, the ifrit attack corpus.
    ("Tools/buk/", Ship),
    ("Tools/rbk/", Ship),
    // The remaining release-rig verb module in the shipped kit tree. The bash
    // harbinger stands until the hierophant earns its retirement through real
    // service; it is withheld from every candidate meanwhile. (Its former
    // siblings — the bash perambulation and expede — died with the cut
    // absorption: the crate you are reading is their one home now.)
    ("Tools/rbk/rblm_harbinger.sh", Withhold),
    // THE WITHHELD KITS. Whole trees: the operator's own tooling, or projects
    // that share the repo but not the delivery. None is uniform by accident —
    // each is a tree the consumer has no seat for.
    ("Tools/apck/", Withhold),
    ("Tools/cmk/", Withhold),
    ("Tools/gad/", Withhold),
    ("Tools/hmk/", Withhold),
    ("Tools/jjk/", Withhold),
    ("Tools/lmci/", Withhold),
    ("Tools/vok/", Withhold),
    ("Tools/vslf-rbw/", Withhold),
    ("Tools/vslk/", Withhold),
    ("Tools/vvc/", Withhold),
    ("Tools/vvk/", Withhold),
    // Residue of retired kits.
    ("Tools/cccr.env", Withhold),
    ("Tools/crgr.render.sh", Withhold),
    ("Tools/crgv.validate.sh", Withhold),
    ("Tools/xxx_rbn.info.sh", Withhold),
    // TABTARGETS ARE FILE-GRAIN, because tt/ is not uniform. The marshal
    // family and the manor raze are the release rig itself; a consumer running
    // any of them holds the wrong end of the tool. The stem rows outrank the
    // family rows by length, so a marshal tabtarget minted tomorrow is
    // withheld without a new row, and the hierophant's own dispatch stem
    // tt/rbthw- rides the same logic.
    ("tt/rbw-M", Withhold),
    ("tt/rbw-mR.", Withhold),
    ("tt/rbthw-", Withhold),
    ("tt/rbw-", Ship),
    ("tt/buw-", Ship),
    ("tt/z-launcher.sh", Ship),
    ("tt/apcw-", Withhold),
    ("tt/jjw-", Withhold),
    ("tt/study-", Withhold),
    ("tt/vow-", Withhold),
    ("tt/vslk-", Withhold),
    ("tt/vvw-", Withhold),
    // THE MOORINGS. Every nameplate ships: README documents each as an example
    // crucible and the onboarding handbooks walk several. The fdkyclk
    // carve-out is why the table must be file-grain where the tree is not
    // uniform — its two proof-stage scripts carry the operator's org id, and a
    // directory-grain "all ship" is exactly what once carried them into every
    // candidate. Its caged credentials, by contrast, DO ship: committed test
    // scaffolding the realm expects by value (RBSFK "two-keys").
    ("rbmm_moorings/fdkyclk/fdkyclk-proof.sh", Withhold),
    ("rbmm_moorings/fdkyclk/fdkyclk-teardown.sh", Withhold),
    ("rbmm_moorings/fdkyclk/", Ship),
    ("rbmm_moorings/ccyolo/", Ship),
    ("rbmm_moorings/moriah/", Ship),
    ("rbmm_moorings/nineveh/", Ship),
    ("rbmm_moorings/pluml/", Ship),
    ("rbmm_moorings/srjcl/", Ship),
    ("rbmm_moorings/tadmor/", Ship),
    ("rbmm_moorings/rbmf_foedera/", Ship),
    ("rbmm_moorings/rbmv_vessels/", Ship),
    ("rbmm_moorings/burc.env", Ship),
    ("rbmm_moorings/rbrd.env", Ship),
    ("rbmm_moorings/rbrp.env", Ship),
    ("rbmm_moorings/rbrr.env", Ship),
    ("rbmm_moorings/rbrw.env", Ship),
    // The operator's remote machines.
    ("rbmm_moorings/rbmn_nodes/", Withhold),
    ("rbmm_moorings/rbmu_users/", Withhold),
    // LAUNCHERS ship only for the workbenches that ship — file-grain for tt/'s
    // reason: a launcher for a withheld workbench is a dangling reference.
    ("rbmm_moorings/rbml_launchers/launcher.buw_workbench.sh", Ship),
    ("rbmm_moorings/rbml_launchers/launcher.rbw_workbench.sh", Ship),
    ("rbmm_moorings/rbml_launchers/", Withhold),
    // The operator's own trees.
    ("Memos/", Withhold),
    ("Study/", Withhold),
    (".claude/", Withhold),
    (".idea/", Withhold),
    (".jjk/", Withhold),
    ("_slickedit/", Withhold),
    // The delivered face.
    ("README.md", Ship),
    ("CLAUDE.md", Ship),
    ("LICENSE", Ship),
    ("diagrams/", Ship),
    ("rbm-abstract-drawio.svg", Ship),
    (".gitattributes", Ship),
    (".gitignore", Ship),
    // ROOT FILES THAT STAY BEHIND. RELEASE.md is the release-qualification
    // procedure, the rig's own runbook; the MCP config names the operator's
    // servers; the iml is IDE furniture; the gateway proposal is an unsettled
    // internal design note.
    ("RELEASE.md", Withhold),
    (".mcp.json", Withhold),
    ("brm_recipemuster.iml", Withhold),
    ("podman-gateway-proposal.md", Withhold),
];

/// Validate a table's structural invariants — the checks the bash table ran at
/// enrollment, in front of the author who wrote the row:
///
///   - an empty prefix would match every path;
///   - a DUPLICATE prefix would tie-break by table order — precisely the
///     order-dependent shadowing the longest-wins rule exists to abolish.
///
/// Takes the rows as a parameter so the proofs can feed it malformed tables;
/// the cut validates `RBTHDR_ROWS` through it before any judgment is trusted.
pub fn validate(rows: &[(&str, rbthdr_Disposition)]) -> Result<(), String> {
    for (i, (prefix, _)) in rows.iter().enumerate() {
        if prefix.is_empty() {
            return Err("perambulation row with an empty prefix — it would match every path".to_string());
        }
        for (other, _) in rows.iter().skip(i + 1) {
            if prefix == other {
                return Err(format!(
                    "perambulation prefix '{}' is enrolled twice — equal-length rows would shadow by table order, which the longest-wins rule exists to abolish",
                    prefix
                ));
            }
        }
    }
    Ok(())
}

/// Rule one repo-relative path: the longest matching row wins. Returns the
/// verdict and the winning row's index, or None when the path is UNJUDGED —
/// the state the whole table exists to make loud. An unjudged path is never
/// treated as either verdict; there is no default in either direction.
pub fn judge(path: &str) -> Option<(rbthdr_Disposition, usize)> {
    let mut best: Option<(rbthdr_Disposition, usize, usize)> = None;
    for (i, (prefix, disposition)) in RBTHDR_ROWS.iter().enumerate() {
        if !path.starts_with(prefix) {
            continue;
        }
        if best.map_or(true, |(_, _, len)| prefix.len() > len) {
            best = Some((*disposition, i, prefix.len()));
        }
    }
    best.map(|(d, i, _)| (d, i))
}

/// Every tracked path no row judges. Red until someone rules — a new file may
/// not ship because nobody said not to, and it may not vanish because nobody
/// said to keep it.
pub fn unjudged(tracked: &[String]) -> Vec<String> {
    tracked
        .iter()
        .filter(|path| judge(path).is_none())
        .cloned()
        .collect()
}

/// Every row that wins for no tracked path. Two failures wear this one face —
/// a STALE row judging a path that no longer exists, and a SHADOWED row
/// outranked everywhere by a longer one, so its judgment never lands. Both
/// mean the table is lying about the tree, and both go red.
pub fn dead_rows(tracked: &[String]) -> Vec<(&'static str, rbthdr_Disposition)> {
    let mut won = vec![false; RBTHDR_ROWS.len()];
    for path in tracked {
        if let Some((_, index)) = judge(path) {
            won[index] = true;
        }
    }
    RBTHDR_ROWS
        .iter()
        .zip(won)
        .filter(|(_, won)| !won)
        .map(|((prefix, disposition), _)| (*prefix, *disposition))
        .collect()
}

/// Every tracked path the perambulation ships — the cut's materialization
/// list, in the caller's (git's) order.
pub fn shipped(tracked: &[String]) -> Vec<String> {
    tracked
        .iter()
        .filter(|path| matches!(judge(path), Some((rbthdr_Disposition::Ship, _))))
        .cloned()
        .collect()
}

/// Judge a path list as an OBJECT GRAPH, not a tree: every judged-withheld
/// path in it is a leak. This is the assertion the 2026-07-13 candidate had no
/// version of — its TIP was clean while its HISTORY carried the whole
/// pre-strip repository to the remote at 292 MiB. The cut feeds this the
/// candidate's object graph (rev-list --objects), so a withheld path is caught
/// wherever it is reachable from the branch, at any depth. A path no row
/// judges is skipped, not flagged: historical graphs legitimately carry paths
/// the living tree no longer rules on.
pub fn sweep(graph: &[String]) -> Vec<String> {
    graph
        .iter()
        .filter(|path| matches!(judge(path), Some((rbthdr_Disposition::Withhold, _))))
        .cloned()
        .collect()
}
