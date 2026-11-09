// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCCE — essai: the reversible repair lap (RBSHE). The whole cycle end to end
// — gate, assay, cut, prove, temenos — run as many times as it takes, with ZERO
// remote acts. Its product is a walk-ready temenos standing beside a pristine
// regulus that is bit-for-bit what a subsequent ostend would push.
//
// Workers are sequenced as subprocesses, never re-implemented (RBSHC) — with
// the cut as the one ruled exception: rbvcex_expede runs in-process (RBSHC
// "The cut, and the single matcher"). The six steps below are RBSHE's six
// `//axhos_step` blocks in order; the regulus battery is RELEASE.md step 5,
// which essai automates. A finding at any red means RE-CUT, never patch
// forward — the accumulated-state bug class the ceremony exists to catch.
//
// The lap conducts in the world its one argument elects (RBSHC "The
// election"): the base it gates and cuts atop, the regulus directory, and the
// temenos directory are the elected bundle's. In the palaestra alone, step 1
// also reads the stand-in and real public main, and refuses to cut atop a
// stand-in that no longer carries public main's two branches at its commit.

use std::path::{
    Path,
    PathBuf, //
};
use std::process::ExitCode;

use bkp::bkprs_scrivener::bkprs_program_copy;

use crate::rbvccd_docimasy::{
    zrbvccd_find_tt,
    RBVCCD_COL_SUITE,
    RBVCCD_TT_SUBDIR, //
};
use crate::rbvcec_collation;
use crate::rbvceg_temenos;
use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvcer_run;
use crate::rbvceu_loupe;
use crate::rbvcew_world;
use crate::rbvcex_expede;
use crate::rbvgeb_breviary;

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

/// The regulus clone's own bash constants module, relative to its root: the one
/// declaration of the throwaway probe branch's name, and the copy the feign door
/// reads. The essai declares no second name.
const RBVCCE_CONSTANTS_MODULE: &str = "Tools/rbk/rbcc_constants.sh";

/// The bash child that sources that module and prints the probe branch's name.
const RBVCCE_PROBE_READ: &str = r#"source "$1" && printf '%s\n' "${RBCC_probe_branch}""#;

/// The marshal feign tabtarget the regulus must be handed: its verb ships
/// (rblm_cli.sh), but its tabtarget is withheld from delivery, so the regulus
/// carries the verb and no launcher for it. A copy of any regulus tabtarget
/// under this name dispatches to feign (the trampolines are byte-generic).
const RBVCCE_FEIGN_TT: &str = "rbw-MF.MarshalFeigns.sh";

/// The regulus's identity-free station (RELEASE.md step 5 / rbk-expede) — the
/// consumer's first onboarding act reproduced, never a leak.
const RBVCCE_STATION_USER: &str = "regulus";
const RBVCCE_STATION_TINCTURE: &str = "cnd";

/// The real repository's trunk branch, which the practice stand-in carries
/// beside the regulus branch, both at public main's commit (RBSHE step 1).
const RBVCCE_MAIN_BRANCH: &str = "main";

/// The placeholder the one editable seed line prints for the operator's fresh
/// clone, exactly as the procedure writes it.
const RBVCCE_FRESH_CLONE_PLACE: &str = "«a new directory of your choosing»";

/// Conduct one essai lap in the elected world. Fatal (exit 1) on any deficit or
/// red; ExitCode::SUCCESS only when a walk-ready temenos stands beside a
/// proven regulus.
pub fn rbvcce_essai_conduct(world: rbvcew_world::rbvcew_World) -> ExitCode {
    rbvcel_log::rbvcel_elect(world);
    let bundle = rbvcew_world::rbvcew_bundle(world);
    rbvcel_log::rbvcel_section("Hierophant Essai — the reversible repair lap (RBSHE)");
    rbvcel_log::rbvcel_line(
        "Gate, cut, prove, temenos — zero remote acts. A finding means re-cut.",
    );

    let top = rbvcep_repo::rbvcep_toplevel();
    let parent = rbvcep_repo::rbvcep_parent(&top);
    rbvcel_log::rbvcel_line(&format!(
        "Maintainer tree: {}",
        top.display(), //
    ));

    zrbvcce_gate(&top, world, bundle);
    zrbvcce_precut_assays(&top);
    let regulus_clone = zrbvcce_cut(&top, &parent, world);
    zrbvcce_prove(&parent, &regulus_clone, &top, bundle);

    // Steps 5 & 6 — stand up the temenos from the LOCAL regulus, show the fidelity
    // gap loud, hand off the walk, and state the two standing artifacts.
    rbvcel_log::rbvcel_section("Stand up the coldwalk temenos (RBSHE step 5)");
    let clone_source = rbvcep_repo::rbvcep_as_str(&regulus_clone);
    let temenos = rbvceg_temenos::rbvceg_stand_up(
        &rbvceg_temenos::rbvceg_coldwalk_parent(&top),
        &top,
        &clone_source,
        &top,
        bundle,
    );

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_warn("KNOWN FIDELITY GAP — this temenos is a LOCAL proxy:");
    rbvcel_log::rbvcel_line("the HTTPS-clone-from-GitHub step and the public landing face are");
    match world {
        rbvcew_world::rbvcew_World::Agora => {
            rbvcel_log::rbvcel_line(
                "UNPROVEN in this mode. They are covered only by the final instruments",
            );
            rbvcel_log::rbvcel_line(
                "against promoted public main (the harbinger command, a later ceremony).",
            );
        }
        rbvcew_world::rbvcew_World::Palaestra => {
            rbvcel_log::rbvcel_line(&format!(
                "UNPROVEN in this mode, and in the {} no instrument proves them: its {}",
                rbvcew_world::rbvcew_word(world),
                rbvgeb_breviary::RBVGEB_HARBINGER_BASE, //
            ));
            rbvcel_log::rbvcel_line(&format!(
                "clones the {} over SSH, the gap the practice accepts.",
                bundle.base_noun, //
            ));
        }
    }

    rbvceg_temenos::rbvceg_emit_handoff(&temenos);

    rbvcel_log::rbvcel_section("Two standing artifacts (RBSHE step 6)");
    rbvcel_log::rbvcel_line(&format!(
        "Pristine regulus (untouched by the walk): {}",
        regulus_clone.display(), //
    ));
    rbvcel_log::rbvcel_line(&format!(
        "Disposable temenos:                             {}",
        temenos.temenos_dir.display(), //
    ));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_success(
        "Essai lap complete — a walk-ready temenos stands beside a proven regulus.",
    );
    rbvcel_log::rbvcel_line("Dispose and re-cut, or hand the standing regulus to docimasy — the reveal's proving act (RBSHE completion).");

    ExitCode::SUCCESS
}

// ── Step 1: gate the maintainer tree and the base ───────────

fn zrbvcce_gate(
    top: &Path,
    world: rbvcew_world::rbvcew_World,
    bundle: &rbvcew_world::rbvcew_Bundle,
) {
    rbvcel_log::rbvcel_section("Gate the maintainer tree and the base (RBSHE step 1)");

    // Clean, fully-pushed working tree. The regulus is cut from COMMITTED
    // bytes, so an uncommitted edit would silently be absent; an unpushed commit
    // means the base the operator later reveals from is behind the tree cut.
    let status = rbvcer_run::rbvcer_capture("git", &["status", "--porcelain"], top);
    zrbvcce_require_source(status.code, "git status");
    if !status.stdout.trim().is_empty() {
        crate::rbvcel_fatal!(
            "working tree not clean — commit before essai; the regulus is cut from committed bytes:\n{}",
            status.stdout.trim(), //
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
            upstream.stderr.trim(), //
        );
    }
    let unpushed = rbvcer_run::rbvcer_capture("git", &["rev-list", "--count", "@{u}..HEAD"], top);
    zrbvcce_require_source(unpushed.code, "git rev-list");
    if unpushed.stdout.trim() != "0" {
        crate::rbvcel_fatal!(
            "HEAD has {} unpushed commit(s) — push before essai (a session that opened an officium starts one ahead; push it)",
            unpushed.stdout.trim(), //
        );
    }
    rbvcel_log::rbvcel_line("working tree clean and fully pushed");

    // The base — read-only by construction. The gate and the cut read the one
    // elected bundle (rbvcew_world), so this preflight can never drift from
    // the cut-time refusal.
    let base_remote = bundle.base_remote;
    let push_sentinel = bundle.base_push_disabled;

    let fetch = rbvcer_run::rbvcer_capture("git", &["remote", "get-url", base_remote], top);
    if fetch.code != 0 {
        crate::rbvcel_fatal!(
            "base remote {} is not configured — the {} is built by addition atop the {}, so a remote pointing at it is required:\n{}",
            base_remote,
            rbvgeb_breviary::RBVGEB_REGULUS_BASE,
            bundle.base_noun,
            fetch.stderr.trim(), //
        );
    }
    let push =
        rbvcer_run::rbvcer_capture("git", &["remote", "get-url", "--push", base_remote], top);
    if push.code != 0 {
        crate::rbvcel_fatal!(
            "cannot read {} push url:\n{}",
            base_remote,
            push.stderr.trim(), //
        );
    }
    if push.stdout.trim() != push_sentinel {
        crate::rbvcel_fatal!(
            "{} has a push-capable url ({}) — the base must be READ-ONLY. Neuter it: git remote set-url --push {} {}",
            base_remote, push.stdout.trim(), base_remote, push_sentinel, //
        );
    }
    rbvcel_log::rbvcel_line(&format!(
        "base {} configured, push side neutered",
        base_remote, //
    ));

    if world == rbvcew_world::rbvcew_World::Palaestra {
        zrbvcce_gate_standin(top, bundle);
    }
}

/// The practice stand-in's fitness to cut atop (RBSHE step 1, the palaestra
/// arm): the stand-in carries exactly real public main's `main` and the regulus
/// branch, both at public main's commit, or the lap refuses and prints the
/// procedure's recreate and seed steps. Here and never in the cut: the cut is
/// also the freshness matcher docimasy and ostend re-run, after a practice
/// round has already deleted the stand-in's regulus branch.
fn zrbvcce_gate_standin(top: &Path, bundle: &rbvcew_world::rbvcew_Bundle) {
    let seed_url = rbvcew_world::rbvcew_palaestra_seed_source();
    rbvcel_log::rbvcel_step(&format!(
        "Reading the stand-in {} and real public main {}",
        bundle.base_url,
        seed_url, //
    ));
    let (standin, seed) = zrbvcce_standin_reads(bundle.base_url, seed_url, top);
    match zrbvcce_standin_verdict(&standin, &seed, bundle.base_url, seed_url) {
        Ok(commit) => {
            rbvcel_log::rbvcel_line(&format!(
                "stand-in carries {} and {} at real public main's commit {}",
                RBVCCE_MAIN_BRANCH,
                rbvcex_expede::RBVCEX_REGULUS_BRANCH,
                commit, //
            ));
        }
        Err(refusal) => {
            rbvcel_log::rbvcel_blank();
            rbvcel_log::rbvcel_raw(&refusal);
            rbvcel_log::rbvcel_blank();
            crate::rbvcel_fatal!(
                "the stand-in is not fit to cut atop — act on the lines above, then run {} again (RBSHE step 1)",
                rbvgeb_breviary::RBVGEB_ESSAI_BASE, //
            );
        }
    }
}

/// The two reads the stand-in gate judges, each with an unreachable remote
/// captured rather than fatal: the stand-in's refs, then the seed source's.
pub(crate) fn zrbvcce_standin_reads(
    standin_url: &str,
    seed_url: &str,
    cwd: &Path,
) -> (
    Result<Vec<(String, String)>, String>,
    Result<Vec<(String, String)>, String>,
) {
    (
        rbvcep_repo::rbvcep_ls_remote_capture(standin_url, cwd),
        rbvcep_repo::rbvcep_ls_remote_capture(seed_url, cwd),
    )
}

/// The stand-in verdict, pure: real public main's commit where the stand-in
/// carries exactly `refs/heads/main` and the regulus branch, both at that
/// commit; otherwise the whole refusal text. A tag, a third branch, a missing
/// branch, a branch at another commit, an empty or unreachable stand-in each
/// refuse with the recreate and seed steps; real public main unreadable or
/// absent refuses naming the read, since no reset can be judged without it.
/// The URLs name the repositories in the text, apart from where the reads were
/// taken.
pub(crate) fn zrbvcce_standin_verdict(
    standin: &Result<Vec<(String, String)>, String>,
    seed: &Result<Vec<(String, String)>, String>,
    standin_url: &str,
    seed_url: &str,
) -> Result<String, String> {
    let main_ref = format!(
        "refs/heads/{}",
        RBVCCE_MAIN_BRANCH, //
    );
    let regulus_ref = format!(
        "refs/heads/{}",
        rbvcex_expede::RBVCEX_REGULUS_BRANCH, //
    );
    let seed_refs = seed.as_ref().map_err(|e| {
        format!(
            "cannot read real public main at {} — the stand-in cannot be judged without it. This read must answer from where {} runs:\n        git ls-remote --heads {} {}\n{}",
            seed_url,
            rbvgeb_breviary::RBVGEB_ESSAI_BASE,
            seed_url,
            RBVCCE_MAIN_BRANCH,
            e, //
        )
    })?;
    let commit = seed_refs
        .iter()
        .find(|(_, name)| name == &main_ref)
        .map(|(sha, _)| sha.clone())
        .ok_or_else(|| {
            format!(
                "real public main is absent: {} carries no {}",
                seed_url,
                main_ref, //
            )
        })?;
    let steps = zrbvcce_standin_steps(standin_url, seed_url);
    let refs = standin.as_ref().map_err(|e| {
        format!(
            "cannot read the stand-in at {}:\n{}\n\n{}",
            standin_url,
            e,
            steps, //
        )
    })?;
    let carries = |name: &str| refs.iter().any(|(sha, n)| n == name && sha == &commit);
    if refs.len() == 2 && carries(&main_ref) && carries(&regulus_ref) {
        return Ok(commit);
    }
    let listing = if refs.is_empty() {
        "  (no refs)".to_string()
    } else {
        refs.iter()
            .map(|(sha, name)| {
                format!(
                    "  {} {}",
                    sha,
                    name, //
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    Err(format!(
        "the stand-in {} does not carry exactly {} and {} at real public main's commit {}; it carries:\n{}\n\n{}",
        standin_url,
        RBVCCE_MAIN_BRANCH,
        rbvcex_expede::RBVCEX_REGULUS_BRANCH,
        commit,
        listing,
        steps, //
    ))
}

/// The stand-in's recreate and seed steps, exactly as the procedure's "Opening
/// a round" renders them: the console sentence, the six seed lines in order,
/// and the closing re-read. Pure.
pub(crate) fn zrbvcce_standin_steps(standin_url: &str, seed_url: &str) -> String {
    let main = RBVCCE_MAIN_BRANCH;
    let branch = rbvcex_expede::RBVCEX_REGULUS_BRANCH;
    let fresh = rbvcex_expede::RBVCEX_FRESH_CLONE_VAR;
    [
        format!(
            "Recreate the stand-in (RELEASE.md \"Opening a round\"). At the GitHub console, by hand, delete the stand-in {} and create it again, private and empty, with no README, ignore file or license.",
            rbvcep_repo::rbvcep_github_slug(standin_url), //
        ),
        format!(
            "Then seed it from a fresh clone of real public main by two single-refspec pushes, {} first and then {} at the same commit. Type each line in order, editing only the first to name a new directory, and copy every other line as shown:",
            main,
            branch, //
        ),
        format!(
            "        export {}={}",
            fresh,
            RBVCCE_FRESH_CLONE_PLACE, //
        ),
        format!(
            "        git clone {} \"${{{}}}\"",
            seed_url,
            fresh, //
        ),
        format!(
            "        cd \"${{{}}}\"",
            fresh, //
        ),
        "        git remote remove origin".to_string(),
        format!(
            "        git push {} {}:{}",
            standin_url,
            main,
            main, //
        ),
        format!(
            "        git push {} {}:{}",
            standin_url,
            main,
            branch, //
        ),
        "Then read the stand-in again and confirm both branches stand at real public main's commit:".to_string(),
        format!(
            "        git ls-remote {}",
            standin_url, //
        ),
        format!(
            "Discard the fresh clone afterwards with `rm -rf \"${{{}}}\"`; it is never a working directory of the ceremony.",
            fresh, //
        ),
    ]
    .join("\n")
}

// ── Step 2: pre-cut assays, on the maintainer tree ──────────

fn zrbvcce_precut_assays(top: &Path) {
    rbvcel_log::rbvcel_section("Pre-cut assays on the maintainer tree (RBSHE step 2)");
    let tt = top.join(RBVCCD_TT_SUBDIR);

    // main must be green before it is worth cutting.
    let reveille = zrbvccd_find_tt(&tt, RBVCCD_COL_SUITE, Some(RBVCCE_SUITE_REVEILLE));
    zrbvcce_require_source(
        rbvcer_run::rbvcer_stream(&reveille, &[], top, &[]),
        "reveille suite",
    );

    // The veiled-tree assay, in-process (RBSHC "Worker, never authority": the
    // veil assay is one of the hierophant's own absorbed modules, beside the cut
    // and the temenos). It reads the veiled trees to harvest its census, meaningful
    // only here; in the regulus it is red by construction. The perambulation's
    // totality gate is likewise not a fixture: it is the cut's own first refusal,
    // in-process (step 3).
    let leaks = rbvceu_loupe::rbvceu_assay(top);
    if !leaks.is_empty() {
        for leak in &leaks {
            rbvcel_log::rbvcel_line(&format!(
                "veil leak: {}",
                leak, //
            ));
        }
        crate::rbvcel_fatal!(
            "the veiled-tree assay found {} leak(s) — repair on the maintainer tree, then run essai again (RBSHE)",
            leaks.len(), //
        );
    }
    rbvcel_log::rbvcel_line("maintainer tree green; the veiled-tree assay passes");

    // The one-home corpus check, in-process beside the veil assay (RBSHC "Worker,
    // never authority": one of the hierophant's own absorbed modules). It reads
    // the corpus road the kraal's register names, so it is meaningful only on a
    // maintainer tree standing in a kraal. No register claiming the tree is an
    // absence to log and pass, the locator's self-proof carrying the trust that
    // the absence is honest; a register that cannot be followed is fatal, and so
    // is any finding.
    match rbvcec_collation::rbvcec_assay(top) {
        rbvcec_collation::rbvcec_Reading::Absent(reason) => {
            rbvcel_log::rbvcel_warn(&format!(
                "the one-home corpus check stands down: {}",
                reason, //
            ));
        }
        rbvcec_collation::rbvcec_Reading::Unfollowable(reason) => {
            crate::rbvcel_fatal!(
                "the one-home corpus check cannot follow the register that claims this tree: {} — repair the register or its corpus road, then run {} again (RBSHE)",
                reason,
                rbvgeb_breviary::RBVGEB_ESSAI_BASE, //
            );
        }
        rbvcec_collation::rbvcec_Reading::Ran { findings, tally } => {
            if !findings.is_empty() {
                for finding in &findings {
                    rbvcel_log::rbvcel_line(&format!(
                        "one-home: {}",
                        finding, //
                    ));
                }
                crate::rbvcel_fatal!(
                    "the one-home corpus check found {} violation(s) — repair in the corpus or on the maintainer tree, then run {} again (RBSHE)",
                    findings.len(),
                    rbvgeb_breviary::RBVGEB_ESSAI_BASE, //
                );
            }
            rbvcel_log::rbvcel_line(&tally);
            rbvcel_log::rbvcel_line("the one-home corpus check passes");
        }
    }
}

// ── Step 3: cut the regulus — the absorbed cut, in-process ─

/// Returns the regulus clone path ({parent}/{the world's regulus dirname}/regulus).
fn zrbvcce_cut(top: &Path, parent: &Path, world: rbvcew_world::rbvcew_World) -> PathBuf {
    rbvcel_log::rbvcel_section("Cut the regulus (RBSHE step 3)");

    let regulus_dirname = rbvcew_world::rbvcew_bundle(world).regulus_dirname;
    let regulus_parent = parent.join(regulus_dirname);
    rbvcep_repo::rbvcep_guard_disposable(&regulus_parent, regulus_dirname, top);
    rbvcel_log::rbvcel_step(&format!(
        "Disposing any prior regulus: {}",
        regulus_parent.display(), //
    ));
    if !rbvcep_repo::rbvcep_sepose(&regulus_parent, top) {
        rbvcel_log::rbvcel_line("no prior regulus to sepose");
    }

    // The absorbed cut (RBSHC "The cut, and the single matcher"): builds by
    // addition into a clone of the public base, refusing on its own gates —
    // clean tree, total perambulation, base identity + neuter, template
    // present, byte-assert, single-commit, delta sweep, zero remotes — all
    // judged in-process by the one matcher. Fatal on any deficit; a return
    // is the verdict.
    rbvcel_log::rbvcel_step(&format!(
        "Expediting the regulus into {}",
        regulus_parent.display(), //
    ));
    let regulus_clone = rbvcex_expede::rbvcex_cut(top, &regulus_parent, world);

    rbvcel_log::rbvcel_line(&format!(
        "regulus cut: {}",
        regulus_clone.display(), //
    ));
    regulus_clone
}

// ── Step 4: prove the regulus (the battery, in order) ─────

fn zrbvcce_prove(
    parent: &Path,
    regulus_clone: &Path,
    top: &Path,
    bundle: &rbvcew_world::rbvcew_Bundle,
) {
    rbvcel_log::rbvcel_section("Prove the regulus — the battery (RBSHE step 4)");
    let regulus_parent = parent.join(bundle.regulus_dirname);
    let tt = regulus_clone.join(RBVCCD_TT_SUBDIR);

    zrbvcce_write_station(&regulus_parent);

    // The regulus's OWN tabtargets, on its sterile POSTULANT_LOCAL branch. The
    // regulus's z-launcher normalizes cwd to the regulus root, so these assay
    // the regulus though essai never leaves the maintainer tree.
    let qualify = zrbvccd_find_tt(&tt, RBVCCE_COL_QUALIFY_FAST, None);
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&qualify, &[], top, &[]),
        "regulus fast-qualify",
    );

    let fixture = zrbvccd_find_tt(&tt, RBVCCE_COL_FIXTURE, None);
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&fixture, &[RBVCCE_FIX_CUPEL], top, &[]),
        "regulus cupel",
    );
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&fixture, &[RBVCCE_FIX_PYX], top, &[]),
        "regulus pyx",
    );
    // Damnatio on POSTULANT_LOCAL, BEFORE any feigning — the proof of erasure.
    // It reddens on feigned fields by construction, which is what keeps a probe
    // branch from ever being mistaken for a regulus.
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&fixture, &[RBVCCE_FIX_DAMNATIO], top, &[]),
        "regulus damnatio",
    );

    // The regulus's root CLAUDE.md, shipped from the maintainer's committed
    // bytes like every other path, must carry no veil needle —
    // re-homed in-process from the theurge damnatio fixture's veil_stripped case,
    // whose census-bearing scan could only run where the veiled trees still
    // stand. The path-grain half (a withheld tree survived the strip) is already
    // covered by expede's object-graph delta sweep at cut time.
    let needles = rbvceu_loupe::rbvceu_assay_regulus(regulus_clone);
    if !needles.is_empty() {
        for needle in &needles {
            rbvcel_log::rbvcel_line(&format!(
                "regulus veil needle: {}",
                needle, //
            ));
        }
        crate::rbvcel_fatal!(
            "the regulus's root CLAUDE.md carries {} veil needle(s) — abandon the regulus, repair on the maintainer tree, and re-cut (RBSHE)",
            needles.len(), //
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
    let tackroom_dir = regulus_parent.join(rbvcep_repo::RBVCEP_TACKROOM_SUBDIR);

    zrbvcce_mkdir(&station_dir);
    zrbvcce_mkdir(&secrets_dir);
    zrbvcce_mkdir(&logs_dir);
    zrbvcce_mkdir(&tackroom_dir);

    // The station regime names its stores relative to the project root and
    // refuses an absolute one; the regulus clone is a sibling of both, so each
    // is named one level up from it.
    let burs = format!(
        "BURS_USER={}\nBURS_TINCTURE={}\nBURS_LOG_DIR=../{}\nBURS_TACKROOM=../{}\n",
        RBVCCE_STATION_USER,
        RBVCCE_STATION_TINCTURE,
        rbvcep_repo::RBVCEP_LOGS_SUBDIR,
        rbvcep_repo::RBVCEP_TACKROOM_SUBDIR, //
    );
    let burs_path = station_dir.join(rbvcep_repo::RBVCEP_STATION_FILE);
    std::fs::write(&burs_path, burs).unwrap_or_else(|e| {
        crate::rbvcel_fatal!(
            "failed to write station {}: {}",
            burs_path.display(),
            e, //
        )
    });
    rbvcel_log::rbvcel_line(&format!(
        "station written: {}",
        burs_path.display(), //
    ));
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
    let head = rbvcer_run::rbvcer_capture(
        "git",
        &["-C", &clone, "rev-parse", "--abbrev-ref", "HEAD"],
        top,
    );
    zrbvcce_require_regulus(head.code, "read regulus branch");
    let sterile_branch = head.stdout.trim().to_string();

    // The probe branch the feign door admits, read from the regulus clone's own
    // declaration rather than named here.
    let probe_branch = zrbvcce_probe_branch(regulus_clone, top);
    zrbvcce_git_stream(
        &clone,
        &["checkout", "-b", &probe_branch],
        top,
        "cut the probe branch",
    );

    // Hand the regulus its feign tabtarget: a byte-copy of an existing
    // regulus tabtarget under the withheld colophon's name (the trampolines are
    // byte-generic; the runtime `${0##*/}` resolves the rbw-MF colophon).
    let donor = zrbvccd_find_tt(tt, RBVCCE_COL_QUALIFY_FAST, None);
    let feign_tt = tt.join(RBVCCE_FEIGN_TT);
    bkprs_program_copy(&donor, &feign_tt).unwrap_or_else(|e| {
        crate::rbvcel_fatal!(
            "failed to install the feign tabtarget {}: {}",
            feign_tt.display(),
            e, //
        )
    });

    // Rebuild (regenerates the regulus's tabtarget context from the now-complete
    // tt/ set), then commit the seed so the tree is clean before feign — feign's
    // own clean-tree gate demands it, and the probe commit must carry the seed
    // alone.
    let build = zrbvccd_find_tt(tt, RBVCCE_COL_BUILD, None);
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&build, &[], top, &[]),
        "regulus build (probe)",
    );
    zrbvcce_git_stream(&clone, &["add", "-A"], top, "stage the probe seed");
    zrbvcce_git_stream(
        &clone,
        &["commit", "-m", "probe: feign a station"],
        top,
        "commit the probe seed",
    );

    // Feign a visibly-false station (BURE_CONFIRM=skip — the ceremony drives it
    // headlessly), then run the regulus's reveille from the consumer's seat.
    let feign = tt.join(RBVCCE_FEIGN_TT);
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&feign, &[], top, &[("BURE_CONFIRM", "skip")]),
        "regulus feign",
    );
    let reveille = zrbvccd_find_tt(tt, RBVCCD_COL_SUITE, Some(RBVCCE_SUITE_REVEILLE));
    zrbvcce_require_regulus(
        rbvcer_run::rbvcer_stream(&reveille, &[], top, &[]),
        "regulus reveille (consumer seat)",
    );

    // Return to the sterile branch and drop the probe outright — it deliberately
    // holds withheld paths (the feigned station, the copied marshal tabtarget), so
    // dropping it leaves the clone carrying exactly one branch and zero remotes.
    rbvcel_log::rbvcel_step("Returning to the sterile branch and dropping the probe");
    zrbvcce_git_stream(
        &clone,
        &["checkout", &sterile_branch],
        top,
        "return to the sterile branch",
    );
    zrbvcce_git_stream(
        &clone,
        &["branch", "-D", &probe_branch],
        top,
        "drop the probe branch",
    );
    rbvcel_log::rbvcel_line(&format!(
        "consumer-seat reveille green; regulus back on {}",
        sterile_branch, //
    ));
}

/// The throwaway probe branch's name, printed by a bash child that sources the
/// regulus clone's own constants module — the one declaration, and the copy the
/// feign door reads. Fatal on a nonzero exit or an empty value.
fn zrbvcce_probe_branch(regulus_clone: &Path, top: &Path) -> String {
    let module = rbvcep_repo::rbvcep_as_str(&regulus_clone.join(RBVCCE_CONSTANTS_MODULE));
    let got = rbvcer_run::rbvcer_capture("bash", &["-c", RBVCCE_PROBE_READ, "bash", &module], top);
    if got.code != 0 {
        crate::rbvcel_fatal!(
            "failed to read the probe branch from {} (bash exited {}): {}",
            module,
            got.code,
            got.stderr.trim(), //
        );
    }
    let branch = got.stdout.trim().to_string();
    if branch.is_empty() {
        crate::rbvcel_fatal!(
            "the probe branch read from {} is empty",
            module, //
        );
    }
    rbvcel_log::rbvcel_line(&format!(
        "probe branch read from {}: {}",
        module,
        branch, //
    ));
    branch
}

// ── Small shared helpers ────────────────────────────────────

/// git -C <clone> <args>, streamed, fatal on non-zero with the act named.
fn zrbvcce_git_stream(clone: &str, args: &[&str], top: &Path, act: &str) {
    let mut full = vec!["-C", clone];
    full.extend_from_slice(args);
    let code = rbvcer_run::rbvcer_stream("git", &full, top, &[]);
    if code != 0 {
        crate::rbvcel_fatal!(
            "failed to {} (git exited {})",
            act,
            code, //
        );
    }
}

fn zrbvcce_mkdir(dir: &Path) {
    std::fs::create_dir_all(dir).unwrap_or_else(|e| {
        crate::rbvcel_fatal!(
            "failed to create {}: {}",
            dir.display(),
            e, //
        )
    });
}

/// A red in the gate or a pre-cut assay: the fault is on the maintainer tree.
fn zrbvcce_require_source(code: i32, what: &str) {
    if code != 0 {
        crate::rbvcel_fatal!(
            "{} failed (exit {}) — repair on the maintainer tree, then run essai again (RBSHE)",
            what,
            code, //
        );
    }
}

/// A red in the regulus battery: abandon the regulus, repair on the
/// maintainer tree, re-cut. Never patch the regulus forward (RBSHE).
fn zrbvcce_require_regulus(code: i32, what: &str) {
    if code != 0 {
        crate::rbvcel_fatal!(
            "{} failed (exit {}) — abandon the regulus, repair on the maintainer tree, and re-cut (RBSHE: a finding means re-cut, never patch forward)",
            what, code, //
        );
    }
}
