// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
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
// The standing home for evicted-term assertions. When a
// concept is renamed, stray uses of the dead word hide in source and specs;
// hand-grep is unreliable because a stem inside a kept identifier looks like a
// violation and a stem in a tabtarget filename (a sprue) is easy to miss. This
// fixture is the data-driven engine that replaces that grep: each row is one
// {kill_stem, keep_contexts}; the engine scans Tools/ and tt/ with identifier-
// boundary awareness, flagging the stem as a bare token while sparing it inside a
// kept identifier or under an exempt path.
//
// Engine, not word list. The eviction table (ZRBTCHN_EVICTION_ROWS) ships EMPTY:
// population is deferred per-cluster, each term gaining its row behind its own
// cutover, owned by the heat retiring it. What ships now is the proven mechanism.
//
// Checker proves itself. The self-test cases run
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
    PathBuf, //
};

use crate::rbtcee_case;
use crate::rbtcee_engine::{
    rbtcee_Case,
    rbtcee_Disposition,
    rbtcee_Fixture,
    rbtcee_Tariff,
    rbtcee_Verdict, //
};
use crate::rbtcem_manifest::RBTCEM_FIXTURE_CONFORMANCE;

// ── Scan corpus ─────────────────────────────────────────────

/// Repo-relative roots scanned for evicted vocabulary. Tools/ holds the kit
/// source and specs; tt/ holds the tabtarget sprues. Historical record (Memos/,
/// retired heats) lies outside these roots and is spared by construction.
pub(crate) const ZRBTCHN_SCAN_ROOTS: &[&str] = &[
    "Tools",
    "tt", //
];

// ── Eviction row and keep-context ───────────────────────────

/// One way an evicted stem is permitted to survive a scan.
pub(crate) enum zrbtchn_KeepContext {
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
pub(crate) struct zrbtchn_EvictionRow {
    pub(crate) kill_stem: &'static str,
    pub(crate) keep_contexts: &'static [zrbtchn_KeepContext],
}

/// The standing eviction table — the single place a retired term gains its
/// assertion (add a row here, never improvise a grep). Empty until a heat
/// retires a term behind its own cutover; each row engages the live corpus scan
/// the moment it is added. The self-test cases prove the engine independent of
/// this table, so the mechanism ships proven with zero production rows.
pub(crate) const ZRBTCHN_EVICTION_ROWS: &[zrbtchn_EvictionRow] = &[];

/// One surviving occurrence of an evicted stem: where it is and the identifier
/// token that carries it.
pub(crate) struct zrbtchn_Hit {
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) token: String,
    pub(crate) line_text: String,
}

// ── Matcher (pure — the self-proven heart) ──────────────────

/// Split `text` into maximal identifier tokens — runs of `[A-Za-z0-9_]`. This is
/// the identifier-boundary awareness: a stem is judged by the whole token that
/// carries it, not by raw character adjacency.
fn zrbtchn_tokens(text: &str) -> Vec<String> {
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
fn zrbtchn_scan_text(
    rel_path: &str,
    line: usize,
    text: &str,
    row: &zrbtchn_EvictionRow,
    hits: &mut Vec<zrbtchn_Hit>,
) {
    for token in zrbtchn_tokens(text) {
        if !token.contains(row.kill_stem) {
            continue;
        }
        let cleared = row.keep_contexts.iter().any(|keep| match keep {
            zrbtchn_KeepContext::Identifier(id) => token.as_str() == *id,
            zrbtchn_KeepContext::PathPrefix(_) => false,
        });
        if cleared {
            continue;
        }
        hits.push(zrbtchn_Hit {
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
fn zrbtchn_match(rel_path: &str, content: &str, row: &zrbtchn_EvictionRow) -> Vec<zrbtchn_Hit> {
    let mut hits: Vec<zrbtchn_Hit> = Vec::new();

    for keep in row.keep_contexts {
        if let zrbtchn_KeepContext::PathPrefix(prefix) = keep {
            if rel_path.starts_with(prefix) {
                return hits;
            }
        }
    }

    let basename = rel_path.rsplit('/').next().unwrap_or(rel_path);
    zrbtchn_scan_text(rel_path, 0, basename, row, &mut hits);

    for (idx, line) in content.lines().enumerate() {
        zrbtchn_scan_text(rel_path, idx + 1, line, row, &mut hits);
    }

    hits
}

// ── Live-tree walk ──────────────────────────────────────────

/// Recursively collect every file under `dir` into `out`.
fn zrbtchn_walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            zrbtchn_walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Walk the scan roots under `root` and return every surviving occurrence of any
/// eviction row's kill-stem. Returns empty immediately when no rows are declared
/// — the standing engine costs nothing until a term is retired into it.
fn zrbtchn_scan(root: &Path, rows: &[zrbtchn_EvictionRow]) -> Result<Vec<zrbtchn_Hit>, String> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let mut files: Vec<PathBuf> = Vec::new();
    for sub in ZRBTCHN_SCAN_ROOTS {
        zrbtchn_walk(&root.join(sub), &mut files);
    }
    files.sort();

    let mut hits: Vec<zrbtchn_Hit> = Vec::new();
    for path in &files {
        // Non-UTF8 / unreadable files (binaries) hold no vocabulary — skip.
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let rel = crate::rbtcep_platform::rbtcep_repo_rel(root, path);
        for row in rows {
            hits.extend(zrbtchn_match(&rel, &content, row));
        }
    }
    Ok(hits)
}

/// Render hits as a stable one-per-line report.
fn zrbtchn_render(hits: &[zrbtchn_Hit]) -> String {
    let mut report = String::new();
    for h in hits {
        report.push_str(&format!(
            "{}:{}: {} — {}\n",
            h.path,
            h.line,
            h.token,
            h.line_text.trim(), //
        ));
    }
    report
}

// ── Curl containment scan ───────────────────────────────────
//
// The band placement doctrine (bubc_constants.sh
// placement comment) bars handing a curl exit status to the buc_die_now membrane:
// curl 8.6.0+ mints exit codes 100/101 inside the band window, so a bare
// `curl ... || buc_die_now` chain — or a bare curl whose failure propagates under
// set -e — masquerades as an in-band deliberate rejection at the dispatch
// boundary. Rather than hunt bad shapes, this scan legislates the one canonical
// form and errors on everything else (operator ruling 260704):
//
//   - a curl invocation must LEAD its line (after comment skip + quote strip);
//   - its terminator line must end with the byte-exact capture
//     `|| z_curl_status=$?` — the forced variable name is the point: the scan
//     verifies the classify step mechanically, no interpretation required;
//   - the captured z_curl_status must reappear within
//     ZRBTCHN_CURL_TEST_WINDOW lines of the terminator (the classify/test;
//     reappearance is checked on RAW lines because the var rides inside the
//     quoted test operand `"${z_curl_status}"`);
//   - two non-leading forms are admitted, each where a secret must stay in
//     memory so curl cannot lead, and each gated on the zSECRET segment: the
//     secret pipe into curl, where a secret reaches curl's stdin,
//     `«name»=$(printf '%s' "${zSECRET_«x»}" | curl ...) || z_curl_status=$?`,
//     its printf operand a zSECRET-named expansion read on the RAW line, since
//     the strip blanks it (operator rulings 261022, 261023); and the secret
//     capture, where a request carrying no secret is answered with one,
//     `zSECRET_«name»=$(curl ...) || z_curl_status=$?`, the assigned name
//     wearing the segment (operator ruling 261023). Each stands whole on one
//     line with the same byte-exact capture and the same window test; a pipe
//     whose operand, or a capture whose name, wears no segment is misplaced;
//   - a curl token anywhere else on a line is a misplaced-curl violation —
//     sole carve-out `command -v curl`, the presence probe whose `||` hands
//     command -v's exit onward, never curl's;
//   - a line whose quoting the line-local paired strip cannot resolve, when it
//     also carries a curl token, is itself a violation — the scan demands
//     lines simple enough to scan instead of growing bash-quoting
//     sophistication.

/// Repo-relative path prefixes exempt from the curl containment scan: the
/// in-pool cloud-step trees whose curl discipline is its own dialect (`-f`
/// flags, `|| exit N`, no buc_die_now) — the band membrane does not exist there.
const ZRBTCHN_CURL_EXEMPT_PREFIXES: &[&str] = &["Tools/rbk/rbgj"];

/// The byte-exact terminator suffix every curl invocation must carry.
const ZRBTCHN_CURL_CAPTURE: &str = "|| z_curl_status=$?";

/// Lines after the terminator within which the captured z_curl_status must
/// reappear. The bound leaves headroom over the corpus's longest span,
/// which is the retry loop in rbuh_http's request helper.
const ZRBTCHN_CURL_TEST_WINDOW: usize = 24;

/// One curl containment violation: where and which rule.
struct zrbtchn_CurlHit {
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
fn zrbtchn_strip_quoted(line: &str) -> (String, bool) {
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
fn zrbtchn_curl_leads(stripped: &str) -> bool {
    let t = stripped.trim_start();
    match t.strip_prefix("curl") {
        Some(rest) => rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t'),
        None => false,
    }
}

/// True when the stripped text carries a bare `curl` token.
fn zrbtchn_curl_token(stripped: &str) -> bool {
    zrbtchn_tokens(stripped).iter().any(|t| t == "curl")
}

/// Scan one shell file's contents for curl containment violations. A state
/// machine per line: comment lines are skipped; a leading-curl invocation is
/// walked through its backslash continuations to the terminator, which must end
/// with the canonical capture, followed by a z_curl_status reappearance within
/// the window; any other line carrying a curl token is misplaced (carve-out:
/// `command -v curl`) or unscannable (unpaired quote).
fn zrbtchn_curl_match(rel_path: &str, content: &str) -> Vec<zrbtchn_CurlHit> {
    let lines: Vec<&str> = content.lines().collect();
    let mut hits: Vec<zrbtchn_CurlHit> = Vec::new();
    let hit = |line: usize, kind: &'static str, text: &str| zrbtchn_CurlHit {
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
        let (stripped, unpaired) = zrbtchn_strip_quoted(raw);
        if !zrbtchn_curl_token(&stripped) {
            i += 1;
            continue;
        }
        if unpaired {
            hits.push(hit(i + 1, "unscannable", raw));
            i += 1;
            continue;
        }
        // The two secret admissions: one assignment of a substitution ending in
        // the byte-exact capture, then held to the same window test. The secret
        // pipe's body is `printf '%s'` of a zSECRET-named expansion piped into a
        // leading `curl` and nothing further; the secret capture's body is a
        // leading `curl` and nothing further, assigned to a zSECRET-named name.
        let wears_secret = |name: &str| name.starts_with("zSECRET_") || name.contains("_zSECRET_");
        let assignment = stripped
            .trim()
            .split_once("=$(")
            .filter(|(name, _)| {
                !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            })
            .and_then(|(name, rest)| {
                rest.strip_suffix(ZRBTCHN_CURL_CAPTURE)
                    .map(|body| (name, body))
            })
            .and_then(|(name, body)| body.trim_end().strip_suffix(')').map(|body| (name, body)));
        let secret_operand = raw
            .split_once("=$(")
            .and_then(|(_, rest)| rest.trim_start().strip_prefix("printf '%s' \"${"))
            .and_then(|rest| rest.split_once("}\""))
            .is_some_and(|(name, after)| wears_secret(name) && after.trim_start().starts_with('|'));
        let secret_pipe = assignment
            .and_then(|(_, body)| body.split_once('|'))
            .is_some_and(|(left, right)| {
                left.trim() == "printf"
                    && secret_operand
                    && !right.contains(['|', ';', '&', '(', ')'])
                    && zrbtchn_curl_leads(right)
            });
        let secret_capture = assignment.is_some_and(|(name, body)| {
            wears_secret(name)
                && !body.contains(['|', ';', '&', '(', ')'])
                && zrbtchn_curl_leads(body)
        });
        if secret_pipe || secret_capture {
            let window_end = lines.len().min(i + 1 + ZRBTCHN_CURL_TEST_WINDOW);
            let tested = lines[(i + 1)..window_end]
                .iter()
                .any(|l| zrbtchn_tokens(l).iter().any(|tok| tok == "z_curl_status"));
            if !tested {
                hits.push(hit(i + 1, "untested", raw));
            }
            i += 1;
            continue;
        }
        if !zrbtchn_curl_leads(&stripped) {
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
        let (term_stripped, term_unpaired) = zrbtchn_strip_quoted(lines[t]);
        if term_unpaired {
            hits.push(hit(t + 1, "unscannable", lines[t]));
        } else if !term_stripped.trim_end().ends_with(ZRBTCHN_CURL_CAPTURE) {
            hits.push(hit(t + 1, "uncaptured", lines[t]));
        } else {
            let window_end = lines.len().min(t + 1 + ZRBTCHN_CURL_TEST_WINDOW);
            let tested = lines[(t + 1)..window_end]
                .iter()
                .any(|l| zrbtchn_tokens(l).iter().any(|tok| tok == "z_curl_status"));
            if !tested {
                hits.push(hit(t + 1, "untested", lines[t]));
            }
        }
        i = t + 1;
    }
    hits
}

/// Render curl hits as a stable one-per-line report.
fn zrbtchn_curl_render(hits: &[zrbtchn_CurlHit]) -> String {
    let mut report = String::new();
    for h in hits {
        report.push_str(&format!(
            "{}:{}: {} — {}\n",
            h.path,
            h.line,
            h.kind,
            h.text, //
        ));
    }
    report
}

/// The canonical form clears: reset, leading curl through continuations,
/// byte-exact capture, test within the window — plus the sanctioned
/// non-leading shapes (the secret pipe of a zSECRET-named operand, the secret
/// capture into a zSECRET-named local, the `command -v curl` probe, quoted
/// message text, comment lines).
fn rbtchn_self_curl_canon_clears(_dir: &Path) -> rbtcee_Verdict {
    let content = concat!(
        "z_curl_status=0\n",
        "zSECRET_response=$(printf '%s' \"${zSECRET_body}\" | curl -sS -d @- \"${z_url}\") || z_curl_status=$?\n",
        "test \"${z_curl_status}\" -eq 0 || return 1\n",
        "z_code=$(printf '%s' \"${zSECRET_headers}\" | curl -sS -H @- -o \"${z_file}\" -w '%{http_code}' \"${z_url}\") || z_curl_status=$?\n",
        "test \"${z_curl_status}\" -eq 0 || return 1\n",
        "zSECRET_grant=$(curl -sS -X POST --data-urlencode \"scope=openid\" \"${z_url}\") || z_curl_status=$?\n",
        "test \"${z_curl_status}\" -eq 0 || return 1\n",
        "curl -sS --max-time 5 \\\n",
        "  -o \"${z_file}\" \\\n",
        "  \"${z_url}\" > \"${z_code}\" || z_curl_status=$?\n",
        "test \"${z_curl_status}\" -eq 0 || buc_die_now \"probe failed (curl exit ${z_curl_status})\"\n",
        "command -v curl >/dev/null 2>&1 || buc_die_now \"curl not found\"\n",
        "# a bare curl ... || buc_die_now chain in a comment is ignored\n",
        "buc_log_args \"retrying (curl exit ${z_curl_status})\"\n", //
    );
    let hits = zrbtchn_curl_match("Tools/rbk/probe.sh", content);
    if hits.is_empty() {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "canonical form flagged — {} false positive(s):\n{}",
            hits.len(),
            zrbtchn_curl_render(&hits), //
        ))
    }
}

/// Each deviant shape is caught with its kind: the bare buc_die_now chain, the
/// capture-free invocation, the non-leading invocations (capture-assignment,
/// pipe, a secret pipe without the capture and one piped on past curl, curl
/// captured into an unmarked name, and a secret pipe whose printf operand
/// wears no segment), and the
/// captured-but-never-tested window miss, leading and secret-piped alike.
fn rbtchn_self_curl_catches_deviants(_dir: &Path) -> rbtcee_Verdict {
    let filler = "buc_log_args unrelated\n".repeat(ZRBTCHN_CURL_TEST_WINDOW);
    let content = format!(
        concat!(
            "curl -s \\\n",
            "  \"${{z_url}}\" || buc_die_now \"HEAD failed\"\n",
            "curl -s \"${{z_url}}\" > \"${{z_file}}\"\n",
            "z_code=$(curl -s -w '%{{http_code}}' \"${{z_url}}\")\n",
            "echo \"${{z_body}}\" | curl -s -d @- \"${{z_url}}\"\n",
            "zSECRET_r=$(printf '%s' \"${{zSECRET_b}}\" | curl -s -d @- \"${{z_url}}\") || buc_die_now \"post failed\"\n",
            "zSECRET_r=$(printf '%s' \"${{zSECRET_b}}\" | curl -s -d @- \"${{z_url}}\" | jq .) || z_curl_status=$?\n",
            "z_json=$(curl -s \"${{z_url}}\") || z_curl_status=$?\n",
            "zSECRET_r=$(printf '%s' \"${{z_b}}\" | curl -s -d @- \"${{z_url}}\") || z_curl_status=$?\n",
            "curl -s \"${{z_url}}\" || z_curl_status=$?\n",
            "{}",
            "zSECRET_r=$(printf '%s' \"${{zSECRET_b}}\" | curl -s -d @- \"${{z_url}}\") || z_curl_status=$?\n", //
        ),
        filler, //
    );
    let hits = zrbtchn_curl_match("Tools/rbk/probe.sh", &content);
    let kinds: Vec<&str> = hits.iter().map(|h| h.kind).collect();
    let expected = [
        "uncaptured", // bare buc_die_now chain
        "uncaptured", // no capture at all
        "misplaced",  // curl inside a capture-assignment
        "misplaced",  // curl fed by an echo pipe
        "misplaced",  // secret pipe without the capture
        "misplaced",  // secret pipe piped on past curl
        "misplaced",  // curl captured into an unmarked name
        "misplaced",  // secret pipe whose printf operand wears no segment
        "untested",   // leading curl, status never read
        "untested",   // secret pipe, status never read
    ];
    if kinds != expected {
        return rbtcee_Verdict::Fail(format!(
            "deviant shapes misjudged — expected kinds {:?}, got:\n{}",
            expected,
            zrbtchn_curl_render(&hits), //
        ));
    }
    rbtcee_Verdict::Pass
}

/// A line the paired strip cannot resolve, carrying a curl token, is itself a
/// violation — the scan demands scannable lines rather than parsing abstruse
/// quoting.
fn rbtchn_self_curl_unscannable(_dir: &Path) -> rbtcee_Verdict {
    let content = "z_json=' | curl -s -d @- \"${z_url}\"\n";
    let hits = zrbtchn_curl_match("Tools/rbk/probe.sh", content);
    if hits.len() != 1 || hits[0].kind != "unscannable" {
        return rbtcee_Verdict::Fail(format!(
            "expected exactly 1 unscannable hit, got {}:\n{}",
            hits.len(),
            zrbtchn_curl_render(&hits), //
        ));
    }
    rbtcee_Verdict::Pass
}

/// Live-tree curl containment scan: every .sh file under the scan roots (minus
/// the exempt prefixes) must hold only canonical-form curl invocations.
fn rbtchn_curl_containment(dir: &Path) -> rbtcee_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!(
                "cannot get cwd: {}",
                e, //
            ));
        }
    };
    let mut files: Vec<PathBuf> = Vec::new();
    for sub in ZRBTCHN_SCAN_ROOTS {
        zrbtchn_walk(&root.join(sub), &mut files);
    }
    files.sort();

    let mut hits: Vec<zrbtchn_CurlHit> = Vec::new();
    for path in &files {
        if path.extension().and_then(|e| e.to_str()) != Some("sh") {
            continue;
        }
        let rel = crate::rbtcep_platform::rbtcep_repo_rel(&root, path);
        if ZRBTCHN_CURL_EXEMPT_PREFIXES
            .iter()
            .any(|p| rel.starts_with(p))
        {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        hits.extend(zrbtchn_curl_match(&rel, &content));
    }

    let report = zrbtchn_curl_render(&hits);
    let _ = std::fs::write(dir.join("conformance-curl.txt"), &report);

    if hits.is_empty() {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "{} curl containment violation(s) in the live tree:\n{}",
            hits.len(),
            report, //
        ))
    }
}

// ── A8 source-residue backstop ──────────────────────────────
//
// The mechanically-checkable slice of the one-home discipline that reads the
// shipped tree alone: a shipped `.sh` file cites no spec acronym. A
// between-audits guard, not a keystone, pure over an in-memory
// (rel_path, content) corpus — hermetic and self-proven like the
// eviction/curl checkers above — plus a live-tree wrapper.

/// One one-home-discipline finding: where, what kind, and the detail prose.
struct zrbtchn_OneHomeHit {
    path: String,
    line: usize,
    kind: &'static str,
    detail: String,
}

fn zrbtchn_onehome_render(hits: &[zrbtchn_OneHomeHit]) -> String {
    let mut report = String::new();
    for h in hits {
        report.push_str(&format!(
            "{}:{}: {} — {}\n",
            h.path,
            h.line,
            h.kind,
            h.detail, //
        ));
    }
    report
}

/// True when `token` has the shipped spec-acronym shape: `RBS` followed by one
/// or more uppercase-letter/digit characters. A8
/// source-residue bars this shape from shipped `.sh` files — only a bare
/// `RBr_` rivet ID or nothing may cite a spec from source.
fn zrbtchn_is_spec_acronym(token: &str) -> bool {
    match token.strip_prefix("RBS") {
        Some(rest) => {
            !rest.is_empty()
                && rest
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        }
        None => false,
    }
}

/// A8 source-residue: a shipped `.sh` file carries no spec-acronym
/// token — only a bare `RBr_` rivet ID or nothing. The
/// veiled tree reaches this check by no route: it stands at the repo root,
/// outside every scan root, so its residue rules are out of scope without a
/// caller having to exclude anything.
fn zrbtchn_check_a8_residue(sh_files: &[(&str, &str)]) -> Vec<zrbtchn_OneHomeHit> {
    let mut hits = Vec::new();
    for (path, content) in sh_files {
        for (idx, line) in content.lines().enumerate() {
            for token in zrbtchn_tokens(line) {
                if zrbtchn_is_spec_acronym(&token) {
                    hits.push(zrbtchn_OneHomeHit {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "a8-spec-acronym-residue",
                        detail: format!("shipped source cites spec acronym {} — residue must be bare RBr_ or nothing", token, //
                        ),
                    });
                }
            }
        }
    }
    hits
}

/// Scan roots for the A8 residue check — narrower than `ZRBTCHN_SCAN_ROOTS`:
/// RBK's own kit tree plus the shared tabtarget sprues, not the whole `Tools/`
/// corpus. RBK's tree is this fixture's jurisdiction; the other kits answer
/// for their own.
const ZRBTCHN_ONEHOME_SCAN_ROOTS: &[&str] = &[
    "Tools/rbk",
    "tt", //
];

/// Walk the A8 scan roots under `root`, returning the shipped `.sh` corpus as
/// (repo-relative path, content). The veiled tree needs no exclusion here: it
/// stands at the repo root, outside every scan root, so its residue rules are
/// out of scope by construction.
fn zrbtchn_walk_repo_corpus(root: &Path) -> Vec<(String, String)> {
    let mut files: Vec<PathBuf> = Vec::new();
    for sub in ZRBTCHN_ONEHOME_SCAN_ROOTS {
        zrbtchn_walk(&root.join(sub), &mut files);
    }
    files.sort();

    let mut sh_owned: Vec<(String, String)> = Vec::new();
    for path in &files {
        if path.extension().and_then(|e| e.to_str()) != Some("sh") {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let rel = crate::rbtcep_platform::rbtcep_repo_rel(root, path);
        sh_owned.push((rel, content));
    }
    sh_owned
}

/// Live case — A8 source residue. Walks the scan roots for the shipped `.sh`
/// corpus and bars a spec acronym in it. It reads nothing outside the work
/// tree, so it runs identically on every station.
fn rbtchn_onehome_residue_live(dir: &Path) -> rbtcee_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!(
                "cannot get cwd: {}",
                e, //
            ));
        }
    };
    let sh_owned = zrbtchn_walk_repo_corpus(&root);
    let sh_refs: Vec<(&str, &str)> = sh_owned
        .iter()
        .map(|(p, c)| (p.as_str(), c.as_str()))
        .collect();

    let hits = zrbtchn_check_a8_residue(&sh_refs);
    let mut report = zrbtchn_onehome_render(&hits);
    report.push_str(&format!(
        "scanned {} shipped .sh file(s)\n",
        sh_owned.len(), //
    ));
    let _ = std::fs::write(dir.join("conformance-onehome-residue.txt"), &report);

    if hits.is_empty() {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "{} A8 source-residue violation(s) in the live tree:\n{}",
            hits.len(),
            report, //
        ))
    }
}

// ── A8 self-test — the check proves itself ──────────────────

/// A8 source-residue: a bare `RBr_` residue clears; a spec-acronym citation
/// (`RBSZZZ`, a made-up specimen) is flagged.
fn rbtchn_self_a8_residue(_dir: &Path) -> rbtcee_Verdict {
    let sh = vec![(
        "Tools/rbk/probe.sh",
        "# clean residue cites RBr_a11 only\nz_x=1\n# violation cites RBSZZZ directly\n",
    )];
    let hits = zrbtchn_check_a8_residue(&sh);
    if hits.len() != 1 || hits[0].kind != "a8-spec-acronym-residue" || hits[0].line != 3 {
        return rbtcee_Verdict::Fail(format!(
            "expected exactly 1 a8-spec-acronym-residue hit at line 3, got:\n{}",
            zrbtchn_onehome_render(&hits), //
        ));
    }
    rbtcee_Verdict::Pass
}

// ── Self-test cases — the checker proves itself ─────────────

/// A synthetic stem that appears in no real source — the self-tests run the
/// matcher against in-memory corpora only, never the live tree, so the proof is
/// hermetic and plants nothing in the repo.
const ZRBTCHN_SELF_STEM: &str = "zzdeadstem";

/// A sanctioned identifier that embeds ZRBTCHN_SELF_STEM by construction — the
/// "stem inside a kept identifier" case. (The enum holds `&'static str`, so this
/// coupling is a literal, not a derivation.)
const ZRBTCHN_SELF_KEPT_ID: &str = "rbtchn_zzdeadstem_kept";

/// One corpus, both halves of the discrimination: a bare stem that MUST be
/// caught and the same stem inside a kept identifier that MUST be respected.
fn rbtchn_self_catch_and_keep_identifier(_dir: &Path) -> rbtcee_Verdict {
    let row = zrbtchn_EvictionRow {
        kill_stem: ZRBTCHN_SELF_STEM,
        keep_contexts: &[zrbtchn_KeepContext::Identifier(ZRBTCHN_SELF_KEPT_ID)],
    };
    let content = "a bare zzdeadstem here\nlet x = rbtchn_zzdeadstem_kept();\n";
    let hits = zrbtchn_match("Tools/rbk/probe_selftest.txt", content, &row);

    if hits.len() != 1 {
        return rbtcee_Verdict::Fail(format!(
            "expected exactly 1 hit (the bare stem; the kept identifier respected), got {}:\n{}",
            hits.len(),
            zrbtchn_render(&hits), //
        ));
    }
    let h = &hits[0];
    if h.line != 1 || h.token != ZRBTCHN_SELF_STEM {
        return rbtcee_Verdict::Fail(format!(
            "hit shape wrong: line {} token '{}' (expected line 1 token '{}')",
            h.line,
            h.token,
            ZRBTCHN_SELF_STEM, //
        ));
    }
    rbtcee_Verdict::Pass
}

/// A path-prefix keep exempts the whole file — the stem may stay under an
/// in-flight / vendored path a cutover deliberately leaves untouched.
fn rbtchn_self_keep_path_prefix(_dir: &Path) -> rbtcee_Verdict {
    let row = zrbtchn_EvictionRow {
        kill_stem: ZRBTCHN_SELF_STEM,
        keep_contexts: &[zrbtchn_KeepContext::PathPrefix("Tools/rbk/vendor/legacy/")],
    };
    let content = "zzdeadstem all over\nanother zzdeadstem line\n";
    let hits = zrbtchn_match("Tools/rbk/vendor/legacy/old.adoc", content, &row);

    if hits.is_empty() {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "path-prefix keep not respected — {} hit(s) under an exempt path:\n{}",
            hits.len(),
            zrbtchn_render(&hits), //
        ))
    }
}

/// A stem in a tabtarget filename (a sprue) MUST be caught — boundary awareness
/// reaches the basename, not just the contents.
fn rbtchn_self_catch_sprue(_dir: &Path) -> rbtcee_Verdict {
    let row = zrbtchn_EvictionRow {
        kill_stem: ZRBTCHN_SELF_STEM,
        keep_contexts: &[],
    };
    let hits = zrbtchn_match("tt/rbw-zzdeadstem.Run.sh", "", &row);

    if hits.len() != 1 || hits[0].line != 0 || hits[0].token != ZRBTCHN_SELF_STEM {
        return rbtcee_Verdict::Fail(format!(
            "expected exactly 1 sprue hit at line 0 token '{}', got {}:\n{}",
            ZRBTCHN_SELF_STEM,
            hits.len(),
            zrbtchn_render(&hits), //
        ));
    }
    rbtcee_Verdict::Pass
}

/// A pure corpse (empty keep_contexts) flags a bare use, and the engine reports
/// nothing where the stem is absent — no false positives.
fn rbtchn_self_pure_corpse(_dir: &Path) -> rbtcee_Verdict {
    let row = zrbtchn_EvictionRow {
        kill_stem: ZRBTCHN_SELF_STEM,
        keep_contexts: &[],
    };

    let present = zrbtchn_match("Tools/rbk/x.md", "x zzdeadstem y\n", &row);
    if present.len() != 1 {
        return rbtcee_Verdict::Fail(format!(
            "pure corpse: expected 1 hit on a bare use, got {}:\n{}",
            present.len(),
            zrbtchn_render(&present), //
        ));
    }

    let absent = zrbtchn_match("Tools/rbk/x.md", "no dead vocabulary on this line\n", &row);
    if !absent.is_empty() {
        return rbtcee_Verdict::Fail(format!(
            "pure corpse: expected 0 hits where the stem is absent, got {}:\n{}",
            absent.len(),
            zrbtchn_render(&absent), //
        ));
    }
    rbtcee_Verdict::Pass
}

/// Live-tree scan against the standing eviction table. With zero production rows
/// this passes by construction; the instant a row is added it engages the walker
/// over Tools/ and tt/ and fails on any surviving occurrence.
fn rbtchn_live_scan(dir: &Path) -> rbtcee_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!(
                "cannot get cwd: {}",
                e, //
            ));
        }
    };
    let hits = match zrbtchn_scan(&root, ZRBTCHN_EVICTION_ROWS) {
        Ok(h) => h,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    let report = zrbtchn_render(&hits);
    let _ = std::fs::write(dir.join("conformance-evictions.txt"), &report);

    if hits.is_empty() {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "{} evicted-term survival(s) in the live tree:\n{}",
            hits.len(),
            report, //
        ))
    }
}

// ── Cases and fixture ───────────────────────────────────────

pub static RBTCHN_CASES_CONFORMANCE: &[rbtcee_Case] = &[
    rbtcee_case!(rbtchn_self_catch_and_keep_identifier),
    rbtcee_case!(rbtchn_self_keep_path_prefix),
    rbtcee_case!(rbtchn_self_catch_sprue),
    rbtcee_case!(rbtchn_self_pure_corpse),
    rbtcee_case!(rbtchn_live_scan),
    rbtcee_case!(rbtchn_self_curl_canon_clears),
    rbtcee_case!(rbtchn_self_curl_catches_deviants),
    rbtcee_case!(rbtchn_self_curl_unscannable),
    rbtcee_case!(rbtchn_curl_containment),
    rbtcee_case!(rbtchn_self_a8_residue),
    rbtcee_case!(rbtchn_onehome_residue_live), //
];

pub static RBTCHN_FIXTURE_CONFORMANCE: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CONFORMANCE,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHN_CASES_CONFORMANCE,
    credless: true,
    tariff: rbtcee_Tariff {
        min_secs: None,
        max_secs: None,
        invocations: Some(0),
    },
};
