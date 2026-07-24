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
// RBTHDR — loupe: the absorbed veil assay.
//
// A loupe is the jeweler's glass held to a coin still in the mint — the veiled
// trees have not been cut away yet. The assay reads them: it harvests its census
// from the withheld documents themselves, so it is meaningful only on the
// maintainer tree, before the cut. In the candidate the veiled trees are gone,
// the census is empty by construction, and an empty census is a finding outright.
//
// This module absorbs, in-process, the content-grain leak detection that used to
// ride as the theurge `loupe` fixture in shipped code (RBSHC "Worker, never
// authority": the veil assay joins the cut and the rig as one of the hierophant's
// own absorbed modules). Moving it here is what makes the shipped tree carry no
// veil apparatus: the census that enumerates withheld documents by name, the
// matcher that hunts them, and its self-proof all live behind the veil now.
//
// Two grains, two seats:
//   - `assay` runs PRE-CUT on the maintainer tree (RBSHE step 2): the veil-leak
//     content scan (no shipping file may NAME what the distribution withholds —
//     the veiled tree by path, or a withheld document by basename) and the
//     hostname-leak scan (no shipping file may name an operator's own test
//     machine). Both harvest their needle sets live rather than from a
//     hand-listed table, so a document veiled tomorrow, or a node enrolled
//     tomorrow, is protected tomorrow.
//   - `assay_candidate` runs POST-CUT on the candidate (RBSHE step 4), re-homed
//     from the theurge damnatio fixture's `veil_stripped` case: it hunts the veil
//     token in the candidate's transposed root CLAUDE.md — a leak the pre-cut
//     scan cannot catch, because its census is harvested where the veiled trees
//     still stand. The OTHER half of `veil_stripped` — "a withheld tree survived
//     the strip" — is not re-homed here: expede's object-graph delta sweep
//     already catches any withheld PATH in the candidate graph at any history
//     depth (RBSHC "The cut, and the single matcher"), so only the content-grain
//     CLAUDE.md needle remains.
//
// This is the veiled crate: it names veiled things freely, that is what being
// veiled means. The exemption table below names the few SHIPPED files that
// legitimately spell the veiled-dir literal in a skip-list — the sanctioned
// residue frozen by operator ruling, an allowlist the assay holds at exactly
// those sites.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

// ── Finding ─────────────────────────────────────────────────

/// One leak: where it is and what it is. `line` is 0 for a whole-file finding
/// that names no line.
#[derive(Clone, Debug)]
pub(crate) struct zrbthdr_Finding {
    pub(crate) file: String,
    pub(crate) line: usize,
    pub(crate) detail: String,
}

/// Render findings as stable one-per-line strings. A zero line number names a
/// whole file rather than a position within it.
fn zrbthdr_render(findings: &[zrbthdr_Finding]) -> Vec<String> {
    findings
        .iter()
        .map(|f| {
            if f.line == 0 {
                format!("{}: {}", f.file, f.detail)
            } else {
                format!("{}:{}: {}", f.file, f.line, f.detail)
            }
        })
        .collect()
}

// ── Veil constants ──────────────────────────────────────────

/// The directory basename that marks a tree as withheld from the distribution.
/// Both the needle the scan hunts in shipping files and the directory the scan
/// refuses to walk.
const ZRBTHDR_VEIL_DIR: &str = "vov_veiled";

/// Directories the census walk never descends: build output. The census
/// deliberately DOES descend into the veiled trees — that is where the withheld
/// documents it harvests live.
const ZRBTHDR_CENSUS_SKIP_DIRS: &[&str] = &["target"];

/// Directories the veil file-scan never descends: build output, and the veiled
/// trees themselves — a veiled file may name its veiled siblings freely.
const ZRBTHDR_VEIL_SKIP_DIRS: &[&str] = &["target", ZRBTHDR_VEIL_DIR];

/// Repo-relative roots walked by the veil scan — the shipping tree. `diagrams/`
/// rides here because a rendered diagram displays its source text to a reader: a
/// withheld name in a `.puml` title is baked into the committed `.svg` and is
/// read by a consumer who never greps anything.
const ZRBTHDR_VEIL_ROOTS: &[&str] =
    &["Tools/buk", "Tools/rbk", "tt", "rbmm_moorings", "diagrams"];

/// Repo-relative single files added to the veil corpus. The repo-root `CLAUDE.md`
/// is deliberately ABSENT here — pre-cut it is the maintainer's own context and
/// names withheld material on purpose; the candidate's transposed CLAUDE.md is
/// covered by `assay_candidate` instead. `LICENSE`/`.gitignore`/`.gitattributes`
/// are named because they ship but sit at the repo root, outside every walked
/// root — a withheld path named in an ignore pattern would otherwise ride unseen.
const ZRBTHDR_VEIL_FILES: &[&str] = &[
    "README.md",
    "Tools/rbk/vov_veiled/CLAUDE.consumer.md",
    "LICENSE",
    ".gitignore",
    ".gitattributes",
];

/// Repo-relative root under which veiled trees are hunted to build the census.
const ZRBTHDR_VEIL_CENSUS_ROOT: &str = "Tools";

/// Extensions of the withheld documents whose BASENAMES may not be named by a
/// shipping file. Documents only: a withheld `.sh` is reachable in prose only by
/// its path, which the veiled-dir needle already catches, while a bare document
/// basename (`SOMEDOC-Topic.adoc`) is the citation form that slips past a path
/// check.
const ZRBTHDR_VEIL_DOC_EXTS: &[&str] = &["adoc", "md"];

/// Substrings a line must carry before it can possibly name a withheld document.
/// A pure speed gate over the census loop, and exactly the extensions above.
const ZRBTHDR_VEIL_DOC_MARKS: &[&str] = &[".adoc", ".md"];

/// Repo-relative SHIPPED paths exempt from the veil scan, each with the reason —
/// the sanctioned residue (operator ruling): a shipped file may spell the
/// veiled-dir literal only to keep a tree-walk out of the veiled tree, never to
/// name a withheld document. Exact path, never a prefix. This table IS the
/// growth-containment allowlist: the residue stays frozen at exactly these sites.
const ZRBTHDR_VEIL_EXEMPT: &[(&str, &str)] = &[
    (
        "Tools/rbk/rbtd/src/rbtdrq_pyx.rs",
        "its no-.adoc case's skip-dir list spells the veiled-dir literal it must not descend into",
    ),
    (
        "Tools/rbk/rbtd/src/rbtdrq_damnatio.rs",
        "its identity sweep's skip-dir list spells the veiled-dir literal it must not descend into",
    ),
    (
        "Tools/rbk/rbtd/src/rbtdrn_conformance.rs",
        "its curl-scan exemption table addresses a withheld tree by path; dead in the stripped tree, where that tree is gone",
    ),
    (
        "rbmm_moorings/rbml_launchers/launcher.rbthw_workbench.sh",
        "the hierophant launcher must name the veiled workbench it dispatches to (the crate is veiled by construction), and is itself withheld — no candidate carries it",
    ),
];

/// One line may name a withheld thing two ways. Reported distinctly so a finding
/// says which law it broke.
const ZRBTHDR_VEIL_PATH_DETAIL: &str = "names the withheld tree";
const ZRBTHDR_VEIL_DOC_DETAIL: &str = "names withheld document";

/// File extensions skipped by the file walk — compiled, compressed, or raster
/// payloads in which a name cannot be authored by hand and a byte-coincidence
/// would be a false positive.
const ZRBTHDR_SKIP_EXTS: &[&str] =
    &["gz", "ico", "jpg", "jpeg", "lock", "png", "tar", "tgz", "webp", "zip"];

/// Per-file byte cap for the file walk. A file above this is not a hand-authored
/// source or config and reading it whole would make a fast assay slow.
const ZRBTHDR_SIZE_CAP: u64 = 1_048_576;

// ── Hostname constants ──────────────────────────────────────

/// Repo-relative root of the BURN node registry — one subdirectory per operator
/// test machine (the investiture dirname), each carrying a `burn.env` with a
/// `BURN_HOST=` value. Withheld by the perambulation, so it never reaches a
/// candidate; like the veil census, this harvest can only mean anything pre-cut.
const ZRBTHDR_HOST_CENSUS_ROOT: &str = "rbmm_moorings/rbmn_nodes";

/// Basenames the host census walk ignores — the registry's own README, never a
/// node identity.
const ZRBTHDR_HOST_CENSUS_SKIP_FILES: &[&str] = &["README.md"];

/// Directories skipped while scanning shipping files for a hostname leak. The
/// registry directories themselves carry the very identities the census is
/// harvested from — scanning them would match the census against its own source.
const ZRBTHDR_HOST_SCAN_SKIP_DIRS: &[&str] =
    &["target", "vov_veiled", "rbmn_nodes", "rbmu_users"];

/// Tokens exempt from the hostname census — an exact string that happens to also
/// be ordinary vocabulary. Exact token, operator act, same doctrine as
/// `ZRBTHDR_VEIL_EXEMPT`.
const ZRBTHDR_HOST_EXEMPT: &[&str] = &[];

// ── Repo-relative path ──────────────────────────────────────

/// Repo-relative, forward-slash form of `path` under `root`. On the maintainer
/// station paths are already `/`-joined; strip the root and lean on that.
fn zrbthdr_repo_rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

// ── Census and matcher ──────────────────────────────────────

/// True when `basename` is a withheld document — one of the extensions above.
fn zrbthdr_is_veil_doc(basename: &str) -> bool {
    ZRBTHDR_VEIL_DOC_EXTS
        .iter()
        .any(|ext| basename.ends_with(&format!(".{}", ext)))
}

/// Collect the basenames of every withheld document beneath `dir`, descending
/// into a veiled tree whole once one is entered.
fn zrbthdr_census_walk(dir: &Path, inside: bool, out: &mut BTreeSet<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if path.is_dir() {
            if ZRBTHDR_CENSUS_SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            zrbthdr_census_walk(&path, inside || name == ZRBTHDR_VEIL_DIR, out);
            continue;
        }
        if inside && zrbthdr_is_veil_doc(&name) {
            out.insert(name);
        }
    }
}

/// Scan one shipping file's text for both veil needles, appending findings.
pub(crate) fn zrbthdr_veil_scan_text(
    rel: &str,
    text: &str,
    census: &BTreeSet<String>,
    findings: &mut Vec<zrbthdr_Finding>,
) {
    for (index, line) in text.lines().enumerate() {
        if line.contains(ZRBTHDR_VEIL_DIR) {
            findings.push(zrbthdr_Finding {
                file: rel.to_string(),
                line: index + 1,
                detail: ZRBTHDR_VEIL_PATH_DETAIL.to_string(),
            });
        }
        if !ZRBTHDR_VEIL_DOC_MARKS.iter().any(|mark| line.contains(mark)) {
            continue;
        }
        for doc in census {
            if line.contains(doc.as_str()) {
                findings.push(zrbthdr_Finding {
                    file: rel.to_string(),
                    line: index + 1,
                    detail: format!("{} {}", ZRBTHDR_VEIL_DOC_DETAIL, doc),
                });
            }
        }
    }
}

/// The veil matcher's self-proof, run before its live-tree verdict is trusted.
/// The census it proves against is synthetic, so the proof holds in a tree whose
/// veiled documents have all been stripped away.
pub(crate) fn zrbthdr_veil_self_proof() -> Vec<zrbthdr_Finding> {
    let mut findings = Vec::new();
    let census: BTreeSet<String> = ["ZZQ-Example.adoc".to_string()].into_iter().collect();

    let positives: &[&str] = &[
        "  - see Tools/rbk/vov_veiled/whatever.sh for the rule",
        "# Contract: ZZQ-Example.adoc.",
        "- **ZZQ**  → `zzk/vov_veiled/ZZQ-Example.adoc` (a maintainer-context acronym row)",
    ];
    for probe in positives {
        let mut hits = Vec::new();
        zrbthdr_veil_scan_text("self-proof", probe, &census, &mut hits);
        if hits.is_empty() {
            findings.push(zrbthdr_Finding {
                file: "rbthdr_loupe.rs".to_string(),
                line: 0,
                detail: format!("veil matcher missed a known leak: {:?}", probe),
            });
        }
    }

    let negatives: &[&str] = &[
        "start with the README.md at the project root",
        "the terrier records which citizens hold which mantles",
        "ZZQ-Example.txt is not a withheld document",
    ];
    for probe in negatives {
        let mut hits = Vec::new();
        zrbthdr_veil_scan_text("self-proof", probe, &census, &mut hits);
        if !hits.is_empty() {
            findings.push(zrbthdr_Finding {
                file: "rbthdr_loupe.rs".to_string(),
                line: 0,
                detail: format!("veil matcher fired on a benign line: {:?}", probe),
            });
        }
    }

    findings
}

/// Recursively collect scannable files under `dir` into `out`, skipping the
/// `skip_dirs` directories, the unreadable extensions, and oversize payloads that
/// cannot hold a hand-authored token.
fn zrbthdr_walk(dir: &Path, skip_dirs: &[&str], out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let skipped = path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|name| skip_dirs.contains(&name))
                .unwrap_or(false);
            if !skipped {
                zrbthdr_walk(&path, skip_dirs, out);
            }
            continue;
        }
        let skipped = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| ZRBTHDR_SKIP_EXTS.contains(&ext))
            .unwrap_or(false);
        if skipped {
            continue;
        }
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > ZRBTHDR_SIZE_CAP {
                continue;
            }
        }
        out.push(path);
    }
}

// ── Hostname census and matcher ─────────────────────────────

/// Harvest operator machine identity from the BURN node registry: every
/// investiture dirname and every `BURN_HOST=` value beneath it. Both name a
/// specific machine.
fn zrbthdr_host_census_walk(dir: &Path, out: &mut BTreeSet<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if path.is_dir() {
            out.insert(name.clone());
            zrbthdr_host_census_walk(&path, out);
            continue;
        }
        if ZRBTHDR_HOST_CENSUS_SKIP_FILES.contains(&name.as_str()) {
            continue;
        }
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let text = String::from_utf8_lossy(&bytes);
        for line in text.lines() {
            if let Some(value) = line.strip_prefix("BURN_HOST=") {
                let value = value.trim();
                if !value.is_empty() {
                    out.insert(value.to_string());
                }
            }
        }
    }
}

/// True when `token` appears in `line` on a word boundary — not merely as a
/// substring of a longer, unrelated identifier.
pub(crate) fn zrbthdr_names_token(line: &str, token: &str) -> bool {
    fn word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '-'
    }

    let bytes = line.as_bytes();
    let mut start = 0;
    while let Some(pos) = line[start..].find(token) {
        let idx = start + pos;
        let before_ok = idx == 0 || !word_char(bytes[idx - 1] as char);
        let after = idx + token.len();
        let after_ok = after >= bytes.len() || !word_char(bytes[after] as char);
        if before_ok && after_ok {
            return true;
        }
        start = idx + 1;
    }
    false
}

// ── The pre-cut assay (maintainer tree) ─────────────────────

/// Scan the maintainer tree pre-cut for both leak grains. Returns rendered
/// findings, empty on a clean tree. `top` is the maintainer repo root.
pub fn assay(top: &Path) -> Vec<String> {
    let mut findings = zrbthdr_veil_leak(top);
    findings.extend(zrbthdr_hostname_leak(top));
    zrbthdr_render(&findings)
}

/// No shipping file may name what the distribution withholds — not the veiled
/// tree by path, and not a withheld document by basename. The census is
/// harvested from the tree, so a document veiled tomorrow is protected tomorrow.
/// An empty census is a FINDING outright: this assay runs only where the veiled
/// trees stand, so an empty result can only mean the extractor stopped
/// extracting.
fn zrbthdr_veil_leak(root: &Path) -> Vec<zrbthdr_Finding> {
    let mut findings = zrbthdr_veil_self_proof();

    let census_root = root.join(ZRBTHDR_VEIL_CENSUS_ROOT);
    let mut census = BTreeSet::new();
    zrbthdr_census_walk(&census_root, false, &mut census);
    if census.is_empty() {
        findings.push(zrbthdr_Finding {
            file: ZRBTHDR_VEIL_CENSUS_ROOT.to_string(),
            line: 0,
            detail: "the census matched no documents — the extractor stopped extracting".to_string(),
        });
    }

    let mut files = Vec::new();
    for sub in ZRBTHDR_VEIL_ROOTS {
        let path = root.join(sub);
        if path.is_dir() {
            zrbthdr_walk(&path, ZRBTHDR_VEIL_SKIP_DIRS, &mut files);
        }
    }

    // A basename naming BOTH a withheld document and a shipping file is an
    // ambiguity, not a leak — reading it as a leak would redden every honest
    // mention of the shipping file. Drop it from the census.
    for path in &files {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            census.remove(name);
        }
    }

    for sub in ZRBTHDR_VEIL_FILES {
        let path = root.join(sub);
        if path.is_file() {
            files.push(path);
        }
    }

    for path in files {
        let rel = zrbthdr_repo_rel(root, &path);
        if ZRBTHDR_VEIL_EXEMPT.iter().any(|(exempt, _)| *exempt == rel) {
            continue;
        }
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let text = String::from_utf8_lossy(&bytes);
        zrbthdr_veil_scan_text(&rel, &text, &census, &mut findings);
    }

    findings
}

/// No shipping file may name an operator's own test machine. The needle set is
/// harvested live from the BURN node registry, so a node enrolled tomorrow is
/// protected tomorrow. Source-tree only, same as the veil leak: the registry is
/// stripped whole at release, so an empty census here is a FINDING outright.
fn zrbthdr_hostname_leak(root: &Path) -> Vec<zrbthdr_Finding> {
    let mut findings = Vec::new();

    let census_root = root.join(ZRBTHDR_HOST_CENSUS_ROOT);
    let mut census = BTreeSet::new();
    zrbthdr_host_census_walk(&census_root, &mut census);
    if census.is_empty() {
        findings.push(zrbthdr_Finding {
            file: ZRBTHDR_HOST_CENSUS_ROOT.to_string(),
            line: 0,
            detail: "the census matched no operator machine names — the extractor stopped extracting"
                .to_string(),
        });
    }
    for name in ZRBTHDR_HOST_EXEMPT {
        census.remove(*name);
    }

    let mut files = Vec::new();
    for sub in ZRBTHDR_VEIL_ROOTS {
        let path = root.join(sub);
        if path.is_dir() {
            zrbthdr_walk(&path, ZRBTHDR_HOST_SCAN_SKIP_DIRS, &mut files);
        }
    }
    for sub in ZRBTHDR_VEIL_FILES {
        let path = root.join(sub);
        if path.is_file() {
            files.push(path);
        }
    }

    for path in files {
        let rel = zrbthdr_repo_rel(root, &path);
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let text = String::from_utf8_lossy(&bytes);
        for (index, line) in text.lines().enumerate() {
            for tok in &census {
                if zrbthdr_names_token(line, tok) {
                    findings.push(zrbthdr_Finding {
                        file: rel.clone(),
                        line: index + 1,
                        detail: format!("names operator machine {}", tok),
                    });
                }
            }
        }
    }

    findings
}

// ── The post-cut assay (candidate) ──────────────────────────

/// Re-homed from the theurge damnatio fixture's `veil_stripped` case: hunt the
/// veil token in the candidate's transposed root CLAUDE.md, once no veiled tree
/// stands to census from. The census is empty here by construction, so only the
/// veiled-dir needle can fire. The path-grain "a withheld tree survived" half is
/// NOT here — expede's object-graph delta sweep already catches any withheld
/// path in the candidate graph. Returns rendered findings, empty when clean.
pub fn assay_candidate(candidate_root: &Path) -> Vec<String> {
    let mut findings = zrbthdr_veil_self_proof();
    let empty_census = BTreeSet::new();

    let claude = candidate_root.join("CLAUDE.md");
    if let Ok(bytes) = std::fs::read(&claude) {
        let text = String::from_utf8_lossy(&bytes);
        zrbthdr_veil_scan_text("CLAUDE.md", &text, &empty_census, &mut findings);
    }

    zrbthdr_render(&findings)
}
