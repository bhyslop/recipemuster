// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTEU — the veil assay's self-proofs, ported from the retired theurge loupe
// fixture when the content-grain leak detection was absorbed into the hierophant.
// They, not a green lap, are what prove the matcher.
//
//   SCAN     the content matcher catches a planted veiled-tree mention and a
//            planted withheld-document basename, and stays silent on benign
//            lines. A matcher that cannot catch a planted leak cannot be trusted
//            to catch a real one; one that reddens on a clean line gets waved
//            through in a week.
//
//   PROOF    the self-proof the live-tree verdict rests on runs green against
//            its own synthetic census — the guarantee the assay's finding is a
//            leak in the tree, not a broken matcher.
//
//   EMPTY    the live document census may be empty, the veiled tree holding no
//            document, and the assay admits it; what keeps that admission honest
//            is the self-proof, which still fails a matcher that leaks.
//
//   BOUNDARY the hostname word-boundary test: a machine name is a leak only when
//            it stands as its own token, never as a substring of a longer,
//            unrelated identifier.
//
//   CENSUS   the hostname needle set is non-empty after exemptions, and every
//            needle in it is one the matcher can actually fire on. This stands in
//            for the empty-census finding the scan carried while its needles were
//            harvested live: a literal cannot silently stop extracting, but it CAN
//            be emptied by edit, or exempted away to nothing, and either would
//            leave the scan passing every tree vacuously.
//
// The veil census is synthetic, so those tests hold in any tree — no filesystem,
// no git. The hostname census is the real literal: it is the thing under test.

use std::collections::BTreeSet;

use crate::rbvceu_loupe::{
    zrbvceu_class_names,
    zrbvceu_class_self_proof,
    zrbvceu_class_self_proof_by,
    zrbvceu_class_specimens,
    zrbvceu_exempt_from,
    zrbvceu_Class,
    zrbvceu_Finding,
    zrbvceu_names_token,
    zrbvceu_veil_leak,
    zrbvceu_veil_scan_text,
    zrbvceu_veil_self_proof,
    zrbvceu_veil_self_proof_by,
    ZRBVCEU_HOST_CENSUS,
    ZRBVCEU_HOST_EXEMPT, //
};

/// A synthetic one-document census: the withheld-basename half of the matcher is
/// proved against this, never against the live tree.
fn zrbvteu_census() -> BTreeSet<String> {
    ["ZZQ-Example.adoc".to_string()].into_iter().collect()
}

fn zrbvteu_scan(probe: &str, census: &BTreeSet<String>) -> Vec<zrbvceu_Finding> {
    let mut hits = Vec::new();
    zrbvceu_veil_scan_text("probe", probe, census, &mut hits);
    hits
}

// ── The content matcher ─────────────────────────────────────

/// Lines that name a withheld thing — the veiled-dir token, or the census
/// document's basename in a citation form. Each must produce at least one hit.
const ZRBVTEU_SCAN_PLANTED: &[&str] = &[
    "  - see vov_veiled/whatever.sh for the rule",
    "# Contract: ZZQ-Example.adoc.",
    "- **ZZQ**  → `zzk/vov_veiled/ZZQ-Example.adoc` (a maintainer-context row)", //
];

/// Benign lines a coarser scanner might redden on: a shipping README mention, a
/// prose line with no withheld name, and a same-stem non-document extension.
const ZRBVTEU_SCAN_CLEAN: &[&str] = &[
    "start with the README.md at the project root",
    "the terrier records which citizens hold which mantles",
    "ZZQ-Example.txt is not a withheld document", //
];

#[test]
fn rbvteu_veil_scan_catches_planted_leaks() {
    let census = zrbvteu_census();
    for planted in ZRBVTEU_SCAN_PLANTED {
        let hits = zrbvteu_scan(planted, &census);
        assert!(
            !hits.is_empty(),
            "the veil matcher missed a planted leak: {:?} — it would ride a regulus",
            planted, //
        );
    }
}

#[test]
fn rbvteu_veil_scan_silent_on_clean_lines() {
    let census = zrbvteu_census();
    for clean in ZRBVTEU_SCAN_CLEAN {
        let hits = zrbvteu_scan(clean, &census);
        assert!(
            hits.is_empty(),
            "the veil matcher reddened on a benign line {:?}: {:?} — a matcher that cries wolf gets disabled",
            clean,
            hits.iter().map(|f| f.detail.clone()).collect::<Vec<_>>(), //
        );
    }
}

// ── The self-proof ──────────────────────────────────────────

/// The self-proof the live-tree verdict rests on must run clean: an empty result
/// means the matcher catches every planted positive and reddens on no negative.
#[test]
fn rbvteu_veil_self_proof_holds() {
    let findings = zrbvceu_veil_self_proof();
    assert!(
        findings.is_empty(),
        "the veil matcher self-proof failed — the live-tree verdict cannot be trusted: {:?}",
        findings
            .iter()
            .map(|f| f.detail.clone())
            .collect::<Vec<_>>(), //
    );
}

// ── The empty census ────────────────────────────────────────

/// A matcher that leaks: it reads every line and reports nothing.
fn zrbvteu_blind_scan(
    _rel: &str,
    _text: &str,
    _census: &BTreeSet<String>,
    _findings: &mut Vec<zrbvceu_Finding>,
) {
}

/// The veiled tree holds no document, so the live census is empty, and the
/// assay admits it rather than reddening a clean tree: over a root holding no
/// veiled tree and no shipping file at all, the leak scan raises nothing.
#[test]
fn rbvteu_veil_leak_admits_an_empty_census() {
    let root = std::env::temp_dir().join(format!(
        "rbvteu-absent-root-{}",
        std::process::id(), //
    ));
    assert!(
        !root.exists(),
        "the absent root {:?} stands — the hurdle needs a tree holding nothing",
        root, //
    );
    let findings = zrbvceu_veil_leak(&root);
    assert!(
        findings.is_empty(),
        "an empty census reddened the assay: {:?}",
        findings
            .iter()
            .map(|f| f.detail.clone())
            .collect::<Vec<_>>(), //
    );
}

/// The walked roots say where shipping files live, never that everything under
/// them ships: the perambulation is the one judge. A withheld file inside a
/// walked root may name the veiled tree, and the same line in a shipped file
/// beside it is still a leak.
#[test]
fn rbvteu_veil_leak_defers_to_the_perambulation() {
    let root = std::env::temp_dir().join(format!(
        "rbvteu-perambulation-root-{}",
        std::process::id(), //
    ));
    let withheld = "rbmm_moorings/bkd_weaned/rbvn_app/bkrr.env";
    let shipped = "rbmm_moorings/rben_tadmor/rbrn.env";
    let line = "BKRR_MANIFEST=\"vov_veiled/rbv0/Cargo.toml\"\n";
    for rel in [withheld, shipped] {
        let path = root.join(rel);
        let parent = path.parent().expect("a planted path has a parent");
        std::fs::create_dir_all(parent).expect("plant the directory");
        std::fs::write(&path, line).expect("plant the file");
    }
    let findings = zrbvceu_veil_leak(&root);
    let _ = std::fs::remove_dir_all(&root);
    let named: Vec<&str> = findings.iter().map(|f| f.file.as_str()).collect();
    assert!(
        named.contains(&shipped),
        "the shipped file naming the veiled tree went unflagged: {:?}",
        named, //
    );
    assert!(
        !named.contains(&withheld),
        "the withheld file was flagged though it never ships: {:?}",
        named, //
    );
}

/// The admission rests on the self-proof, so the proof must still fail a matcher
/// that leaks: every planted positive it is handed goes missed, and each is named.
#[test]
fn rbvteu_veil_self_proof_fails_a_leaking_matcher() {
    let findings = zrbvceu_veil_self_proof_by(zrbvteu_blind_scan);
    assert!(
        !findings.is_empty(),
        "the self-proof passed a matcher that catches nothing", //
    );
    assert!(
        findings
            .iter()
            .all(|f| f.detail.starts_with("veil matcher missed a known leak")),
        "the self-proof failed a leaking matcher for the wrong reason: {:?}",
        findings
            .iter()
            .map(|f| f.detail.clone())
            .collect::<Vec<_>>(), //
    );
}

// ── The shape classes ───────────────────────────────────────

/// A matcher that leaks: it names nothing.
fn zrbvteu_blind_class(_class: zrbvceu_Class, _line: &str) -> bool {
    false
}

fn zrbvteu_class_proves(class: zrbvceu_Class) {
    let (planted, clean) = zrbvceu_class_specimens(class);
    assert!(
        !planted.is_empty() && !clean.is_empty(),
        "class {:?} carries no specimens", //
        class,
    );
    for line in planted {
        assert!(
            zrbvceu_class_names(class, line),
            "class {:?} missed a planted leak: {:?}",
            class,
            line, //
        );
    }
    for line in clean {
        assert!(
            !zrbvceu_class_names(class, line),
            "class {:?} reddened on its clean twin: {:?}",
            class,
            line, //
        );
    }
}

#[test]
fn rbvteu_token_class_fires_on_plant_and_spares_clean() {
    zrbvteu_class_proves(zrbvceu_Class::Token);
}

#[test]
fn rbvteu_memo_class_fires_on_plant_and_spares_clean() {
    zrbvteu_class_proves(zrbvceu_Class::Memo);
}

#[test]
fn rbvteu_doc_class_fires_on_plant_and_spares_clean() {
    zrbvteu_class_proves(zrbvceu_Class::Doc);
}

#[test]
fn rbvteu_class_self_proof_holds() {
    let findings = zrbvceu_class_self_proof();
    assert!(
        findings.is_empty(),
        "the shape classes' self-proof failed: {:?}",
        findings
            .iter()
            .map(|f| f.detail.clone())
            .collect::<Vec<_>>(), //
    );
}

/// Handed a matcher that leaks, the proof fails for each class by name.
#[test]
fn rbvteu_class_self_proof_fails_a_leaking_matcher() {
    let findings = zrbvceu_class_self_proof_by(zrbvteu_blind_class);
    for class in ["Token", "Memo", "Doc"] {
        assert!(
            findings.iter().any(|f| f
                .detail
                .starts_with(&format!("{} class matcher missed a known leak", class))),
            "the self-proof passed a leaking matcher for class {}: {:?}",
            class,
            findings
                .iter()
                .map(|f| f.detail.clone())
                .collect::<Vec<_>>(), //
        );
    }
}

/// The four standing rows excuse the veiled-dir class alone, and no other.
#[test]
fn rbvteu_standing_exemptions_keep_only_the_veil_class() {
    for rel in [
        "Tools/rbk/rbtd/src/rbtdrq_damnatio.rs",
        "Tools/rbk/rbtd/src/rbtdrn_conformance.rs",
        "rbmm_moorings/buml_launchers/launcher.rbvw_workbench.sh",
    ] {
        assert!(zrbvceu_exempt_from(rel, zrbvceu_Class::Veil), "{rel} lost its veil row");
        for class in [
            zrbvceu_Class::Token,
            zrbvceu_Class::Memo,
            zrbvceu_Class::Doc,
        ] {
            assert!(
                !zrbvceu_exempt_from(rel, class),
                "{} is excused from {:?} — a standing row widened",
                rel,
                class, //
            );
        }
    }
}

/// A store-token line planted in an excused residue path is not named for that
/// class, while the same line in a non-excused shipped path is; and the excuse
/// is per class, so a memo-shaped line in the same residue path is still named.
#[test]
fn rbvteu_residue_excuses_its_class_only() {
    let root = std::env::temp_dir().join(format!(
        "rbvteu-residue-root-{}",
        std::process::id(), //
    ));
    let residue = "Tools/buk/buym_yelp.sh";
    let shipped = "Tools/buk/bud_dispatch.sh";
    let token_line = "STAND=../jjqz_example\n";
    let memo_line = "NOTE=memo-29990101-example-slug\n";
    for (rel, text) in [
        (residue, format!("{}{}", token_line, memo_line)),
        (shipped, token_line.to_string()),
    ] {
        let path = root.join(rel);
        let parent = path.parent().expect("a planted path has a parent");
        std::fs::create_dir_all(parent).expect("plant the directory");
        std::fs::write(&path, text).expect("plant the file");
    }
    let findings = zrbvceu_veil_leak(&root);
    let _ = std::fs::remove_dir_all(&root);
    let named = |file: &str, detail: &str| {
        findings
            .iter()
            .any(|f| f.file == file && f.detail == detail)
    };
    assert!(
        named(shipped, "names an internal path token"),
        "the shipped file carrying a store token went unflagged: {:?}",
        findings
            .iter()
            .map(|f| (f.file.clone(), f.detail.clone()))
            .collect::<Vec<_>>(), //
    );
    assert!(
        !named(residue, "names an internal path token"),
        "the excused residue path was flagged for its excused class", //
    );
    assert!(
        named(residue, "names a dated internal name"),
        "the residue path's excuse spread to a class it does not name", //
    );
}

// ── The hostname word boundary ──────────────────────────────

/// A machine name is a leak only as its own token — never as a substring of a
/// longer, unrelated identifier.
#[test]
fn rbvteu_hostname_census_is_live() {
    let effective: Vec<&str> = ZRBVCEU_HOST_CENSUS
        .iter()
        .copied()
        .filter(|name| !ZRBVCEU_HOST_EXEMPT.contains(name))
        .collect();

    assert!(
        !effective.is_empty(),
        "the hostname census is empty after exemptions — the scan would pass every tree vacuously", //
    );

    // A needle the matcher cannot fire on is a needle that protects nothing: an
    // empty or whitespace-bearing entry would sit in the list looking like cover.
    for name in &effective {
        assert!(
            zrbvceu_names_token(
                &format!(
                    "host is {} today",
                    name, //
                ),
                name
            ),
            "census entry {:?} is not a token the matcher can catch",
            name, //
        );
    }
}

#[test]
fn rbvteu_hostname_names_token_word_boundary() {
    assert!(
        zrbvceu_names_token("host is falcon today", "falcon"),
        "a whole-word match is a leak", //
    );
    assert!(
        zrbvceu_names_token("falcon", "falcon"),
        "the bare token is a leak", //
    );
    assert!(
        zrbvceu_names_token("BURN_HOST=falcon", "falcon"),
        "a value after = is a leak", //
    );
    assert!(
        !zrbvceu_names_token("the falconry mews stands empty", "falcon"),
        "a substring of a longer word is not a leak", //
    );
    assert!(
        !zrbvceu_names_token("gyrfalcon circles", "falcon"),
        "a trailing substring of a longer word is not a leak", //
    );
}
