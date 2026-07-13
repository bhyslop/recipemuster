// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_FarrierLock,
    jjrfr_RejectionKind,
};
use super::jjrds_spine::{
    JJRDS_KIND_PLAIN_GIT,
    JJRDS_PEDIGREES_REL_PATH,
};
use super::jjrt_types::jjrg_Gallops;
use super::jjrvb_blotter::{
    jjdb_found,
    jjdb_gallops_journal_load,
    jjdb_gallops_journal_save,
    jjdb_journal,
    jjdb_read,
    jjdb_studbook_config,
    jjdb_BlotterConfig,
    JJDB_CATCHWORD_FOUNDING,
    JJDB_CATCHWORD_SIGIL,
    JJDB_GALLOPS_OVER_STUDBOOK_ENABLED,
};
use super::jjtu_testdir::JjkTestDir;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::{
    Path,
    PathBuf,
};

const ZJJTVB_TRUNK: &str = "jjtvb-trunk";

fn zjjtvb_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtvb_init_local(dir: &Path) {
    zjjtvb_git(dir, &["init", "-q", "-b", ZJJTVB_TRUNK]);
    zjjtvb_git(dir, &["config", "user.email", "jjtvb@example.invalid"]);
    zjjtvb_git(dir, &["config", "user.name", "jjtvb"]);
}

fn zjjtvb_init_bare(dir: &Path) {
    zjjtvb_git(dir, &["init", "-q", "--bare", "-b", ZJJTVB_TRUNK]);
}

fn zjjtvb_write(dir: &Path, name: &str, content: &str) {
    std::fs::write(dir.join(name), content).unwrap();
}

fn zjjtvb_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    zjjtvb_write(dir, name, content);
    zjjtvb_git(dir, &["add", "--", name]);
    zjjtvb_git(dir, &["commit", "-q", "-m", message]);
    zjjtvb_git(dir, &["rev-parse", "HEAD"])
}

/// The expected baked-subject form for an ordinal/message pair, matching
/// `zjjrvb_bake_ordinal`'s composition — the one place tests need to know the
/// bake's exact shape.
fn zjjtvb_baked_subject(ordinal: u64, message: &str) -> String {
    format!("{}{}: {}", JJDB_CATCHWORD_SIGIL, ordinal, message)
}

/// A bare remote plus one local clone tracking it, with a baseline commit
/// pushed — the blotter engine's standard scratch fixture. `name` must be
/// unique per caller: tests run concurrently against real on-disk git repos.
fn zjjtvb_scratch(name: &str) -> (JjkTestDir, JjkTestDir, jjdb_BlotterConfig) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtvb_init_bare(bare.path());
    let local = JjkTestDir::new(&format!("{}_local", name));
    zjjtvb_init_local(local.path());
    zjjtvb_commit_all(local.path(), "base.txt", "base", "init");
    zjjtvb_git(local.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtvb_git(local.path(), &["push", "-q", "-u", "origin", ZJJTVB_TRUNK]);
    let config = jjdb_BlotterConfig {
        local_root: local.path().to_path_buf(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    (bare, local, config)
}

#[test]
fn jjtvb_journal_lodges_content_and_pushes_it() {
    let (bare, _local, config) = zjjtvb_scratch("jjtvb_journal_happy");

    let sha = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-happy", |root| {
        zjjtvb_write(root, "entry.txt", "journaled content");
        (vec![PathBuf::from("entry.txt")], "journal entry".to_string())
    })
    .unwrap();

    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha);
    let subject = zjjtvb_git(bare.path(), &["log", "-1", "--pretty=%s", ZJJTVB_TRUNK]);
    assert_eq!(subject, zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 1, "journal entry"));
}

#[test]
fn jjtvb_journal_releases_the_lock_after_completion() {
    let (bare, _local, config) = zjjtvb_scratch("jjtvb_journal_release");

    jjdb_journal(&jjrfg_PlainGit, &config, "guidon-release", |root| {
        zjjtvb_write(root, "entry.txt", "content");
        (vec![PathBuf::from("entry.txt")], "entry".to_string())
    })
    .unwrap();

    let remaining = zjjtvb_git(bare.path(), &["for-each-ref", "refs/jjv"]);
    assert!(remaining.is_empty(), "guidon must be released after a completed ceremony");
}

#[test]
fn jjtvb_journal_rejects_lock_held_and_never_calls_mutate() {
    let (_bare, local, config) = zjjtvb_scratch("jjtvb_journal_lock_held");
    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-another-station").unwrap();

    let called = Cell::new(false);
    let result = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-contender", |root| {
        called.set(true);
        zjjtvb_write(root, "should-not-land.txt", "x");
        (vec![PathBuf::from("should-not-land.txt")], "should not land".to_string())
    });

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::LockHeld);
    assert!(!called.get(), "mutate must not run when the lock cannot be acquired");
}

#[test]
fn jjtvb_journal_rejects_lock_broken_mid_ceremony_and_pushes_nothing() {
    let (bare, local, config) = zjjtvb_scratch("jjtvb_journal_lock_broken");
    let baseline = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);

    // Another station breaks our lock and stakes its own while we sit between
    // sight and consign — the race the lock-ref lease exists to close. The
    // mutate closure is where the ceremony dwells at that point, so the break
    // is staged from inside it.
    let result = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-victim", |root| {
        jjrfg_PlainGit.jjrfr_pluck(root, "guidon-victim").unwrap();
        jjrfg_PlainGit.jjrfr_stake(root, "guidon-usurper").unwrap();
        zjjtvb_write(root, "stranded.txt", "must never reach the remote");
        (vec![PathBuf::from("stranded.txt")], "stranded entry".to_string())
    });

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::LockBroken);
    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, baseline, "the broken-lock ceremony must land nothing on the remote");
    let flying = jjrfg_PlainGit.jjrfr_sight(local.path()).unwrap();
    assert_eq!(
        flying.as_deref(),
        Some("guidon-usurper"),
        "the usurper's lock must survive both the failed consign and our guard's release"
    );
}

#[test]
fn jjtvb_journal_advances_past_a_prior_journaled_entry_before_writing() {
    let (bare, local, config) = zjjtvb_scratch("jjtvb_journal_advance");

    // A second clone runs its own full ceremony first and lands on the
    // remote — simulating a prior write from another station.
    let other = JjkTestDir::new("jjtvb_journal_advance_other");
    zjjtvb_git(bare.path(), &["clone", "-q", "-b", ZJJTVB_TRUNK, &bare.path().to_string_lossy(), &other.path().to_string_lossy()]);
    zjjtvb_git(other.path(), &["config", "user.email", "jjtvb@example.invalid"]);
    zjjtvb_git(other.path(), &["config", "user.name", "jjtvb"]);
    let other_config = jjdb_BlotterConfig {
        local_root: other.path().to_path_buf(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    jjdb_journal(&jjrfg_PlainGit, &other_config, "guidon-other-station", |root| {
        zjjtvb_write(root, "from-other.txt", "other station's entry");
        (vec![PathBuf::from("from-other.txt")], "from other station".to_string())
    })
    .unwrap();

    // The original clone is now behind; its own ceremony must advance past
    // the other station's entry before adding its own.
    let sha = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-mine", |root| {
        zjjtvb_write(root, "mine.txt", "my entry");
        (vec![PathBuf::from("mine.txt")], "mine".to_string())
    })
    .unwrap();

    assert!(local.path().join("from-other.txt").exists(), "advance must pick up the other station's entry");
    assert!(local.path().join("mine.txt").exists());
    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha);
}

#[test]
fn jjtvb_read_reads_local_content_without_network() {
    let (_bare, local, config) = zjjtvb_scratch("jjtvb_read_local");
    zjjtvb_write(local.path(), "readable.txt", "local content");
    zjjtvb_git(local.path(), &["remote", "set-url", "origin", "/nonexistent/jjtvb-nowhere"]);

    let bytes = jjdb_read(&config, Path::new("readable.txt")).unwrap();

    assert_eq!(bytes, b"local content");
}

#[test]
fn jjtvb_read_reports_absent_content_as_an_ordinary_io_error() {
    let (_bare, _local, config) = zjjtvb_scratch("jjtvb_read_absent");

    let result = jjdb_read(&config, Path::new("never-written.txt"));

    assert!(result.is_err());
}

fn zjjtvb_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    }
}

#[test]
fn jjtvb_gallops_journal_save_then_load_round_trips() {
    let (_bare, _local, config) = zjjtvb_scratch("jjtvb_gallops_roundtrip");
    let gallops = zjjtvb_valid_gallops();

    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-gallops", &gallops, "save gallops".to_string()).unwrap();

    let loaded = jjdb_gallops_journal_load(&config).unwrap();
    assert_eq!(loaded.inner().next_heat_seed, gallops.next_heat_seed);
    assert!(loaded.inner().heats.is_empty());
}

#[test]
fn jjtvb_gallops_over_studbook_enablement_seam_defaults_off() {
    assert!(!JJDB_GALLOPS_OVER_STUDBOOK_ENABLED, "the studbook-backed surface must stay inert until the conversion heat flips it");
}

// ---- Founding ceremony ----

fn zjjtvb_valid_gallops_seed() -> String {
    serde_json::to_string(&zjjtvb_valid_gallops()).expect("a fresh Gallops must serialize")
}

#[test]
fn jjtvb_found_stands_a_fresh_instance_up_from_nothing_against_a_bare_remote() {
    let infield = JjkTestDir::new("jjtvb_found_infield");
    let bare = JjkTestDir::new("jjtvb_found_bare");
    zjjtvb_init_bare(bare.path());
    // local_root does not exist yet — jjdb_found must create it, exactly as
    // it would for a not-yet-founded instance's infield-resident path.
    let local_root = infield.path().join("scratch_studbook");
    let config = jjdb_BlotterConfig {
        local_root: local_root.clone(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    let sha = jjdb_found(&config, |root| {
        zjjtvb_write(root, "gallops.json", &zjjtvb_valid_gallops_seed());
        (vec![PathBuf::from("gallops.json")], "found".to_string())
    });

    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha, "the founding commit must land on the remote's trunk");
    let subject = zjjtvb_git(bare.path(), &["log", "-1", "--pretty=%s", ZJJTVB_TRUNK]);
    assert_eq!(
        subject,
        zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING, "found"),
        "the genesis commit must take the store's founding ordinal, no zeroth special case"
    );
    assert!(local_root.join("gallops.json").exists());
}

#[test]
fn jjtvb_found_instance_is_immediately_ready_for_the_journal_ceremony() {
    let infield = JjkTestDir::new("jjtvb_found_ready_infield");
    let bare = JjkTestDir::new("jjtvb_found_ready_bare");
    zjjtvb_init_bare(bare.path());
    let config = jjdb_BlotterConfig {
        local_root: infield.path().join("scratch_studbook"),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    jjdb_found(&config, |root| {
        zjjtvb_write(root, "gallops.json", &zjjtvb_valid_gallops_seed());
        (vec![PathBuf::from("gallops.json")], "found".to_string())
    });

    // The rehearsal proper (two-station lock contention, stale-lock break,
    // atomic-push lease failure, dispatch round-trip) is ₢BrAAW's own pace;
    // this asserts the founded instance is a normal, live blotter the
    // ordinary journal ceremony can already write through.
    let sha = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-rehearsal", |root| {
        zjjtvb_write(root, "entry.txt", "post-founding entry");
        (vec![PathBuf::from("entry.txt")], "rehearsal entry".to_string())
    })
    .unwrap();

    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha);
    let subject = zjjtvb_git(bare.path(), &["log", "-1", "--pretty=%s", ZJJTVB_TRUNK]);
    assert_eq!(
        subject,
        zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 1, "rehearsal entry"),
        "the first journaled write after founding must advance one past the genesis ordinal"
    );
    let loaded = jjdb_gallops_journal_load(&config).unwrap();
    assert_eq!(loaded.inner().next_heat_seed, zjjtvb_valid_gallops().next_heat_seed);
}

#[test]
fn jjtvb_journal_bakes_a_monotonically_advancing_ordinal_across_writes() {
    let infield = JjkTestDir::new("jjtvb_ordinal_monotonic_infield");
    let bare = JjkTestDir::new("jjtvb_ordinal_monotonic_bare");
    zjjtvb_init_bare(bare.path());
    let config = jjdb_BlotterConfig {
        local_root: infield.path().join("scratch_studbook"),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    jjdb_found(&config, |root| {
        zjjtvb_write(root, "gallops.json", &zjjtvb_valid_gallops_seed());
        (vec![PathBuf::from("gallops.json")], "found".to_string())
    });

    jjdb_journal(&jjrfg_PlainGit, &config, "guidon-first", |root| {
        zjjtvb_write(root, "first.txt", "first");
        (vec![PathBuf::from("first.txt")], "first entry".to_string())
    })
    .unwrap();
    jjdb_journal(&jjrfg_PlainGit, &config, "guidon-second", |root| {
        zjjtvb_write(root, "second.txt", "second");
        (vec![PathBuf::from("second.txt")], "second entry".to_string())
    })
    .unwrap();

    let subjects = zjjtvb_git(bare.path(), &["log", "--pretty=%s", "--reverse", ZJJTVB_TRUNK]);
    let lines: Vec<&str> = subjects.lines().collect();
    assert_eq!(
        lines,
        vec![
            zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING, "found"),
            zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 1, "first entry"),
            zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 2, "second entry"),
        ],
        "each journaled write must advance the ordinal by exactly one past the genesis value"
    );
}

/// The station's own hippodrome upstream, canonicalized (a trailing `.git`
/// stripped) — the address every clone of it under this infield derives from
/// its `origin`, and therefore the address the seeded pedigree must carry for
/// dispatch to serve any of them.
const ZJJTVB_REAL_SIRE_ADDRESS: &str = "git@github.com:bhyslop/recipemuster";

/// That upstream's main line of work.
const ZJJTVB_REAL_SIRE_TRUNK: &str = "main";

/// The real station's seed pedigree — one entry, serving every clone of the
/// hippodrome upstream under this infield. Shared by the founding ceremony's
/// seed and by the journal write that carries it onto a store already founded
/// without it.
fn zjjtvb_real_pedigrees_json() -> String {
    let pedigrees = serde_json::json!({
        "jjop_sires": [{
            "jjop_kind": JJRDS_KIND_PLAIN_GIT,
            "jjop_addresses": [ZJJTVB_REAL_SIRE_ADDRESS],
            "jjop_trunk": ZJJTVB_REAL_SIRE_TRUNK,
        }]
    });
    serde_json::to_string_pretty(&pedigrees).expect("the seed pedigree must serialize")
}

/// Found the REAL production `jjqs_studbook` against its real GitHub remote
/// at the station's real infield root — the one-shot ceremony ₢BrAAU exists
/// to run, standing the scratch studbook up for ₢BrAAW's rehearsal. Real
/// network, real remote, not run by the ordinary suite: `--ignored`, and
/// `JJTVB_REAL_INFIELD_ROOT` must name the infield explicitly so a stray
/// `--include-ignored` sweep cannot land it anywhere by accident.
///
/// The seed carries BOTH studbook tenants, not just the gallops: `jjrds_plan`
/// reads `pedigrees.json` before anything else, so a station founded without
/// it cannot dispatch at all. The ₢BrAAU run seeded one file short of what
/// JJSVS Founding-and-cutover describes; this is that seed corrected, and the
/// re-found is what carries it onto the real store.
#[test]
#[ignore]
fn jjtvb_found_the_real_studbook_at_its_real_infield_root() {
    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT")
        .expect("set JJTVB_REAL_INFIELD_ROOT to the real infield root to run this ceremony");
    let config = jjdb_studbook_config(Path::new(&infield_root));

    let gallops = jjrg_Gallops {
        next_heat_seed: "AA".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    };
    let gallops_json = serde_json::to_string_pretty(&gallops).expect("a fresh Gallops must serialize");

    let pedigrees_json = zjjtvb_real_pedigrees_json();

    let sha = jjdb_found(&config, |root| {
        zjjtvb_write(root, "gallops.json", &gallops_json);
        zjjtvb_write(root, JJRDS_PEDIGREES_REL_PATH, &pedigrees_json);
        (
            vec![PathBuf::from("gallops.json"), PathBuf::from(JJRDS_PEDIGREES_REL_PATH)],
            "found jjqs_studbook".to_string(),
        )
    });

    println!("founded jjqs_studbook at {} ({})", sha, config.local_root.display());
}

/// Rehearsal, and the substrate proof every other rehearsal item stands on
/// (₢BrAAW): carry the seed pedigree onto the REAL standing studbook through
/// the full journal ceremony, against the real GitHub remote — the ₢BrAAU
/// founding seeded `gallops.json` alone, and the remote cannot be emptied for
/// a re-found without leaving plain git (GitHub refuses to delete a default
/// branch), so the correction arrives as a journaled write.
///
/// One write exercises the whole bracket where no scratch bare repo can reach
/// it: stake's compare-and-swap on the custom `refs/jjv/guidon` ref, sight's
/// ls-remote-then-fetch read-back, advance, lodge, the atomic two-ref consign
/// under lease, the ordinal bake, and pluck — all against a real server whose
/// receive-pack is a Palisade we do not govern. A refusal here (a rejected ref
/// namespace, an unsupported `--atomic` with `--force-with-lease`) is the
/// finding, not a failure to route around.
///
/// Idempotent by refusal: it asserts the pedigree is absent first, so a second
/// run says so plainly instead of journaling a no-op commit.
#[test]
#[ignore]
fn jjtvb_journal_the_real_pedigree_onto_the_standing_studbook() {
    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT")
        .expect("set JJTVB_REAL_INFIELD_ROOT to the real infield root to run this rehearsal");
    let config = jjdb_studbook_config(Path::new(&infield_root));
    let farrier = jjrfg_PlainGit;

    assert!(
        jjdb_read(&config, Path::new(JJRDS_PEDIGREES_REL_PATH)).is_err(),
        "the standing studbook already carries a pedigrees file — this write has already run"
    );

    let pedigrees_json = zjjtvb_real_pedigrees_json();
    let guidon = format!(
        "officium=₢BrAAW-rehearsal station={} op=journal-pedigree",
        zjjtvb_station_name()
    );

    let sha = jjdb_journal(&farrier, &config, &guidon, |root| {
        zjjtvb_write(root, JJRDS_PEDIGREES_REL_PATH, &pedigrees_json);
        (vec![PathBuf::from(JJRDS_PEDIGREES_REL_PATH)], "seed the station pedigree".to_string())
    })
    .expect("the journal ceremony must carry the pedigree onto the real remote");

    // The lock must be released, and the content must be live on the remote —
    // consign is atomic, so a local commit alone would be a lie.
    assert_eq!(farrier.jjrfr_sight(&config.local_root).unwrap(), None, "the guidon must be plucked on the way out");
    println!("journaled the station pedigree onto jjqs_studbook at {}", sha);
}

/// The station's name for the guidon — a lock-holder mark is caller-composed
/// text, and this rehearsal composes a plausible one rather than reach for
/// machinery the engine has not built.
fn zjjtvb_station_name() -> String {
    std::process::Command::new("hostname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
