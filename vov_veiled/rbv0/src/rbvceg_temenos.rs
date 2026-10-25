// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCEG — the coldwalk temenos: the one truly absorbed module (RBSHC "Worker,
// never authority"). It clones a tree, guards it so nothing the walk does can
// reach any remote (severed origin + a pre-push refusal hook), cuts a throwaway
// walk branch, and hands off the launch line and stranger prompt.
//
// ONE implementation, TWO callers, differing only in the clone source string:
//   essai      — clones the LOCAL regulus (a fidelity-reduced local proxy).
//   harbinger  — clones the PUBLIC promoted repo over HTTPS (a later pace).
// `git clone` takes a local path and an https URL identically, so the source is
// a bare string and there is no second code path. This module reads no shared
// authority; it is not a worker reached by subprocess but the hierophant's own
// absorbed surgery.
//
// TEMENOS PUSHES NOTHING, EVER. The station running essai holds real credentials to
// the public repository; the guard is therefore structural, not a rule to
// remember — the clone names no remote, and even a re-added one dies at the hook.

use std::path::{Path, PathBuf};

use bkp::bkprs_scrivener::bkprs_program_write;

use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvcer_run;

/// The temenos's parent-relative dirname, a fixed basename beside the maintainer
/// repo. Shared by essai and harbinger — they never stand simultaneously, so one
/// location, any prior seposed.
/// A conserved spelling, declared in the cosmology's cadaster, and the same
/// directory the bash harbinger stands its rig in.
pub const RBVCEG_TEMENOS_DIRNAME: &str = "rbthdr_coldwalk";

/// The clone lives one level down; the findings memo is a sibling of it under
/// the temenos dir, so discarding the clone leaves the memo standing.
const RBVCEG_TEMENOS_CLONE_SUBDIR: &str = "recipebottle";

/// The pristine reference the walk diffs against is the clone's own default
/// branch; the walker commits kludges on this throwaway branch.
const RBVCEG_TEMENOS_WALK_BRANCH: &str = "coldwalk";

/// The findings-memo basename slug; the full basename is dated at run time.
const RBVCEG_TEMENOS_MEMO_SLUG: &str = "coldwalk-shakedown";

/// The public face and the stranger's first read — its absence means the clone
/// is not what the walk expects.
const RBVCEG_TEMENOS_FACE: &str = "README.md";

/// The pre-push refusal hook body, belt to the sever's suspenders: even if the
/// walk re-adds a remote, every push dies here with a loud, legible reason.
const RBVCEG_TEMENOS_HOOK_BODY: &str = "\
#!/bin/sh
echo \"coldwalk clone: pushing is disabled — this is a throwaway shakedown rig and must never push\" >&2
exit 1
";

/// The stranger prompt handed to the cold agent, verbatim. `__MEMO_PATH__` is
/// substituted with the temenos's dated findings-memo path at emit. This is the
/// absorbed authority the cinch names — one copy, here.
const RBVCEG_STRANGER_PROMPT: &str = r#"----------------------------------------------------------------------
You are trying Recipe Bottle for the first time. You cloned it because you want to
use it to build and run hardened, provenance-checked container images, and you are
starting completely cold: everything you know about this project must come from the
documentation in this repository. Do not reach for outside knowledge of how it
"really" works — if a step is unclear from the docs in front of you, that unclearness
is exactly what you are here to find.

Your operator is a human at the keyboard with you. You drive; they grant permission
prompts, and they own everything involving money, accounts, and credentials.

Walk the onboarding in four beats, in order:

  1. Local setup. Read the README, find the onboarding entry point, and follow it to a
     first working local image build and a charged crucible you can shell into.

  2. Cloud setup. The docs will reach a point that needs a Google Cloud account, a
     billing-backed project, and an identity provider. This is your operator's block,
     not yours. Read the relevant guide, explain each step, and hand the keyboard to
     your operator for every account, billing, and credential action. STOP and hand off
     the moment the docs reach creating accounts or paid infrastructure.

  3. Adversarial suite. Once cloud setup is done, charge the tadmor crucible and run its
     adversarial security suite, confirming the containment holds under attack.

  4. Airgap build. Finally, walk the moriah airgap cloud build to its provenance
     comparison.

Hard boundaries, always:
  - Create no accounts and spend no money on your own. Those are your operator's, at the
    gates above.
  - Push nowhere. This clone is a throwaway; treat every remote as off-limits.
  - Never stop, quench, or delete any workload, container, or cloud resource you did not
    start yourself.

Keep a findings log the whole way. Every time a doc is unclear, a command fails, a step
assumes knowledge you do not have, or you have to guess — write it down, with the exact
command and what you expected versus what happened. Put the log here, and update it as
you go:

  __MEMO_PATH__

Begin by reading the README, then find the onboarding entry point and take the first
beat.
----------------------------------------------------------------------"#;

/// The standing artifacts of a stood-up temenos.
pub struct rbvceg_Temenos {
    pub temenos_dir: PathBuf,
    pub clone_dir: PathBuf,
    pub memo_path: PathBuf,
}

/// Stand up the guarded coldwalk temenos from `clone_source` (a local regulus path
/// or a public URL). Narrates each act. Returns the standing artifacts; the
/// launch line and stranger prompt are emitted separately by `emit_handoff`, so
/// a caller may interpose its own advisories (essai shows its fidelity gap first).
pub fn rbvceg_stand_up(parent: &Path, top: &Path, clone_source: &str, cwd: &Path) -> rbvceg_Temenos {
    let temenos_dir = parent.join(RBVCEG_TEMENOS_DIRNAME);
    let clone_dir = temenos_dir.join(RBVCEG_TEMENOS_CLONE_SUBDIR);

    // Guard before anything moves: the temenos dir is the fixed name and not the repo
    // root. The disposal is a rename, so a wrong guard destroys nothing — the
    // guard makes a wrong move impossible in the first place.
    rbvcep_repo::rbvcep_guard_disposable(&temenos_dir, RBVCEG_TEMENOS_DIRNAME, top);

    let walk_date = rbvcer_run::rbvcer_datestamp(cwd);
    let memo_path = temenos_dir.join(format!("memo-{}-{}.md", walk_date, RBVCEG_TEMENOS_MEMO_SLUG));

    rbvcel_log::rbvcel_step(&format!("Seposing any existing temenos: {}", temenos_dir.display()));
    if !rbvcep_repo::rbvcep_sepose(&temenos_dir, cwd) {
        rbvcel_log::rbvcel_line("no prior temenos to sepose");
    }
    std::fs::create_dir_all(&temenos_dir)
        .unwrap_or_else(|e| crate::rbvcel_fatal!("failed to create the temenos dir {}: {}", temenos_dir.display(), e));

    rbvcel_log::rbvcel_step(&format!("Cloning into the temenos from {}", clone_source));
    let clone_dir_str = zrbvceg_path_str(&clone_dir);
    let code = rbvcer_run::rbvcer_stream("git", &["clone", clone_source, &clone_dir_str], cwd, &[]);
    if code != 0 {
        crate::rbvcel_fatal!("failed to clone {} into {}", clone_source, clone_dir.display());
    }

    // The clone must carry the onboarding face, or the walk would start on nothing.
    if !clone_dir.join(RBVCEG_TEMENOS_FACE).is_file() {
        crate::rbvcel_fatal!(
            "the clone carries no {} — refusing to hand the walker a tree with no onboarding face",
            RBVCEG_TEMENOS_FACE
        );
    }

    // Sever the origin. From here the clone names no remote, so nothing it does
    // can reach any repository. Materialization is already complete, so the sever
    // costs the walk nothing.
    rbvcel_log::rbvcel_step("Severing the clone from its origin");
    zrbvceg_git_c(&clone_dir, &["remote", "remove", "origin"], cwd, "sever the clone's origin");

    rbvcel_log::rbvcel_step("Installing the pre-push refusal hook");
    let hook = clone_dir.join(".git").join("hooks").join("pre-push");
    bkprs_program_write(&hook, RBVCEG_TEMENOS_HOOK_BODY)
        .unwrap_or_else(|e| crate::rbvcel_fatal!("failed to write the pre-push hook {}: {}", hook.display(), e));

    // Cut the throwaway walk branch; the clone's own default branch stays the
    // pristine reference to diff the walk against.
    rbvcel_log::rbvcel_step(&format!("Cutting the throwaway walk branch {}", RBVCEG_TEMENOS_WALK_BRANCH));
    zrbvceg_git_c(&clone_dir, &["checkout", "-b", RBVCEG_TEMENOS_WALK_BRANCH], cwd, "cut the walk branch");

    rbvceg_Temenos { temenos_dir, clone_dir, memo_path }
}

/// Emit the launch line and the stranger prompt — the shared handoff both essai
/// and harbinger end on. The walk is the operator's own, launched by hand from
/// the printed line; the temenos launches nothing and pushes nothing.
pub fn rbvceg_emit_handoff(temenos: &rbvceg_Temenos) {
    let clone = zrbvceg_path_str(&temenos.clone_dir);
    let memo = zrbvceg_path_str(&temenos.memo_path);

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("Temenos ready. Two steps, by your hand:");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("1. Launch a cold session in the clone (NEW terminal):");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_raw(&format!("        (cd {} && claude --model sonnet --permission-mode auto)", clone));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("2. Paste the stranger prompt below:");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_raw(&RBVCEG_STRANGER_PROMPT.replace("__MEMO_PATH__", &memo));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line(&format!(
        "When the walk is done, review {} in the temenos, commit it into",
        zrbvceg_basename(&temenos.memo_path)
    ));
    rbvcel_log::rbvcel_line("the studbook (jjqs_studbook/) affiliated to the pace, then discard the temenos.");
}

/// git -C <dir> <args>, fatal on non-zero with the act named.
fn zrbvceg_git_c(dir: &Path, args: &[&str], cwd: &Path, act: &str) {
    let dir_str = zrbvceg_path_str(dir);
    let mut full = vec!["-C", &dir_str];
    full.extend_from_slice(args);
    let code = rbvcer_run::rbvcer_stream("git", &full, cwd, &[]);
    if code != 0 {
        crate::rbvcel_fatal!("failed to {} in {}", act, dir.display());
    }
}

/// A path as a &str — the shared UTF-8-or-fatal conversion (rbvcep_repo::rbvcep_as_str).
fn zrbvceg_path_str(p: &Path) -> String {
    rbvcep_repo::rbvcep_as_str(p)
}

fn zrbvceg_basename(p: &Path) -> String {
    p.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| crate::rbvcel_fatal!("cannot read basename of {}", p.display()))
        .to_string()
}
