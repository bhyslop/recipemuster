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
// RBTHDR — the coldwalk rig: the one truly absorbed module (RBSHC "Worker,
// never authority"). It clones a tree, guards it so nothing the walk does can
// reach any remote (severed origin + a pre-push refusal hook), cuts a throwaway
// walk branch, and hands off the launch line and stranger prompt.
//
// ONE implementation, TWO callers, differing only in the clone source string:
//   essai      — clones the LOCAL candidate (a fidelity-reduced local proxy).
//   harbinger  — clones the PUBLIC promoted repo over HTTPS (a later pace).
// `git clone` takes a local path and an https URL identically, so the source is
// a bare string and there is no second code path. This module reads no shared
// authority; it is not a worker reached by subprocess but the hierophant's own
// absorbed surgery, formerly rblm_harbinger.sh (a superseded bash instrument).
//
// RIG PUSHES NOTHING, EVER. The station running essai holds real credentials to
// the public repository; the guard is therefore structural, not a rule to
// remember — the clone names no remote, and even a re-added one dies at the hook.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::rbthdr_log;
use crate::rbthdr_repo;
use crate::rbthdr_run;

/// The rig's parent-relative dirname, a fixed basename beside the maintainer
/// repo. Shared by essai and harbinger — they never stand simultaneously, so one
/// location, retire-aside disposing any prior.
pub const RBTHDR_RIG_DIRNAME: &str = "rbm_coldwalk";

/// The clone lives one level down; the findings memo is a sibling of it under
/// the rig dir, so discarding the clone leaves the memo standing.
const RBTHDR_RIG_CLONE_SUBDIR: &str = "recipebottle";

/// The pristine reference the walk diffs against is the clone's own default
/// branch; the walker commits kludges on this throwaway branch.
const RBTHDR_RIG_WALK_BRANCH: &str = "coldwalk";

/// The findings-memo basename slug; the full basename is dated at run time.
const RBTHDR_RIG_MEMO_SLUG: &str = "coldwalk-shakedown";

/// The public face and the stranger's first read — its absence means the clone
/// is not what the walk expects.
const RBTHDR_RIG_FACE: &str = "README.md";

/// The pre-push refusal hook body, belt to the sever's suspenders: even if the
/// walk re-adds a remote, every push dies here with a loud, legible reason.
const RBTHDR_RIG_HOOK_BODY: &str = "\
#!/bin/sh
echo \"coldwalk clone: pushing is disabled — this is a throwaway shakedown rig and must never push\" >&2
exit 1
";

/// The stranger prompt handed to the cold agent, verbatim. `__MEMO_PATH__` is
/// substituted with the rig's dated findings-memo path at emit. This is the
/// absorbed authority the cinch names — one copy, here.
const RBTHDR_STRANGER_PROMPT: &str = r#"----------------------------------------------------------------------
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

/// The standing artifacts of a stood-up rig.
pub struct rbthdr_Rig {
    pub rig_dir: PathBuf,
    pub clone_dir: PathBuf,
    pub memo_path: PathBuf,
}

/// Stand up the guarded coldwalk rig from `clone_source` (a local candidate path
/// or a public URL). Narrates each act. Returns the standing artifacts; the
/// launch line and stranger prompt are emitted separately by `emit_handoff`, so
/// a caller may interpose its own advisories (essai shows its fidelity gap first).
pub fn stand_up(parent: &Path, top: &Path, clone_source: &str, cwd: &Path) -> rbthdr_Rig {
    let rig_dir = parent.join(RBTHDR_RIG_DIRNAME);
    let clone_dir = rig_dir.join(RBTHDR_RIG_CLONE_SUBDIR);

    // Guard before anything moves: the rig dir is the fixed name and not the repo
    // root. The disposal is a rename, so a wrong guard destroys nothing — the
    // guard makes a wrong move impossible in the first place.
    rbthdr_repo::guard_disposable(&rig_dir, RBTHDR_RIG_DIRNAME, top);

    let walk_date = rbthdr_run::datestamp(cwd);
    let memo_path = rig_dir.join(format!("memo-{}-{}.md", walk_date, RBTHDR_RIG_MEMO_SLUG));

    rbthdr_log::step(&format!("Retiring any existing rig aside: {}", rig_dir.display()));
    if !rbthdr_repo::retire_aside(&rig_dir, cwd) {
        rbthdr_log::line("no prior rig to retire");
    }
    std::fs::create_dir_all(&rig_dir)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to create the rig dir {}: {}", rig_dir.display(), e));

    rbthdr_log::step(&format!("Cloning into the rig from {}", clone_source));
    let clone_dir_str = zrbthdr_path_str(&clone_dir);
    let code = rbthdr_run::stream("git", &["clone", clone_source, &clone_dir_str], cwd, &[]);
    if code != 0 {
        crate::rbthdr_fatal!("failed to clone {} into {}", clone_source, clone_dir.display());
    }

    // The clone must carry the onboarding face, or the walk would start on nothing.
    if !clone_dir.join(RBTHDR_RIG_FACE).is_file() {
        crate::rbthdr_fatal!(
            "the clone carries no {} — refusing to hand the walker a tree with no onboarding face",
            RBTHDR_RIG_FACE
        );
    }

    // Sever the origin. From here the clone names no remote, so nothing it does
    // can reach any repository. Materialization is already complete, so the sever
    // costs the walk nothing.
    rbthdr_log::step("Severing the clone from its origin");
    zrbthdr_git_c(&clone_dir, &["remote", "remove", "origin"], cwd, "sever the clone's origin");

    rbthdr_log::step("Installing the pre-push refusal hook");
    let hook = clone_dir.join(".git").join("hooks").join("pre-push");
    std::fs::write(&hook, RBTHDR_RIG_HOOK_BODY)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to write the pre-push hook {}: {}", hook.display(), e));
    let mut perms = std::fs::metadata(&hook)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to stat the pre-push hook: {}", e))
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&hook, perms)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to make the pre-push hook executable: {}", e));

    // Cut the throwaway walk branch; the clone's own default branch stays the
    // pristine reference to diff the walk against.
    rbthdr_log::step(&format!("Cutting the throwaway walk branch {}", RBTHDR_RIG_WALK_BRANCH));
    zrbthdr_git_c(&clone_dir, &["checkout", "-b", RBTHDR_RIG_WALK_BRANCH], cwd, "cut the walk branch");

    rbthdr_Rig { rig_dir, clone_dir, memo_path }
}

/// Emit the launch line and the stranger prompt — the shared handoff both essai
/// and harbinger end on. The walk is the operator's own, launched by hand from
/// the printed line; the rig launches nothing and pushes nothing.
pub fn emit_handoff(rig: &rbthdr_Rig) {
    let clone = zrbthdr_path_str(&rig.clone_dir);
    let memo = zrbthdr_path_str(&rig.memo_path);

    rbthdr_log::blank();
    rbthdr_log::line("Rig ready. Two steps, by your hand:");
    rbthdr_log::blank();
    rbthdr_log::line("1. Launch a cold session in the clone (NEW terminal):");
    rbthdr_log::blank();
    rbthdr_log::raw(&format!("        (cd {} && claude --model sonnet --permission-mode auto)", clone));
    rbthdr_log::blank();
    rbthdr_log::line("2. Paste the stranger prompt below:");
    rbthdr_log::blank();
    rbthdr_log::raw(&RBTHDR_STRANGER_PROMPT.replace("__MEMO_PATH__", &memo));
    rbthdr_log::blank();
    rbthdr_log::line(&format!(
        "When the walk is done, review {} in the rig, commit it into",
        zrbthdr_basename(&rig.memo_path)
    ));
    rbthdr_log::line("Memos/ affiliated to the pace, then discard the rig.");
}

/// git -C <dir> <args>, fatal on non-zero with the act named.
fn zrbthdr_git_c(dir: &Path, args: &[&str], cwd: &Path, act: &str) {
    let dir_str = zrbthdr_path_str(dir);
    let mut full = vec!["-C", &dir_str];
    full.extend_from_slice(args);
    let code = rbthdr_run::stream("git", &full, cwd, &[]);
    if code != 0 {
        crate::rbthdr_fatal!("failed to {} in {}", act, dir.display());
    }
}

/// A path as a &str — the shared UTF-8-or-fatal conversion (rbthdr_repo::as_str).
fn zrbthdr_path_str(p: &Path) -> String {
    rbthdr_repo::as_str(p)
}

fn zrbthdr_basename(p: &Path) -> String {
    p.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| crate::rbthdr_fatal!("cannot read basename of {}", p.display()))
        .to_string()
}
