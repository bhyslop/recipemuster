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
// RBTDRN — conformance: the vocabulary-eviction static-analysis fixture.
//
// The standing home for evicted-term assertions (ACG "The named home"). When a
// concept is renamed, stray uses of the dead word hide in source and specs;
// hand-grep is unreliable because a stem inside a kept identifier looks like a
// violation and a stem in a tabtarget filename (a sprue) is easy to miss. This
// fixture is the data-driven engine that replaces that grep: each row is one
// {kill_stem, keep_contexts}; the engine scans Tools/ and tt/ with identifier-
// boundary awareness, flagging the stem as a bare token while sparing it inside a
// kept identifier or under an exempt path.
//
// Engine, not word list. The eviction table (ZRBTDRN_EVICTION_ROWS) ships EMPTY:
// population is deferred per-cluster, each term gaining its row behind its own
// cutover, owned by the heat retiring it. What ships now is the proven mechanism.
//
// Checker proves itself (ACG move discipline, rule 2). The self-test cases run
// the matcher against known in-memory inputs — one bare use that MUST be caught,
// a kept identifier and an exempt path that MUST be respected, a filename sprue
// that MUST be caught — so the verdict on the live tree is trustworthy. The
// self-proof is hermetic: it never plants a real violation in the repo, so the
// fast suite stays green with zero production rows. The live-tree walker engages
// the instant the first row is added.
//
// Identifier-boundary awareness — the whole reason this is code, not grep. A
// "token" is a maximal run of [A-Za-z0-9_]. An occurrence is judged by the whole
// token that carries the stem: a bare `pale` is a violation; `pale` inside the
// sanctioned identifier `rbpale_resolve` is cleared only when that identifier is
// declared as a keep-context. Grep cannot make that distinction cheaply.

use std::path::{
    Path,
    PathBuf,
};

use crate::case;
use crate::rbtdre_engine::{
    rbtdre_Case,
    rbtdre_Disposition,
    rbtdre_Fixture,
    rbtdre_Verdict,
};
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_CONFORMANCE;

// ── Scan corpus ─────────────────────────────────────────────

/// Repo-relative roots scanned for evicted vocabulary. Tools/ holds the kit
/// source and specs; tt/ holds the tabtarget sprues. Historical record (Memos/,
/// retired heats) lies outside these roots and is spared by construction.
pub(crate) const ZRBTDRN_SCAN_ROOTS: &[&str] = &["Tools", "tt"];

// ── Eviction row and keep-context ───────────────────────────

/// One way an evicted stem is permitted to survive a scan.
pub(crate) enum zrbtdrn_KeepContext {
    /// The stem may appear inside this exact identifier token — the carrying
    /// `[A-Za-z0-9_]` run equals this string. This is the boundary distinction:
    /// a bare kill-stem is flagged while the same stem inside a sanctioned
    /// compound identifier is cleared.
    Identifier(&'static str),
    /// The stem may appear anywhere under this repo-relative path prefix — for
    /// the in-flight or vendored files a cutover deliberately leaves untouched.
    PathPrefix(&'static str),
}

/// One evicted vocabulary stem and the contexts where it may still appear. An
/// empty `keep_contexts` is a *pure corpse* — the stem must appear nowhere in the
/// scanned corpus. One engine serves both: a pure corpse is just a row with no
/// keep-contexts, never a separate grep-for-simple path.
pub(crate) struct zrbtdrn_EvictionRow {
    pub(crate) kill_stem: &'static str,
    pub(crate) keep_contexts: &'static [zrbtdrn_KeepContext],
}

/// The standing eviction table — the single place a retired term gains its
/// assertion (ACG: add a row here, never improvise a grep). Empty until a heat
/// retires a term behind its own cutover; each row engages the live corpus scan
/// the moment it is added. The self-test cases prove the engine independent of
/// this table, so the mechanism ships proven with zero production rows.
pub(crate) const ZRBTDRN_EVICTION_ROWS: &[zrbtdrn_EvictionRow] = &[];

/// One surviving occurrence of an evicted stem: where it is and the identifier
/// token that carries it.
pub(crate) struct zrbtdrn_Hit {
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) token: String,
    pub(crate) line_text: String,
}

// ── Matcher (pure — the self-proven heart) ──────────────────

/// Split `text` into maximal identifier tokens — runs of `[A-Za-z0-9_]`. This is
/// the identifier-boundary awareness: a stem is judged by the whole token that
/// carries it, not by raw character adjacency.
fn zrbtdrn_tokens(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    for c in text.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            cur.push(c);
        } else if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Record every occurrence of `row.kill_stem` in `text` whose carrying token is
/// not cleared by an `Identifier` keep-context. `line` is the 1-based content
/// line, or 0 for the file's basename (sprue position).
fn zrbtdrn_scan_text(
    rel_path: &str,
    line: usize,
    text: &str,
    row: &zrbtdrn_EvictionRow,
    hits: &mut Vec<zrbtdrn_Hit>,
) {
    for token in zrbtdrn_tokens(text) {
        if !token.contains(row.kill_stem) {
            continue;
        }
        let cleared = row.keep_contexts.iter().any(|keep| match keep {
            zrbtdrn_KeepContext::Identifier(id) => token.as_str() == *id,
            zrbtdrn_KeepContext::PathPrefix(_) => false,
        });
        if cleared {
            continue;
        }
        hits.push(zrbtdrn_Hit {
            path: rel_path.to_string(),
            line,
            token: token.clone(),
            line_text: text.to_string(),
        });
    }
}

/// Scan one file's identity and contents for a row's kill-stem, returning every
/// surviving occurrence not cleared by a keep-context. A `PathPrefix` keep
/// exempts the whole file; an `Identifier` keep clears occurrences whose carrying
/// token matches. The basename is scanned as line 0 so an evicted stem in a
/// tabtarget filename (a sprue) is caught alongside one in the contents.
fn zrbtdrn_match(rel_path: &str, content: &str, row: &zrbtdrn_EvictionRow) -> Vec<zrbtdrn_Hit> {
    let mut hits: Vec<zrbtdrn_Hit> = Vec::new();

    for keep in row.keep_contexts {
        if let zrbtdrn_KeepContext::PathPrefix(prefix) = keep {
            if rel_path.starts_with(prefix) {
                return hits;
            }
        }
    }

    let basename = rel_path.rsplit('/').next().unwrap_or(rel_path);
    zrbtdrn_scan_text(rel_path, 0, basename, row, &mut hits);

    for (idx, line) in content.lines().enumerate() {
        zrbtdrn_scan_text(rel_path, idx + 1, line, row, &mut hits);
    }

    hits
}

// ── Live-tree walk ──────────────────────────────────────────

/// Recursively collect every file under `dir` into `out`.
fn zrbtdrn_walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            zrbtdrn_walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Walk the scan roots under `root` and return every surviving occurrence of any
/// eviction row's kill-stem. Returns empty immediately when no rows are declared
/// — the standing engine costs nothing until a term is retired into it.
fn zrbtdrn_scan(root: &Path, rows: &[zrbtdrn_EvictionRow]) -> Result<Vec<zrbtdrn_Hit>, String> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let mut files: Vec<PathBuf> = Vec::new();
    for sub in ZRBTDRN_SCAN_ROOTS {
        zrbtdrn_walk(&root.join(sub), &mut files);
    }
    files.sort();

    let mut hits: Vec<zrbtdrn_Hit> = Vec::new();
    for path in &files {
        // Non-UTF8 / unreadable files (binaries) hold no vocabulary — skip.
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let rel = path.strip_prefix(root).unwrap_or(path).display().to_string();
        for row in rows {
            hits.extend(zrbtdrn_match(&rel, &content, row));
        }
    }
    Ok(hits)
}

/// Render hits as a stable one-per-line report.
fn zrbtdrn_render(hits: &[zrbtdrn_Hit]) -> String {
    let mut report = String::new();
    for h in hits {
        report.push_str(&format!("{}:{}: {} — {}\n", h.path, h.line, h.token, h.line_text.trim()));
    }
    report
}

// ── Self-test cases — the checker proves itself ─────────────

/// A synthetic stem that appears in no real source — the self-tests run the
/// matcher against in-memory corpora only, never the live tree, so the proof is
/// hermetic and plants nothing in the repo.
const ZRBTDRN_SELF_STEM: &str = "zzdeadstem";

/// A sanctioned identifier that embeds ZRBTDRN_SELF_STEM by construction — the
/// "stem inside a kept identifier" case. (The enum holds `&'static str`, so this
/// coupling is a literal, not a derivation.)
const ZRBTDRN_SELF_KEPT_ID: &str = "rb_zzdeadstem_kept";

/// One corpus, both halves of the discrimination: a bare stem that MUST be
/// caught and the same stem inside a kept identifier that MUST be respected.
fn rbtdrn_self_catch_and_keep_identifier(_dir: &Path) -> rbtdre_Verdict {
    let row = zrbtdrn_EvictionRow {
        kill_stem: ZRBTDRN_SELF_STEM,
        keep_contexts: &[zrbtdrn_KeepContext::Identifier(ZRBTDRN_SELF_KEPT_ID)],
    };
    let content = "a bare zzdeadstem here\nlet x = rb_zzdeadstem_kept();\n";
    let hits = zrbtdrn_match("Tools/rbk/probe_selftest.txt", content, &row);

    if hits.len() != 1 {
        return rbtdre_Verdict::Fail(format!(
            "expected exactly 1 hit (the bare stem; the kept identifier respected), got {}:\n{}",
            hits.len(),
            zrbtdrn_render(&hits)
        ));
    }
    let h = &hits[0];
    if h.line != 1 || h.token != ZRBTDRN_SELF_STEM {
        return rbtdre_Verdict::Fail(format!(
            "hit shape wrong: line {} token '{}' (expected line 1 token '{}')",
            h.line, h.token, ZRBTDRN_SELF_STEM
        ));
    }
    rbtdre_Verdict::Pass
}

/// A path-prefix keep exempts the whole file — the stem may stay under an
/// in-flight / vendored path a cutover deliberately leaves untouched.
fn rbtdrn_self_keep_path_prefix(_dir: &Path) -> rbtdre_Verdict {
    let row = zrbtdrn_EvictionRow {
        kill_stem: ZRBTDRN_SELF_STEM,
        keep_contexts: &[zrbtdrn_KeepContext::PathPrefix("Tools/rbk/vov_veiled/legacy/")],
    };
    let content = "zzdeadstem all over\nanother zzdeadstem line\n";
    let hits = zrbtdrn_match("Tools/rbk/vov_veiled/legacy/old.adoc", content, &row);

    if hits.is_empty() {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "path-prefix keep not respected — {} hit(s) under an exempt path:\n{}",
            hits.len(),
            zrbtdrn_render(&hits)
        ))
    }
}

/// A stem in a tabtarget filename (a sprue) MUST be caught — boundary awareness
/// reaches the basename, not just the contents.
fn rbtdrn_self_catch_sprue(_dir: &Path) -> rbtdre_Verdict {
    let row = zrbtdrn_EvictionRow {
        kill_stem: ZRBTDRN_SELF_STEM,
        keep_contexts: &[],
    };
    let hits = zrbtdrn_match("tt/rbw-zzdeadstem.Run.sh", "", &row);

    if hits.len() != 1 || hits[0].line != 0 || hits[0].token != ZRBTDRN_SELF_STEM {
        return rbtdre_Verdict::Fail(format!(
            "expected exactly 1 sprue hit at line 0 token '{}', got {}:\n{}",
            ZRBTDRN_SELF_STEM,
            hits.len(),
            zrbtdrn_render(&hits)
        ));
    }
    rbtdre_Verdict::Pass
}

/// A pure corpse (empty keep_contexts) flags a bare use, and the engine reports
/// nothing where the stem is absent — no false positives.
fn rbtdrn_self_pure_corpse(_dir: &Path) -> rbtdre_Verdict {
    let row = zrbtdrn_EvictionRow {
        kill_stem: ZRBTDRN_SELF_STEM,
        keep_contexts: &[],
    };

    let present = zrbtdrn_match("Tools/rbk/x.md", "x zzdeadstem y\n", &row);
    if present.len() != 1 {
        return rbtdre_Verdict::Fail(format!(
            "pure corpse: expected 1 hit on a bare use, got {}:\n{}",
            present.len(),
            zrbtdrn_render(&present)
        ));
    }

    let absent = zrbtdrn_match("Tools/rbk/x.md", "no dead vocabulary on this line\n", &row);
    if !absent.is_empty() {
        return rbtdre_Verdict::Fail(format!(
            "pure corpse: expected 0 hits where the stem is absent, got {}:\n{}",
            absent.len(),
            zrbtdrn_render(&absent)
        ));
    }
    rbtdre_Verdict::Pass
}

/// Live-tree scan against the standing eviction table. With zero production rows
/// this passes by construction; the instant a row is added it engages the walker
/// over Tools/ and tt/ and fails on any surviving occurrence.
fn rbtdrn_live_scan(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let hits = match zrbtdrn_scan(&root, ZRBTDRN_EVICTION_ROWS) {
        Ok(h) => h,
        Err(e) => return rbtdre_Verdict::Fail(e),
    };
    let report = zrbtdrn_render(&hits);
    let _ = std::fs::write(dir.join("conformance-evictions.txt"), &report);

    if hits.is_empty() {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "{} evicted-term survival(s) in the live tree:\n{}",
            hits.len(),
            report
        ))
    }
}

// ── Cases and fixture ───────────────────────────────────────

pub static RBTDRN_CASES_CONFORMANCE: &[rbtdre_Case] = &[
    case!(rbtdrn_self_catch_and_keep_identifier),
    case!(rbtdrn_self_keep_path_prefix),
    case!(rbtdrn_self_catch_sprue),
    case!(rbtdrn_self_pure_corpse),
    case!(rbtdrn_live_scan),
];

pub static RBTDRN_FIXTURE_CONFORMANCE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CONFORMANCE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRN_CASES_CONFORMANCE,
};
