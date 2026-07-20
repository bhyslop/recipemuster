// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrds_spine::{
    jjrds_billet_dirname,
    jjrds_board,
    jjrds_currency,
    jjrds_pair_admitted,
    jjrds_pedigree_lookup,
    jjrds_plan,
    jjrds_resolve_launch,
    jjrds_resolve_saddle,
    jjrds_roster_row,
    jjrds_staleness_notice,
    jjrds_stirrup_command,
    jjrds_type_target,
    jjrds_Door,
    jjrds_Rejection,
    jjrds_Target,
    jjrds_TierRow,
    JJRDS_CONDUCT_CORE,
    JJRDS_JUDGMENT_EFFORT,
    JJRDS_JUDGMENT_TIER,
    JJRDS_KIND_PLAIN_GIT,
    JJRDS_PEDIGREES_REL_PATH,
    JJRDS_TIER_ROSTER,
};
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_LineOfWork,
};
use super::jjrt_types::{
    jjrg_Effort,
    jjrg_Gallops,
    jjrg_Heat,
    jjrg_HeatStatus,
    jjrg_Pace,
    jjrg_PaceState,
    jjrg_Tack,
    jjrg_Tier,
};
use super::jjrvb_blotter::{
    jjdb_pin,
    jjdb_BlotterConfig,
    JJDB_CATCHWORD_FOUNDING,
    JJDB_CATCHWORD_SIGIL,
    JJDB_STUDBOOK_DIRNAME,
};
use super::jjtu_testdir::JjkTestDir;
use std::path::{
    Path,
    PathBuf,
};
use std::time::Duration;

const ZJJTDS_TRUNK: &str = "jjtds-trunk";

// ---- Scaffolding ----

fn zjjtds_git(dir: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("test harness git invocation must spawn");
    assert!(
        out.status.success(),
        "test harness git -C {} {:?} failed: {}",
        dir.display(),
        args,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("git stdout must be UTF-8").trim().to_string()
}

fn zjjtds_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtds_git(dir, &["add", "--", name]);
    zjjtds_git(dir, &["commit", "-q", "-m", message]);
    zjjtds_git(dir, &["rev-parse", "HEAD"])
}

fn zjjtds_tack(state: jjrg_PaceState, tier: Option<jjrg_Tier>, effort: Option<jjrg_Effort>) -> jjrg_Tack {
    jjrg_Tack {
        ts: "260712-1200".to_string(),
        state,
        tier,
        effort,
        text: vec!["a docket line".to_string()],
        silks: "spine-test-pace".to_string(),
        basis: "0000000".to_string(),
    }
}

/// A gallops with one heat ₣AA: order runs a complete pace, then a rough one,
/// then a sonnet/high-bridled one — enough shape for both saddle resolutions.
fn zjjtds_gallops() -> jjrg_Gallops {
    let mut paces = std::collections::BTreeMap::new();
    paces.insert("₢AAAAC".to_string(), jjrg_Pace {
        tacks: vec![zjjtds_tack(jjrg_PaceState::Complete, Some(jjrg_Tier::Sonnet), None)],
        ..Default::default()
    });
    paces.insert("₢AAAAA".to_string(), jjrg_Pace {
        tacks: vec![zjjtds_tack(jjrg_PaceState::Rough, None, None)],
        ..Default::default()
    });
    paces.insert("₢AAAAB".to_string(), jjrg_Pace {
        tacks: vec![zjjtds_tack(jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), Some(jjrg_Effort::High))],
        ..Default::default()
    });
    let mut heats = std::collections::BTreeMap::new();
    heats.insert("₣AA".to_string(), jjrg_Heat {
        silks: "spine-test-heat".to_string(),
        creation_time: "260712".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec!["₢AAAAC".to_string(), "₢AAAAA".to_string(), "₢AAAAB".to_string()],
        paces,
    });
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec!["₣AA".to_string()],
        heats,
        retention_since: None,
    }
}

fn zjjtds_write_pedigrees(studbook_root: &Path, address: &str, kind: &str) {
    std::fs::create_dir_all(studbook_root).unwrap();
    let body = serde_json::json!({
        "jjop_sires": [{
            "jjop_kind": kind,
            "jjop_addresses": [address],
            "jjop_trunk": ZJJTDS_TRUNK,
        }]
    });
    std::fs::write(studbook_root.join(JJRDS_PEDIGREES_REL_PATH), serde_json::to_vec_pretty(&body).unwrap()).unwrap();
}

/// A full infield: bare upstream, a hippodrome clone tracking it, a studbook
/// whose one pedigree records the upstream, and a valid gallops in the
/// hippodrome's frozen store. Returns the infield guard and the hippodrome root.
fn zjjtds_infield(name: &str) -> (JjkTestDir, std::path::PathBuf) {
    let infield = JjkTestDir::new(name);
    let bare = infield.path().join("upstream");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtds_git(&bare, &["init", "-q", "--bare", "-b", ZJJTDS_TRUNK]);

    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtds_git(&hippodrome, &["init", "-q", "-b", ZJJTDS_TRUNK]);
    zjjtds_git(&hippodrome, &["config", "user.email", "jjtds@example.invalid"]);
    zjjtds_git(&hippodrome, &["config", "user.name", "jjtds"]);
    zjjtds_commit_all(&hippodrome, "base.txt", "base", "init");
    let bare_url = bare.to_string_lossy().into_owned();
    zjjtds_git(&hippodrome, &["remote", "add", "origin", &bare_url]);
    zjjtds_git(&hippodrome, &["push", "-q", "-u", "origin", ZJJTDS_TRUNK]);

    // The pedigree records the kind-canonical key — what identify derives from
    // the remote URL (no `.git` suffix here, so the URL is its own key).
    zjjtds_write_pedigrees(&infield.path().join(JJDB_STUDBOOK_DIRNAME), &bare_url, JJRDS_KIND_PLAIN_GIT);

    let jjm = hippodrome.join(".claude/jjm");
    std::fs::create_dir_all(&jjm).unwrap();
    crate::jjri_io::jjdr_save(&zjjtds_gallops(), &jjm.join("jjg_gallops.json")).unwrap();

    (infield, hippodrome)
}

// ---- Halter typing ----

#[test]
fn jjtds_type_target_types_by_length_with_glyphs_tolerated() {
    assert_eq!(jjrds_type_target("AA").unwrap(), jjrds_Target::Firemark("AA".to_string()));
    assert_eq!(jjrds_type_target("₣AA").unwrap(), jjrds_Target::Firemark("AA".to_string()));
    assert_eq!(jjrds_type_target("AAAAB").unwrap(), jjrds_Target::Coronet("AAAAB".to_string()));
    assert_eq!(jjrds_type_target("₢AAAAB").unwrap(), jjrds_Target::Coronet("AAAAB".to_string()));
    // A qualified emission: glyph + heat + interpunct + id — the tail resolves.
    assert_eq!(jjrds_type_target("₢AA·AAAAB").unwrap(), jjrds_Target::Coronet("AAAAB".to_string()));
    assert!(matches!(jjrds_type_target("AAA"), Err(jjrds_Rejection::BadTarget { .. })));
}

// ---- Two-source launch choice ----

#[test]
fn jjtds_resolve_launch_takes_a_designation_exactly() {
    assert_eq!(
        jjrds_resolve_launch(Some((jjrg_Tier::Sonnet, Some(jjrg_Effort::High)))),
        (jjrg_Tier::Sonnet, Some(jjrg_Effort::High))
    );
    // A designation without effort launches with the knob omitted — the vendor
    // default governs; the judgment effort must NOT leak in.
    assert_eq!(jjrds_resolve_launch(Some((jjrg_Tier::Fable, None))), (jjrg_Tier::Fable, None));
}

#[test]
fn jjtds_resolve_launch_defaults_to_the_judgment_constant() {
    assert_eq!(jjrds_resolve_launch(None), (JJRDS_JUDGMENT_TIER, Some(JJRDS_JUDGMENT_EFFORT)));
    assert_eq!(JJRDS_JUDGMENT_TIER, jjrg_Tier::Opus);
    assert_eq!(JJRDS_JUDGMENT_EFFORT, jjrg_Effort::Xhigh);
}

#[test]
fn jjtds_tier_roster_is_total_over_the_designable_families() {
    for family in [jjrg_Tier::Haiku, jjrg_Tier::Sonnet, jjrg_Tier::Opus, jjrg_Tier::Fable] {
        let row = jjrds_roster_row(family);
        assert_eq!(row.family, family);
        assert!(!row.model_id.is_empty());
    }
    assert_eq!(JJRDS_TIER_ROSTER.len(), 4);
}

#[test]
fn jjtds_pair_admitted_gates_on_the_row_effort_set() {
    let restricted = jjrds_TierRow {
        family: jjrg_Tier::Haiku,
        model_id: "test-only",
        efforts: &[jjrg_Effort::Low],
    };
    assert!(jjrds_pair_admitted(&restricted, jjrg_Effort::Low));
    assert!(!jjrds_pair_admitted(&restricted, jjrg_Effort::Max));
    assert!(jjrds_pair_admitted(jjrds_roster_row(jjrg_Tier::Opus), jjrg_Effort::Xhigh));
}

// ---- Stirrup ----

#[test]
fn jjtds_stirrup_composes_the_launch_command() {
    let billet = Path::new("/tmp/jjtds-billet");
    let mcp = Path::new("/tmp/jjtds-scratch/mcp.json");
    let scratch = Path::new("/tmp/jjtds-scratch");

    let cmd = jjrds_stirrup_command(billet, jjrg_Tier::Sonnet, Some(jjrg_Effort::High), "mount ₢AAAAB", mcp, scratch)
        .unwrap();

    assert_eq!(cmd.get_program(), "claude");
    let args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().into_owned()).collect();
    assert_eq!(cmd.get_current_dir(), Some(billet));
    assert!(args.windows(2).any(|w| w[0] == "--model" && w[1] == "claude-sonnet-5"));
    assert!(args.windows(2).any(|w| w[0] == "--effort" && w[1] == "high"));
    assert!(args.windows(2).any(|w| w[0] == "--append-system-prompt" && w[1] == JJRDS_CONDUCT_CORE));
    assert_eq!(args.last().map(String::as_str), Some("mount ₢AAAAB"), "the opening prompt rides last");
    let envs: Vec<(String, String)> = cmd
        .get_envs()
        .filter_map(|(k, v)| v.map(|v| (k.to_string_lossy().into_owned(), v.to_string_lossy().into_owned())))
        .collect();
    for knob in ["BURV_OUTPUT_ROOT_DIR", "BURV_TEMP_ROOT_DIR", "BURV_LOG_DIR"] {
        assert!(envs.iter().any(|(k, v)| k == knob && v.starts_with("/tmp/jjtds-scratch")), "{} must export under the scratch root", knob);
    }
}

#[test]
fn jjtds_stirrup_omits_the_effort_knob_when_undesignated() {
    let cmd = jjrds_stirrup_command(
        Path::new("/tmp/jjtds-billet"),
        jjrg_Tier::Fable,
        None,
        "groom ₣AA",
        Path::new("/tmp/mcp.json"),
        Path::new("/tmp/scratch"),
    )
    .unwrap();
    let args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().into_owned()).collect();
    assert!(!args.iter().any(|a| a == "--effort"), "no effort designated → the vendor default governs");
    assert!(args.windows(2).any(|w| w[0] == "--model" && w[1] == "claude-fable-5"));
}

// ---- Saddle resolution ----

#[test]
fn jjtds_resolve_saddle_by_coronet_reads_the_designation() {
    let gallops = zjjtds_gallops();

    let rough = jjrds_resolve_saddle(&gallops, &jjrds_Target::Coronet("AAAAA".to_string())).unwrap();
    assert_eq!(rough.coronet, "AAAAA");
    assert_eq!(rough.designation, None);

    let bridled = jjrds_resolve_saddle(&gallops, &jjrds_Target::Coronet("AAAAB".to_string())).unwrap();
    assert_eq!(bridled.designation, Some((jjrg_Tier::Sonnet, Some(jjrg_Effort::High))));
}

#[test]
fn jjtds_resolve_saddle_rejects_terminal_and_unknown_paces() {
    let gallops = zjjtds_gallops();
    assert!(matches!(
        jjrds_resolve_saddle(&gallops, &jjrds_Target::Coronet("AAAAC".to_string())),
        Err(jjrds_Rejection::BadTarget { .. })
    ));
    assert!(matches!(
        jjrds_resolve_saddle(&gallops, &jjrds_Target::Coronet("AAZZZ".to_string())),
        Err(jjrds_Rejection::BadTarget { .. })
    ));
}

#[test]
fn jjtds_resolve_saddle_by_firemark_lands_on_the_next_actionable_pace() {
    let gallops = zjjtds_gallops();

    // Order leads with a complete pace; resolution must land on the rough one
    // behind it and return the BARE body — the billet branch is machine context.
    let saddled = jjrds_resolve_saddle(&gallops, &jjrds_Target::Firemark("AA".to_string())).unwrap();
    assert_eq!(saddled.coronet, "AAAAA");
}

// ---- Pedigree lookup ----

#[test]
fn jjtds_pedigree_lookup_resolves_kind_checks_and_rejects() {
    let td = JjkTestDir::new("jjtds_pedigree_lookup");
    let studbook_root = td.path().join(JJDB_STUDBOOK_DIRNAME);
    zjjtds_write_pedigrees(&studbook_root, "ssh://example.invalid/repo", JJRDS_KIND_PLAIN_GIT);
    let config = jjdb_BlotterConfig {
        local_root: studbook_root.clone(),
        remote_url: "unused".to_string(),
        trunk: ZJJTDS_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    let pedigree = jjrds_pedigree_lookup(&config, "ssh://example.invalid/repo", JJRDS_KIND_PLAIN_GIT).unwrap();
    assert_eq!(pedigree.trunk, ZJJTDS_TRUNK);
    assert_eq!(pedigree.kind, JJRDS_KIND_PLAIN_GIT);

    assert!(matches!(
        jjrds_pedigree_lookup(&config, "ssh://example.invalid/other", JJRDS_KIND_PLAIN_GIT),
        Err(jjrds_Rejection::UnrecordedSire { .. })
    ));
    assert!(matches!(
        jjrds_pedigree_lookup(&config, "ssh://example.invalid/repo", "exotic-kind"),
        Err(jjrds_Rejection::RecordGroundDrift { .. })
    ));
}

#[test]
fn jjtds_pedigree_lookup_names_an_unfounded_studbook() {
    let td = JjkTestDir::new("jjtds_pedigree_unfounded");
    let config = jjdb_BlotterConfig {
        local_root: td.path().join(JJDB_STUDBOOK_DIRNAME),
        remote_url: "unused".to_string(),
        trunk: ZJJTDS_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    assert!(matches!(
        jjrds_pedigree_lookup(&config, "anything", JJRDS_KIND_PLAIN_GIT),
        Err(jjrds_Rejection::StudbookUnreadable { .. })
    ));
}

// ---- Staleness surfacing ----

#[test]
fn jjtds_staleness_notice_names_refit_only_when_outstripped() {
    let (infield, hippodrome) = zjjtds_infield("jjtds_staleness");
    let billet_root = infield.path().join(jjrds_billet_dirname("AAAAA"));
    jjrfg_PlainGit
        .jjrfr_billet_create(&hippodrome, &jjrfr_BilletBirth::Branch("AAAAA".to_string()), &billet_root, ZJJTDS_TRUNK)
        .unwrap();

    assert_eq!(jjrds_staleness_notice(&jjrfg_PlainGit, &billet_root, ZJJTDS_TRUNK).unwrap(), None);

    zjjtds_commit_all(&hippodrome, "b.txt", "moved", "trunk advances");
    zjjtds_git(&hippodrome, &["push", "-q", "origin", ZJJTDS_TRUNK]);
    let _ = jjrfg_PlainGit.jjrfr_glean(&billet_root);

    let notice = jjrds_staleness_notice(&jjrfg_PlainGit, &billet_root, ZJJTDS_TRUNK).unwrap();
    assert!(notice.as_deref().unwrap_or("").contains("refit"), "the warning must name the remedy");
}

// ---- Plan and board ----

#[test]
fn jjtds_plan_saddle_resolves_billet_tier_and_prompt() {
    let (infield, hippodrome) = zjjtds_infield("jjtds_plan_saddle");
    // identify's root comes back canonicalized (git resolves the macOS temp-dir
    // symlink), so the expected infield must canonicalize to compare.
    let infield_canon = infield.path().canonicalize().unwrap();

    // A rough pace takes the judgment constant; the billet is an infield peer
    // under the jjqb_ signet; the prompt carries the engagement verb.
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    assert_eq!(plan.billet_root, infield_canon.join("jjqb_AAAAA"));
    assert_eq!(plan.birth, jjrfr_BilletBirth::Branch("AAAAA".to_string()));
    assert_eq!((plan.tier, plan.effort), (jjrg_Tier::Opus, Some(jjrg_Effort::Xhigh)));
    assert_eq!(plan.opening_prompt, "mount ₢AAAAA");
    assert_eq!(plan.trunk, ZJJTDS_TRUNK);

    // A bridled pace launches its designation exactly.
    let bridled = jjrds_plan(jjrds_Door::Saddle, "₢AAAAB", &hippodrome, false).unwrap();
    assert_eq!((bridled.tier, bridled.effort), (jjrg_Tier::Sonnet, Some(jjrg_Effort::High)));

    // A firemark saddles the heat's next actionable pace.
    let by_heat = jjrds_plan(jjrds_Door::Saddle, "AA", &hippodrome, false).unwrap();
    assert_eq!(by_heat.billet_root, infield_canon.join("jjqb_AAAAA"));
}

#[test]
fn jjtds_plan_lunge_takes_a_firemark_only() {
    let (infield, hippodrome) = zjjtds_infield("jjtds_plan_lunge");
    let infield_canon = infield.path().canonicalize().unwrap();

    let plan = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();
    assert_eq!(plan.billet_root, infield_canon.join("jjqb_AA"));
    assert_eq!(plan.birth, jjrfr_BilletBirth::Detached);
    assert_eq!((plan.tier, plan.effort), (jjrg_Tier::Opus, Some(jjrg_Effort::Xhigh)));
    assert_eq!(plan.opening_prompt, "groom ₣AA");

    assert!(matches!(
        jjrds_plan(jjrds_Door::Lunge, "AAAAA", &hippodrome, false),
        Err(jjrds_Rejection::BadTarget { .. })
    ));
}

#[test]
fn jjtds_plan_rejects_foreign_ground_and_unrecorded_sires() {
    let foreign = JjkTestDir::new("jjtds_plan_foreign");
    assert!(matches!(
        jjrds_plan(jjrds_Door::Lunge, "AA", foreign.path(), false),
        Err(jjrds_Rejection::ForeignGround(_))
    ));

    let (infield, hippodrome) = zjjtds_infield("jjtds_plan_unrecorded");
    zjjtds_write_pedigrees(
        &infield.path().join(JJDB_STUDBOOK_DIRNAME),
        "ssh://example.invalid/some-other-sire",
        JJRDS_KIND_PLAIN_GIT,
    );
    assert!(matches!(
        jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false),
        Err(jjrds_Rejection::UnrecordedSire { .. })
    ));
}

#[test]
fn jjtds_board_creates_reuses_and_reseats_a_pace_billet() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_board_saddle");
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();

    // Birth.
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan).unwrap(), None);
    let seated = jjrfg_PlainGit.jjrfr_identify(&plan.billet_root).unwrap();
    assert_eq!(seated.line_of_work, jjrfr_LineOfWork::Branch("AAAAA".to_string()));

    // Reuse: a standing billet boards as-is.
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan).unwrap(), None);

    // Reap, then board again: the durable branch re-seats with its history.
    let wip = zjjtds_commit_all(&plan.billet_root, "wip.txt", "carried", "wip on the pace branch");
    jjrfg_PlainGit.jjrfr_billet_remove(&plan.billet_root).unwrap();
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan).unwrap(), None);
    assert_eq!(zjjtds_git(&plan.billet_root, &["rev-parse", "HEAD"]), wip);
}

#[test]
fn jjtds_board_re_detaches_a_groom_billet_and_surfaces_staleness_on_a_pace_billet() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_board_lunge");
    let groom = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &groom).unwrap(), None);

    // Trunk advances; the groom's re-board re-detaches to the fresh tip.
    zjjtds_commit_all(&hippodrome, "b.txt", "moved", "trunk advances");
    zjjtds_git(&hippodrome, &["push", "-q", "origin", ZJJTDS_TRUNK]);
    let _ = jjrfg_PlainGit.jjrfr_glean(&groom.billet_root);
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &groom).unwrap(), None);
    let advanced = zjjtds_git(&hippodrome, &["rev-parse", &format!("refs/remotes/origin/{}", ZJJTDS_TRUNK)]);
    assert_eq!(zjjtds_git(&groom.billet_root, &["rev-parse", "HEAD"]), advanced);

    // A pace billet born before the advance boards with the staleness notice.
    let pace = jjrds_plan(jjrds_Door::Saddle, "AAAAB", &hippodrome, false).unwrap();
    zjjtds_commit_all(&hippodrome, "c.txt", "moved again", "trunk advances again");
    // Board births at the counterpart (pre-fetch, still at the old tip), then
    // gleans — which reveals the newer trunk and trips the probe.
    let notice = {
        jjrds_board(&jjrfg_PlainGit, &pace).unwrap();
        zjjtds_git(&hippodrome, &["push", "-q", "origin", ZJJTDS_TRUNK]);
        jjrds_board(&jjrfg_PlainGit, &pace).unwrap()
    };
    assert!(notice.unwrap_or_default().contains("refit"));
}

// ---- Enabled path: reads over the studbook, and the door's currency step ----
//
// These exercise the `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED` seam through the
// parameter the door pins to the const (`over_studbook`) while the const itself
// stays false. The studbook is a real founded blotter (bare remote + a clone on
// `trunk`), so the pin and the `git show` ref-read run against real object
// databases.

const ZJJTDS_STUDBOOK_TRUNK: &str = "trunk";

/// Found a studbook blotter inside `infield` at the `jjqs_studbook` dirname: a
/// bare remote plus a clone on `trunk` whose origin/trunk carries a canonical
/// gallops.json and a pedigrees.json recording `sire_address`. Built by hand
/// (not `jjdb_found`) so the clone carries a git identity in the test
/// environment. Returns the blotter config `jjdb_studbook_config` would produce
/// for this infield's local clone (remote_url differs harmlessly — plan's
/// pinned reads never consult it).
fn zjjtds_found_studbook(infield: &Path, sire_address: &str) -> jjdb_BlotterConfig {
    let bare = infield.join("studbook_upstream");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtds_git(&bare, &["init", "-q", "--bare", "-b", ZJJTDS_STUDBOOK_TRUNK]);

    let local = infield.join(JJDB_STUDBOOK_DIRNAME);
    std::fs::create_dir_all(&local).unwrap();
    zjjtds_git(&local, &["init", "-q", "-b", ZJJTDS_STUDBOOK_TRUNK]);
    zjjtds_git(&local, &["config", "user.email", "jjtds@example.invalid"]);
    zjjtds_git(&local, &["config", "user.name", "jjtds-studbook"]);

    crate::jjri_io::jjdr_save(&zjjtds_gallops(), &local.join("gallops.json")).unwrap();
    let pedigrees = serde_json::json!({
        "jjop_sires": [{
            "jjop_kind": JJRDS_KIND_PLAIN_GIT,
            "jjop_addresses": [sire_address],
            "jjop_trunk": ZJJTDS_TRUNK,
        }]
    });
    std::fs::write(local.join(JJRDS_PEDIGREES_REL_PATH), serde_json::to_vec_pretty(&pedigrees).unwrap()).unwrap();

    zjjtds_git(&local, &["add", "--", "gallops.json", JJRDS_PEDIGREES_REL_PATH]);
    zjjtds_git(&local, &["commit", "-q", "-m", "found studbook"]);
    zjjtds_git(&local, &["remote", "add", "origin", &bare.to_string_lossy()]);
    zjjtds_git(&local, &["push", "-q", "-u", "origin", ZJJTDS_STUDBOOK_TRUNK]);

    jjdb_BlotterConfig {
        local_root: local,
        remote_url: bare.to_string_lossy().into_owned(),
        trunk: ZJJTDS_STUDBOOK_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    }
}

/// An infield for the enabled path: a sire bare + a hippodrome clone carrying NO
/// `.claude/jjm` at all, and a founded studbook whose one pedigree records the
/// sire. The absent in-repo gallops is the point — the enabled readers resolve
/// gallops AND pedigree from the studbook's pinned snapshot, so they must touch
/// neither the in-repo gallops nor the studbook working tree.
fn zjjtds_infield_over(name: &str) -> (JjkTestDir, PathBuf) {
    let infield = JjkTestDir::new(name);
    let bare = infield.path().join("upstream");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtds_git(&bare, &["init", "-q", "--bare", "-b", ZJJTDS_TRUNK]);

    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtds_git(&hippodrome, &["init", "-q", "-b", ZJJTDS_TRUNK]);
    zjjtds_git(&hippodrome, &["config", "user.email", "jjtds@example.invalid"]);
    zjjtds_git(&hippodrome, &["config", "user.name", "jjtds"]);
    zjjtds_commit_all(&hippodrome, "base.txt", "base", "init");
    let bare_url = bare.to_string_lossy().into_owned();
    zjjtds_git(&hippodrome, &["remote", "add", "origin", &bare_url]);
    zjjtds_git(&hippodrome, &["push", "-q", "-u", "origin", ZJJTDS_TRUNK]);

    zjjtds_found_studbook(infield.path(), &bare_url);
    (infield, hippodrome)
}

/// A standalone founded studbook, for the door's currency step in isolation.
/// Returns the guard, the bare remote's path, and the local clone's config.
fn zjjtds_studbook_only(name: &str) -> (JjkTestDir, PathBuf, jjdb_BlotterConfig) {
    let td = JjkTestDir::new(name);
    let config = zjjtds_found_studbook(td.path(), "ssh://example.invalid/sire");
    let bare = td.path().join("studbook_upstream");
    (td, bare, config)
}

#[test]
fn jjtds_plan_over_studbook_reads_both_from_the_pin_touching_no_worktree_gallops() {
    let (infield, hippodrome) = zjjtds_infield_over("jjtds_plan_over");
    let infield_canon = infield.path().canonicalize().unwrap();
    assert!(!hippodrome.join(".claude/jjm").exists(), "the fixture must carry no in-repo gallops");

    // Behind the seam (over_studbook = true, const still false), gallops and
    // pedigree both resolve from the studbook's pinned snapshot.
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, true).unwrap();
    assert_eq!(plan.billet_root, infield_canon.join("jjqb_AAAAA"));
    assert_eq!(plan.birth, jjrfr_BilletBirth::Branch("AAAAA".to_string()));
    assert_eq!((plan.tier, plan.effort), (jjrg_Tier::Opus, Some(jjrg_Effort::Xhigh)));
    assert_eq!(plan.opening_prompt, "mount ₢AAAAA");
    assert_eq!(plan.trunk, ZJJTDS_TRUNK, "the trunk comes from the pinned pedigree");

    // The frozen path has no in-repo gallops to read here, so it refuses — proof
    // the enabled read really came from the studbook, not a stray local store.
    assert!(matches!(
        jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false),
        Err(jjrds_Rejection::BadTarget { .. })
    ));
}

#[test]
fn jjtds_currency_glean_makes_a_consigned_write_visible_to_the_next_pin() {
    let (td, bare, config) = zjjtds_studbook_only("jjtds_currency_seen");
    let pin_before = jjdb_pin(&config).unwrap();

    // A second station clones the studbook remote and lands a new commit on trunk.
    let other = td.path().join("other_station");
    zjjtds_git(td.path(), &["clone", "-q", "-b", ZJJTDS_STUDBOOK_TRUNK, &bare.to_string_lossy(), &other.to_string_lossy()]);
    zjjtds_git(&other, &["config", "user.email", "jjtds@example.invalid"]);
    zjjtds_git(&other, &["config", "user.name", "jjtds-other"]);
    let new_tip = zjjtds_commit_all(&other, "note.txt", "a later write", "another station writes");
    zjjtds_git(&other, &["push", "-q", "origin", ZJJTDS_STUDBOOK_TRUNK]);

    // Before the door gleans, our clone's pin has not moved.
    assert_eq!(jjdb_pin(&config).unwrap(), pin_before, "no glean yet → the pin is stale");

    // The door's currency step gleans; the next pin now sees the consigned write.
    jjrds_currency(&jjrfg_PlainGit, &config, Duration::ZERO).unwrap();
    assert_eq!(jjdb_pin(&config).unwrap(), new_tip, "after the door glean the next pin is current");
    assert_ne!(new_tip, pin_before);
}

#[test]
fn jjtds_currency_refuses_on_an_unreachable_glean() {
    let (_td, _bare, config) = zjjtds_studbook_only("jjtds_currency_unreachable");
    zjjtds_git(&config.local_root, &["remote", "set-url", "origin", "/nonexistent/jjtds-nowhere"]);

    assert!(matches!(
        jjrds_currency(&jjrfg_PlainGit, &config, Duration::ZERO),
        Err(jjrds_Rejection::StudbookUnreachable { .. })
    ));
}

#[test]
fn jjtds_currency_refuses_a_flying_guidon_naming_the_holder() {
    let (_td, _bare, config) = zjjtds_studbook_only("jjtds_currency_guidon");
    let holder = "station=other op=writing";
    jjrfg_PlainGit.jjrfr_stake(&config.local_root, holder).unwrap();

    match jjrds_currency(&jjrfg_PlainGit, &config, Duration::ZERO) {
        Err(jjrds_Rejection::WriteInFlight { holder: seen }) => assert_eq!(seen, holder),
        other => panic!("a flying guidon must refuse WriteInFlight, naming the holder; got {:?}", other),
    }

    // Release the remote lock the test staked.
    jjrfg_PlainGit.jjrfr_pluck(&config.local_root, holder).unwrap();
}
