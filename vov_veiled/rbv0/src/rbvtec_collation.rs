// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTEC — the one-home corpus check's self-proofs, ported from the theurge
// conformance fixture when the corpus-bound cases were absorbed into the
// hierophant. They, not a green lap, are what prove the checkers and the
// locator.
//
//   CHECKERS  citation integrity and rivet-hoist placement each catch their
//             planted kinds and clear their sanctioned shapes, over in-memory
//             corpora that touch no tree.
//
//   REGISTER  the kraal register parse recovers every tree row by role, an
//             evolved register parses to the same rows, and a vintageless or
//             half-row register refuses.
//
//   LOCATOR   the locator's verdicts, each driven on a synthetic kraal composed
//             on disk: no register, a register claiming another tree, a
//             claiming register naming no corpus road, and a standing road.
//             This is why the lap may trust an absence: the absence arm is the
//             one most able to hide a locator that never finds anything.
//
//   ASSAY     the in-process assay's four arms, driven end to end over a
//             synthetic kraal: no register stands it down, a claiming register
//             with no usable road is unfollowable, a clean corpus runs clean,
//             and a planted broken citation is reported.
//
// Every kraal here is composed under the crate's own ignored build directory
// and the assay reads only what the test wrote there — never a live register,
// never a second repository.

use std::path::{
    Path,
    PathBuf, //
};

use crate::rbvcec_collation::{
    rbvcec_Reading,
    rbvcec_assay,
    zrbvcec_CorpusRoad,
    zrbvcec_Finding,
    zrbvcec_check_citations,
    zrbvcec_check_rivet_hoist,
    zrbvcec_locate_corpus_road,
    zrbvcec_parse_feodary,
    zrbvcec_real_path,
    zrbvcec_render,
    ZRBVCEC_CODEX_SHEAF,
    ZRBVCEC_CORPUS_ROAD_ROLES,
    ZRBVCEC_CORPUS_SPEC_SUBDIR,
    ZRBVCEC_FEODARY_BASENAME,
    ZRBVCEC_HOIST_INDEX_SHEAVES,
    ZRBVCEC_HOIST_WAIVERS, //
};

/// The revision value the synthetic registers below carry. The reader does not
/// gate on the value, only on the key standing, so this is no comparison
/// target.
const ZRBVTEC_FEODARY_REVISION: i64 = 1;

fn zrbvtec_report(hits: &[zrbvcec_Finding]) -> String {
    zrbvcec_render(hits).join("\n")
}

/// A fresh scratch root under the crate's own ignored build directory, never
/// the system temp directory, unique to this test and this run.
fn zrbvtec_scratch_root(tag: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after the epoch")
        .as_nanos();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!(
            "rbvtec_collation_{}_{}_{}",
            tag,
            std::process::id(),
            nanos, //
        ));
    std::fs::create_dir_all(&root).expect("scratch root must be creatable");
    std::fs::canonicalize(&root).expect("scratch root must canonicalize")
}

// ── The checkers ────────────────────────────────────────────

/// Citation integrity: a mapping-declared quoin with no anchor, a `{term}`
/// citation to a real anchor (clears), a `{term}` citation to nothing
/// (broken), and a bare `RBr_xxx` citation to nothing (broken) — all in one
/// synthetic corpus, none touching the live tree.
#[test]
fn rbvtec_citation_integrity_catches_each_kind() {
    let a = (
        "vov_veiled/RBSAA-fake.adoc",
        ":rbk_present:                 <<rbk_present,Present>>\n\
         :rbk_present_s:               <<rbk_present,Presents>>\n\
         :rbk_absent:                  <<rbk_absent,Absent>>\n\
         :rbk_listed:                  <<rbk_listed,Listed>>\n\
         :rbk_sameline:                <<rbk_sameline,Sameline>>\n\
         [[rbk_present]]\n\
         rbk_present:: defined here.\n\
         * [[rbk_listed]]\n\
         {rbk_listed}\n\
         lives on a list-bullet anchor line (JJS0's `* [[name]]` pattern).\n\
         [[rbk_sameline]] {rbk_sameline}:: anchor and term share one line (JJS0's `[[x]] {x}::` pattern).\n\
         Cites the present one {rbk_present}, its variant {rbk_present_s}, the list-anchored {rbk_listed}, the same-line {rbk_sameline}, and the broken one {rbk_missing}.\n\
         Also cites RBr_zzz which has no anchor.\n",
    );
    let exempt = (
        "vov_veiled/RBSAB-fake-exempt.adoc",
        ":rbk_exempt:                  <<rbk_exempt,Exempt>>\n",
    );
    let adoc = vec![a, exempt];
    let all = vec![a, exempt];
    // Synthetic roster, not the live one: the exemption mechanism stays proven
    // whatever the live roster holds — including empty.
    let hits = zrbvcec_check_citations(&adoc, &adoc, &all, &[exempt.0]);
    let mut got: Vec<&str> = hits.iter().map(|h| h.kind).collect();
    got.sort();
    let mut expected = vec![
        "unanchored-quoin",
        "broken-quoin-citation",
        "broken-rivet-citation",
    ];
    expected.sort();
    assert_eq!(
        got,
        expected,
        "citation kinds misjudged:\n{}",
        zrbvtec_report(&hits), //
    );
}

/// Rivet-hoist placement: a rivet cited from its own defining sheaf clears; the
/// same rivet cited from a second sheaf is flagged; a waived rivet cited
/// cross-sheaf clears despite the waiver being live; a rivet anchored in the
/// codex itself clears no matter how many sheaves cite it; a rivet cited from
/// a census/index sheaf clears regardless of home.
#[test]
fn rbvtec_rivet_hoist_flags_only_the_unhoisted() {
    let home = (
        "vov_veiled/RBSAA-home.adoc",
        "[[RBr_a11]]\nRBr_a11:: lives here.\nRBr_a11 cited again in its own sheaf.\n",
    );
    let sibling = (
        "vov_veiled/RBSAB-sibling.adoc",
        "Cites RBr_a11 from a different sheaf.\n",
    );
    let waived_home = (
        "vov_veiled/RBSAC-waived.adoc",
        "[[RBr_m4d]]\nRBr_m4d:: lives here.\n",
    );
    let waived_citer = (
        "vov_veiled/RBSAD-waived-citer.adoc",
        "Cites RBr_m4d from a different sheaf — waived.\n",
    );
    let codex_home = (
        ZRBVCEC_CODEX_SHEAF,
        "[[RBr_c0d]]\nRBr_c0d:: lives in the codex.\n",
    );
    let codex_citer = (
        "vov_veiled/RBSAE-codex-citer.adoc",
        "Cites RBr_c0d, already hoisted — clears.\n",
    );
    let index_citer = (
        ZRBVCEC_HOIST_INDEX_SHEAVES[0],
        "Cites RBr_a11 as a census entry, not consumption — clears.\n",
    );
    let adoc = vec![
        home,
        sibling,
        waived_home,
        waived_citer,
        codex_home,
        codex_citer,
        index_citer,
    ];

    let hits = zrbvcec_check_rivet_hoist(
        &adoc,
        ZRBVCEC_HOIST_WAIVERS,
        ZRBVCEC_CODEX_SHEAF,
        ZRBVCEC_HOIST_INDEX_SHEAVES,
    );
    assert!(
        hits.len() == 1 && hits[0].kind == "unhoisted-rivet" && hits[0].detail.contains("RBr_a11"),
        "expected exactly 1 unhoisted-rivet hit for RBr_a11 (RBr_m4d waived), got:\n{}",
        zrbvtec_report(&hits), //
    );
}

// ── The register ────────────────────────────────────────────

/// A kraal register of the shape the stile composes — three trees, the corpus
/// road among them, keys in the composed order. Held here rather than read off
/// a live outspan so the parse proves itself hermetically.
const ZRBVTEC_SELF_FEODARY: &str = r#"{
  "jjfd_revision": 1,
  "jjfd_trees": [
    {
      "jjfd_path": "/infield/kraal_billet_rb",
      "jjfd_role": "billet",
      "jjfd_sire": "rb"
    },
    {
      "jjfd_path": "/infield/kraal_billet_vulgate",
      "jjfd_role": "vulgate"
    },
    {
      "jjfd_path": "/infield/kraal_sumpter_jj",
      "jjfd_role": "sumpter",
      "jjfd_sire": "jj"
    }
  ],
  "jjfd_corrector_root": "/stand/correctors"
}"#;

/// Register parse, the reading half: every tree row is recovered with its path
/// and role, in document order, and the corpus road is found by role rather
/// than by position — the row order is the stile's, not this reader's contract.
#[test]
fn rbvtec_register_parse_recovers_rows() {
    let trees = zrbvcec_parse_feodary(ZRBVTEC_SELF_FEODARY).expect("well-formed register refused");
    assert_eq!(
        trees.len(),
        3,
        "expected 3 tree rows", //
    );
    assert!(
        trees[0].path == "/infield/kraal_billet_rb" && trees[0].role == "billet",
        "first row parsed as {} / {}",
        trees[0].path,
        trees[0].role, //
    );
    let road = trees
        .iter()
        .find(|t| ZRBVCEC_CORPUS_ROAD_ROLES.contains(&t.role.as_str()))
        .expect("corpus road not found by role");
    assert_eq!(
        road.path,
        "/infield/kraal_billet_vulgate",
        "corpus road parsed wrong", //
    );
}

/// Register parse, the refusing half. A register of a LATER vintage, carrying an
/// unknown top-level key as well, parses to the same rows a well-formed
/// register does — evolution is additive, and an addition must be invisible to
/// a reader that does not want it. The half-row bar is the one way left that a
/// tolerant reader would quietly resolve a corpus road it had no business
/// trusting. A register declaring no vintage at all, and one whose row carries
/// a path but no role, must each refuse; the sumpter row proves a NON-corpus
/// role is not mistaken for one.
#[test]
fn rbvtec_register_refusals_hold() {
    let evolved = ZRBVTEC_SELF_FEODARY.replace(
        "\"jjfd_revision\": 1",
        "\"jjfd_future_key\": true,\n  \"jjfd_revision\": 2",
    );
    let evolved_trees =
        zrbvcec_parse_feodary(&evolved).expect("a later-vintage register carrying an unknown key refused");
    let base_trees = zrbvcec_parse_feodary(ZRBVTEC_SELF_FEODARY).expect("well-formed register refused");
    assert!(
        evolved_trees.len() == base_trees.len()
            && evolved_trees
                .iter()
                .zip(base_trees.iter())
                .all(|(a, b)| a.path == b.path && a.role == b.role),
        "a later-vintage register carrying an unknown key parsed to different rows", //
    );

    let vintageless = ZRBVTEC_SELF_FEODARY.replace("\"jjfd_revision\": 1,", "");
    assert!(
        zrbvcec_parse_feodary(&vintageless).is_err(),
        "a register declaring no vintage parsed clean", //
    );

    let half_row = ZRBVTEC_SELF_FEODARY.replace("\"jjfd_role\": \"vulgate\"", "\"jjfd_kind\": \"vulgate\"");
    assert!(
        zrbvcec_parse_feodary(&half_row).is_err(),
        "a register with a roleless tree row parsed clean", //
    );

    let roadless = ZRBVTEC_SELF_FEODARY.replace("\"jjfd_role\": \"vulgate\"", "\"jjfd_role\": \"sumpter\"");
    let trees = zrbvcec_parse_feodary(&roadless).expect("roadless register refused at parse");
    assert!(
        !trees
            .iter()
            .any(|t| ZRBVCEC_CORPUS_ROAD_ROLES.contains(&t.role.as_str())),
        "a sumpter row was taken for a corpus road", //
    );
}

// ── Synthetic kraals ────────────────────────────────────────

/// Compose a synthetic kraal under `base`: a billet, and — where `register` is
/// given — an outspan holding it. Returns the billet root, which is what the
/// locator and the assay are asked about.
fn zrbvtec_kraal(base: &Path, name: &str, register: Option<&str>) -> PathBuf {
    let kraal = base.join(name);
    let billet = kraal.join("billet");
    std::fs::create_dir_all(&billet).expect("billet must be creatable");
    if let Some(body) = register {
        zrbvtec_write_register(&billet, body);
    }
    billet
}

/// Write `body` as the register in the outspan beside `billet`.
fn zrbvtec_write_register(billet: &Path, body: &str) {
    let outspan = billet.parent().expect("billet has a kraal").join("outspan");
    std::fs::create_dir_all(&outspan).expect("outspan must be creatable");
    std::fs::write(outspan.join(ZRBVCEC_FEODARY_BASENAME), body).expect("register must be writable");
}

/// A register claiming `billet`, carrying `extra` rows verbatim after it.
fn zrbvtec_register(billet: &Path, extra: &str) -> String {
    format!(
        "{{\n  \"jjfd_revision\": {},\n  \"jjfd_trees\": [\n    \
         {{ \"jjfd_path\": \"{}\", \"jjfd_role\": \"billet\", \"jjfd_sire\": \"rb\" }}{}\n  ]\n}}\n",
        ZRBVTEC_FEODARY_REVISION,
        billet.display(),
        extra, //
    )
}

/// A corpus road row for `road`.
fn zrbvtec_road_row(road: &Path) -> String {
    format!(
        ",\n    {{ \"jjfd_path\": \"{}\", \"jjfd_role\": \"vulgate\" }}",
        road.display(), //
    )
}

/// Write `content` at `rel` under `root`, making its parents.
fn zrbvtec_plant(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().expect("planted file has a parent")).expect("parent must be creatable");
    std::fs::write(&path, content).expect("planted file must be writable");
}

/// A kraal whose register claims its billet and names a corpus road standing
/// beside it, with one judged sheaf defining a quoin and a rivet and one
/// in-repo script citing the rivet — every citation resolves. Returns the
/// billet root and the road root.
fn zrbvtec_clean_kraal(base: &Path, name: &str) -> (PathBuf, PathBuf) {
    let billet = zrbvtec_kraal(base, name, None);
    let road = billet.parent().expect("billet has a kraal").join("vulgate");
    zrbvtec_plant(
        &road,
        &format!(
            "{}/ZZQ-home.adoc",
            ZRBVCEC_CORPUS_SPEC_SUBDIR, //
        ),
        ":zzq_present:                 <<zzq_present,Present>>\n\
         [[zzq_present]]\n\
         zzq_present:: defined here, and cited as {zzq_present}.\n\
         [[RBr_a11]]\n\
         RBr_a11:: lives here.\n",
    );
    zrbvtec_plant(
        &billet,
        "Tools/rbk/zzq_probe.sh",
        "# cites RBr_a11 only\nz_x=1\n",
    );
    std::fs::create_dir_all(billet.join("tt")).expect("tt must be creatable");
    zrbvtec_write_register(&billet, &zrbvtec_register(&billet, &zrbvtec_road_row(&road)));
    (billet, road)
}

// ── The locator ─────────────────────────────────────────────

/// The locator's four verdicts, each driven on a synthetic kraal composed on
/// disk — the real walk over a real parent directory, not a stubbed one.
#[test]
fn rbvtec_locator_tells_each_kraal_apart() {
    let base = zrbvtec_scratch_root("locator");

    // No register anywhere beside the billet: a station outside any kraal.
    let bare = zrbvtec_kraal(&base, "bare", None);
    match zrbvcec_locate_corpus_road(&bare) {
        Ok(zrbvcec_CorpusRoad::Absent(reason)) => assert!(
            reason.contains(ZRBVCEC_FEODARY_BASENAME),
            "registerless station stood down without naming what it looked for: {}",
            reason, //
        ),
        Ok(zrbvcec_CorpusRoad::Found(p)) => panic!(
            "registerless station resolved a road at {}",
            p.display(), //
        ),
        Err(e) => panic!(
            "registerless station failed: {}",
            e, //
        ),
    }

    // A register stands beside the billet but claims a different tree — another
    // kraal's business, and taking its corpus road would be the silent wrong
    // answer this locator exists to refuse.
    let foreign_body = zrbvtec_register(
        Path::new("/nowhere/that/stands"),
        ",\n    { \"jjfd_path\": \"/nowhere/that/stands/vulgate\", \"jjfd_role\": \"vulgate\" }",
    );
    let foreign = zrbvtec_kraal(&base, "foreign", Some(&foreign_body));
    match zrbvcec_locate_corpus_road(&foreign) {
        Ok(zrbvcec_CorpusRoad::Absent(_)) => {}
        Ok(zrbvcec_CorpusRoad::Found(p)) => panic!(
            "a register claiming another tree yielded a road at {}",
            p.display(), //
        ),
        Err(e) => panic!(
            "foreign register failed loud: {}",
            e, //
        ),
    }

    // A register that claims this billet and names no corpus road: a broken
    // kraal, loud rather than absent.
    let roadless = zrbvtec_kraal(&base, "roadless", None);
    zrbvtec_write_register(
        &roadless,
        &zrbvtec_register(
            &roadless,
            ",\n    { \"jjfd_path\": \"/some/sumpter\", \"jjfd_role\": \"sumpter\" }",
        ),
    );
    match zrbvcec_locate_corpus_road(&roadless) {
        Err(_) => {}
        Ok(zrbvcec_CorpusRoad::Absent(reason)) => panic!(
            "a claiming register naming no corpus road stood down instead of failing: {}",
            reason, //
        ),
        Ok(zrbvcec_CorpusRoad::Found(p)) => panic!(
            "roadless register yielded a road at {}",
            p.display(), //
        ),
    }

    // A register that claims this billet and names a road whose spec home
    // stands: the found case, and the road root is what comes back.
    let (good, road) = zrbvtec_clean_kraal(&base, "good");
    match zrbvcec_locate_corpus_road(&good) {
        Ok(zrbvcec_CorpusRoad::Found(p)) => assert_eq!(
            zrbvcec_real_path(&p),
            zrbvcec_real_path(&road),
            "located the wrong road", //
        ),
        Ok(zrbvcec_CorpusRoad::Absent(reason)) => panic!(
            "a standing corpus road stood down: {}",
            reason, //
        ),
        Err(e) => panic!(
            "a standing corpus road failed: {}",
            e, //
        ),
    }

    std::fs::remove_dir_all(&base).expect("scratch root must be removable");
}

// ── The assay, end to end ───────────────────────────────────

/// No register claims the tree: the assay stands down and says what it looked
/// for, so the lap can log the absence and proceed.
#[test]
fn rbvtec_assay_stands_down_without_a_register() {
    let base = zrbvtec_scratch_root("absent");
    let bare = zrbvtec_kraal(&base, "bare", None);
    match rbvcec_assay(&bare) {
        rbvcec_Reading::Absent(reason) => assert!(
            reason.contains(ZRBVCEC_FEODARY_BASENAME),
            "the stand-down names no register: {}",
            reason, //
        ),
        rbvcec_Reading::Unfollowable(e) => panic!(
            "a registerless tree read as unfollowable: {}",
            e, //
        ),
        rbvcec_Reading::Ran { tally, .. } => panic!(
            "a registerless tree ran: {}",
            tally, //
        ),
    }
    std::fs::remove_dir_all(&base).expect("scratch root must be removable");
}

/// A register that claims the tree and cannot be followed is unfollowable, both
/// where it names no corpus road and where its road stands with no judged
/// sheaf in it — never an absence the lap would wave past.
#[test]
fn rbvtec_assay_refuses_an_unfollowable_register() {
    let base = zrbvtec_scratch_root("unfollowable");

    let roadless = zrbvtec_kraal(&base, "roadless", None);
    zrbvtec_write_register(
        &roadless,
        &zrbvtec_register(
            &roadless,
            ",\n    { \"jjfd_path\": \"/some/sumpter\", \"jjfd_role\": \"sumpter\" }",
        ),
    );
    assert!(
        matches!(rbvcec_assay(&roadless), rbvcec_Reading::Unfollowable(_)),
        "a claiming register naming no corpus road was not unfollowable", //
    );

    let hollow = zrbvtec_kraal(&base, "hollow", None);
    let road = hollow.parent().expect("billet has a kraal").join("vulgate");
    std::fs::create_dir_all(road.join(ZRBVCEC_CORPUS_SPEC_SUBDIR)).expect("road must be creatable");
    zrbvtec_write_register(&hollow, &zrbvtec_register(&hollow, &zrbvtec_road_row(&road)));
    match rbvcec_assay(&hollow) {
        rbvcec_Reading::Unfollowable(e) => assert!(
            e.contains("no .adoc"),
            "the hollow road's refusal names no empty judged set: {}",
            e, //
        ),
        rbvcec_Reading::Absent(reason) => panic!(
            "a hollow road stood down: {}",
            reason, //
        ),
        rbvcec_Reading::Ran { tally, .. } => panic!(
            "a hollow road ran: {}",
            tally, //
        ),
    }

    std::fs::remove_dir_all(&base).expect("scratch root must be removable");
}

/// A claiming register over a clean corpus: both checks run, find nothing, and
/// the tally counts what was read — the judged sheaf and the in-repo script.
#[test]
fn rbvtec_assay_runs_clean_on_a_clean_corpus() {
    let base = zrbvtec_scratch_root("clean");
    let (billet, _) = zrbvtec_clean_kraal(&base, "clean");
    match rbvcec_assay(&billet) {
        rbvcec_Reading::Ran { findings, tally } => {
            assert!(
                findings.is_empty(),
                "a clean corpus reported:\n{}",
                findings.join("\n"), //
            );
            assert!(
                tally.contains("1 judged") && tally.contains("1 in-repo .sh"),
                "the tally does not count what was read: {}",
                tally, //
            );
        }
        rbvcec_Reading::Absent(reason) => panic!(
            "a claimed tree stood down: {}",
            reason, //
        ),
        rbvcec_Reading::Unfollowable(e) => panic!(
            "a clean kraal was unfollowable: {}",
            e, //
        ),
    }
    std::fs::remove_dir_all(&base).expect("scratch root must be removable");
}

/// The same clean kraal with one planted in-repo script citing a rivet no sheaf
/// anchors: the assay runs and reports exactly that citation.
#[test]
fn rbvtec_assay_reports_a_planted_finding() {
    let base = zrbvtec_scratch_root("planted");
    let (billet, _) = zrbvtec_clean_kraal(&base, "planted");
    zrbvtec_plant(&billet, "tt/zzq-planted.sh", "# cites RBr_zzz, anchored nowhere\n");
    match rbvcec_assay(&billet) {
        rbvcec_Reading::Ran { findings, .. } => assert!(
            findings.len() == 1
                && findings[0].starts_with("tt/zzq-planted.sh:1:")
                && findings[0].contains("broken-rivet-citation")
                && findings[0].contains("RBr_zzz"),
            "expected exactly the planted broken rivet citation, got:\n{}",
            findings.join("\n"), //
        ),
        rbvcec_Reading::Absent(reason) => panic!(
            "a claimed tree stood down: {}",
            reason, //
        ),
        rbvcec_Reading::Unfollowable(e) => panic!(
            "a planted kraal was unfollowable: {}",
            e, //
        ),
    }
    std::fs::remove_dir_all(&base).expect("scratch root must be removable");
}
