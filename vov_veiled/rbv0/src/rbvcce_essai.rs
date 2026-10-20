// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCCE — essai: the reversible repair lap (RBSHE). The whole cycle end to end
// — gate, assay, cut, prove, rig — run as many times as it takes, with ZERO
// remote acts. Its product is a walk-ready rig standing beside a pristine
// regulus that is bit-for-bit what a subsequent ostend would push.
//
// Workers are sequenced as subprocesses, never re-implemented (RBSHC) — with
// the cut as the one ruled exception: rbvcex_expede runs in-process (RBSHC
// "The cut, and the single matcher"). The six steps below are RBSHE's six
// `//axhos_step` blocks in order; the regulus battery is RELEASE.md step 5,
// which essai automates. A finding at any red means RE-CUT, never patch
// forward — the accumulated-state bug class the ceremony exists to catch.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use bkp::bkprs_scrivener::bkprs_program_copy;

use crate::rbvccd_docimasy::{zrbvccd_find_tt, RBVCCD_COL_SUITE, RBVCCD_TT_SUBDIR};
use crate::rbvcex_expede;
use crate::rbvcel_log;
use crate::rbvceu_loupe;
use crate::rbvcep_repo;
use crate::rbvceg_rig;
use crate::rbvcer_run;

// ── Worker location: colophons and imprints ─────────────────

/// Tabtarget colophon prefixes, matched against the leading segment of a
/// tabtarget filename. The frontispiece varies; the colophon does not.
const RBVCCE_COL_FIXTURE: &str = "rbw-tf.";
const RBVCCE_COL_QUALIFY_FAST: &str = "rbw-tq.";
const RBVCCE_COL_BUILD: &str = "rbw-tb.";

/// Suite and fixture imprints.
const RBVCCE_SUITE_REVEILLE: &str = "reveille";
const RBVCCE_FIX_CUPEL: &str = "cupel";
const RBVCCE_FIX_PYX: &str = "pyx";
const RBVCCE_FIX_DAMNATIO: &str = "damnatio";

/// The throwaway probe branch for the consumer-seat reveille. Named nothing the
/// feign verb's own branch guard refuses (never main, never regulus-*).
const RBVCCE_PROBE_BRANCH: &str = "probe";

/// The marshal feign tabtarget the regulus must be handed: its verb ships
/// (rblm_cli.sh), but its tabtarget is withheld from delivery, so the regulus
/// carries the verb and no launcher for it. A copy of any regulus tabtarget
/// under this name dispatches to feign (the trampolines are byte-generic).
const RBVCCE_FEIGN_TT: &str = "rbw-MF.MarshalFeigns.sh";

/// The regulus's identity-free station (RELEASE.md step 5 / rbk-expede) — the
/// consumer's first onboarding act reproduced, never a leak.
const RBVCCE_STATION_USER: &str = "regulus";
const RBVCCE_STATION_TINCTURE: &str = "cnd";

/// Conduct one essai lap. Fatal (exit 1) on any deficit or red; ExitCode::SUCCESS
/// only when a walk-ready rig stands beside a proven regulus.
pub fn rbvcce_essai_conduct() -> ExitCode {
    rbvcel_log::rbvcel_section("Hierophant Essai — the reversible repair lap (RBSHE)");
    rbvcel_log::rbvcel_line("Gate, cut, prove, rig — zero remote acts. A finding means re-cut.");

    let top = rbvcep_repo::rbvcep_toplevel();
    let parent = rbvcep_repo::rbvcep_parent(&top);
    rbvcel_log::rbvcel_line(&format!("Maintainer tree: {}", top.display()));

    zrbvcce_gate(&top);
    zrbvcce_precut_assays(&top);
    let regulus_clone = zrbvcce_cut(&top, &parent);
    zrbvcce_prove(&parent, &regulus_clone, &top);

    // Steps 5 & 6 — stand up the rig from the LOCAL regulus, show the fidelity
    // gap loud, hand off the walk, and state the two standing artifacts.
    rbvcel_log::rbvcel_section("Stand up the coldwalk rig (RBSHE step 5)");
    let clone_source = rbvcep_repo::rbvcep_as_str(&regulus_clone);
    let rig = rbvceg_rig::rbvceg_stand_up(&parent, &top, &clone_source, &top);

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_warn("KNOWN FIDELITY GAP — this rig is a LOCAL proxy:");
    rbvcel_log::rbvcel_line("the HTTPS-clone-from-GitHub step and the public landing face are");
    rbvcel_log::rbvcel_line("UNPROVEN in this mode. They are covered only by the final instruments");
    rbvcel_log::rbvcel_line("against promoted public main (the harbinger command, a later ceremony).");

    rbvceg_rig::rbvceg_emit_handoff(&rig);

    rbvcel_log::rbvcel_section("Two standing artifacts (RBSHE step 6)");
    rbvcel_log::rbvcel_line(&format!("Pristine regulus (untouched by the walk): {}", regulus_clone.display()));
    rbvcel_log::rbvcel_line(&format!("Disposable rig:                             {}", rig.rig_dir.display()));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_success("Essai lap complete — a walk-ready rig stands beside a proven regulus.");
    rbvcel_log::rbvcel_line("Dispose and re-cut, or hand the standing regulus to docimasy — the reveal's proving act (RBSHE completion).");

    ExitCode::SUCCESS
}

// ── Step 1: gate the maintainer tree and the base ───────────

fn zrbvcce_gate(top: &Path) {
    rbvcel_log::rbvcel_section("Gate the maintainer tree and the base (RBSHE step 1)");

    // Clean, fully-pushed working tree. The regulus is cut from COMMITTED
    // bytes, so an uncommitted edit would silently be absent; an unpushed commit
    // means the base the operator later reveals from is behind the tree cut.
    let status = rbvcer_run::rbvcer_capture("git", &["status", "--porcelain"], top);
    zrbvcce_require_source(status.code, "git status");
    if !status.stdout.trim().is_empty() {
        crate::rbvcel_fatal!(
            "working tree not clean — commit before essai; the regulus is cut from committed bytes:\n{}",
            status.stdout.trim()
        );
    }

    let upstream = rbvcer_run::rbvcer_capture(
        "git",
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
        top,
    );
    if upstream.code != 0 {
        crate::rbvcel_fatal!(
            "current branch has no upstream — set tracking and push before essai:\n{}",
            upstream.stderr.trim()
        );
    }
    let unpushed = rbvcer_run::rbvcer_capture("git", &["rev-list", "--count", "@{u}..HEAD"], top);
    zrbvcce_require_source(unpushed.code, "git rev-list");
    if unpushed.stdout.trim() != "0" {
        crate::rbvcel_fatal!(
            "HEAD has {} unpushed commit(s) — push before essai (a session that opened an officium starts one ahead; push it)",
            unpushed.stdout.trim()
        );
    }
    rbvcel_log::rbvcel_line("working tree clean and fully pushed");

    // The base — read-only by construction. The gate and the cut share the
    // crate's own constants (rbvcex_expede), so this preflight can never
    // drift from the cut-time refusal.
    let base_remote = rbvcex_expede::RBVCEX_BASE_REMOTE;
    let push_sentinel = rbvcex_expede::RBVCEX_BASE_PUSH_DISABLED;

    let fetch = rbvcer_run::rbvcer_capture("git", &["remote", "get-url", base_remote], top);
    if fetch.code != 0 {
        crate::rbvcel_fatal!(
            "base remote {} is not configured — the regulus is built by addition atop the real public repo, so a remote pointing at it is required:\n{}",
            base_remote, fetch.stderr.trim()
        );
    }
    let push = rbvcer_run::rbvcer_capture("git", &["remote", "get-url", "--push", base_remote], top);
    if push.code != 0 {
        crate::rbvcel_fatal!("cannot read {} push url:\n{}", base_remote, push.stderr.trim());
    }
    if push.stdout.trim() != push_sentinel {
        crate::rbvcel_fatal!(
            "{} has a push-capable url ({}) — the base must be READ-ONLY. Neuter it: git remote set-url --push {} {}",
            base_remote, push.stdout.trim(), base_remote, push_sentinel
        );
    }
    rbvcel_log::rbvcel_line(&format!("base {} configured, push side neutered", base_remote));
}

// ── Step 2: pre-cut assays, on the maintainer tree ──────────

fn zrbvcce_precut_assays(top: &Path) {
    rbvcel_log::rbvcel_section("Pre-cut assays on the maintainer tree (RBSHE step 2)");
    let tt = top.join(RBVCCD_TT_SUBDIR);

    // main must be green before it is worth cutting.
    let reveille = zrbvccd_find_tt(&tt, RBVCCD_COL_SUITE, Some(RBVCCE_SUITE_REVEILLE));
    zrbvcce_require_source(rbvcer_run::rbvcer_stream(&reveille, &[], top, &[]), "reveille suite");

    // The veiled-tree assay, in-process (RBSHC "Worker, never authority": the
    // veil assay is one of the hierophant's own absorbed modules, beside the cut
    // and the rig). It reads the veiled trees to harvest its census, meaningful
    // only here; in the regulus it is red by construction. The perambulation's
    // totality gate is likewise not a fixture: it is the cut's own first refusal,
    // in-process (step 3).
    let leaks = rbvceu_loupe::rbvceu_assay(top);
    if !leaks.is_empty() {
        for leak in &leaks {
            rbvcel_log::rbvcel_line(&format!("veil leak: {}", leak));
        }
        crate::rbvcel_fatal!(
            "the veiled-tree assay found {} leak(s) — repair on the maintainer tree, then run essai again (RBSHE)",
            leaks.len()
        );
    }
    rbvcel_log::rbvcel_line("maintainer tree green; the veiled-tree assay passes");
}

// ── Step 3: cut the regulus — the absorbed cut, in-process ─

/// Returns the regulus clone path ({parent}/rbvcep_regulus/regulus).
fn zrbvcce_cut(top: &Path, parent: &Path) -> PathBuf {
    rbvcel_log::rbvcel_section("Cut the regulus (RBSHE step 3)");

    let regulus_parent = parent.join(rbvcep_repo::RBVCEP_REGULUS_DIRNAME);
    rbvcep_repo::rbvcep_guard_disposable(&regulus_parent, rbvcep_repo::RBVCEP_REGULUS_DIRNAME, top);
    rbvcel_log::rbvcel_step(&format!("Disposing any prior regulus: {}", regulus_parent.display()));
    if !rbvcep_repo::rbvcep_sepose(&regulus_parent, top) {
        rbvcel_log::rbvcel_line("no prior regulus to sepose");
    }

    // The absorbed cut (RBSHC "The cut, and the single matcher"): builds by
    // addition into a clone of the public base, refusing on its own gates —
    // clean tree, total perambulation, base identity + neuter, template
    // present, byte-assert, single-commit, delta sweep, zero remotes — all
    // judged in-process by the one matcher. Fatal on any deficit; a return
    // is the verdict.
    rbvcel_log::rbvcel_step(&format!("Expediting the regulus into {}", regulus_parent.display()));
    let regulus_clone = rbvcex_expede::rbvcex_cut(top, &regulus_parent);

    rbvcel_log::rbvcel_line(&format!("regulus cut: {}", regulus_clone.display()));
    regulus_clone
}

// ── Step 4: prove the regulus (the battery, in order) ─────

fn zrbvcce_prove(parent: &Path, regulus_clone: &Path, top: &Path) {
    rbvcel_log::rbvcel_section("Prove the regulus — the battery (RBSHE step 4)");
    let regulus_parent = parent.join(rbvcep_repo::RBVCEP_REGULUS_DIRNAME);
    let tt = regulus_clone.join(RBVCCD_TT_SUBDIR);

    zrbvcce_write_station(&regulus_parent);

    // The regulus's OWN tabtargets, on its sterile POSTULANT_LOCAL branch. The
    // regulus's z-launcher normalizes cwd to the regulus root, so these assay
    // the regulus though essai never leaves the maintainer tree.
    let qualify = zrbvccd_find_tt(&tt, RBVCCE_COL_QUALIFY_FAST, None);
    zrbvcce_require_regulus(rbvcer_run::rbvcer_stream(&qualify, &[], top, &[]), "regulus fast-qualify");

    let fixture = zrbvccd_find_tt(&tt, RBVCCE_COL_FIXTURE, None);
    zrbvcce_require_regulus(rbvcer_run::rbvcer_stream(&fixture, &[RBVCCE_FIX_CUPEL], top, &[]), "regulus cupel");
    zrbvcce_require_regulus(rbvcer_run::rbvcer_stream(&fixture, &[RBVCCE_FIX_PYX], top, &[]), "regulus pyx");
    // Damnatio on POSTULANT_LOCAL, BEFORE any feigning — the proof of erasure.
    // It reddens on feigned fields by construction, which is what keeps a probe
    // branch from ever being mistaken for a regulus.
    zrbvcce_require_regulus(rbvcer_run::rbvcer_stream(&fixture, &[RBVCCE_FIX_DAMNATIO], top, &[]), "regulus damnatio");

    // The regulus's transposed root CLAUDE.md must carry no veil needle —
    // re-homed in-process from the theurge damnatio fixture's veil_stripped case,
    // whose census-bearing scan could only run where the veiled trees still
    // stand. The path-grain half (a withheld tree survived the strip) is already
    // covered by expede's object-graph delta sweep at cut time.
    let needles = rbvceu_loupe::rbvceu_assay_regulus(regulus_clone);
    if !needles.is_empty() {
        for needle in &needles {
            rbvcel_log::rbvcel_line(&format!("regulus veil needle: {}", needle));
        }
        crate::rbvcel_fatal!(
            "the regulus's root CLAUDE.md carries {} veil needle(s) — abandon the regulus, repair on the maintainer tree, and re-cut (RBSHE)",
            needles.len()
        );
    }

    zrbvcce_feign_probe(regulus_clone, &tt, top);
}

/// Step 4a — write the regulus's identity-free station and empty secrets dir.
fn zrbvcce_write_station(regulus_parent: &Path) {
    rbvcel_log::rbvcel_step("Writing the regulus's identity-free station");
    let station_dir = regulus_parent.join(rbvcep_repo::RBVCEP_STATION_SUBDIR);
    let secrets_dir = station_dir.join(rbvcep_repo::RBVCEP_SECRETS_SUBDIR);
    let logs_dir = regulus_parent.join(rbvcep_repo::RBVCEP_LOGS_SUBDIR);

    zrbvcce_mkdir(&station_dir);
    zrbvcce_mkdir(&secrets_dir);
    zrbvcce_mkdir(&logs_dir);

    let burs = format!(
        "BURS_USER={}\nBURS_TINCTURE={}\nBURS_LOG_DIR={}\n",
        RBVCCE_STATION_USER,
        RBVCCE_STATION_TINCTURE,
        rbvcep_repo::rbvcep_as_str(&logs_dir),
    );
    let burs_path = station_dir.join(rbvcep_repo::RBVCEP_STATION_FILE);
    std::fs::write(&burs_path, burs)
        .unwrap_or_else(|e| crate::rbvcel_fatal!("failed to write station {}: {}", burs_path.display(), e));
    rbvcel_log::rbvcel_line(&format!("station written: {}", burs_path.display()));
}

/// Step 4f — the consumer-seat probe: cut a throwaway probe branch, hand the
/// regulus its withheld feign tabtarget, feign a false station, run the
/// regulus's reveille from the consumer's seat, then return to the sterile
/// branch and drop the probe branch outright.
fn zrbvcce_feign_probe(regulus_clone: &Path, tt: &Path, top: &Path) {
    rbvcel_log::rbvcel_step("Consumer-seat probe: feign a station on a throwaway branch");
    let clone = rbvcep_repo::rbvcep_as_str(regulus_clone);

    // The sterile branch to return to (expede left the regulus on it). Captured,
    // not hardcoded — expede owns the branch name.
    let head = rbvcer_run::rbvcer_capture("git", &["-C", &clone, "rev-parse", "--abbrev-ref", "HEAD"], top);
    zrbvcce_require_regulus(head.code, "read regulus branch");
    let sterile_branch = head.stdout.trim().to_string();

    zrbvcce_git_stream(&clone, &["checkout", "-b", RBVCCE_PROBE_BRANCH], top, "cut the probe branch");

    // Hand the regulus its feign tabtarget: a byte-copy of an existing
    // regulus tabtarget under the withheld colophon's name (the trampolines are
    // byte-generic; the runtime `${0##*/}` resolves the rbw-MF colophon).
    let donor = zrbvccd_find_tt(tt, RBVCCE_COL_QUALIFY_FAST, None);
    let feign_tt = tt.join(RBVCCE_FEIGN_TT);
    bkprs_program_copy(&donor, &feign_tt)
        .unwrap_or_else(|e| crate::rbvcel_fatal!("failed to install the feign tabtarget {}: {}", feign_tt.display(), e));

    // Rebuild (regenerates the regulus's tabtarget context from the now-complete
    // tt/ set), then commit the seed so the tree is clean before feign — feign's
    // own clean-tree gate demands it, and the probe commit must carry the seed
    // alone.
    let build = zrbvccd_find_tt(tt, RBVCCE_COL_BUILD, None);
    zrbvcce_require_regulus(rbvcer_run::rbvcer_stream(&build, &[], top, &[]), "regulus build (probe)");
    zrbvcce_git_stream(&clone, &["add", "-A"], top, "stage the probe seed");
    zrbvcce_git_stream(&clone, &["commit", "-m", "probe: feign a station"], top, "commit the probe seed");

    // Feign a visibly-false station (BURE_CONFIRM=skip — the ceremony drives it
    // headlessly), then run the regulus's reveille from the consumer's seat.
    let feign = tt.join(RBVCCE_FEIGN_TT);
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&feign, &[], top, &[("BURE_CONFIRM", "skip")]),
        "regulus feign",
    );
    let reveille = zrbvccd_find_tt(tt, RBVCCD_COL_SUITE, Some(RBVCCE_SUITE_REVEILLE));
    zrbvcce_require_regulus(rbvcer_run::rbvcer_stream(&reveille, &[], top, &[]), "regulus reveille (consumer seat)");

    // Return to the sterile branch and drop the probe outright — it deliberately
    // holds withheld paths (the feigned station, the copied marshal tabtarget), so
    // dropping it leaves the clone carrying exactly one branch and zero remotes.
    rbvcel_log::rbvcel_step("Returning to the sterile branch and dropping the probe");
    zrbvcce_git_stream(&clone, &["checkout", &sterile_branch], top, "return to the sterile branch");
    zrbvcce_git_stream(&clone, &["branch", "-D", RBVCCE_PROBE_BRANCH], top, "drop the probe branch");
    rbvcel_log::rbvcel_line(&format!("consumer-seat reveille green; regulus back on {}", sterile_branch));
}

// ── Small shared helpers ────────────────────────────────────

/// git -C <clone> <args>, streamed, fatal on non-zero with the act named.
fn zrbvcce_git_stream(clone: &str, args: &[&str], top: &Path, act: &str) {
    let mut full = vec!["-C", clone];
    full.extend_from_slice(args);
    let code = rbvcer_run::rbvcer_stream("git", &full, top, &[]);
    if code != 0 {
        crate::rbvcel_fatal!("failed to {} (git exited {})", act, code);
    }
}

fn zrbvcce_mkdir(dir: &Path) {
    std::fs::create_dir_all(dir)
        .unwrap_or_else(|e| crate::rbvcel_fatal!("failed to create {}: {}", dir.display(), e));
}

/// A red in the gate or a pre-cut assay: the fault is on the maintainer tree.
fn zrbvcce_require_source(code: i32, what: &str) {
    if code != 0 {
        crate::rbvcel_fatal!(
            "{} failed (exit {}) — repair on the maintainer tree, then run essai again (RBSHE)",
            what, code
        );
    }
}

/// A red in the regulus battery: abandon the regulus, repair on the
/// maintainer tree, re-cut. Never patch the regulus forward (RBSHE).
fn zrbvcce_require_regulus(code: i32, what: &str) {
    if code != 0 {
        crate::rbvcel_fatal!(
            "{} failed (exit {}) — abandon the regulus, repair on the maintainer tree, and re-cut (RBSHE: a finding means re-cut, never patch forward)",
            what, code
        );
    }
}
