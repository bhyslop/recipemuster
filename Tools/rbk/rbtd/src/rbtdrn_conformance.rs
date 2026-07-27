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
// reveille suite stays green with zero production rows. The live-tree walker engages
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
    rbtdre_Tariff,
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
        let rel = crate::rbtdrx_platform::rbtdrx_repo_rel(root, path);
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

// ── Curl containment scan ───────────────────────────────────
//
// The band placement doctrine (BCG "Precision Exit-Code Band", bubc_constants.sh
// placement comment) bars handing a curl exit status to the buc_die membrane:
// curl 8.6.0+ mints exit codes 100/101 inside the band window, so a bare
// `curl ... || buc_die` chain — or a bare curl whose failure propagates under
// set -e — masquerades as an in-band deliberate rejection at the dispatch
// boundary. Rather than hunt bad shapes, this scan legislates the one canonical
// form and errors on everything else (operator ruling 260704):
//
//   - a curl invocation must LEAD its line (after comment skip + quote strip);
//   - its terminator line must end with the byte-exact capture
//     `|| z_curl_status=$?` — the forced variable name is the point: the scan
//     verifies the classify step mechanically, no interpretation required;
//   - the captured z_curl_status must reappear within
//     ZRBTDRN_CURL_TEST_WINDOW lines of the terminator (the classify/test;
//     reappearance is checked on RAW lines because the var rides inside the
//     quoted test operand `"${z_curl_status}"`);
//   - a curl token anywhere else on a line is a misplaced-curl violation —
//     sole carve-out `command -v curl`, the presence probe whose `||` hands
//     command -v's exit onward, never curl's;
//   - a line whose quoting the line-local paired strip cannot resolve, when it
//     also carries a curl token, is itself a violation — the scan demands
//     lines simple enough to scan instead of growing bash-quoting
//     sophistication.

/// Repo-relative path prefixes exempt from the curl containment scan: retired
/// code deliberately left untouched, and the in-pool cloud-step trees whose
/// curl discipline is CBG/JDG dialect (`-f` flags, `|| exit N`, no buc_die) —
/// the band membrane does not exist there.
const ZRBTDRN_CURL_EXEMPT_PREFIXES: &[&str] = &[
    "Tools/rbk/vov_veiled/ABANDONED-github/",
    "Tools/rbk/rbgj",
];

/// The byte-exact terminator suffix every curl invocation must carry.
const ZRBTDRN_CURL_CAPTURE: &str = "|| z_curl_status=$?";

/// Lines after the terminator within which the captured z_curl_status must
/// reappear. Corpus maximum is rbuh_http's dual-variant if/else, whose first
/// branch's capture sits 21 lines from the shared test.
const ZRBTDRN_CURL_TEST_WINDOW: usize = 24;

/// One curl containment violation: where and which rule.
struct zrbtdrn_CurlHit {
    path: String,
    line: usize,
    kind: &'static str,
    text: String,
}

/// Strip paired quote spans from one line, replacing each span with a single
/// space so token boundaries survive. Backslash-escaped characters inside a
/// double-quoted span are skipped; single-quoted spans take no escapes (bash
/// semantics). An unpaired opening quote keeps the remainder verbatim and
/// reports it — the caller errors when an unscannable line carries a curl
/// token, rather than this parser growing multi-line string awareness.
fn zrbtdrn_strip_quoted(line: &str) -> (String, bool) {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '"' || c == '\'' {
            let mut j = i + 1;
            while j < chars.len() {
                if c == '"' && chars[j] == '\\' {
                    j += 2;
                    continue;
                }
                if chars[j] == c {
                    break;
                }
                j += 1;
            }
            if j < chars.len() {
                out.push(' ');
                i = j + 1;
                continue;
            }
            // Unpaired — keep the remainder verbatim and flag it.
            while i < chars.len() {
                out.push(chars[i]);
                i += 1;
            }
            return (out, true);
        }
        out.push(c);
        i += 1;
    }
    (out, false)
}

/// True when the stripped line's first token is a curl invocation.
fn zrbtdrn_curl_leads(stripped: &str) -> bool {
    let t = stripped.trim_start();
    match t.strip_prefix("curl") {
        Some(rest) => rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t'),
        None => false,
    }
}

/// True when the stripped text carries a bare `curl` token.
fn zrbtdrn_curl_token(stripped: &str) -> bool {
    zrbtdrn_tokens(stripped).iter().any(|t| t == "curl")
}

/// Scan one shell file's contents for curl containment violations. A state
/// machine per line: comment lines are skipped; a leading-curl invocation is
/// walked through its backslash continuations to the terminator, which must end
/// with the canonical capture, followed by a z_curl_status reappearance within
/// the window; any other line carrying a curl token is misplaced (carve-out:
/// `command -v curl`) or unscannable (unpaired quote).
fn zrbtdrn_curl_match(rel_path: &str, content: &str) -> Vec<zrbtdrn_CurlHit> {
    let lines: Vec<&str> = content.lines().collect();
    let mut hits: Vec<zrbtdrn_CurlHit> = Vec::new();
    let hit = |line: usize, kind: &'static str, text: &str| zrbtdrn_CurlHit {
        path: rel_path.to_string(),
        line,
        kind,
        text: text.trim().to_string(),
    };
    let mut i = 0;
    while i < lines.len() {
        let raw = lines[i];
        if raw.trim_start().starts_with('#') {
            i += 1;
            continue;
        }
        let (stripped, unpaired) = zrbtdrn_strip_quoted(raw);
        if !zrbtdrn_curl_token(&stripped) {
            i += 1;
            continue;
        }
        if unpaired {
            hits.push(hit(i + 1, "unscannable", raw));
            i += 1;
            continue;
        }
        if !zrbtdrn_curl_leads(&stripped) {
            if !stripped.contains("command -v curl") {
                hits.push(hit(i + 1, "misplaced", raw));
            }
            i += 1;
            continue;
        }
        // Leading-curl invocation: walk continuations to the terminator.
        let mut t = i;
        while t < lines.len() && lines[t].trim_end().ends_with('\\') {
            t += 1;
        }
        if t >= lines.len() {
            hits.push(hit(i + 1, "uncaptured", raw));
            break;
        }
        let (term_stripped, term_unpaired) = zrbtdrn_strip_quoted(lines[t]);
        if term_unpaired {
            hits.push(hit(t + 1, "unscannable", lines[t]));
        } else if !term_stripped.trim_end().ends_with(ZRBTDRN_CURL_CAPTURE) {
            hits.push(hit(t + 1, "uncaptured", lines[t]));
        } else {
            let window_end = lines.len().min(t + 1 + ZRBTDRN_CURL_TEST_WINDOW);
            let tested = lines[(t + 1)..window_end]
                .iter()
                .any(|l| zrbtdrn_tokens(l).iter().any(|tok| tok == "z_curl_status"));
            if !tested {
                hits.push(hit(t + 1, "untested", lines[t]));
            }
        }
        i = t + 1;
    }
    hits
}

/// Render curl hits as a stable one-per-line report.
fn zrbtdrn_curl_render(hits: &[zrbtdrn_CurlHit]) -> String {
    let mut report = String::new();
    for h in hits {
        report.push_str(&format!("{}:{}: {} — {}\n", h.path, h.line, h.kind, h.text));
    }
    report
}

/// The canonical form clears: reset, leading curl through continuations,
/// byte-exact capture, test within the window — plus the sanctioned
/// non-leading shapes (the `command -v curl` probe, quoted message text,
/// comment lines).
fn rbtdrn_self_curl_canon_clears(_dir: &Path) -> rbtdre_Verdict {
    let content = concat!(
        "z_curl_status=0\n",
        "curl -sS --max-time 5 \\\n",
        "  -o \"${z_file}\" \\\n",
        "  \"${z_url}\" > \"${z_code}\" || z_curl_status=$?\n",
        "test \"${z_curl_status}\" -eq 0 || buc_die \"probe failed (curl exit ${z_curl_status})\"\n",
        "command -v curl >/dev/null 2>&1 || buc_die \"curl not found\"\n",
        "# a bare curl ... || buc_die chain in a comment is ignored\n",
        "buc_log_args \"retrying (curl exit ${z_curl_status})\"\n",
    );
    let hits = zrbtdrn_curl_match("Tools/rbk/probe.sh", content);
    if hits.is_empty() {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "canonical form flagged — {} false positive(s):\n{}",
            hits.len(),
            zrbtdrn_curl_render(&hits)
        ))
    }
}

/// Each deviant shape is caught with its kind: the bare buc_die chain, the
/// capture-free invocation, the non-leading invocations (capture-assignment
/// and pipe), and the captured-but-never-tested window miss.
fn rbtdrn_self_curl_catches_deviants(_dir: &Path) -> rbtdre_Verdict {
    let filler = "buc_log_args unrelated\n".repeat(ZRBTDRN_CURL_TEST_WINDOW);
    let content = format!(
        concat!(
            "curl -s \\\n",
            "  \"${{z_url}}\" || buc_die \"HEAD failed\"\n",
            "curl -s \"${{z_url}}\" > \"${{z_file}}\"\n",
            "z_code=$(curl -s -w '%{{http_code}}' \"${{z_url}}\")\n",
            "echo \"${{z_body}}\" | curl -s -d @- \"${{z_url}}\"\n",
            "curl -s \"${{z_url}}\" || z_curl_status=$?\n",
            "{}"
        ),
        filler
    );
    let hits = zrbtdrn_curl_match("Tools/rbk/probe.sh", &content);
    let kinds: Vec<&str> = hits.iter().map(|h| h.kind).collect();
    let expected = ["uncaptured", "uncaptured", "misplaced", "misplaced", "untested"];
    if kinds != expected {
        return rbtdre_Verdict::Fail(format!(
            "deviant shapes misjudged — expected kinds {:?}, got:\n{}",
            expected,
            zrbtdrn_curl_render(&hits)
        ));
    }
    rbtdre_Verdict::Pass
}

/// A line the paired strip cannot resolve, carrying a curl token, is itself a
/// violation — the scan demands scannable lines rather than parsing abstruse
/// quoting.
fn rbtdrn_self_curl_unscannable(_dir: &Path) -> rbtdre_Verdict {
    let content = "z_json=' | curl -s -d @- \"${z_url}\"\n";
    let hits = zrbtdrn_curl_match("Tools/rbk/probe.sh", content);
    if hits.len() != 1 || hits[0].kind != "unscannable" {
        return rbtdre_Verdict::Fail(format!(
            "expected exactly 1 unscannable hit, got {}:\n{}",
            hits.len(),
            zrbtdrn_curl_render(&hits)
        ));
    }
    rbtdre_Verdict::Pass
}

/// Live-tree curl containment scan: every .sh file under the scan roots (minus
/// the exempt prefixes) must hold only canonical-form curl invocations.
fn rbtdrn_curl_containment(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let mut files: Vec<PathBuf> = Vec::new();
    for sub in ZRBTDRN_SCAN_ROOTS {
        zrbtdrn_walk(&root.join(sub), &mut files);
    }
    files.sort();

    let mut hits: Vec<zrbtdrn_CurlHit> = Vec::new();
    for path in &files {
        if path.extension().and_then(|e| e.to_str()) != Some("sh") {
            continue;
        }
        let rel = crate::rbtdrx_platform::rbtdrx_repo_rel(&root, path);
        if ZRBTDRN_CURL_EXEMPT_PREFIXES.iter().any(|p| rel.starts_with(p)) {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        hits.extend(zrbtdrn_curl_match(&rel, &content));
    }

    let report = zrbtdrn_curl_render(&hits);
    let _ = std::fs::write(dir.join("conformance-curl.txt"), &report);

    if hits.is_empty() {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "{} curl containment violation(s) in the live tree:\n{}",
            hits.len(),
            report
        ))
    }
}

// ── One-home discipline backstop (citation integrity, rivet hoist, A8 residue)
//
// The mechanically-checkable slice of ACG's one-home discipline: a between-
// audits guard, not a keystone. Paraphrase detection (the actual word cancer
// the manual audits hunt) is irreducibly semantic and stays out of scope.
// Three pure checks over an in-memory (rel_path, content) corpus — hermetic
// and self-proven like the eviction/curl checkers above — plus a live-tree
// wrapper per check.

/// One one-home-discipline finding: where, what kind, and the detail prose.
struct zrbtdrn_OneHomeHit {
    path: String,
    line: usize,
    kind: &'static str,
    detail: String,
}

fn zrbtdrn_onehome_render(hits: &[zrbtdrn_OneHomeHit]) -> String {
    let mut report = String::new();
    for h in hits {
        report.push_str(&format!("{}:{}: {} — {}\n", h.path, h.line, h.kind, h.detail));
    }
    report
}

/// True when `line`, trimmed, is exactly an AsciiDoc anchor `[[name]]` or
/// `[[name,display]]` — the shared definition-site syntax for both quoins and
/// rivets. Returns the bare name.
fn zrbtdrn_parse_anchor(line: &str) -> Option<String> {
    let t = line.trim();
    let inner = t.strip_prefix("[[")?.strip_suffix("]]")?;
    let name = inner.split(',').next().unwrap_or("").trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// True when `line` is a mapping-section attribute declaration
/// `:name:  <<target,Display Text>>` — the MCM quoin-registration line.
/// Returns `(name, target)`. A variant (`name` = `axo_entity_s`) points its
/// `target` at the base quoin's anchor (`axo_entity`), never its own — MCM's
/// `_s`/`_p`/`_ed`/`_ing` suffix mechanism, so `name` and `target` differ by
/// design and only `target` need resolve to an anchor.
fn zrbtdrn_parse_quoin_mapping(line: &str) -> Option<(String, String)> {
    let t = line.trim_start();
    let rest = t.strip_prefix(':')?;
    let colon = rest.find(':')?;
    let name = &rest[..colon];
    if name.is_empty() || !name.chars().next()?.is_ascii_lowercase() {
        return None;
    }
    if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        return None;
    }
    let tail = rest[colon + 1..].trim_start();
    let target = tail.strip_prefix("<<")?;
    let target = target.split(',').next().unwrap_or("").trim();
    if target.is_empty() {
        None
    } else {
        Some((name.to_string(), target.to_string()))
    }
}

/// Extract every `{term}` brace citation on `line` whose inner text is a
/// snake_case identifier carrying an underscore — the project-quoin shape,
/// distinguishing a real linked-term citation from AsciiDoc's own built-in
/// attributes (`{basebackend}`, `{docdir}`, …), which never carry one.
fn zrbtdrn_parse_braced_citations(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '{' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '}') {
                let inner: String = chars[i + 1..i + 1 + end].iter().collect();
                let is_ident = !inner.is_empty()
                    && inner.chars().next().unwrap().is_ascii_lowercase()
                    && inner.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                    && inner.contains('_');
                if is_ident {
                    out.push(inner);
                }
                i = i + 1 + end + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Every real `RBr_xxx` token on `line` — `RBr_` followed by the mint
/// convention's exactly-3-char lowercase-alnum opaque tail (`RBr_a3f`,
/// `RBr_m4d`, …). A bare `RBr_` with no tail is prose *describing* the naming
/// convention itself (MCM/ACG doctrine text), never a real citation, so it is
/// excluded by the length/shape requirement rather than by name.
fn zrbtdrn_parse_rivet_tokens(line: &str) -> Vec<String> {
    zrbtdrn_tokens(line)
        .into_iter()
        .filter(|t| match t.strip_prefix("RBr_") {
            Some(tail) => tail.len() == 3 && tail.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
            None => false,
        })
        .collect()
}

/// True when `token` has the shipped spec-acronym shape: `RBS` followed by one
/// or more uppercase-letter/digit characters (`RBS0`, `RBSPB`, `RBSIJ`, …). A8
/// source-residue bars this shape from shipped `.sh` files — only a bare
/// `RBr_` rivet ID or nothing may cite a spec from source.
fn zrbtdrn_is_spec_acronym(token: &str) -> bool {
    match token.strip_prefix("RBS") {
        Some(rest) => !rest.is_empty() && rest.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()),
        None => false,
    }
}

/// The prefix segment of a snake_case identifier — everything before the
/// first underscore. Used to recognize a *known quoin family*: a prefix with
/// at least one other mapping-declared member elsewhere in the corpus.
fn zrbtdrn_prefix_of(name: &str) -> &str {
    name.split('_').next().unwrap_or(name)
}

/// Check 1 — citation integrity: every mapping-declared quoin and every
/// `{term}`/`RBr_xxx` citation anywhere in the corpus resolves to a real
/// `[[anchor]]` definition site. `all_files` covers both `.adoc` and `.sh`
/// content (a rivet may be cited from either); braced-citation and mapping
/// scanning apply only to the `.adoc` corpus, since `{term}` and `:term:` are
/// AsciiDoc syntax.
///
/// A `{term}` citation is judged only when `term`'s prefix is a *known quoin
/// family* — some other mapping-declared name shares the same prefix
/// elsewhere in the corpus. AsciiDoc's `{...}` syntax collides with incidental
/// curly-brace prose (wire-format templates, shell-variable interpolation,
/// example error strings); restricting to established families is the
/// mechanical way to tell a real citation from that noise without parsing
/// quote/code-span context.
fn zrbtdrn_check_citations(
    adoc_files: &[(&str, &str)],
    all_files: &[(&str, &str)],
) -> Vec<zrbtdrn_OneHomeHit> {
    let mut anchors: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut mapping: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for (_, content) in adoc_files {
        for line in content.lines() {
            if let Some(name) = zrbtdrn_parse_anchor(line) {
                anchors.insert(name);
            }
            if let Some((name, target)) = zrbtdrn_parse_quoin_mapping(line) {
                mapping.insert(name, target);
            }
        }
    }
    let mut prefix_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for name in mapping.keys() {
        *prefix_counts.entry(zrbtdrn_prefix_of(name)).or_insert(0) += 1;
    }
    let known_families: std::collections::HashSet<&str> =
        prefix_counts.into_iter().filter(|(_, n)| *n > 0).map(|(p, _)| p).collect();

    let mut hits = Vec::new();

    for (path, content) in adoc_files {
        for (idx, line) in content.lines().enumerate() {
            if let Some((name, target)) = zrbtdrn_parse_quoin_mapping(line) {
                if !anchors.contains(&target) {
                    hits.push(zrbtdrn_OneHomeHit {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "unanchored-quoin",
                        detail: format!("mapping declares '{}' -> <<{}>> but no [[{}]] anchor exists", name, target, target),
                    });
                }
            }
            for term in zrbtdrn_parse_braced_citations(line) {
                if !known_families.contains(zrbtdrn_prefix_of(&term)) {
                    continue;
                }
                let resolves = mapping.get(&term).map_or(false, |target| anchors.contains(target));
                if !resolves {
                    hits.push(zrbtdrn_OneHomeHit {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "broken-quoin-citation",
                        detail: format!("{{{}}} resolves to no declared-and-anchored quoin", term),
                    });
                }
            }
        }
    }

    for (path, content) in all_files {
        for (idx, line) in content.lines().enumerate() {
            if zrbtdrn_parse_anchor(line).is_some() {
                continue;
            }
            for id in zrbtdrn_parse_rivet_tokens(line) {
                if !anchors.contains(&id) {
                    hits.push(zrbtdrn_OneHomeHit {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "broken-rivet-citation",
                        detail: format!("{} resolves to no [[{}]] anchor", id, id),
                    });
                }
            }
        }
    }

    hits
}

/// Rivets whose cross-sheaf citation is a deliberate, recorded hoist
/// deferral — never an unexamined exemption. Each entry names its removal
/// condition; delete the entry (and perform the hoist) once met.
///
/// - `RBr_m4d`: hoist deferred while RBS0 sits cinched hot (₣Bs
///   rbk-10-create-staging carries two unlanded spec-doctrine paces). Remove
///   this entry when RBr_m4d's definition prose moves from
///   RBSPB-citizen_brevet.adoc to RBS0-SpecTop.adoc.
const ZRBTDRN_HOIST_WAIVERS: &[&str] = &["RBr_m4d"];

/// The codex sheaf itself — RBS0, the hoist target MCM's rivet law names. A
/// rivet anchored here is definitionally already hoisted, so citing it from
/// any other sheaf is the intended end-state, never a violation.
const ZRBTDRN_CODEX_SHEAF: &str = "Tools/rbk/vov_veiled/RBS0-SpecTop.adoc";

/// Check 2 — rivet-hoist placement (MCM `mcm_rivet`: a rivet hoists to the
/// codex the moment a second sheaf cites it). A rivet anchored in one `.adoc`
/// sheaf and cited by bare token from a *different* `.adoc` sheaf should have
/// hoisted; `waivers` names the rivets whose deferral is recorded and
/// conditioned rather than silently exempted. A rivet anchored in `codex`
/// (RBS0) is exempt outright — that IS the hoisted state.
fn zrbtdrn_check_rivet_hoist(
    adoc_files: &[(&str, &str)],
    waivers: &[&str],
    codex: &str,
) -> Vec<zrbtdrn_OneHomeHit> {
    let mut def_sheaf: std::collections::HashMap<String, &str> = std::collections::HashMap::new();
    for (path, content) in adoc_files {
        for line in content.lines() {
            if let Some(name) = zrbtdrn_parse_anchor(line) {
                if name.starts_with("RBr_") {
                    def_sheaf.entry(name).or_insert(path);
                }
            }
        }
    }

    let mut hits = Vec::new();
    for (path, content) in adoc_files {
        for (idx, line) in content.lines().enumerate() {
            if zrbtdrn_parse_anchor(line).is_some() {
                continue;
            }
            for id in zrbtdrn_parse_rivet_tokens(line) {
                if waivers.contains(&id.as_str()) {
                    continue;
                }
                if let Some(&home) = def_sheaf.get(&id) {
                    if home != *path && home != codex {
                        hits.push(zrbtdrn_OneHomeHit {
                            path: path.to_string(),
                            line: idx + 1,
                            kind: "unhoisted-rivet",
                            detail: format!("{} defined in {} cited cross-sheaf here — hoist to the codex", id, home),
                        });
                    }
                }
            }
        }
    }
    hits
}

/// Check 3 — A8 source-residue: a shipped `.sh` file carries no spec-acronym
/// token (`RBS0`, `RBSPB`, …) — only a bare `RBr_` rivet ID or nothing. Callers
/// exclude any `vov_veiled/` path before calling — that tree is shelved/veiled,
/// never shipped, so its residue rules are out of scope.
fn zrbtdrn_check_a8_residue(sh_files: &[(&str, &str)]) -> Vec<zrbtdrn_OneHomeHit> {
    let mut hits = Vec::new();
    for (path, content) in sh_files {
        for (idx, line) in content.lines().enumerate() {
            for token in zrbtdrn_tokens(line) {
                if zrbtdrn_is_spec_acronym(&token) {
                    hits.push(zrbtdrn_OneHomeHit {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "a8-spec-acronym-residue",
                        detail: format!("shipped source cites spec acronym {} — residue must be bare RBr_ or nothing", token),
                    });
                }
            }
        }
    }
    hits
}

/// Live-tree wrapper: walk the scan roots, split into `.adoc` and `.sh`
/// corpora (the latter excluding any `vov_veiled/` path — shelved, not
/// shipped), and run all three one-home checks.
fn rbtdrn_onehome_live(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let mut files: Vec<PathBuf> = Vec::new();
    for sub in ZRBTDRN_SCAN_ROOTS {
        zrbtdrn_walk(&root.join(sub), &mut files);
    }
    files.sort();

    let mut adoc_owned: Vec<(String, String)> = Vec::new();
    let mut sh_owned: Vec<(String, String)> = Vec::new();
    for path in &files {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let rel = crate::rbtdrx_platform::rbtdrx_repo_rel(&root, path);
        match path.extension().and_then(|e| e.to_str()) {
            Some("adoc") => adoc_owned.push((rel, content)),
            Some("sh") if !rel.contains("/vov_veiled/") => sh_owned.push((rel, content)),
            _ => {}
        }
    }

    let adoc_refs: Vec<(&str, &str)> =
        adoc_owned.iter().map(|(p, c)| (p.as_str(), c.as_str())).collect();
    let mut all_owned: Vec<(String, String)> = adoc_owned.clone();
    all_owned.extend(sh_owned.iter().cloned());
    let all_refs: Vec<(&str, &str)> =
        all_owned.iter().map(|(p, c)| (p.as_str(), c.as_str())).collect();
    let sh_refs: Vec<(&str, &str)> =
        sh_owned.iter().map(|(p, c)| (p.as_str(), c.as_str())).collect();

    let mut hits = zrbtdrn_check_citations(&adoc_refs, &all_refs);
    hits.extend(zrbtdrn_check_rivet_hoist(&adoc_refs, ZRBTDRN_HOIST_WAIVERS, ZRBTDRN_CODEX_SHEAF));
    hits.extend(zrbtdrn_check_a8_residue(&sh_refs));

    let report = zrbtdrn_onehome_render(&hits);
    let _ = std::fs::write(dir.join("conformance-onehome.txt"), &report);

    if hits.is_empty() {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "{} one-home discipline violation(s) in the live tree:\n{}",
            hits.len(),
            report
        ))
    }
}

// ── One-home self-test cases — each check proves itself ─────

/// Citation integrity: a mapping-declared quoin with no anchor, a `{term}`
/// citation to a real anchor (clears), a `{term}` citation to nothing
/// (broken), and a bare `RBr_xxx` citation to nothing (broken) — all in one
/// synthetic corpus, none touching the live tree.
fn rbtdrn_self_citation_integrity(_dir: &Path) -> rbtdre_Verdict {
    let a = (
        "Tools/rbk/vov_veiled/RBSAA-fake.adoc",
        ":rbk_present:                 <<rbk_present,Present>>\n\
         :rbk_present_s:               <<rbk_present,Presents>>\n\
         :rbk_absent:                  <<rbk_absent,Absent>>\n\
         [[rbk_present]]\n\
         rbk_present:: defined here.\n\
         Cites the present one {rbk_present}, its variant {rbk_present_s}, and the broken one {rbk_missing}.\n\
         Also cites RBr_zzz which has no anchor.\n",
    );
    let adoc = vec![a];
    let all = vec![a];
    let hits = zrbtdrn_check_citations(&adoc, &all);
    let kinds: Vec<&str> = hits.iter().map(|h| h.kind).collect();
    let mut expected = vec!["unanchored-quoin", "broken-quoin-citation", "broken-rivet-citation"];
    let mut got = kinds.clone();
    got.sort();
    expected.sort();
    if got != expected {
        return rbtdre_Verdict::Fail(format!(
            "expected kinds {:?}, got {:?}:\n{}",
            expected,
            kinds,
            zrbtdrn_onehome_render(&hits)
        ));
    }
    rbtdre_Verdict::Pass
}

/// Rivet-hoist placement: a rivet cited from its own defining sheaf clears; the
/// same rivet cited from a second sheaf is flagged; a waived rivet cited
/// cross-sheaf clears despite the waiver being live; a rivet anchored in the
/// codex itself clears no matter how many sheaves cite it.
fn rbtdrn_self_rivet_hoist(_dir: &Path) -> rbtdre_Verdict {
    let home = ("Tools/rbk/vov_veiled/RBSAA-home.adoc", "[[RBr_a11]]\nRBr_a11:: lives here.\nRBr_a11 cited again in its own sheaf.\n");
    let sibling = ("Tools/rbk/vov_veiled/RBSAB-sibling.adoc", "Cites RBr_a11 from a different sheaf.\n");
    let waived_home = ("Tools/rbk/vov_veiled/RBSAC-waived.adoc", "[[RBr_m4d]]\nRBr_m4d:: lives here.\n");
    let waived_citer = ("Tools/rbk/vov_veiled/RBSAD-waived-citer.adoc", "Cites RBr_m4d from a different sheaf — waived.\n");
    let codex_home = (ZRBTDRN_CODEX_SHEAF, "[[RBr_c0d]]\nRBr_c0d:: lives in the codex.\n");
    let codex_citer = ("Tools/rbk/vov_veiled/RBSAE-codex-citer.adoc", "Cites RBr_c0d, already hoisted — clears.\n");
    let adoc = vec![home, sibling, waived_home, waived_citer, codex_home, codex_citer];

    let hits = zrbtdrn_check_rivet_hoist(&adoc, ZRBTDRN_HOIST_WAIVERS, ZRBTDRN_CODEX_SHEAF);
    if hits.len() != 1 || hits[0].kind != "unhoisted-rivet" || !hits[0].detail.contains("RBr_a11") {
        return rbtdre_Verdict::Fail(format!(
            "expected exactly 1 unhoisted-rivet hit for RBr_a11 (RBr_m4d waived), got:\n{}",
            zrbtdrn_onehome_render(&hits)
        ));
    }
    rbtdre_Verdict::Pass
}

/// A8 source-residue: a bare `RBr_` residue clears; a spec-acronym citation
/// (`RBSAA`) is flagged.
fn rbtdrn_self_a8_residue(_dir: &Path) -> rbtdre_Verdict {
    let sh = vec![(
        "Tools/rbk/probe.sh",
        "# clean residue cites RBr_a11 only\nz_x=1\n# violation cites RBSAA directly\n",
    )];
    let hits = zrbtdrn_check_a8_residue(&sh);
    if hits.len() != 1 || hits[0].kind != "a8-spec-acronym-residue" || hits[0].line != 3 {
        return rbtdre_Verdict::Fail(format!(
            "expected exactly 1 a8-spec-acronym-residue hit at line 3, got:\n{}",
            zrbtdrn_onehome_render(&hits)
        ));
    }
    rbtdre_Verdict::Pass
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
        keep_contexts: &[zrbtdrn_KeepContext::PathPrefix("Tools/rbk/vendor/legacy/")],
    };
    let content = "zzdeadstem all over\nanother zzdeadstem line\n";
    let hits = zrbtdrn_match("Tools/rbk/vendor/legacy/old.adoc", content, &row);

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
    case!(rbtdrn_self_curl_canon_clears),
    case!(rbtdrn_self_curl_catches_deviants),
    case!(rbtdrn_self_curl_unscannable),
    case!(rbtdrn_curl_containment),
    case!(rbtdrn_self_citation_integrity),
    case!(rbtdrn_self_rivet_hoist),
    case!(rbtdrn_self_a8_residue),
    case!(rbtdrn_onehome_live),
];

pub static RBTDRN_FIXTURE_CONFORMANCE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CONFORMANCE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRN_CASES_CONFORMANCE,
    credless: true,
    tariff: rbtdre_Tariff { min_secs: None, max_secs: None, invocations: Some(0) },
};
