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
// RBTHDR — essai: the reversible repair lap (RBSHE). The whole cycle end to end
// — gate, assay, cut, prove, rig — run as many times as it takes, with ZERO
// remote acts. Its product is a walk-ready rig standing beside a pristine
// candidate that is bit-for-bit what a subsequent ostend would push.
//
// Workers are sequenced as subprocesses, never re-implemented (RBSHC) — with
// the cut as the one ruled exception: rbthdr_expede runs in-process (RBSHC
// "The cut, and the single matcher"). The six steps below are RBSHE's six
// `//axhos_step` blocks in order; the candidate battery is RELEASE.md step 5,
// which essai automates. A finding at any red means RE-CUT, never patch
// forward — the accumulated-state bug class the ceremony exists to catch.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::rbthdr_expede;
use crate::rbthdr_log;
use crate::rbthdr_loupe;
use crate::rbthdr_repo;
use crate::rbthdr_rig;
use crate::rbthdr_run;

// ── Worker location: colophons and imprints ─────────────────

/// Tabtarget colophon prefixes, matched against the leading segment of a
/// tabtarget filename. The frontispiece varies; the colophon does not.
const RBTHDR_COL_SUITE: &str = "rbw-ts.";
const RBTHDR_COL_FIXTURE: &str = "rbw-tf.";
const RBTHDR_COL_QUALIFY_FAST: &str = "rbw-tq.";
const RBTHDR_COL_BUILD: &str = "rbw-tb.";

/// Suite and fixture imprints.
const RBTHDR_SUITE_REVEILLE: &str = "reveille";
const RBTHDR_FIX_CUPEL: &str = "cupel";
const RBTHDR_FIX_PYX: &str = "pyx";
const RBTHDR_FIX_DAMNATIO: &str = "damnatio";

/// The throwaway probe branch for the consumer-seat reveille. Named nothing the
/// feign verb's own branch guard refuses (never main, never candidate-*).
const RBTHDR_PROBE_BRANCH: &str = "probe";

/// The marshal feign tabtarget the candidate must be handed: its verb ships
/// (rblm_cli.sh), but its tabtarget is withheld from delivery, so the candidate
/// carries the verb and no launcher for it. A copy of any candidate tabtarget
/// under this name dispatches to feign (the trampolines are byte-generic).
const RBTHDR_FEIGN_TT: &str = "rbw-MF.MarshalFeigns.sh";

/// The candidate's identity-free station (RELEASE.md step 5 / rbk-expede) — the
/// consumer's first onboarding act reproduced, never a leak.
const RBTHDR_STATION_USER: &str = "candidate";
const RBTHDR_STATION_TINCTURE: &str = "cnd";

/// The `tt/` subdirectory holding tabtargets, relative to a repo root.
const RBTHDR_TT_SUBDIR: &str = "tt";

/// Conduct one essai lap. Fatal (exit 1) on any deficit or red; ExitCode::SUCCESS
/// only when a walk-ready rig stands beside a proven candidate.
pub fn conduct() -> ExitCode {
    rbthdr_log::section("Hierophant Essai — the reversible repair lap (RBSHE)");
    rbthdr_log::line("Gate, cut, prove, rig — zero remote acts. A finding means re-cut.");

    let top = rbthdr_repo::toplevel();
    let parent = rbthdr_repo::parent(&top);
    rbthdr_log::line(&format!("Maintainer tree: {}", top.display()));

    zrbthdr_gate(&top);
    zrbthdr_precut_assays(&top);
    let candidate_clone = zrbthdr_cut(&top, &parent);
    zrbthdr_prove(&parent, &candidate_clone, &top);

    // Steps 5 & 6 — stand up the rig from the LOCAL candidate, show the fidelity
    // gap loud, hand off the walk, and state the two standing artifacts.
    rbthdr_log::section("Stand up the coldwalk rig (RBSHE step 5)");
    let clone_source = rbthdr_repo::as_str(&candidate_clone);
    let rig = rbthdr_rig::stand_up(&parent, &top, &clone_source, &top);

    rbthdr_log::blank();
    rbthdr_log::warn("KNOWN FIDELITY GAP — this rig is a LOCAL proxy:");
    rbthdr_log::line("the HTTPS-clone-from-GitHub step and the public landing face are");
    rbthdr_log::line("UNPROVEN in this mode. They are covered only by the final instruments");
    rbthdr_log::line("against promoted public main (the harbinger command, a later ceremony).");

    rbthdr_rig::emit_handoff(&rig);

    rbthdr_log::section("Two standing artifacts (RBSHE step 6)");
    rbthdr_log::line(&format!("Pristine candidate (untouched by the walk): {}", candidate_clone.display()));
    rbthdr_log::line(&format!("Disposable rig:                             {}", rig.rig_dir.display()));
    rbthdr_log::blank();
    rbthdr_log::success("Essai lap complete — a walk-ready rig stands beside a proven candidate.");
    rbthdr_log::line("Dispose and re-cut, or hand the standing candidate to docimasy — the reveal's proving act (RBSHE completion).");

    ExitCode::SUCCESS
}

// ── Step 1: gate the maintainer tree and the base ───────────

fn zrbthdr_gate(top: &Path) {
    rbthdr_log::section("Gate the maintainer tree and the base (RBSHE step 1)");

    // Clean, fully-pushed working tree. The candidate is cut from COMMITTED
    // bytes, so an uncommitted edit would silently be absent; an unpushed commit
    // means the base the operator later reveals from is behind the tree cut.
    let status = rbthdr_run::capture("git", &["status", "--porcelain"], top);
    zrbthdr_require_source(status.code, "git status");
    if !status.stdout.trim().is_empty() {
        crate::rbthdr_fatal!(
            "working tree not clean — commit before essai; the candidate is cut from committed bytes:\n{}",
            status.stdout.trim()
        );
    }

    let upstream = rbthdr_run::capture(
        "git",
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
        top,
    );
    if upstream.code != 0 {
        crate::rbthdr_fatal!(
            "current branch has no upstream — set tracking and push before essai:\n{}",
            upstream.stderr.trim()
        );
    }
    let unpushed = rbthdr_run::capture("git", &["rev-list", "--count", "@{u}..HEAD"], top);
    zrbthdr_require_source(unpushed.code, "git rev-list");
    if unpushed.stdout.trim() != "0" {
        crate::rbthdr_fatal!(
            "HEAD has {} unpushed commit(s) — push before essai (a session that opened an officium starts one ahead; push it)",
            unpushed.stdout.trim()
        );
    }
    rbthdr_log::line("working tree clean and fully pushed");

    // The base — read-only by construction. The gate and the cut share the
    // crate's own constants (rbthdr_expede), so this preflight can never
    // drift from the cut-time refusal.
    let base_remote = rbthdr_expede::RBTHDR_BASE_REMOTE;
    let push_sentinel = rbthdr_expede::RBTHDR_BASE_PUSH_DISABLED;

    let fetch = rbthdr_run::capture("git", &["remote", "get-url", base_remote], top);
    if fetch.code != 0 {
        crate::rbthdr_fatal!(
            "base remote {} is not configured — the candidate is built by addition atop the real public repo, so a remote pointing at it is required:\n{}",
            base_remote, fetch.stderr.trim()
        );
    }
    let push = rbthdr_run::capture("git", &["remote", "get-url", "--push", base_remote], top);
    if push.code != 0 {
        crate::rbthdr_fatal!("cannot read {} push url:\n{}", base_remote, push.stderr.trim());
    }
    if push.stdout.trim() != push_sentinel {
        crate::rbthdr_fatal!(
            "{} has a push-capable url ({}) — the base must be READ-ONLY. Neuter it: git remote set-url --push {} {}",
            base_remote, push.stdout.trim(), base_remote, push_sentinel
        );
    }
    rbthdr_log::line(&format!("base {} configured, push side neutered", base_remote));
}

// ── Step 2: pre-cut assays, on the maintainer tree ──────────

fn zrbthdr_precut_assays(top: &Path) {
    rbthdr_log::section("Pre-cut assays on the maintainer tree (RBSHE step 2)");
    let tt = top.join(RBTHDR_TT_SUBDIR);

    // main must be green before it is worth cutting.
    let reveille = zrbthdr_find_tt(&tt, RBTHDR_COL_SUITE, Some(RBTHDR_SUITE_REVEILLE));
    zrbthdr_require_source(rbthdr_run::stream(&reveille, &[], top, &[]), "reveille suite");

    // The veiled-tree assay, in-process (RBSHC "Worker, never authority": the
    // veil assay is one of the hierophant's own absorbed modules, beside the cut
    // and the rig). It reads the veiled trees to harvest its census, meaningful
    // only here; in the candidate it is red by construction. The perambulation's
    // totality gate is likewise not a fixture: it is the cut's own first refusal,
    // in-process (step 3).
    let leaks = rbthdr_loupe::assay(top);
    if !leaks.is_empty() {
        for leak in &leaks {
            rbthdr_log::line(&format!("veil leak: {}", leak));
        }
        crate::rbthdr_fatal!(
            "the veiled-tree assay found {} leak(s) — repair on the maintainer tree, then run essai again (RBSHE)",
            leaks.len()
        );
    }
    rbthdr_log::line("maintainer tree green; the veiled-tree assay passes");
}

// ── Step 3: cut the candidate — the absorbed cut, in-process ─

/// Returns the candidate clone path ({parent}/rbm_candidate/candidate).
fn zrbthdr_cut(top: &Path, parent: &Path) -> PathBuf {
    rbthdr_log::section("Cut the candidate (RBSHE step 3)");

    let candidate_parent = parent.join(rbthdr_repo::RBTHDR_CANDIDATE_DIRNAME);
    rbthdr_repo::guard_disposable(&candidate_parent, rbthdr_repo::RBTHDR_CANDIDATE_DIRNAME, top);
    rbthdr_log::step(&format!("Disposing any prior candidate: {}", candidate_parent.display()));
    if !rbthdr_repo::retire_aside(&candidate_parent, top) {
        rbthdr_log::line("no prior candidate to retire");
    }

    // The absorbed cut (RBSHC "The cut, and the single matcher"): builds by
    // addition into a clone of the public base, refusing on its own gates —
    // clean tree, total perambulation, base identity + neuter, template
    // present, byte-assert, single-commit, delta sweep, zero remotes — all
    // judged in-process by the one matcher. Fatal on any deficit; a return
    // is the verdict.
    rbthdr_log::step(&format!("Expediting the candidate into {}", candidate_parent.display()));
    let candidate_clone = rbthdr_expede::cut(top, &candidate_parent);

    rbthdr_log::line(&format!("candidate cut: {}", candidate_clone.display()));
    candidate_clone
}

// ── Step 4: prove the candidate (the battery, in order) ─────

fn zrbthdr_prove(parent: &Path, candidate_clone: &Path, top: &Path) {
    rbthdr_log::section("Prove the candidate — the battery (RBSHE step 4)");
    let candidate_parent = parent.join(rbthdr_repo::RBTHDR_CANDIDATE_DIRNAME);
    let tt = candidate_clone.join(RBTHDR_TT_SUBDIR);

    zrbthdr_write_station(&candidate_parent);

    // The candidate's OWN tabtargets, on its sterile POSTULANT_LOCAL branch. The
    // candidate's z-launcher normalizes cwd to the candidate root, so these assay
    // the candidate though essai never leaves the maintainer tree.
    let qualify = zrbthdr_find_tt(&tt, RBTHDR_COL_QUALIFY_FAST, None);
    zrbthdr_require_candidate(rbthdr_run::stream(&qualify, &[], top, &[]), "candidate fast-qualify");

    let fixture = zrbthdr_find_tt(&tt, RBTHDR_COL_FIXTURE, None);
    zrbthdr_require_candidate(rbthdr_run::stream(&fixture, &[RBTHDR_FIX_CUPEL], top, &[]), "candidate cupel");
    zrbthdr_require_candidate(rbthdr_run::stream(&fixture, &[RBTHDR_FIX_PYX], top, &[]), "candidate pyx");
    // Damnatio on POSTULANT_LOCAL, BEFORE any feigning — the proof of erasure.
    // It reddens on feigned fields by construction, which is what keeps a probe
    // branch from ever being mistaken for a candidate.
    zrbthdr_require_candidate(rbthdr_run::stream(&fixture, &[RBTHDR_FIX_DAMNATIO], top, &[]), "candidate damnatio");

    // The candidate's transposed root CLAUDE.md must carry no veil needle —
    // re-homed in-process from the theurge damnatio fixture's veil_stripped case,
    // whose census-bearing scan could only run where the veiled trees still
    // stand. The path-grain half (a withheld tree survived the strip) is already
    // covered by expede's object-graph delta sweep at cut time.
    let needles = rbthdr_loupe::assay_candidate(candidate_clone);
    if !needles.is_empty() {
        for needle in &needles {
            rbthdr_log::line(&format!("candidate veil needle: {}", needle));
        }
        crate::rbthdr_fatal!(
            "the candidate's root CLAUDE.md carries {} veil needle(s) — abandon the candidate, repair on the maintainer tree, and re-cut (RBSHE)",
            needles.len()
        );
    }

    zrbthdr_feign_probe(candidate_clone, &tt, top);
}

/// Step 4a — write the candidate's identity-free station and empty secrets dir.
fn zrbthdr_write_station(candidate_parent: &Path) {
    rbthdr_log::step("Writing the candidate's identity-free station");
    let station_dir = candidate_parent.join(rbthdr_repo::RBTHDR_STATION_SUBDIR);
    let secrets_dir = station_dir.join(rbthdr_repo::RBTHDR_SECRETS_SUBDIR);
    let logs_dir = candidate_parent.join(rbthdr_repo::RBTHDR_LOGS_SUBDIR);

    zrbthdr_mkdir(&station_dir);
    zrbthdr_mkdir(&secrets_dir);
    zrbthdr_mkdir(&logs_dir);

    let burs = format!(
        "BURS_USER={}\nBURS_TINCTURE={}\nBURS_LOG_DIR={}\n",
        RBTHDR_STATION_USER,
        RBTHDR_STATION_TINCTURE,
        rbthdr_repo::as_str(&logs_dir),
    );
    let burs_path = station_dir.join(rbthdr_repo::RBTHDR_STATION_FILE);
    std::fs::write(&burs_path, burs)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to write station {}: {}", burs_path.display(), e));
    rbthdr_log::line(&format!("station written: {}", burs_path.display()));
}

/// Step 4f — the consumer-seat probe: cut a throwaway probe branch, hand the
/// candidate its withheld feign tabtarget, feign a false station, run the
/// candidate's reveille from the consumer's seat, then return to the sterile
/// branch and drop the probe branch outright.
fn zrbthdr_feign_probe(candidate_clone: &Path, tt: &Path, top: &Path) {
    rbthdr_log::step("Consumer-seat probe: feign a station on a throwaway branch");
    let clone = rbthdr_repo::as_str(candidate_clone);

    // The sterile branch to return to (expede left the candidate on it). Captured,
    // not hardcoded — expede owns the branch name.
    let head = rbthdr_run::capture("git", &["-C", &clone, "rev-parse", "--abbrev-ref", "HEAD"], top);
    zrbthdr_require_candidate(head.code, "read candidate branch");
    let sterile_branch = head.stdout.trim().to_string();

    zrbthdr_git(&clone, &["checkout", "-b", RBTHDR_PROBE_BRANCH], top, "cut the probe branch");

    // Hand the candidate its feign tabtarget: a byte-copy of an existing
    // candidate tabtarget under the withheld colophon's name (the trampolines are
    // byte-generic; the runtime `${0##*/}` resolves the rbw-MF colophon).
    let donor = zrbthdr_find_tt(tt, RBTHDR_COL_QUALIFY_FAST, None);
    let feign_tt = tt.join(RBTHDR_FEIGN_TT);
    std::fs::copy(&donor, &feign_tt)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to install the feign tabtarget {}: {}", feign_tt.display(), e));
    zrbthdr_chmod_exec(&feign_tt);

    // Rebuild (regenerates the candidate's tabtarget context from the now-complete
    // tt/ set), then commit the seed so the tree is clean before feign — feign's
    // own clean-tree gate demands it, and the probe commit must carry the seed
    // alone.
    let build = zrbthdr_find_tt(tt, RBTHDR_COL_BUILD, None);
    zrbthdr_require_candidate(rbthdr_run::stream(&build, &[], top, &[]), "candidate build (probe)");
    zrbthdr_git(&clone, &["add", "-A"], top, "stage the probe seed");
    zrbthdr_git(&clone, &["commit", "-m", "probe: feign a station"], top, "commit the probe seed");

    // Feign a visibly-false station (BURE_CONFIRM=skip — the ceremony drives it
    // headlessly), then run the candidate's reveille from the consumer's seat.
    let feign = tt.join(RBTHDR_FEIGN_TT);
    zrbthdr_require_candidate(
        rbthdr_run::stream(&feign, &[], top, &[("BURE_CONFIRM", "skip")]),
        "candidate feign",
    );
    let reveille = zrbthdr_find_tt(tt, RBTHDR_COL_SUITE, Some(RBTHDR_SUITE_REVEILLE));
    zrbthdr_require_candidate(rbthdr_run::stream(&reveille, &[], top, &[]), "candidate reveille (consumer seat)");

    // Return to the sterile branch and drop the probe outright — it deliberately
    // holds withheld paths (the feigned station, the copied marshal tabtarget), so
    // dropping it leaves the clone carrying exactly one branch and zero remotes.
    rbthdr_log::step("Returning to the sterile branch and dropping the probe");
    zrbthdr_git(&clone, &["checkout", &sterile_branch], top, "return to the sterile branch");
    zrbthdr_git(&clone, &["branch", "-D", RBTHDR_PROBE_BRANCH], top, "drop the probe branch");
    rbthdr_log::line(&format!("consumer-seat reveille green; candidate back on {}", sterile_branch));
}

// ── Small shared helpers ────────────────────────────────────

/// Locate the single tabtarget under `tt` whose name starts with `colophon` and,
/// if given, embeds `.{imprint}.`. Fatal on zero or multiple matches — a
/// conductor that guesses which worker to run is worse than one that stops.
fn zrbthdr_find_tt(tt: &Path, colophon: &str, imprint: Option<&str>) -> PathBuf {
    let entries = std::fs::read_dir(tt)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("cannot read tabtarget dir {}: {}", tt.display(), e));
    let needle = imprint.map(|i| format!(".{}.", i));
    let mut hits: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with(colophon) || !name.ends_with(".sh") {
            continue;
        }
        if let Some(needle) = &needle {
            if !name.contains(needle.as_str()) {
                continue;
            }
        }
        hits.push(entry.path());
    }
    match hits.len() {
        1 => hits.remove(0),
        0 => crate::rbthdr_fatal!(
            "no tabtarget '{}*{}' under {}",
            colophon,
            imprint.map(|i| format!("{}.", i)).unwrap_or_default(),
            tt.display()
        ),
        n => crate::rbthdr_fatal!(
            "ambiguous: {} tabtargets match '{}*{}' under {}",
            n,
            colophon,
            imprint.map(|i| format!("{}.", i)).unwrap_or_default(),
            tt.display()
        ),
    }
}

/// git -C <clone> <args>, streamed, fatal on non-zero with the act named.
fn zrbthdr_git(clone: &str, args: &[&str], top: &Path, act: &str) {
    let mut full = vec!["-C", clone];
    full.extend_from_slice(args);
    let code = rbthdr_run::stream("git", &full, top, &[]);
    if code != 0 {
        crate::rbthdr_fatal!("failed to {} (git exited {})", act, code);
    }
}

fn zrbthdr_mkdir(dir: &Path) {
    std::fs::create_dir_all(dir)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to create {}: {}", dir.display(), e));
}

fn zrbthdr_chmod_exec(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to stat {}: {}", path.display(), e))
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to make {} executable: {}", path.display(), e));
}

/// A red in the gate or a pre-cut assay: the fault is on the maintainer tree.
fn zrbthdr_require_source(code: i32, what: &str) {
    if code != 0 {
        crate::rbthdr_fatal!(
            "{} failed (exit {}) — repair on the maintainer tree, then run essai again (RBSHE)",
            what, code
        );
    }
}

/// A red in the candidate battery: abandon the candidate, repair on the
/// maintainer tree, re-cut. Never patch the candidate forward (RBSHE).
fn zrbthdr_require_candidate(code: i32, what: &str) {
    if code != 0 {
        crate::rbthdr_fatal!(
            "{} failed (exit {}) — abandon the candidate, repair on the maintainer tree, and re-cut (RBSHE: a finding means re-cut, never patch forward)",
            what, code
        );
    }
}
