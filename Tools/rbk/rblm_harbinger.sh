#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBLM Harbinger - the cold-agent onboarding shakedown rig: stand up a disposable,
# guarded clone of the promoted public repository and hand off a stranger prompt for
# a zero-context agent to walk the shipped onboarding literally.
#
# A harbinger rides ahead of the host to test the road. This verb rides ahead of the
# real consumer: it lands a fresh clone of exactly what the public sees, guards it so
# nothing the walk does can escape, and prints the launch line and prompt the operator
# uses to send a cold Claude session down the onboarding path. The agent has no
# maintainer intuition to paper over a doc gap with, so its stumbles are the map of
# where the shipped docs assume knowledge a stranger does not have.
#
# THIS FILE STAYS BEHIND, and its tabtarget (tt/rbw-MH) with it. Harbinger is a
# release-rig verb — it exists to shake down a delivery before real users arrive, and
# a consumer holding a candidate has no delivery to shake down. It is withheld by the
# perambulation exactly as expede is: the module by an explicit row, the tabtarget by
# the tt/rbw-M stem. The delivered rblm_cli.sh's furnish sources this module only when
# the command is rblm_harbinger, so a stripped tree that never carries it dies naming
# what is missing rather than dangling on an absent function.
#
# HARBINGER PUSHES NOTHING, EVER. It clones over HTTPS (anonymous read of the public
# repo), then severs the clone's origin and installs a pre-push hook that refuses every
# push — belt and suspenders, because THIS station holds real credentials to the public
# repository and a stray push from the walk clone is the one act the rig must make
# structurally impossible. The walk that follows is the operator's own, launched by
# hand from the printed line; harbinger sets the stage and steps off it.

set -euo pipefail

######################################################################
# Harbinger constants

# The disposable clone's parent directory, a fixed basename beside the maintainer
# repository. Derived from git (the repo's parent), never hand-typed, so the rig
# always lands next to the tree it was launched from. The basename is a constant and
# is asserted before the nuke below: harbinger removes this directory wholesale, so it
# may only ever name THIS one, never the maintainer tree it sits beside.
RBLM_harbinger_dirname="rbm_coldwalk"

# The clone lives one level down, in its own subdirectory. The findings memo is a
# sibling of it under the parent dir, so discarding the clone subdirectory leaves the
# memo standing for the operator to review and commit.
RBLM_harbinger_clone_subdir="recipebottle"

# The promoted public repository — HTTPS, anonymous read. This is the exact face a
# real stranger clones: the default branch of the public repo after promotion. Named
# in full here because harbinger stays behind (withheld) and the walk must land on the
# genuine public tree, not a maintainer remote alias.
RBLM_harbinger_public_url="https://github.com/scaleinv/recipebottle.git"

# The pristine reference the walk diffs against. The clone's own default branch is left
# untouched as the reference; the walker commits its kludges on a throwaway branch, so
# a diff at the end shows exactly what the walk changed.
RBLM_harbinger_walk_branch="coldwalk"

# The findings-memo basename slug. The full basename is dated at run time:
# memo-<YYYYMMDD>-coldwalk-shakedown.md — the maintainer reviews it and commits it into
# Memos/ affiliated to the pace.
RBLM_harbinger_memo_slug="coldwalk-shakedown"

# The prompt is written to this file in the parent dir as well as printed, so the
# operator has a clean, copy-pasteable source that no terminal line-wrap can mangle.
RBLM_harbinger_prompt_file="coldwalk-prompt.txt"

######################################################################
# Command: harbinger - stand up the guarded cold-walk clone and hand off
#
# The verb is a pure local constructor with one destructive step (the nuke of the
# disposable parent, gated and asserted) and no reach toward the public repository
# beyond an anonymous read. It ends by printing the operator's launch line and the
# stranger prompt; the walk itself is the operator's, not harbinger's.
rblm_harbinger() {
  buc_doc_brief "Harbinger the cold-agent onboarding shakedown - stand up a guarded disposable clone of promoted public main and print the launch line and stranger prompt"
  buc_doc_shown || return 0

  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"

  # Where we are. The rig lands beside this tree, so its identity is the anchor for
  # every derived path below — and the guard that the nuke can never hit it.
  local -r z_toplevel_temp="${BURD_TEMP_DIR}/rblm_harbinger_toplevel.txt"
  git rev-parse --show-toplevel > "${z_toplevel_temp}" || buc_die "git rev-parse --show-toplevel failed — harbinger must run inside a git repository"
  local -r z_toplevel=$(<"${z_toplevel_temp}")
  test -n "${z_toplevel}" || buc_die "git returned an empty repository root"
  case "${z_toplevel}" in
    /*) ;;
    *)  buc_die "Repository root is not an absolute path: ${z_toplevel}" ;;
  esac

  local -r z_parent="${z_toplevel%/*}"
  local -r z_target_dir="${z_parent}/${RBLM_harbinger_dirname}"

  # Nuke guard. Harbinger removes the target wholesale, so it must be exactly the
  # constant-named disposable dir and never the maintainer tree it sits beside. Both
  # are asserted before a single byte is removed: the basename must be the fixed name,
  # and the target must differ from this repository's own root.
  local -r z_target_base="${z_target_dir##*/}"
  test "${z_target_base}" = "${RBLM_harbinger_dirname}" \
    || buc_die "Refusing to nuke '${z_target_dir}' — its basename is not the disposable rig name '${RBLM_harbinger_dirname}'"
  test "${z_target_dir}" != "${z_toplevel}" \
    || buc_die "Refusing to nuke '${z_target_dir}' — it is this repository's own root"

  local -r z_clone_dir="${z_target_dir}/${RBLM_harbinger_clone_subdir}"

  # The dated findings-memo path — absolute, and a sibling of the clone so it survives
  # the clone's discard. The prompt below hands this exact path to the walker; the
  # maintainer reviews it and commits it into Memos/ after the walk.
  local -r z_walkdate_temp="${BURD_TEMP_DIR}/rblm_harbinger_walkdate.txt"
  date +%Y%m%d > "${z_walkdate_temp}" || buc_die "date failed"
  local -r z_walkdate=$(<"${z_walkdate_temp}")
  test -n "${z_walkdate}" || buc_die "walk date resolved empty"
  local -r z_memo_path="${z_target_dir}/memo-${z_walkdate}-${RBLM_harbinger_memo_slug}.md"
  local -r z_prompt_path="${z_target_dir}/${RBLM_harbinger_prompt_file}"

  buh_section "Marshal Harbinger — cold-agent onboarding shakedown"
  buh_line "  Maintainer tree:   ${z_toplevel}"
  buh_line "  Disposable rig:    ${z_target_dir}  (nuked and recreated)"
  buh_line "  Public clone:      ${z_clone_dir}"
  buh_line "  Public source:     ${RBLM_harbinger_public_url}"
  buh_line "  Walk branch:       ${RBLM_harbinger_walk_branch}  (default branch left pristine)"
  buh_line "  Findings memo:     ${z_memo_path}"
  buh_e
  buh_line "  A fresh clone of the promoted public repository, guarded so nothing the"
  buh_line "  walk does can reach any remote: the clone's origin is severed and a"
  buh_line "  pre-push hook refuses every push. THIS station holds real credentials to"
  buh_line "  the public repo, so the guard is structural, not a rule to remember."
  buh_e
  buh_line "  The walk is yours to run, by hand, from the launch line printed at the"
  buh_line "  end. Harbinger clones, guards, and hands off — it launches nothing and"
  buh_line "  pushes nothing."
  buh_e
  buc_require "Nuke and rebuild the disposable cold-walk rig at ${z_target_dir}?" "harbinger"

  buc_step "Clearing the disposable rig"
  rm -rf "${z_target_dir}" || buc_die "Failed to clear the disposable rig: ${z_target_dir}"
  mkdir -p "${z_target_dir}" || buc_die "Failed to create the disposable rig: ${z_target_dir}"

  buc_step "Cloning the promoted public repository"
  git clone "${RBLM_harbinger_public_url}" "${z_clone_dir}" || buc_die "Failed to clone the public repository — is ${RBLM_harbinger_public_url} reachable and public?"

  # The clone must carry the promoted content, not an empty or wrong tree. The README
  # is the public face and the stranger's first read; its absence means the clone is
  # not what the walk expects, and the walk would start on nothing.
  test -f "${z_clone_dir}/README.md" \
    || buc_die "The clone carries no README.md — the promoted public tree is empty or wrong; refusing to hand the walker a tree with no onboarding face"

  # Sever the origin. From here the clone names no remote, so nothing it does can
  # reach the public repository. The materialization is already complete, so the
  # sever costs the walk nothing.
  buc_step "Severing the clone from its origin"
  git -C "${z_clone_dir}" remote remove origin || buc_die "Failed to sever the clone's origin"

  # Install the pre-push refusal. Belt to the sever's suspenders: even if the walk (or
  # a helpful agent) re-adds a remote, every push dies here. Written with a loud reason
  # so the refusal is legible, not a silent non-zero.
  buc_step "Installing the pre-push refusal hook"
  local -r z_hook="${z_clone_dir}/.git/hooks/pre-push"
  cat > "${z_hook}" <<'HOOK'
#!/bin/sh
echo "coldwalk clone: pushing is disabled — this is a throwaway shakedown rig and must never push" >&2
exit 1
HOOK
  chmod +x "${z_hook}" || buc_die "Failed to make the pre-push hook executable"

  # Cut the throwaway walk branch. The consumer flow commits as it kludges; the walker
  # works here, and the clone's own default branch stays untouched as the pristine
  # reference to diff the walk against.
  buc_step "Cutting the throwaway walk branch ${RBLM_harbinger_walk_branch}"
  git -C "${z_clone_dir}" checkout -b "${RBLM_harbinger_walk_branch}" || buc_die "Failed to cut the walk branch"

  # Write the stranger prompt to a file (copy-pasteable, unwrapped) and print it below.
  # The prompt carries the memo path so the walker knows where to log its findings.
  buc_step "Writing the stranger prompt"
  cat > "${z_prompt_path}" <<PROMPT
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

  ${z_memo_path}

Begin by reading the README, then find the onboarding entry point and take the first
beat.
PROMPT

  buh_e
  buh_line "  Rig ready. Two steps, by your hand:"
  buh_e
  buh_line "  1. Launch a cold session in the clone (NEW terminal):"
  buh_e
  buc_bare "        cd ${z_clone_dir}"
  buc_bare "        claude --model sonnet"
  buh_e
  buh_line "     No --dangerously-skip-permissions: you grant each prompt and supply"
  buh_line "     credentials at the cloud gates."
  buh_e
  buh_line "  2. Paste the stranger prompt below (also saved, copy-pasteable, at"
  buh_line "     ${z_prompt_path}):"
  buh_e

  # Print the prompt verbatim — plain cat to stderr, no yelp formatter, so nothing in
  # the prompt body is reinterpreted as a display marker.
  printf '%s\n' "----------------------------------------------------------------------" >&2
  cat "${z_prompt_path}" >&2
  printf '%s\n' "----------------------------------------------------------------------" >&2

  buh_e
  buh_line "  When the walk is done, review ${z_memo_path##*/} in the rig, commit it into"
  buh_line "  Memos/ as memo-${z_walkdate}-${RBLM_harbinger_memo_slug}.md, then discard the rig."
  buh_e
  buc_success "Harbinger ready — guarded cold-walk clone stood up, launch line and stranger prompt printed"
}

# eof
