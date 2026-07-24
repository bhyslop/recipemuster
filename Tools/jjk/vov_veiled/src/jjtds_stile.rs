// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrds_stile::{
    jjrds_billet_dirname,
    jjrds_billet_identity,
    jjrds_board,
    jjrds_currency,
    jjrds_dispatch_record,
    jjrds_ground,
    jjrds_Ground,
    jjrds_pair_admitted,
    jjrds_pedigree_lookup,
    jjrds_plan,
    jjrds_record_dispatch,
    jjrds_rediscover,
    jjrds_resolve_launch,
    jjrds_resolve_saddle,
    jjrds_roster_row,
    jjrds_staleness_notice,
    jjrds_stirrup_command,
    jjrds_trailing_step,
    jjrds_type_target,
    jjrds_yard,
    jjrds_Door,
    jjrds_LaunchPlan,
    jjrds_Rejection,
    jjrds_Target,
    jjrds_TierRow,
    jjrds_Yard,
    JJRDS_CONDUCT_CORE,
    JJRDS_GROOM_POSTURE,
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
    jjrfr_RejectionKind,
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
    jjdb_studbook_config,
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
        silks: "stile-test-pace".to_string(),
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
        silks: "stile-test-heat".to_string(),
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

/// The yard step as the door runs it, with a caller-named serial standing in
/// for the dispatch record's catchword — so a boarding test drives the whole
/// rediscover-or-mint choice without a founded studbook to journal against.
fn zjjtds_yard(plan: &jjrds_LaunchPlan, catchword: u64) -> jjrds_Yard {
    let root = jjrds_rediscover(&jjrfg_PlainGit, plan)
        .unwrap()
        .unwrap_or_else(|| plan.infield_root.join(jjrds_billet_dirname(catchword, &plan.identity_body)));
    jjrds_yard(&plan.infield_root, root)
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
    assert!(args.windows(2).any(|w| w[0] == "--permission-mode" && w[1] == "auto"), "classifier-gated autonomy: out-of-billet acts route to the classifier, not the operator");
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
    let billet_root = infield.path().join(jjrds_billet_dirname(200500, "AAAAA"));
    jjrfg_PlainGit
        .jjrfr_billet_create(&hippodrome, &jjrfr_BilletBirth::Branch("jjls_pace/AAAAA".to_string()), &billet_root, ZJJTDS_TRUNK)
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
    // under the jjqb_ signet while its branch wears the livery badge — the two
    // surfaces diverge deliberately, the yard being JJ's own and the ref store
    // the sire's; the prompt carries the engagement verb.
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    assert_eq!(plan.identity_body, "AAAAA");
    assert_eq!(zjjtds_yard(&plan, 200500).billet_root, infield_canon.join("jjqb_200500_AAAAA"));
    assert_eq!(plan.birth, jjrfr_BilletBirth::Branch("jjls_pace/AAAAA".to_string()));
    assert_eq!((plan.tier, plan.effort), (jjrg_Tier::Opus, Some(jjrg_Effort::Xhigh)));
    assert_eq!(plan.opening_prompt, "mount ₢AAAAA");
    assert_eq!(plan.trunk, ZJJTDS_TRUNK);

    // A bridled pace launches its designation exactly.
    let bridled = jjrds_plan(jjrds_Door::Saddle, "₢AAAAB", &hippodrome, false).unwrap();
    assert_eq!((bridled.tier, bridled.effort), (jjrg_Tier::Sonnet, Some(jjrg_Effort::High)));

    // A firemark saddles the heat's next actionable pace.
    let by_heat = jjrds_plan(jjrds_Door::Saddle, "AA", &hippodrome, false).unwrap();
    assert_eq!(zjjtds_yard(&by_heat, 200500).billet_root, infield_canon.join("jjqb_200500_AAAAA"));
}

#[test]
fn jjtds_plan_lunge_takes_a_firemark_only() {
    let (infield, hippodrome) = zjjtds_infield("jjtds_plan_lunge");
    let infield_canon = infield.path().canonicalize().unwrap();

    let plan = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();
    assert_eq!(zjjtds_yard(&plan, 200501).billet_root, infield_canon.join("jjqb_200501_AA"));
    assert_eq!(plan.birth, jjrfr_BilletBirth::Detached);
    assert_eq!((plan.tier, plan.effort), (jjrg_Tier::Opus, Some(jjrg_Effort::Xhigh)));
    // The door's first impression: the verb, then the posture the engine repeats.
    assert!(plan.opening_prompt.starts_with("groom ₣AA"));
    assert!(plan.opening_prompt.contains(JJRDS_GROOM_POSTURE));

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

// ---- Ground ----

#[test]
fn jjtds_ground_reads_the_three_kinds_off_real_billets() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_ground_kinds");

    // The operator's own clone.
    assert_eq!(jjrds_ground(&jjrfg_PlainGit, &hippodrome), Some(jjrds_Ground::Hippodrome));

    // A pace billet, boarded by the saddle door: the coronet comes back off the
    // livery badge, bare.
    let pace = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let pace_yard = zjjtds_yard(&pace, 200500);
    jjrds_board(&jjrfg_PlainGit, &pace, &pace_yard).unwrap();
    assert_eq!(
        jjrds_ground(&jjrfg_PlainGit, &pace_yard.billet_root),
        Some(jjrds_Ground::PaceBillet { coronet: "AAAAA".to_string() })
    );

    // A groom billet, boarded by the lunge door.
    let groom = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();
    let groom_yard = zjjtds_yard(&groom, 200501);
    jjrds_board(&jjrfg_PlainGit, &groom, &groom_yard).unwrap();
    assert_eq!(jjrds_ground(&jjrfg_PlainGit, &groom_yard.billet_root), Some(jjrds_Ground::GroomBillet));
}

#[test]
fn jjtds_ground_calls_an_unbadged_partition_unboarded() {
    let (infield, hippodrome) = zjjtds_infield("jjtds_ground_unboarded");

    // A worktree the operator made by hand, on a branch JJ never authored:
    // a partition, but not a billet — so neither of the two billet kinds.
    let hand = infield.path().join("hand_made");
    zjjtds_git(&hippodrome, &["worktree", "add", "-q", "-b", "operator-branch", hand.to_str().unwrap()]);
    assert_eq!(
        jjrds_ground(&jjrfg_PlainGit, &hand),
        Some(jjrds_Ground::Unboarded { line: "operator-branch".to_string() })
    );

    // The reserved groom roster word parses, so its contract violation is
    // nameable rather than silently read as a line of work.
    let reserved = infield.path().join("reserved_word");
    zjjtds_git(&hippodrome, &["worktree", "add", "-q", "-b", "jjls_groom/AA", reserved.to_str().unwrap()]);
    assert_eq!(
        jjrds_ground(&jjrfg_PlainGit, &reserved),
        Some(jjrds_Ground::Unboarded { line: "jjls_groom/AA".to_string() })
    );
}

#[test]
fn jjtds_ground_declines_on_foreign_ground() {
    let foreign = JjkTestDir::new("jjtds_ground_foreign");
    assert_eq!(jjrds_ground(&jjrfg_PlainGit, foreign.path()), None);
}

#[test]
fn jjtds_board_creates_reuses_and_reseats_a_pace_billet() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_board_saddle");
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();

    // Birth: nothing stands, so the yard step mints.
    let born = zjjtds_yard(&plan, 200500);
    assert_eq!(born.billet_dirname, "jjqb_200500_AAAAA");
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan, &born).unwrap(), None);
    let seated = jjrfg_PlainGit.jjrfr_identify(&born.billet_root).unwrap();
    assert_eq!(seated.line_of_work, jjrfr_LineOfWork::Branch("jjls_pace/AAAAA".to_string()));

    // Reuse: rediscovery finds the standing billet by its seat, so a second
    // dispatch rejoins it under its ORIGINAL serial and never mints a second.
    let rejoined = zjjtds_yard(&plan, 999999);
    assert_eq!(rejoined.billet_dirname, "jjqb_200500_AAAAA", "reuse rediscovers rather than re-mints");
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan, &rejoined).unwrap(), None);

    // Reap, then board again: nothing stands to rediscover, so a fresh serial
    // is minted and the durable branch re-seats there with its history.
    let wip = zjjtds_commit_all(&born.billet_root, "wip.txt", "carried", "wip on the pace branch");
    jjrfg_PlainGit.jjrfr_billet_remove(&born.billet_root, false).unwrap();
    let reseated = zjjtds_yard(&plan, 200507);
    assert_eq!(reseated.billet_dirname, "jjqb_200507_AAAAA");
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan, &reseated).unwrap(), None);
    assert_eq!(zjjtds_git(&reseated.billet_root, &["rev-parse", "HEAD"]), wip);
}

/// A second station saddles a pace this station already worked and pushed: it
/// clones the sire, seats the pace's livery branch, commits, and publishes.
fn zjjtds_pace_worked_on_another_station(infield: &Path, name: &str, branch: &str) -> String {
    let other = infield.join(name);
    std::fs::create_dir_all(&other).unwrap();
    zjjtds_git(&other, &["clone", "-q", &infield.join("upstream").to_string_lossy(), "."]);
    zjjtds_git(&other, &["config", "user.email", "jjtds@example.invalid"]);
    zjjtds_git(&other, &["config", "user.name", "jjtds-other-station"]);
    zjjtds_git(&other, &["checkout", "-q", "-b", branch]);
    let tip = zjjtds_commit_all(&other, "abroad.txt", "worked elsewhere", "wip on another station");
    zjjtds_git(&other, &["push", "-q", "origin", branch]);
    tip
}

#[test]
fn jjtds_board_adopts_a_pace_line_another_station_pushed() {
    let (infield, hippodrome) = zjjtds_infield("jjtds_board_adopt");
    let abroad = zjjtds_pace_worked_on_another_station(infield.path(), "other_station", "jjls_pace/AAAAA");

    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200500);
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap(), None);

    // One pace, one line of work across stations: the billet stands ON the other
    // station's tip, not on a rival birth from trunk.
    assert_eq!(zjjtds_git(&yard.billet_root, &["rev-parse", "HEAD"]), abroad);
    assert_eq!(
        jjrfg_PlainGit.jjrfr_identify(&yard.billet_root).unwrap().line_of_work,
        jjrfr_LineOfWork::Branch("jjls_pace/AAAAA".to_string())
    );
}

#[test]
fn jjtds_board_births_from_trunk_when_no_line_stands_abroad() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_board_no_abroad");
    let trunk_tip = zjjtds_git(&hippodrome, &["rev-parse", &format!("refs/remotes/origin/{}", ZJJTDS_TRUNK)]);

    // Nothing pushed under the pace's livery badge anywhere: the adopt probe
    // answers no and boarding falls through to an ordinary birth.
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200500);
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap(), None);

    assert_eq!(zjjtds_git(&yard.billet_root, &["rev-parse", "HEAD"]), trunk_tip);
    assert_eq!(
        jjrfg_PlainGit.jjrfr_identify(&yard.billet_root).unwrap().line_of_work,
        jjrfr_LineOfWork::Branch("jjls_pace/AAAAA".to_string())
    );
}

#[test]
fn jjtds_board_re_detaches_a_groom_billet_and_surfaces_staleness_on_a_pace_billet() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_board_lunge");
    let groom = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();
    let groom_yard = zjjtds_yard(&groom, 200500);
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &groom, &groom_yard).unwrap(), None);

    // Trunk advances; boarding the same yard again re-detaches to the fresh tip.
    zjjtds_commit_all(&hippodrome, "b.txt", "moved", "trunk advances");
    zjjtds_git(&hippodrome, &["push", "-q", "origin", ZJJTDS_TRUNK]);
    let _ = jjrfg_PlainGit.jjrfr_glean(&groom_yard.billet_root);
    assert_eq!(jjrds_board(&jjrfg_PlainGit, &groom, &groom_yard).unwrap(), None);
    let advanced = zjjtds_git(&hippodrome, &["rev-parse", &format!("refs/remotes/origin/{}", ZJJTDS_TRUNK)]);
    assert_eq!(zjjtds_git(&groom_yard.billet_root, &["rev-parse", "HEAD"]), advanced);

    // A pace billet born before the advance boards with the staleness notice.
    let pace = jjrds_plan(jjrds_Door::Saddle, "AAAAB", &hippodrome, false).unwrap();
    let pace_yard = zjjtds_yard(&pace, 200501);
    zjjtds_commit_all(&hippodrome, "c.txt", "moved again", "trunk advances again");
    // Board births at the counterpart (pre-fetch, still at the old tip), then
    // gleans — which reveals the newer trunk and trips the probe.
    let notice = {
        jjrds_board(&jjrfg_PlainGit, &pace, &pace_yard).unwrap();
        zjjtds_git(&hippodrome, &["push", "-q", "origin", ZJJTDS_TRUNK]);
        jjrds_board(&jjrfg_PlainGit, &pace, &pace_yard).unwrap()
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
    assert_eq!(zjjtds_yard(&plan, 200500).billet_root, infield_canon.join("jjqb_200500_AAAAA"));
    assert_eq!(plan.birth, jjrfr_BilletBirth::Branch("jjls_pace/AAAAA".to_string()));
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

#[test]
fn jjtds_rediscover_names_the_livery_rename_for_a_pre_mint_billet() {
    // The mint's crossing: a billet seated before the livery badge sits on the
    // retired bare-body branch, so the registry seats the badge nowhere while
    // the yard carries a label for the pace. Rediscovery refuses rather than
    // minting past it — minting would birth a fresh branch off trunk and fork
    // the standing billet's work away in silence — and the refusal carries the
    // rename, this being the one drift with a known cause and a one-line fix.
    let (infield, hippodrome) = zjjtds_infield("jjtds_rediscover_pre_mint_billet");
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let pre_mint = infield.path().join("jjqb_AAAAA");
    jjrfg_PlainGit
        .jjrfr_billet_create(
            &hippodrome,
            &jjrfr_BilletBirth::Branch("AAAAA".to_string()),
            &pre_mint,
            ZJJTDS_TRUNK,
        )
        .unwrap();

    let rejection = jjrds_rediscover(&jjrfg_PlainGit, &plan).unwrap_err();
    let detail = rejection.to_string();
    assert!(detail.contains("predates the livery badge"), "got: {}", detail);
    assert!(detail.contains("branch -m jjls_pace/AAAAA"), "got: {}", detail);
}

// ---- The yard: serial labels and the tail-token read ----

#[test]
fn jjtds_billet_identity_reads_the_tail_token_past_any_serial() {
    // The minted shape, and the pre-catchword shape that still stands in the
    // yard: both answer with the identity alone.
    assert_eq!(jjrds_billet_identity("jjqb_200500_AAAAA"), Some("AAAAA"));
    assert_eq!(jjrds_billet_identity("jjqb_200500_AA"), Some("AA"));
    assert_eq!(jjrds_billet_identity("jjqb_AAAAA"), Some("AAAAA"));
    assert_eq!(jjrds_billet_identity("jjqb_AA"), Some("AA"));

    // `_` is in the insignia charset, so an identity may carry one. The read
    // steps over the SERIAL, never over the last underscore.
    assert_eq!(jjrds_billet_identity("jjqb_200500_CAA_B"), Some("CAA_B"));
    assert_eq!(jjrds_billet_identity("jjqb_200500_A_"), Some("A_"));
    assert_eq!(jjrds_billet_identity("jjqb_CAA_B"), Some("CAA_B"));

    // A digit run no longer than an identity body is not a serial: the whole
    // suffix is the token, so a pre-catchword coronet of digits reads whole.
    assert_eq!(jjrds_billet_identity("jjqb_12_AB"), Some("12_AB"));

    // Not a billet at all.
    assert_eq!(jjrds_billet_identity("jjqs_studbook"), None);
    assert_eq!(jjrds_billet_identity("jjqd_scratch"), None);
}

#[test]
fn jjtds_billet_dirname_wears_the_serial_ahead_of_the_identity() {
    // Creation-timeline sortable in a plain listing: the serial leads, so
    // lexical order over the yard is mint order.
    assert_eq!(jjrds_billet_dirname(200500, "AAAAA"), "jjqb_200500_AAAAA");
    let mut yard = vec![
        jjrds_billet_dirname(200512, "AB"),
        jjrds_billet_dirname(200500, "AAAAA"),
        jjrds_billet_dirname(200507, "AB"),
    ];
    yard.sort();
    assert_eq!(yard, vec!["jjqb_200500_AAAAA", "jjqb_200507_AB", "jjqb_200512_AB"]);

    // And the label round-trips through the read that consumes it.
    assert_eq!(jjrds_billet_identity(&jjrds_billet_dirname(200500, "CAA_B")), Some("CAA_B"));
}

#[test]
fn jjtds_rediscover_never_rejoins_a_groom_billet_so_grooms_coexist() {
    // Concurrent grooms of one heat are deliberately legal: a groom billet
    // seats no branch, so nothing rediscovers, and each dispatch mints its own
    // yard slot — uniqueness riding the serial alone.
    let (_infield, hippodrome) = zjjtds_infield("jjtds_rediscover_groom");
    let plan = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();

    let first = zjjtds_yard(&plan, 200500);
    jjrds_board(&jjrfg_PlainGit, &plan, &first).unwrap();
    assert_eq!(jjrds_rediscover(&jjrfg_PlainGit, &plan).unwrap(), None, "a groom billet never rediscovers");

    let second = zjjtds_yard(&plan, 200501);
    assert_ne!(first.billet_root, second.billet_root);
    jjrds_board(&jjrfg_PlainGit, &plan, &second).unwrap();

    // Both stand, both are groom ground, and their scratch is keyed apart.
    assert_eq!(jjrds_ground(&jjrfg_PlainGit, &first.billet_root), Some(jjrds_Ground::GroomBillet));
    assert_eq!(jjrds_ground(&jjrfg_PlainGit, &second.billet_root), Some(jjrds_Ground::GroomBillet));
    assert_ne!(first.scratch_root, second.scratch_root);
}

// ---- The dispatch record ----

/// The studbook's trunk tip subject, as the record wrote it.
fn zjjtds_tip_subject(config: &jjdb_BlotterConfig) -> String {
    zjjtds_git(&config.local_root, &["log", "-1", "--format=%s", &jjdb_pin(config).unwrap()])
}

#[test]
fn jjtds_record_dispatch_lands_the_event_and_yields_the_billet_serial() {
    let (infield, hippodrome) = zjjtds_infield_over("jjtds_record_dispatch");
    let infield_canon = infield.path().canonicalize().unwrap();
    let studbook = jjdb_studbook_config(&infield_canon);
    let station = "jjtds-station";

    // A saddle: the record names door, kind, target, and station — and nothing
    // else. No worktree path appears in it.
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, true).unwrap();
    let before = jjdb_pin(&studbook).unwrap();
    let catchword = jjrds_record_dispatch(&jjrfg_PlainGit, &studbook, &plan, station).unwrap();

    let subject = zjjtds_tip_subject(&studbook);
    assert_eq!(
        subject,
        format!("₶{}: dispatch saddle — pace billet for ₢AAAAA at station {}", catchword, station)
    );
    assert!(!subject.contains("jjqb_"), "the record is of the event, never the worktree path");

    // Content-less by construction: the record's tree is its parent's own.
    let tip = jjdb_pin(&studbook).unwrap();
    assert_ne!(tip, before);
    assert_eq!(
        zjjtds_git(&studbook.local_root, &["rev-parse", &format!("{}^{{tree}}", tip)]),
        zjjtds_git(&studbook.local_root, &["rev-parse", &format!("{}^{{tree}}", before)]),
        "an event has no file: the record carries its parent's tree"
    );

    // The catchword the ceremony allocated is the serial the billet wears.
    assert_eq!(
        jjrds_yard(&plan.infield_root, plan.infield_root.join(jjrds_billet_dirname(catchword, &plan.identity_body)))
            .billet_dirname,
        format!("jjqb_{}_AAAAA", catchword)
    );

    // A lunge behind it: the next serial, and the groom kind named with the
    // heat's own sigil.
    let groom = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, true).unwrap();
    let next = jjrds_record_dispatch(&jjrfg_PlainGit, &studbook, &groom, station).unwrap();
    assert_eq!(next, catchword + 1, "serials advance with the journal, so the yard sorts by birth");
    assert_eq!(
        zjjtds_tip_subject(&studbook),
        format!("₶{}: dispatch lunge — groom billet for ₣AA at station {}", next, station)
    );
}

#[test]
fn jjtds_record_dispatch_refuses_under_a_held_guidon_and_lands_nothing() {
    // The accepted cost of making birth a journaled write: a dispatch is
    // LockHeld-refusable, exactly as every other studbook write is.
    let (infield, hippodrome) = zjjtds_infield_over("jjtds_record_lockheld");
    let infield_canon = infield.path().canonicalize().unwrap();
    let studbook = jjdb_studbook_config(&infield_canon);
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, true).unwrap();

    let holder = "officium=other station=elsewhere operation=curry";
    jjrfg_PlainGit.jjrfr_stake(&studbook.local_root, holder).unwrap();
    let before = jjdb_pin(&studbook).unwrap();

    match jjrds_record_dispatch(&jjrfg_PlainGit, &studbook, &plan, "jjtds-station") {
        Err(jjrds_Rejection::Farrier(r)) => assert_eq!(r.kind, jjrfr_RejectionKind::LockHeld),
        other => panic!("a held guidon must refuse the dispatch record; got {:?}", other),
    }
    assert_eq!(jjdb_pin(&studbook).unwrap(), before, "a refused record lands nothing");

    jjrfg_PlainGit.jjrfr_pluck(&studbook.local_root, holder).unwrap();
}

#[test]
fn jjtds_dispatch_record_names_the_event_alone() {
    // Composition in isolation, so the record's shape is pinned independently
    // of the ceremony that carries it.
    assert_eq!(
        jjrds_dispatch_record(jjrds_Door::Saddle, &jjrfr_BilletBirth::Branch("jjls_pace/CAAAB".to_string()), "CAAAB", "macmini"),
        "dispatch saddle — pace billet for ₢CAAAB at station macmini"
    );
    assert_eq!(
        jjrds_dispatch_record(jjrds_Door::Lunge, &jjrfr_BilletBirth::Detached, "B9", "cerebro"),
        "dispatch lunge — groom billet for ₣B9 at station cerebro"
    );
}

// ---- The stile's trailing step ----

#[test]
fn jjtds_trailing_step_destroys_a_clean_and_pushed_pace_billet() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_trailing_pace_destructs");
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200500);
    jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap();

    jjrfg_PlainGit.jjrfr_consign(&yard.billet_root, "jjls_pace/AAAAA").unwrap();

    let report = jjrds_trailing_step(&jjrfg_PlainGit, &yard.billet_root, &plan.trunk);
    assert!(report.contains("cleared"), "expected a clearance report, got: {}", report);
    assert!(
        report.contains("work stands on branch jjls_pace/AAAAA"),
        "a cleared pace billet must name where the work stands — its durable branch (JJSVD \"The stile\"): {}",
        report
    );
    assert!(!yard.billet_root.exists(), "a clean-and-pushed pace billet must be destroyed");
}

#[test]
fn jjtds_trailing_step_stands_a_dirty_pace_billet() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_trailing_pace_dirty");
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200500);
    jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap();
    jjrfg_PlainGit.jjrfr_consign(&yard.billet_root, "jjls_pace/AAAAA").unwrap();

    std::fs::write(yard.billet_root.join("uncommitted.txt"), "dirt").unwrap();

    let report = jjrds_trailing_step(&jjrfg_PlainGit, &yard.billet_root, &plan.trunk);
    assert!(report.contains("stands"), "expected a standing report, got: {}", report);
    assert!(report.contains("muck"), "a standing billet must name muck as the remedy: {}", report);
    assert!(report.contains("uncommitted changes"), "the report must name the failed conjunct (JJSVD \"The stile\"): {}", report);
    assert!(yard.billet_root.exists(), "a dirty billet must never be destroyed");
}

#[test]
fn jjtds_trailing_step_stands_an_unpushed_pace_billet() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_trailing_pace_unpushed");
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200500);
    jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap();

    // Committed, so the tree is clean — but never consigned, so the litmus's
    // ignorance-stands arm holds: nothing is proven held in remote custody.
    zjjtds_commit_all(&yard.billet_root, "wip.txt", "wip", "wip on the pace branch");

    let report = jjrds_trailing_step(&jjrfg_PlainGit, &yard.billet_root, &plan.trunk);
    assert!(report.contains("stands"), "expected a standing report, got: {}", report);
    assert!(report.contains("never consigned"), "the report must name the failed conjunct (JJSVD \"The stile\"): {}", report);
    assert!(yard.billet_root.exists(), "an unpushed billet must never be destroyed");
}

#[test]
fn jjtds_trailing_step_stands_a_groom_billet_with_a_raw_local_commit() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_trailing_groom_stands");
    let plan = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200501);
    jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap();

    // A raw local commit on the detached tip: reachable from nothing, so the
    // groom-billet arm of the litmus refuses.
    zjjtds_commit_all(&yard.billet_root, "raw.txt", "raw", "raw local groom commit");

    let report = jjrds_trailing_step(&jjrfg_PlainGit, &yard.billet_root, &plan.trunk);
    assert!(report.contains("stands"), "expected a standing report, got: {}", report);
    assert!(
        report.contains("not reachable from trunk's counterpart"),
        "the report must name the failed conjunct (JJSVD \"The stile\"): {}",
        report
    );
    assert!(yard.billet_root.exists(), "an unreachable groom billet must never be destroyed");
}

#[test]
fn jjtds_trailing_step_clears_a_groom_billet_left_at_trunk_tip() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_trailing_groom_clears");
    let plan = jjrds_plan(jjrds_Door::Lunge, "AA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200501);
    jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap();

    // A groom that made no commit sits exactly at trunk's counterpart — clean
    // and reachable, so it passes and clears, and the work it carried (none of
    // its own) already stands in trunk.
    let report = jjrds_trailing_step(&jjrfg_PlainGit, &yard.billet_root, &plan.trunk);
    assert!(report.contains("cleared"), "a clean groom billet at trunk tip must clear: {}", report);
    assert!(
        report.contains(&format!("work stands in trunk {}", plan.trunk)),
        "a cleared groom billet must name trunk as where the work stands (JJSVD \"The stile\"): {}",
        report
    );
    assert!(!yard.billet_root.exists(), "a passing groom billet must be destroyed");
}

#[test]
fn jjtds_trailing_step_leaves_scratch_untouched_either_way() {
    let (_infield, hippodrome) = zjjtds_infield("jjtds_trailing_scratch_untouched");
    let plan = jjrds_plan(jjrds_Door::Saddle, "AAAAA", &hippodrome, false).unwrap();
    let yard = zjjtds_yard(&plan, 200500);
    jjrds_board(&jjrfg_PlainGit, &plan, &yard).unwrap();
    jjrfg_PlainGit.jjrfr_consign(&yard.billet_root, "jjls_pace/AAAAA").unwrap();

    std::fs::create_dir_all(&yard.scratch_root).unwrap();
    let marker = yard.scratch_root.join("logs-buk-marker.txt");
    std::fs::write(&marker, "forensics").unwrap();

    let report = jjrds_trailing_step(&jjrfg_PlainGit, &yard.billet_root, &plan.trunk);
    assert!(report.contains("cleared"), "expected the billet to clear: {}", report);
    assert!(!yard.billet_root.exists());
    assert!(marker.exists(), "the per-billet scratch is forensics and must survive the billet's destruction");
}
