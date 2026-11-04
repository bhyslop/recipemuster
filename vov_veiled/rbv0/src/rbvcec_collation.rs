// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCEC — collation: the absorbed one-home corpus check.
//
// A collation sets each citation beside the source it claims and reads whether
// the two agree. This module collates RBK's corpus: every mapping-declared quoin
// and every `{term}` or `RBr_xxx` citation must resolve to a real `[[anchor]]`
// definition site, and a rivet cited from a second sheaf must have hoisted to
// the codex. It is the mechanically-checkable slice of the one-home discipline,
// a between-audits guard rather than a keystone: paraphrase detection, the word
// cancer the manual audits hunt, is irreducibly semantic and stays out of scope.
//
// This module absorbs, in-process, the corpus-bound half of what used to ride
// as the theurge `conformance` fixture in shipped code (RBSHC "Worker, never
// authority": the one-home check joins the veil assay, the cut and the temenos
// as one of the hierophant's own absorbed modules). Moving it here is what
// makes the shipped tree carry no kraal-register reader and no internal spec
// name: the register parse, the corpus-road locator, the citation-integrity and
// rivet-hoist checkers with their codex and index constants all live behind the
// veil now. The A8 source-residue check, which reads the shipped tree alone,
// stays in the theurge fixture.
//
// One seat: `rbvcec_assay` runs PRE-CUT on the maintainer tree (RBSHE step 2),
// beside the veil assay. Its reach is the kraal it stands in. The `.adoc`
// anchor corpus lives outside the repository, and WHERE outside is a
// per-session fact rather than a layout constant: a session dispatched onto a
// pace stands in a kraal, an outspan and its work trees standing as siblings,
// one of them the corpus road. The outspan holds `feodary.json`, the kraal's
// register of holdings, naming each tree with its path and its role. That
// register is the single resolution source.
//
// Two rules bind the resolution, and both are load-bearing rather than
// cautious. The register is READ, never inferred from a directory-name
// convention: a convention resolves to a plausible WRONG tree as silently as a
// dead path resolves to no tree. And the three outcomes stay apart: no register
// claiming the tree is an absence the lap logs and proceeds past, the locator's
// own self-proof carrying the trust that absence is honest; a register that
// claims the tree and cannot be followed is a broken kraal, fatal; and a check
// that ran and found anything is fatal.

use std::collections::{
    HashMap,
    HashSet, //
};
use std::path::{
    Path,
    PathBuf, //
};

// ── Finding ─────────────────────────────────────────────────

/// One one-home finding: where, what kind, and the detail prose.
#[derive(Clone, Debug)]
pub(crate) struct zrbvcec_Finding {
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) kind: &'static str,
    pub(crate) detail: String,
}

/// Render findings as stable one-per-line strings.
pub(crate) fn zrbvcec_render(findings: &[zrbvcec_Finding]) -> Vec<String> {
    findings
        .iter()
        .map(|f| {
            format!(
                "{}:{}: {} — {}",
                f.path,
                f.line,
                f.kind,
                f.detail, //
            )
        })
        .collect()
}

// ── Parsers (pure — the self-proven heart) ──────────────────

/// Split `text` into maximal identifier tokens — runs of `[A-Za-z0-9_]`, so a
/// rivet ID is judged by the whole token that carries it.
fn zrbvcec_tokens(text: &str) -> Vec<String> {
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

/// True when `line`, after stripping a leading AsciiDoc list-bullet marker
/// (`*`, `**`, …) if present, OPENS with an anchor `[[name]]` or
/// `[[name,display]]` — the shared definition-site syntax for both quoins and
/// rivets. Trailing content after the anchor is ignored (the
/// `[[jjdpe_defined]] {jjdpe_defined}:: ...` same-line form); a bullet prefix
/// is tolerated (the `* [[jjdgm_order]]` list-item form). Returns the bare
/// name.
fn zrbvcec_parse_anchor(line: &str) -> Option<String> {
    let t = line.trim();
    let t = t.trim_start_matches('*').trim_start();
    let rest = t.strip_prefix("[[")?;
    let end = rest.find("]]")?;
    let inner = &rest[..end];
    let name = inner.split(',').next().unwrap_or("").trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// True when `line` is a mapping-section attribute declaration
/// `:name:  <<target,Display Text>>` — the quoin-registration line.
/// Returns `(name, target)`. A variant (`name` = `axo_entity_s`) points its
/// `target` at the base quoin's anchor (`axo_entity`), never its own — the
/// `_s`/`_p`/`_ed`/`_ing` suffix mechanism, so `name` and `target` differ by
/// design and only `target` need resolve to an anchor.
fn zrbvcec_parse_quoin_mapping(line: &str) -> Option<(String, String)> {
    let t = line.trim_start();
    let rest = t.strip_prefix(':')?;
    let colon = rest.find(':')?;
    let name = &rest[..colon];
    if name.is_empty() || !name.chars().next()?.is_ascii_lowercase() {
        return None;
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
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
fn zrbvcec_parse_braced_citations(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '{' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '}') {
                let inner: String = chars[i + 1..i + 1 + end].iter().collect();
                let is_ident = !inner.is_empty()
                    && inner.chars().next().unwrap().is_ascii_lowercase()
                    && inner
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
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
/// convention itself (doctrine text), never a real citation, so it is
/// excluded by the length/shape requirement rather than by name.
fn zrbvcec_parse_rivet_tokens(line: &str) -> Vec<String> {
    zrbvcec_tokens(line)
        .into_iter()
        .filter(|t| match t.strip_prefix("RBr_") {
            Some(tail) => {
                tail.len() == 3
                    && tail
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            }
            None => false,
        })
        .collect()
}

/// The prefix segment of a snake_case identifier — everything before the
/// first underscore. Used to recognize a *known quoin family*: a prefix
/// carried by at least one mapping-declared name in the corpus.
fn zrbvcec_prefix_of(name: &str) -> &str {
    name.split('_').next().unwrap_or(name)
}

// ── Check 1: citation integrity ─────────────────────────────

/// Sheaves whose mapping-declared quoins are pre-registered ahead of their
/// defining prose — a foreign-kit drafting gap, outside this check's
/// jurisdiction to close. Each entry names its removal condition; delete the
/// entry once met. Empty: no sheaf in this estate's corpus holds such a gap.
const ZRBVCEC_UNANCHORED_QUOIN_EXEMPT_PATHS: &[&str] = &[];

/// Citation integrity: every mapping-declared quoin and every
/// `{term}`/`RBr_xxx` citation in the JUDGED set resolves to a real
/// `[[anchor]]` definition site somewhere in the ANCHOR set.
///
/// The two sets are distinct because definition and jurisdiction are distinct
/// questions. `anchor_files` is where a definition may lawfully live: the whole
/// spec corpus, every kit in it, since a sheaf borrowing a sibling kit's term
/// by declaring the mapping locally is ordinary practice and its anchor is
/// then simply elsewhere. `judged_adoc` and `judged_all` are the sites this
/// check rules on — RBK's own, its jurisdiction. Harvesting anchors only from
/// the judged set would read every lawful cross-kit borrow as a dangling
/// citation; judging every file the anchors were harvested from would drag
/// other kits' documents before an RBK checker.
///
/// `judged_all` covers both `.adoc` and `.sh` content (a rivet may be cited
/// from either); braced-citation and mapping scanning apply only to
/// `judged_adoc`, since `{term}` and `:term:` are AsciiDoc syntax.
/// `unanchored_exempt_paths` names sheaves whose pre-existing unanchored
/// mapping entries are a recorded, out-of-scope drafting gap rather than a
/// fresh violation.
///
/// A `{term}` citation is judged only when `term`'s prefix is a *known quoin
/// family* — at least one mapping-declared name carries that same prefix
/// somewhere in the corpus. AsciiDoc's `{...}` syntax collides with incidental
/// curly-brace prose (wire-format templates, shell-variable interpolation,
/// example error strings); restricting to established families is the
/// mechanical way to tell a real citation from that noise without parsing
/// quote/code-span context.
pub(crate) fn zrbvcec_check_citations(
    anchor_files: &[(&str, &str)],
    judged_adoc: &[(&str, &str)],
    judged_all: &[(&str, &str)],
    unanchored_exempt_paths: &[&str],
) -> Vec<zrbvcec_Finding> {
    let mut anchors: HashSet<String> = HashSet::new();
    let mut mapping: HashMap<String, String> = HashMap::new();
    for (_, content) in anchor_files {
        for line in content.lines() {
            if let Some(name) = zrbvcec_parse_anchor(line) {
                anchors.insert(name);
            }
            if let Some((name, target)) = zrbvcec_parse_quoin_mapping(line) {
                mapping.insert(name, target);
            }
        }
    }
    let known_families: HashSet<&str> = mapping.keys().map(|name| zrbvcec_prefix_of(name)).collect();

    let mut hits = Vec::new();

    for (path, content) in judged_adoc {
        if unanchored_exempt_paths.contains(path) {
            continue;
        }
        for (idx, line) in content.lines().enumerate() {
            if let Some((name, target)) = zrbvcec_parse_quoin_mapping(line) {
                if !anchors.contains(&target) {
                    hits.push(zrbvcec_Finding {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "unanchored-quoin",
                        detail: format!(
                            "mapping declares '{}' -> <<{}>> but no [[{}]] anchor exists",
                            name,
                            target,
                            target, //
                        ),
                    });
                }
            }
            for term in zrbvcec_parse_braced_citations(line) {
                if !known_families.contains(zrbvcec_prefix_of(&term)) {
                    continue;
                }
                let resolves = mapping
                    .get(&term)
                    .map_or(false, |target| anchors.contains(target));
                if !resolves {
                    hits.push(zrbvcec_Finding {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "broken-quoin-citation",
                        detail: format!(
                            "{{{}}} resolves to no declared-and-anchored quoin",
                            term, //
                        ),
                    });
                }
            }
        }
    }

    for (path, content) in judged_all {
        for (idx, line) in content.lines().enumerate() {
            if zrbvcec_parse_anchor(line).is_some() {
                continue;
            }
            for id in zrbvcec_parse_rivet_tokens(line) {
                if !anchors.contains(&id) {
                    hits.push(zrbvcec_Finding {
                        path: path.to_string(),
                        line: idx + 1,
                        kind: "broken-rivet-citation",
                        detail: format!(
                            "{} resolves to no [[{}]] anchor",
                            id,
                            id, //
                        ),
                    });
                }
            }
        }
    }

    hits
}

// ── Check 2: rivet-hoist placement ──────────────────────────

/// Rivets whose cross-sheaf citation is a deliberate, recorded hoist
/// deferral — never an unexamined exemption. Each entry names its removal
/// condition; delete the entry (and perform the hoist) once met.
///
/// - `RBr_m4d`: hoist deferred while the codex remains under active spec-doctrine
///   work. Remove this entry when RBr_m4d's definition prose is hoisted.
pub(crate) const ZRBVCEC_HOIST_WAIVERS: &[&str] = &["RBr_m4d"];

/// The codex sheaf itself — the hoist target. A rivet anchored here is
/// definitionally already hoisted, so citing it from any other sheaf is the
/// intended end-state, never a violation. The path form matches the
/// corpus-root-relative rendering the assay produces (see
/// `ZRBVCEC_CORPUS_SPEC_SUBDIR`), so it names the sheaf once whichever tree the
/// corpus stands in.
pub(crate) const ZRBVCEC_CODEX_SHEAF: &str = "specs/rbk/RBS0-SpecTop.adoc";

/// Sheaves whose role is a rivet *census/index* — pointing at where a rivet is
/// homed, never consuming its content as working vocabulary. The hoist law
/// guards against a sibling sheaf silently depending on another's interior; an
/// index entry is the opposite — a directory pointer whose whole job is to name
/// rivets without restating them. Hoisting on an index citation would drag
/// every catalogued rivet into the codex and destroy the census function, so a
/// sheaf named here is exempt as a *citer* (never as a definer).
///
/// - `RBSWB-Watchbill.adoc`: the indeterminacy-membrane census (`//axvd_sheaf
///   axd_normative`; doctrine text: "the watchbill points at the chapters and
///   never restates them"). Its citations of `RBr_7a9`/`RBr_3f4`/
///   `RBr_c81` are directory entries, not content consumption.
///
/// Corpus-root-relative path form, matching the assay's rendering.
pub(crate) const ZRBVCEC_HOIST_INDEX_SHEAVES: &[&str] = &["specs/rbk/RBSWB-Watchbill.adoc"];

/// Rivet-hoist placement (a rivet hoists to the codex the moment a second
/// sheaf cites it). A rivet anchored in one `.adoc` sheaf and cited by bare
/// token from a *different* `.adoc` sheaf should have hoisted; `waivers` names
/// the rivets whose deferral is recorded and conditioned rather than silently
/// exempted. A rivet anchored in `codex` is exempt outright — that IS the
/// hoisted state. A citation FROM a sheaf in `index_sheaves` never counts — the
/// citer's whole role is to point at content homed elsewhere, not to consume it.
pub(crate) fn zrbvcec_check_rivet_hoist(
    adoc_files: &[(&str, &str)],
    waivers: &[&str],
    codex: &str,
    index_sheaves: &[&str],
) -> Vec<zrbvcec_Finding> {
    let mut def_sheaf: HashMap<String, &str> = HashMap::new();
    for (path, content) in adoc_files {
        for line in content.lines() {
            if let Some(name) = zrbvcec_parse_anchor(line) {
                if name.starts_with("RBr_") {
                    def_sheaf.entry(name).or_insert(path);
                }
            }
        }
    }

    let mut hits = Vec::new();
    for (path, content) in adoc_files {
        if index_sheaves.contains(path) {
            continue;
        }
        for (idx, line) in content.lines().enumerate() {
            if zrbvcec_parse_anchor(line).is_some() {
                continue;
            }
            for id in zrbvcec_parse_rivet_tokens(line) {
                if waivers.contains(&id.as_str()) {
                    continue;
                }
                if let Some(&home) = def_sheaf.get(&id) {
                    if home != *path && home != codex {
                        hits.push(zrbvcec_Finding {
                            path: path.to_string(),
                            line: idx + 1,
                            kind: "unhoisted-rivet",
                            detail: format!(
                                "{} defined in {} cited cross-sheaf here — hoist to the codex",
                                id,
                                home, //
                            ),
                        });
                    }
                }
            }
        }
    }
    hits
}

// ── The kraal register and the corpus road ──────────────────

/// The kraal register's basename, at the outspan root.
pub(crate) const ZRBVCEC_FEODARY_BASENAME: &str = "feodary.json";

/// The roles a corpus road carries. A kraal holds at most one, and which one
/// follows from the door that stood the kraal up.
pub(crate) const ZRBVCEC_CORPUS_ROAD_ROLES: &[&str] = &[
    "vulgate",
    "lectern", //
];

/// RBK's spec home on the corpus road — the JUDGED set's root. Scoped to
/// `specs/rbk` (not the whole `specs/`) to hold the same RBK-only jurisdiction
/// the in-repo roots keep. Also the rendering root's tail: a corpus file renders
/// CORPUS-ROOT-relative whichever tree holds the corpus.
pub(crate) const ZRBVCEC_CORPUS_SPEC_SUBDIR: &str = "specs/rbk";

/// The whole spec corpus on the corpus road — the ANCHOR set's root, every kit
/// in it. Wider than the judged root above on purpose: a definition may
/// lawfully live in a sibling kit's sheaf, and a sheaf borrowing CMK's
/// `axw_wythe` by declaring the mapping locally is the standing case. Reading
/// anchors only from RBK's own subtree reports that borrow as a dangling
/// citation — a false positive that says the corpus is broken when it is the
/// checker's horizon that is short.
const ZRBVCEC_CORPUS_ANCHOR_SUBDIR: &str = "specs";

/// One tree row of the kraal register: where it stands, and what it is for.
pub(crate) struct zrbvcec_FeodaryTree {
    pub(crate) path: String,
    pub(crate) role: String,
}

/// What the locator found. `Found` carries the corpus road's ROOT (not its
/// spec subdirectory) because that root is also the rendering root — every
/// corpus path is rendered relative to it.
pub(crate) enum zrbvcec_CorpusRoad {
    Found(PathBuf),
    Absent(String),
}

/// Read the string value that follows `"key":` in `text`, unescaping the two
/// escapes a filesystem path can carry (`\\` and `\"`). None when the key is
/// absent or its value is not a string.
fn zrbvcec_json_string_field(text: &str, key: &str) -> Option<String> {
    let needle = format!(
        "\"{}\"",
        key, //
    );
    let pos = text.find(&needle)?;
    let rest = text[pos + needle.len()..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let rest = rest.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => out.push(chars.next()?),
            _ => out.push(c),
        }
    }
    None
}

/// Read the integer value that follows `"key":` in `text`. None when the key
/// is absent or its value is not a bare integer.
fn zrbvcec_json_int_field(text: &str, key: &str) -> Option<i64> {
    let needle = format!(
        "\"{}\"",
        key, //
    );
    let pos = text.find(&needle)?;
    let rest = text[pos + needle.len()..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit() && c != '-')
        .unwrap_or(rest.len());
    rest[..end].parse::<i64>().ok()
}

/// Parse the kraal register: `jjfd_revision`'s presence first, then one row
/// per object in `jjfd_trees`. This reader is the FOREIGN reader — it takes the
/// keys it needs, passes over the rest, and does not gate on the revision's
/// VALUE, only that the key stands (a document that cannot say what it is is
/// not a feodary). Pure over the document text — the walk that FINDS the
/// document is `zrbvcec_locate_corpus_road`. `Err` is a register this reader
/// must not act on; `Ok` with no rows is a well-formed register standing no
/// trees.
///
/// Deliberately not a general JSON parser: the subject is one dispatch-generated
/// document of known shape, and a scanner over its two declared fields is what
/// the checker can prove itself against (the self-proofs in `rbvtec_collation`).
/// A row missing either field is a refusal rather than a skipped row — a
/// register that half-parses is not one this reader may trust.
pub(crate) fn zrbvcec_parse_feodary(text: &str) -> Result<Vec<zrbvcec_FeodaryTree>, String> {
    if zrbvcec_json_int_field(text, "jjfd_revision").is_none() {
        return Err("declares no jjfd_revision".to_string());
    }
    let needle = "\"jjfd_trees\"";
    let pos = match text.find(needle) {
        Some(p) => p,
        None => return Err("declares no jjfd_trees".to_string()),
    };
    let rest = &text[pos + needle.len()..];
    let open = match rest.find('[') {
        Some(i) => i,
        None => return Err("declares a jjfd_trees that is not an array".to_string()),
    };
    let body = &rest[open..];
    let mut trees = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, c) in body.char_indices() {
        match c {
            '{' => {
                if depth == 0 {
                    start = i + 1;
                }
                depth += 1;
            }
            '}' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
                if depth == 0 {
                    let chunk = &body[start..i];
                    match (
                        zrbvcec_json_string_field(chunk, "jjfd_path"),
                        zrbvcec_json_string_field(chunk, "jjfd_role"),
                    ) {
                        (Some(path), Some(role)) => trees.push(zrbvcec_FeodaryTree { path, role }),
                        _ => {
                            return Err("carries a tree row without both jjfd_path and jjfd_role"
                                .to_string())
                        }
                    }
                }
            }
            ']' if depth == 0 => break,
            _ => {}
        }
    }
    Ok(trees)
}

/// Canonical form for path comparison, falling back to the path as given when
/// it does not resolve — a register naming a tree that no longer stands must
/// compare unequal rather than abort the walk.
pub(crate) fn zrbvcec_real_path(p: &Path) -> PathBuf {
    std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

/// Locate the corpus road for the tree at `root`, by finding the kraal register
/// that CLAIMS it. The kraal is flat — every peer is a sibling — so the search
/// is the root's own parent, one level, and a register is accepted only when
/// one of its tree rows resolves to this very root. The path match is what
/// makes the walk safe: nothing is inferred from a directory name, and a
/// register belonging to another kraal is passed over rather than
/// half-believed.
///
/// `Ok(Found)` — a register claims this tree and names a corpus road whose
/// `specs/rbk` stands. `Ok(Absent)` — no register claims it: a station outside
/// any kraal. `Err` — a register claims it and is then unusable: no corpus road
/// named, or a road that does not stand. That is a broken kraal, and failing
/// loud is the point.
pub(crate) fn zrbvcec_locate_corpus_road(root: &Path) -> Result<zrbvcec_CorpusRoad, String> {
    let me = zrbvcec_real_path(root);
    let parent = match root.parent() {
        Some(p) => p,
        None => {
            return Ok(zrbvcec_CorpusRoad::Absent(
                "the tree's root has no parent, so no kraal register can stand beside it"
                    .to_string(),
            ))
        }
    };

    let mut registers: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(parent) {
        for entry in entries.flatten() {
            let register = entry.path().join(ZRBVCEC_FEODARY_BASENAME);
            if register.is_file() {
                registers.push(register);
            }
        }
    }
    registers.sort();

    let mut unreadable: Vec<String> = Vec::new();
    for register in &registers {
        let text = match std::fs::read_to_string(register) {
            Ok(t) => t,
            Err(e) => {
                unreadable.push(format!(
                    "{} ({})",
                    register.display(),
                    e, //
                ));
                continue;
            }
        };
        let trees = match zrbvcec_parse_feodary(&text) {
            Ok(t) => t,
            Err(e) => {
                unreadable.push(format!(
                    "{} ({})",
                    register.display(),
                    e, //
                ));
                continue;
            }
        };
        if !trees
            .iter()
            .any(|t| zrbvcec_real_path(Path::new(&t.path)) == me)
        {
            continue;
        }

        // This register claims us. Past here every fault is loud: a kraal whose
        // corpus road cannot be resolved is broken, and reporting that as an
        // absence would restore exactly the silence the locator exists to remove.
        let road = match trees
            .iter()
            .find(|t| ZRBVCEC_CORPUS_ROAD_ROLES.contains(&t.role.as_str()))
        {
            Some(t) => t,
            None => {
                return Err(format!(
                    "{} claims this tree but names no corpus road (looked for role {}) — \
                     a kraal without one cannot answer the citation-integrity or rivet-hoist check",
                    register.display(),
                    ZRBVCEC_CORPUS_ROAD_ROLES.join(" or "), //
                ));
            }
        };
        let road_root = PathBuf::from(&road.path);
        let specs = road_root.join(ZRBVCEC_CORPUS_SPEC_SUBDIR);
        if !specs.is_dir() {
            return Err(format!(
                "{} names the corpus road at {} (role {}), but {} does not stand",
                register.display(),
                road.path,
                road.role,
                specs.display(), //
            ));
        }
        return Ok(zrbvcec_CorpusRoad::Found(road_root));
    }

    let mut reason = format!(
        "no kraal register ({}) beside {} claims this tree, so the lap holds no corpus road",
        ZRBVCEC_FEODARY_BASENAME,
        me.display(), //
    );
    if !unreadable.is_empty() {
        reason.push_str(&format!(
            "; registers passed over unread: {}",
            unreadable.join(", "), //
        ));
    }
    Ok(zrbvcec_CorpusRoad::Absent(reason))
}

// ── The in-repo corpus ──────────────────────────────────────

/// Repo-relative roots whose `.adoc` and `.sh` files join the check: RBK's own
/// kit tree plus the shared tabtarget sprues. Both join the anchor set and the
/// judged set, since a rivet may be cited from either. RBK's tree is this
/// check's jurisdiction; the other kits' foundational specs carry pre-existing
/// one-home violations in bulk — genuine corpus debt, not a design flaw in the
/// checks (the Linked Term law makes the anchor constitutive, so a dangling
/// `<<target>>` really is broken). Closing that debt belongs to each kit.
const ZRBVCEC_REPO_ROOTS: &[&str] = &[
    "Tools/rbk",
    "tt", //
];

/// Recursively collect every file under `dir` into `out`.
fn zrbvcec_walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            zrbvcec_walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Root-relative, forward-slash form of `path` under `root`.
fn zrbvcec_rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

/// Walk the in-repo roots under `root`, returning the `.adoc` corpus and the
/// `.sh` corpus, each as (repo-relative path, content). The veiled tree stands
/// at the repository root, outside every root walked, so nothing needs
/// excluding here.
fn zrbvcec_walk_repo(root: &Path) -> (Vec<(String, String)>, Vec<(String, String)>) {
    let mut files: Vec<PathBuf> = Vec::new();
    for sub in ZRBVCEC_REPO_ROOTS {
        zrbvcec_walk(&root.join(sub), &mut files);
    }
    files.sort();

    let mut adoc_owned: Vec<(String, String)> = Vec::new();
    let mut sh_owned: Vec<(String, String)> = Vec::new();
    for path in &files {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let rel = zrbvcec_rel(root, path);
        match path.extension().and_then(|e| e.to_str()) {
            Some("adoc") => adoc_owned.push((rel, content)),
            Some("sh") => sh_owned.push((rel, content)),
            _ => {}
        }
    }
    (adoc_owned, sh_owned)
}

fn zrbvcec_refs(owned: &[(String, String)]) -> Vec<(&str, &str)> {
    owned
        .iter()
        .map(|(p, c)| (p.as_str(), c.as_str()))
        .collect()
}

// ── The pre-cut rbvcec_assay (maintainer tree) ──────────────

/// What the one-home corpus check read. Three outcomes, kept apart because the
/// lap treats each differently.
pub enum rbvcec_Reading {
    /// No kraal register beside the tree claims it, so there is no corpus to
    /// read; the reason names what was looked for. The lap logs it and proceeds.
    Absent(String),
    /// A register claims the tree and cannot be followed, or its corpus road
    /// yields no judged sheaf — a broken kraal, never a legitimate absence.
    Unfollowable(String),
    /// Both checks ran: the rendered findings (empty on a clean corpus), and a
    /// tally line saying what was read.
    Ran {
        findings: Vec<String>,
        tally: String,
    },
}

/// Run the citation-integrity and rivet-hoist checks for the tree at `top`.
/// Locates the kraal's corpus road and reads `specs/` off it, every kit's
/// sheaves joining the ANCHOR set while only `specs/rbk` joins the JUDGED set
/// (`zrbvcec_check_citations` carries why the two differ). The in-repo `.adoc`
/// and `.sh` sets join both sides. Rivet-hoist rules the judged `.adoc` set
/// alone — placement is a jurisdiction question, and a sibling kit's rivets are
/// not RBK's to rule on.
///
/// Corpus files render CORPUS-ROOT-relative, so the rendered form is the same
/// whichever tree holds the corpus and `ZRBVCEC_CODEX_SHEAF` names the sheaf
/// once. In-repo files render repo-relative (`Tools/rbk/…`, `tt/…`); the two
/// rendering roots share no leading segment, so one path namespace serves both
/// without collision.
pub fn rbvcec_assay(top: &Path) -> rbvcec_Reading {
    let road = match zrbvcec_locate_corpus_road(top) {
        Err(e) => return rbvcec_Reading::Unfollowable(e),
        Ok(zrbvcec_CorpusRoad::Absent(reason)) => return rbvcec_Reading::Absent(reason),
        Ok(zrbvcec_CorpusRoad::Found(p)) => p,
    };
    let corpus_root = road.join(ZRBVCEC_CORPUS_ANCHOR_SUBDIR);

    let (mut judged_adoc_owned, sh_owned) = zrbvcec_walk_repo(top);
    let mut anchor_owned: Vec<(String, String)> = judged_adoc_owned.clone();
    let in_repo_adoc = anchor_owned.len();
    let judged_prefix = format!(
        "{}/",
        ZRBVCEC_CORPUS_SPEC_SUBDIR, //
    );

    let mut spec_files: Vec<PathBuf> = Vec::new();
    zrbvcec_walk(&corpus_root, &mut spec_files);
    spec_files.sort();
    let mut judged_corpus_adoc = 0usize;
    for path in &spec_files {
        if path.extension().and_then(|e| e.to_str()) != Some("adoc") {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let rel = zrbvcec_rel(&road, path);
        if rel.starts_with(&judged_prefix) {
            judged_adoc_owned.push((rel.clone(), content.clone()));
            judged_corpus_adoc += 1;
        }
        anchor_owned.push((rel, content));
    }
    let anchor_corpus_adoc = anchor_owned.len() - in_repo_adoc;
    if judged_corpus_adoc == 0 {
        return rbvcec_Reading::Unfollowable(format!(
            "corpus road stands at {} but {} yielded no .adoc under {} — a half-broken \
             checkout, refusing to run the citation-integrity and rivet-hoist checks against \
             an empty judged set",
            road.display(),
            corpus_root.display(),
            ZRBVCEC_CORPUS_SPEC_SUBDIR, //
        ));
    }

    let mut judged_all_owned: Vec<(String, String)> = judged_adoc_owned.clone();
    judged_all_owned.extend(sh_owned.iter().cloned());

    let anchor_refs = zrbvcec_refs(&anchor_owned);
    let judged_adoc_refs = zrbvcec_refs(&judged_adoc_owned);
    let judged_all_refs = zrbvcec_refs(&judged_all_owned);

    let mut hits = zrbvcec_check_citations(
        &anchor_refs,
        &judged_adoc_refs,
        &judged_all_refs,
        ZRBVCEC_UNANCHORED_QUOIN_EXEMPT_PATHS,
    );
    hits.extend(zrbvcec_check_rivet_hoist(
        &judged_adoc_refs,
        ZRBVCEC_HOIST_WAIVERS,
        ZRBVCEC_CODEX_SHEAF,
        ZRBVCEC_HOIST_INDEX_SHEAVES,
    ));

    let tally = format!(
        "corpus road {} — {} anchor .adoc under {}, of which {} judged under {}; \
         {} in-repo .adoc, {} in-repo .sh",
        road.display(),
        anchor_corpus_adoc,
        ZRBVCEC_CORPUS_ANCHOR_SUBDIR,
        judged_corpus_adoc,
        ZRBVCEC_CORPUS_SPEC_SUBDIR,
        in_repo_adoc,
        sh_owned.len(), //
    );
    rbvcec_Reading::Ran {
        findings: zrbvcec_render(&hits),
        tally,
    }
}
