// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCEU — loupe: the absorbed veil assay.
//
// A loupe is the jeweler's glass held to a coin still in the mint — the veiled
// trees have not been cut away yet. The assay reads them: it harvests its census
// from the withheld documents themselves, so it is meaningful only on the
// maintainer tree, before the cut. In the regulus the veiled trees are gone,
// the census is empty by construction, and an empty census is a finding outright.
//
// This module absorbs, in-process, the content-grain leak detection that used to
// ride as the theurge `loupe` fixture in shipped code (RBSHC "Worker, never
// authority": the veil assay joins the cut and the temenos as one of the hierophant's
// own absorbed modules). Moving it here is what makes the shipped tree carry no
// veil apparatus: the census that enumerates withheld documents by name, the
// matcher that hunts them, and its self-proof all live behind the veil now.
//
// Two grains, two seats:
//   - `assay` runs PRE-CUT on the maintainer tree (RBSHE step 2): the veil-leak
//     content scan (no shipping file may NAME what the distribution withholds —
//     the veiled tree by path, or a withheld document by basename) and the
//     hostname-leak scan (no shipping file may name an operator's own test
//     machine). The veil census harvests its needle set live rather than from a
//     hand-listed table, so a document veiled tomorrow is protected tomorrow;
//     the hostname needles are curated by hand, for the reason given at
//     `ZRBVCEU_HOST_CENSUS`.
//   - `assay_regulus` runs POST-CUT on the regulus (RBSHE step 4), re-homed
//     from the theurge damnatio fixture's `veil_stripped` case: it hunts the veil
//     token in the regulus's root CLAUDE.md as the cut committed it, the last
//     word on what the regulus ships whatever the pre-cut scan read. The OTHER half of `veil_stripped` — "a withheld tree survived
//     the strip" — is not re-homed here: expede's object-graph delta sweep
//     already catches any withheld PATH in the regulus graph at any history
//     depth (RBSHC "The cut, and the single matcher"), so only the content-grain
//     CLAUDE.md needle remains.
//
// This is the veiled crate: it names veiled things freely, that is what being
// veiled means. The exemption table below names the few SHIPPED files that
// legitimately spell the veiled-dir literal in a skip-list — the sanctioned
// residue frozen by operator ruling, an allowlist the assay holds at exactly
// those sites.

use std::collections::BTreeSet;
use std::path::{
    Path,
    PathBuf, //
};

use crate::rbvcem_perambulation;

// ── Finding ─────────────────────────────────────────────────

/// One leak: where it is and what it is. `line` is 0 for a whole-file finding
/// that names no line.
#[derive(Clone, Debug)]
pub(crate) struct zrbvceu_Finding {
    pub(crate) file: String,
    pub(crate) line: usize,
    pub(crate) detail: String,
}

/// Render findings as stable one-per-line strings. A zero line number names a
/// whole file rather than a position within it.
fn zrbvceu_render(findings: &[zrbvceu_Finding]) -> Vec<String> {
    findings
        .iter()
        .map(|f| {
            if f.line == 0 {
                format!(
                    "{}: {}",
                    f.file,
                    f.detail, //
                )
            } else {
                format!(
                    "{}:{}: {}",
                    f.file,
                    f.line,
                    f.detail, //
                )
            }
        })
        .collect()
}

// ── Veil constants ──────────────────────────────────────────

/// The directory basename that marks a tree as withheld from the distribution.
/// Both the needle the scan hunts in shipping files and the directory the scan
/// refuses to walk.
const ZRBVCEU_VEIL_DIR: &str = "vov_veiled";

/// Directories the census walk never descends: build output. Nothing else is
/// refused — the census is rooted INSIDE the veiled tree, so every document it
/// meets on the way down is withheld by construction.
const ZRBVCEU_CENSUS_SKIP_DIRS: &[&str] = &["target"];

/// Directories the veil file-scan never descends: build output, and the veiled
/// trees themselves — a veiled file may name its veiled siblings freely.
const ZRBVCEU_VEIL_SKIP_DIRS: &[&str] = &[
    "target",
    ZRBVCEU_VEIL_DIR, //
];

/// Repo-relative roots walked by the veil scan — the shipping tree. `diagrams/`
/// rides here because a rendered diagram displays its source text to a reader: a
/// withheld name in a `.puml` title is baked into the committed `.svg` and is
/// read by a consumer who never greps anything.
const ZRBVCEU_VEIL_ROOTS: &[&str] = &[
    "Tools/buk",
    "Tools/rbk",
    "tt",
    "rbmm_moorings",
    "diagrams", //
];

/// Repo-relative single files added to the veil corpus. The repo-root `CLAUDE.md`
/// is the consumer's context, rendered and shipped byte for byte, so it is held
/// to the veil here like `README.md`. `LICENSE`/`.gitignore`/`.gitattributes`
/// are named because they ship but sit at the repo root, outside every walked
/// root — a withheld path named in an ignore pattern would otherwise ride unseen.
const ZRBVCEU_VEIL_FILES: &[&str] = &[
    "README.md",
    "CLAUDE.md",
    "LICENSE",
    ".gitignore",
    ".gitattributes", //
];

/// Repo-relative root the census walks: the veiled tree itself, which stands at
/// the repo root beside the kits. Rooting the walk here rather than above it is
/// what lets the walk harvest unconditionally — a root above would have to hunt
/// for the tree on the way down, and a root at the repo root would descend
/// `.git` and the operator's own trees to do it.
const ZRBVCEU_VEIL_CENSUS_ROOT: &str = ZRBVCEU_VEIL_DIR;

/// Extensions of the withheld documents whose BASENAMES may not be named by a
/// shipping file. Documents only: a withheld `.sh` is reachable in prose only by
/// its path, which the veiled-dir needle already catches, while a bare document
/// basename (`SOMEDOC-Topic.adoc`) is the citation form that slips past a path
/// check.
const ZRBVCEU_VEIL_DOC_EXTS: &[&str] = &[
    "adoc",
    "md", //
];

/// Substrings a line must carry before it can possibly name a withheld document.
/// A pure speed gate over the census loop, and exactly the extensions above.
const ZRBVCEU_VEIL_DOC_MARKS: &[&str] = &[
    ".adoc",
    ".md", //
];

/// The classes of leak the assay hunts. A class is the unit of exemption: a
/// residue row names the classes its file is excused from, never the file whole.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum zrbvceu_Class {
    /// The veiled-dir literal, and a withheld document's basename from the census.
    Veil,
    /// An internal path token: the `jjq` family's directory prefixes, and the
    /// kraal-register basename.
    Token,
    /// The dated-memo name shape.
    Memo,
    /// The spec document-name shape: an upper-case acronym, a hyphen, a name,
    /// and a document extension. Never a bare acronym.
    Doc,
}

/// The three shape classes, each hunted by `zrbvceu_class_names`.
const ZRBVCEU_SHAPE_CLASSES: &[zrbvceu_Class] = &[
    zrbvceu_Class::Token,
    zrbvceu_Class::Memo,
    zrbvceu_Class::Doc, //
];

/// Why the federation diagrams are residue: their titles cite a dated memo and
/// were ruled accepted until the diagrams next change.
const ZRBVCEU_DIAGRAM_REASON: &str =
    "accepted residue (operator ruling): the diagram's title cites an internal record, kept until the diagram next changes";

/// Repo-relative SHIPPED paths exempt from the veil scan, each with the classes it
/// is excused from and the reason — the sanctioned residue (operator ruling).
/// Exact path, never a prefix; a class the row does not name is still hunted in
/// that file. This table IS the growth-containment allowlist: the residue stays
/// frozen at exactly these sites. The four veiled-dir rows may spell the literal
/// only to AIM A TREE-WALK — to keep one out of the veiled tree, or to send one
/// in — never to name a withheld document.
const ZRBVCEU_VEIL_EXEMPT: &[(&str, &[zrbvceu_Class], &str)] = &[
    (
        "Tools/rbk/rbtd/src/rbtdrq_pyx.rs",
        &[zrbvceu_Class::Veil, zrbvceu_Class::Doc],
        "its no-.adoc case's skip-dir list spells the veiled-dir literal it must not descend into, and its secret-scan roots spell it again as a tree the scan must descend INTO; its sample document names are made-up data (accepted)",
    ),
    (
        "Tools/rbk/rbtd/src/rbtdrq_damnatio.rs",
        &[zrbvceu_Class::Veil],
        "its identity sweep's skip-dir list spells the veiled-dir literal it must not descend into",
    ),
    (
        "Tools/rbk/rbtd/src/rbtdrn_conformance.rs",
        &[zrbvceu_Class::Veil],
        "its curl-scan exemption table addresses a withheld tree by path; dead in the stripped tree, where that tree is gone",
    ),
    (
        "rbmm_moorings/buml_launchers/launcher.rbvw_workbench.sh",
        &[zrbvceu_Class::Veil],
        "the hierophant launcher must name the veiled workbench it dispatches to (the crate is veiled by construction), and is itself withheld — no regulus carries it",
    ),
    (
        "Tools/buk/buym_yelp.sh",
        &[zrbvceu_Class::Token],
        "the parcel door spells the stand directory's name (accepted residue, operator ruling)",
    ),
    (
        "diagrams/rbdgl_federation-login.puml",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgl_federation-login-light.svg",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgl_federation-login-dark.svg",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgm_federation-seam.puml",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgm_federation-seam-light.svg",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgm_federation-seam-dark.svg",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgs_federation-setup.puml",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgs_federation-setup-light.svg",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ),
    (
        "diagrams/rbdgs_federation-setup-dark.svg",
        &[zrbvceu_Class::Token, zrbvceu_Class::Memo],
        ZRBVCEU_DIAGRAM_REASON,
    ), //
];

/// True when the residue table excuses `rel` from `class`.
pub(crate) fn zrbvceu_exempt_from(rel: &str, class: zrbvceu_Class) -> bool {
    ZRBVCEU_VEIL_EXEMPT
        .iter()
        .any(|(path, classes, _)| *path == rel && classes.contains(&class))
}

/// One line may name a withheld thing two ways. Reported distinctly so a finding
/// says which law it broke.
const ZRBVCEU_VEIL_PATH_DETAIL: &str = "names the withheld tree";
const ZRBVCEU_VEIL_DOC_DETAIL: &str = "names withheld document";
const ZRBVCEU_TOKEN_DETAIL: &str = "names an internal path token";
const ZRBVCEU_MEMO_DETAIL: &str = "names an internal dated record";
const ZRBVCEU_DOC_DETAIL: &str = "names an internal spec document";

/// File extensions skipped by the file walk — compiled, compressed, or raster
/// payloads in which a name cannot be authored by hand and a byte-coincidence
/// would be a false positive.
const ZRBVCEU_SKIP_EXTS: &[&str] = &[
    "gz",
    "ico",
    "jpg",
    "jpeg",
    "lock",
    "png",
    "tar",
    "tgz",
    "webp",
    "zip", //
];

/// Per-file byte cap for the file walk. A file above this is not a hand-authored
/// source or config and reading it whole would make a fast assay slow.
const ZRBVCEU_SIZE_CAP: u64 = 1_048_576;

// ── Hostname constants ──────────────────────────────────────

/// The operator's own test machines — the names no shipping file may carry.
/// Curated by hand, and by hand deliberately: these were once harvested live
/// from the BURN node registry, but that registry served the jurisdiction
/// apparatus that has since retired out of this consumer tree, and node
/// enrollment now happens only at the kit's primary home. A literal here is
/// therefore as current as reality permits, and it is safe here for the reason
/// this whole crate is: the hierophant is veiled and reaches no regulus.
///
/// A machine acquired later is protected only once its name is added — the one
/// cost of the retired harvest, and the reason to add it at acquisition.
pub(crate) const ZRBVCEU_HOST_CENSUS: &[&str] = &[
    "bujn-winpc",
    "rocket",
    "cerebro", //
];

/// Directories skipped while scanning shipping files for a hostname leak.
const ZRBVCEU_HOST_SCAN_SKIP_DIRS: &[&str] = &[
    "target",
    "vov_veiled", //
];

/// Tokens exempt from the hostname census — an exact string that happens to also
/// be ordinary vocabulary. Exact token, operator act, same doctrine as
/// `ZRBVCEU_VEIL_EXEMPT`.
pub(crate) const ZRBVCEU_HOST_EXEMPT: &[&str] = &[];

// ── Repo-relative path ──────────────────────────────────────

/// Repo-relative, forward-slash form of `path` under `root`. On the maintainer
/// station paths are already `/`-joined; strip the root and lean on that.
fn zrbvceu_repo_rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

// ── Census and matcher ──────────────────────────────────────

/// True when `basename` is a withheld document — one of the extensions above.
/// True when the perambulation withholds `rel`, the one judge of what ships
/// (RBSHC "The cut, and the single matcher"). The walked roots say where
/// shipping files live, never that everything under them ships: a withheld file
/// standing inside one, such as a kennel collar under the moorings, never
/// reaches a regulus, so its naming the veiled tree is no leak. A path no row
/// judges is still scanned, so an unjudged file stays held to the veil.
pub(crate) fn zrbvceu_withheld(rel: &str) -> bool {
    matches!(
        rbvcem_perambulation::rbvcem_judge(rel),
        Some((rbvcem_perambulation::rbvcem_Disposition::Withhold, _)) //
    )
}

/// Drop from a walked file list every path the perambulation withholds.
fn zrbvceu_retain_shipping(root: &Path, files: &mut Vec<PathBuf>) {
    files.retain(|path| !zrbvceu_withheld(&zrbvceu_repo_rel(root, path)));
}

fn zrbvceu_is_veil_doc(basename: &str) -> bool {
    ZRBVCEU_VEIL_DOC_EXTS.iter().any(|ext| {
        basename.ends_with(&format!(
            ".{}",
            ext, //
        ))
    })
}

/// Collect the basenames of every withheld document beneath `dir`, which the
/// caller roots inside the veiled tree — so every document met is withheld and
/// the walk carries no in-or-out state of its own.
fn zrbvceu_census_walk(dir: &Path, out: &mut BTreeSet<String>) {
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
            if ZRBVCEU_CENSUS_SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            zrbvceu_census_walk(&path, out);
            continue;
        }
        if zrbvceu_is_veil_doc(&name) {
            out.insert(name);
        }
    }
}

/// Scan one shipping file's text for both veil needles, appending findings.
pub(crate) fn zrbvceu_veil_scan_text(
    rel: &str,
    text: &str,
    census: &BTreeSet<String>,
    findings: &mut Vec<zrbvceu_Finding>,
) {
    for (index, line) in text.lines().enumerate() {
        if line.contains(ZRBVCEU_VEIL_DIR) {
            findings.push(zrbvceu_Finding {
                file: rel.to_string(),
                line: index + 1,
                detail: ZRBVCEU_VEIL_PATH_DETAIL.to_string(),
            });
        }
        if !ZRBVCEU_VEIL_DOC_MARKS
            .iter()
            .any(|mark| line.contains(mark))
        {
            continue;
        }
        for doc in census {
            if line.contains(doc.as_str()) {
                findings.push(zrbvceu_Finding {
                    file: rel.to_string(),
                    line: index + 1,
                    detail: format!(
                        "{} {}",
                        ZRBVCEU_VEIL_DOC_DETAIL,
                        doc, //
                    ),
                });
            }
        }
    }
}

/// The veil matcher's self-proof, run before its live-tree verdict is trusted.
/// The census it proves against is synthetic, so the proof holds in a tree whose
/// veiled documents have all been stripped away — and in the maintainer tree,
/// whose veiled tree holds no document today, it is the one proof the matcher
/// extracts at all.
pub(crate) fn zrbvceu_veil_self_proof() -> Vec<zrbvceu_Finding> {
    zrbvceu_veil_self_proof_by(zrbvceu_veil_scan_text)
}

/// The self-proof over a given matcher, so a hurdle can hand it one that leaks
/// and watch the proof fail.
pub(crate) fn zrbvceu_veil_self_proof_by(
    scan: fn(&str, &str, &BTreeSet<String>, &mut Vec<zrbvceu_Finding>),
) -> Vec<zrbvceu_Finding> {
    let mut findings = Vec::new();
    let census: BTreeSet<String> = ["ZZQ-Example.adoc".to_string()].into_iter().collect();

    let positives: &[&str] = &[
        "  - see vov_veiled/whatever.sh for the rule",
        "# Contract: ZZQ-Example.adoc.",
        "- **ZZQ**  → `zzk/vov_veiled/ZZQ-Example.adoc` (a maintainer-context acronym row)",
    ];
    for probe in positives {
        let mut hits = Vec::new();
        scan("self-proof", probe, &census, &mut hits);
        if hits.is_empty() {
            findings.push(zrbvceu_Finding {
                file: "rbvceu_loupe.rs".to_string(),
                line: 0,
                detail: format!(
                    "veil matcher missed a known leak: {:?}",
                    probe, //
                ),
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
        scan("self-proof", probe, &census, &mut hits);
        if !hits.is_empty() {
            findings.push(zrbvceu_Finding {
                file: "rbvceu_loupe.rs".to_string(),
                line: 0,
                detail: format!(
                    "veil matcher fired on a benign line: {:?}",
                    probe, //
                ),
            });
        }
    }

    findings
}

// ── The shape classes ───────────────────────────────────────

/// True when the byte before `idx` is not part of an identifier, so a needle
/// starting at `idx` stands at a left edge.
fn zrbvceu_left_edge(bytes: &[u8], idx: usize) -> bool {
    idx == 0 || !(bytes[idx - 1].is_ascii_alphanumeric() || bytes[idx - 1] == b'_')
}

/// An internal path token: `jjq`, one lower-case letter and an underscore at a
/// left edge — the directory prefixes of the `jjq` family — or the kraal-register
/// basename. A token, never a bare word: the bare words stand in shipped kit text.
fn zrbvceu_names_store_token(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut start = 0;
    while let Some(pos) = line[start..].find("jjq") {
        let idx = start + pos;
        if idx + 5 <= bytes.len()
            && bytes[idx + 3].is_ascii_lowercase()
            && bytes[idx + 4] == b'_'
            && zrbvceu_left_edge(bytes, idx)
        {
            return true;
        }
        start = idx + 1;
    }
    zrbvceu_names_token(line, "feodary.json")
}

/// The dated-memo name shape: `memo-`, eight digits and a hyphen, at a left edge.
fn zrbvceu_names_memo(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut start = 0;
    while let Some(pos) = line[start..].find("memo-") {
        let idx = start + pos;
        let digits = idx + 5;
        if digits + 9 <= bytes.len()
            && bytes[digits..digits + 8].iter().all(|b| b.is_ascii_digit())
            && bytes[digits + 8] == b'-'
            && zrbvceu_left_edge(bytes, idx)
        {
            return true;
        }
        start = idx + 1;
    }
    false
}

/// The spec document-name shape: a document extension on a stem that opens with an
/// upper-case acronym of two to eight characters, a hyphen and a name. A stem with
/// no hyphen after its acronym — `README.md` — is no spec name.
fn zrbvceu_names_doc(line: &str) -> bool {
    let bytes = line.as_bytes();
    for ext in [".adoc", ".md"] {
        let mut start = 0;
        while let Some(pos) = line[start..].find(ext) {
            let dot = start + pos;
            start = dot + 1;
            let after = dot + ext.len();
            if after < bytes.len() && (bytes[after].is_ascii_alphanumeric() || bytes[after] == b'_')
            {
                continue;
            }
            let mut from = dot;
            while from > 0
                && (bytes[from - 1].is_ascii_alphanumeric()
                    || bytes[from - 1] == b'_'
                    || bytes[from - 1] == b'-')
            {
                from -= 1;
            }
            let stem = &bytes[from..dot];
            if stem.is_empty() || !stem[0].is_ascii_uppercase() {
                continue;
            }
            let run = stem
                .iter()
                .take_while(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
                .count();
            if (2..=8).contains(&run) && stem.len() > run + 1 && stem[run] == b'-' {
                return true;
            }
        }
    }
    false
}

/// True when `line` carries a needle of the shape `class`. The veil class has no
/// shape of its own — it is the scan above — and never fires here.
pub(crate) fn zrbvceu_class_names(class: zrbvceu_Class, line: &str) -> bool {
    match class {
        zrbvceu_Class::Token => zrbvceu_names_store_token(line),
        zrbvceu_Class::Memo => zrbvceu_names_memo(line),
        zrbvceu_Class::Doc => zrbvceu_names_doc(line),
        zrbvceu_Class::Veil => false,
    }
}

fn zrbvceu_class_detail(class: zrbvceu_Class) -> &'static str {
    match class {
        zrbvceu_Class::Token => ZRBVCEU_TOKEN_DETAIL,
        zrbvceu_Class::Memo => ZRBVCEU_MEMO_DETAIL,
        zrbvceu_Class::Doc => ZRBVCEU_DOC_DETAIL,
        zrbvceu_Class::Veil => ZRBVCEU_VEIL_PATH_DETAIL,
    }
}

/// Scan one shipping file's text for the shape classes `rel` is not excused from.
pub(crate) fn zrbvceu_shape_scan_text(
    rel: &str,
    text: &str,
    findings: &mut Vec<zrbvceu_Finding>,
) {
    for class in ZRBVCEU_SHAPE_CLASSES {
        if zrbvceu_exempt_from(rel, *class) {
            continue;
        }
        for (index, line) in text.lines().enumerate() {
            if zrbvceu_class_names(*class, line) {
                findings.push(zrbvceu_Finding {
                    file: rel.to_string(),
                    line: index + 1,
                    detail: zrbvceu_class_detail(*class).to_string(),
                });
            }
        }
    }
}

/// Planted leaks and their clean twins per shape class. The values are made up —
/// they fit each shape and name nothing real.
pub(crate) fn zrbvceu_class_specimens(
    class: zrbvceu_Class,
) -> (&'static [&'static str], &'static [&'static str]) {
    match class {
        zrbvceu_Class::Token => (
            &[
                "  see jjqz_example/notes for the rule",
                "# kept in ../jjqz_sample-tree",
                "reads feodary.json beside the entry",
            ],
            &[
                "the estate rules name an infield and a kraal",
                "no jjq prefix without its letter and underscore: jjq_x",
                "a longer word such as xjjqz_example is no token",
                "feodary.jsonl is a different file",
            ],
        ),
        zrbvceu_Class::Memo => (
            &[
                "  (see memo-29990101-example-slug for the reasoning)",
                "../notes/memo-29990231-another-sample.md",
            ],
            &[
                "write a memo-style summary",
                "memo-2999 is too short a date",
                "a rememo-29990101-x is no name",
            ],
        ),
        zrbvceu_Class::Doc => (
            &[
                "# Contract: ZZQ-Sample.adoc.",
                "see specs/zzk/ZZQ9-Other-Sample.adoc for the law",
                "cited as `ZZQ-Sample.md`",
            ],
            &[
                "start with the README.md at the project root",
                "ZZQ-Sample.txt is not a document",
                "zzq-sample.adoc is lower case",
                "ZZQ.adoc names a bare acronym",
                "ZZQ-Sample.mdx is another extension",
            ],
        ),
        zrbvceu_Class::Veil => (&[], &[]),
    }
}

/// The shape classes' self-proof over a given matcher, so a hurdle can hand it one
/// that leaks and watch the proof fail. Every planted positive must fire and every
/// clean twin must stay silent.
pub(crate) fn zrbvceu_class_self_proof_by(
    names: fn(zrbvceu_Class, &str) -> bool,
) -> Vec<zrbvceu_Finding> {
    let mut findings = Vec::new();
    for class in ZRBVCEU_SHAPE_CLASSES {
        let (positives, negatives) = zrbvceu_class_specimens(*class);
        for probe in positives {
            if !names(*class, probe) {
                findings.push(zrbvceu_Finding {
                    file: "rbvceu_loupe.rs".to_string(),
                    line: 0,
                    detail: format!(
                        "{:?} class matcher missed a known leak: {:?}",
                        class,
                        probe, //
                    ),
                });
            }
        }
        for probe in negatives {
            if names(*class, probe) {
                findings.push(zrbvceu_Finding {
                    file: "rbvceu_loupe.rs".to_string(),
                    line: 0,
                    detail: format!(
                        "{:?} class matcher fired on a benign line: {:?}",
                        class,
                        probe, //
                    ),
                });
            }
        }
    }
    findings
}

/// The shape classes' self-proof over the real matcher.
pub(crate) fn zrbvceu_class_self_proof() -> Vec<zrbvceu_Finding> {
    zrbvceu_class_self_proof_by(zrbvceu_class_names)
}

/// Recursively collect scannable files under `dir` into `out`, skipping the
/// `skip_dirs` directories, the unreadable extensions, and oversize payloads that
/// cannot hold a hand-authored token.
fn zrbvceu_walk(dir: &Path, skip_dirs: &[&str], out: &mut Vec<PathBuf>) {
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
                zrbvceu_walk(&path, skip_dirs, out);
            }
            continue;
        }
        let skipped = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| ZRBVCEU_SKIP_EXTS.contains(&ext))
            .unwrap_or(false);
        if skipped {
            continue;
        }
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > ZRBVCEU_SIZE_CAP {
                continue;
            }
        }
        out.push(path);
    }
}

// ── Hostname matcher ────────────────────────────────────────

/// True when `token` appears in `line` on a word boundary — not merely as a
/// substring of a longer, unrelated identifier.
pub(crate) fn zrbvceu_names_token(line: &str, token: &str) -> bool {
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

// ── The pre-cut rbvceu_assay (maintainer tree) ─────────────────────

/// Scan the maintainer tree pre-cut for both leak grains. Returns rendered
/// findings, empty on a clean tree. `top` is the maintainer repo root.
pub fn rbvceu_assay(top: &Path) -> Vec<String> {
    let mut findings = zrbvceu_veil_leak(top);
    findings.extend(zrbvceu_hostname_leak(top));
    zrbvceu_render(&findings)
}

/// No shipping file may name what the distribution withholds — not the veiled
/// tree by path, not a withheld document by basename, and not an internal path
/// token, dated record name or spec document name (the shape classes, each with
/// its own self-proof run first). The census is
/// harvested from the tree, so a document veiled tomorrow is protected tomorrow.
/// An empty census is admitted: the consumer context was the one document the
/// veiled tree held, and it now ships rendered at the root, so a census of none
/// is the tree as it stands. The self-proof run first stays the proof that the
/// matcher extracts, and the veiled-dir needle fires whatever the census holds.
pub(crate) fn zrbvceu_veil_leak(root: &Path) -> Vec<zrbvceu_Finding> {
    let mut findings = zrbvceu_veil_self_proof();
    findings.extend(zrbvceu_class_self_proof());

    let census_root = root.join(ZRBVCEU_VEIL_CENSUS_ROOT);
    let mut census = BTreeSet::new();
    zrbvceu_census_walk(&census_root, &mut census);

    let mut files = Vec::new();
    for sub in ZRBVCEU_VEIL_ROOTS {
        let path = root.join(sub);
        if path.is_dir() {
            zrbvceu_walk(&path, ZRBVCEU_VEIL_SKIP_DIRS, &mut files);
        }
    }
    zrbvceu_retain_shipping(root, &mut files);

    // A basename naming BOTH a withheld document and a shipping file is an
    // ambiguity, not a leak — reading it as a leak would redden every honest
    // mention of the shipping file. Drop it from the census.
    for path in &files {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            census.remove(name);
        }
    }

    for sub in ZRBVCEU_VEIL_FILES {
        let path = root.join(sub);
        if path.is_file() {
            files.push(path);
        }
    }

    for path in files {
        let rel = zrbvceu_repo_rel(root, &path);
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let text = String::from_utf8_lossy(&bytes);
        if !zrbvceu_exempt_from(&rel, zrbvceu_Class::Veil) {
            zrbvceu_veil_scan_text(&rel, &text, &census, &mut findings);
        }
        zrbvceu_shape_scan_text(&rel, &text, &mut findings);
    }

    findings
}

/// No shipping file may name an operator's own test machine. The needle set is
/// the curated `ZRBVCEU_HOST_CENSUS` literal. Source-tree only, same as the veil
/// leak. There is no empty-census guard: the tripwire that stood here watched a
/// live extractor that could silently stop extracting, and a literal cannot —
/// emptying it is a visible, deliberate edit.
fn zrbvceu_hostname_leak(root: &Path) -> Vec<zrbvceu_Finding> {
    let mut findings = Vec::new();

    let mut census: BTreeSet<String> = ZRBVCEU_HOST_CENSUS
        .iter()
        .map(|name| name.to_string())
        .collect();
    for name in ZRBVCEU_HOST_EXEMPT {
        census.remove(*name);
    }

    let mut files = Vec::new();
    for sub in ZRBVCEU_VEIL_ROOTS {
        let path = root.join(sub);
        if path.is_dir() {
            zrbvceu_walk(&path, ZRBVCEU_HOST_SCAN_SKIP_DIRS, &mut files);
        }
    }
    zrbvceu_retain_shipping(root, &mut files);
    for sub in ZRBVCEU_VEIL_FILES {
        let path = root.join(sub);
        if path.is_file() {
            files.push(path);
        }
    }

    for path in files {
        let rel = zrbvceu_repo_rel(root, &path);
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let text = String::from_utf8_lossy(&bytes);
        for (index, line) in text.lines().enumerate() {
            for tok in &census {
                if zrbvceu_names_token(line, tok) {
                    findings.push(zrbvceu_Finding {
                        file: rel.clone(),
                        line: index + 1,
                        detail: format!(
                            "names operator machine {}",
                            tok, //
                        ),
                    });
                }
            }
        }
    }

    findings
}

// ── The post-cut rbvceu_assay (regulus) ──────────────────────────

/// Re-homed from the theurge damnatio fixture's `veil_stripped` case: hunt the
/// veil token in the regulus's root CLAUDE.md, once no veiled tree stands to
/// census from. The census is empty here by construction, so only the
/// veiled-dir needle can fire. The path-grain "a withheld tree survived" half is
/// NOT here — expede's object-graph delta sweep already catches any withheld
/// path in the regulus graph. Returns rendered findings, empty when clean.
pub fn rbvceu_assay_regulus(regulus_root: &Path) -> Vec<String> {
    let mut findings = zrbvceu_veil_self_proof();
    let empty_census = BTreeSet::new();

    let claude = regulus_root.join("CLAUDE.md");
    if let Ok(bytes) = std::fs::read(&claude) {
        let text = String::from_utf8_lossy(&bytes);
        zrbvceu_veil_scan_text("CLAUDE.md", &text, &empty_census, &mut findings);
    }

    zrbvceu_render(&findings)
}
