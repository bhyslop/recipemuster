#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# a consumer holding a regulus has no delivery to shake down. It is withheld by the
# perambulation: the module by an explicit row, the tabtarget by the tt/rbw-M stem.
# The delivered rblm_cli.sh's furnish sources this module only when the command is
# rblm_harbinger, so a stripped tree that never carries it dies naming what is
# missing rather than dangling on an absent function.
#
# HARBINGER PUSHES NOTHING, EVER. It clones over HTTPS (anonymous read of the public
# repo), then severs the clone's origin and reads the remotes back to none, pins the
# clone's hooks path to its own hooks directory, redirects every push to a public host
# to a destination that cannot stand, and installs a pre-push hook that refuses every
# push — because THIS station holds real credentials to the public repository and a
# stray push from the walk clone is the one act the rig must make structurally
# impossible. The walk that follows is the operator's own, launched by hand from the
# printed line; harbinger sets the stage and steps off it.

set -euo pipefail

test -z "${ZRBLM_HARBINGER_SOURCED:-}" || buc_die_now "Module rblm_harbinger multiply sourced - check sourcing hierarchy"
ZRBLM_HARBINGER_SOURCED=1

######################################################################
# Harbinger constants

# The rig directory's stem; the walk date completes its basename, so each day's walk
# stands its own rig and a second run on the same day seposes that day's — a rename to
# a timestamped sibling, never a delete, so even a wrong guard could not destroy data.
readonly RBLM_harbinger_dirname="rbthdr_coldwalk"

# The directory every rig stands in, beside the infield holding the maintainer tree
# rather than inside it, so a cold session launched in the clone loads no context file
# of that infield. Derived from git (the repo root's grandparent), never hand-typed.
readonly RBLM_harbinger_infield="infield-coldwalk"

# The hooks path the clone is pinned to, relative so git resolves it against the
# clone's own root wherever a seposition moves the rig.
readonly RBLM_harbinger_hooks_path=".git/hooks"

# Where every push to a public host is redirected: a path beneath /dev/null, which can
# never stand as a repository, spelled so git's own error states the refusal.
readonly RBLM_harbinger_refused_destination="/dev/null/coldwalk-clone-pushes-are-refused"

# Every public-host prefix a push is redirected from. The org prefixes are longer than
# any host-wide rewrite a station's global config may carry for its own pushes, so the
# clone's rule wins git's longest-match.
readonly RBLM_harbinger_public_prefixes=(
  "https://github.com/"
  "git@github.com:"
  "ssh://git@github.com/"
  "https://github.com/scaleinv/"
  "git@github.com:scaleinv/"
  "ssh://git@github.com/scaleinv/"
)

# The clone lives one level down, in its own subdirectory. The findings memo is a
# sibling of it under the parent dir, so discarding the clone subdirectory leaves the
# memo standing for the operator to review and commit.
readonly RBLM_harbinger_clone_subdir="recipebottle"

# The promoted public repository — HTTPS, anonymous read. This is the exact face a
# real stranger clones: the default branch of the public repo after promotion. Named
# in full here because harbinger stays behind (withheld) and the walk must land on the
# genuine public tree, not a maintainer remote alias.
readonly RBLM_harbinger_public_url="https://github.com/scaleinv/recipebottle.git"

# The pristine reference the walk diffs against. The clone's own default branch is left
# untouched as the reference; the walker commits its kludges on a throwaway branch, so
# a diff at the end shows exactly what the walk changed.
readonly RBLM_harbinger_walk_branch="coldwalk"

# The findings-memo basename slug. The full basename is dated at run time:
# memo-<YYYYMMDD>-coldwalk-shakedown.md — the maintainer reviews it and commits it into
# the studbook (jjqs_studbook/), affiliated to the pace.
readonly RBLM_harbinger_memo_slug="coldwalk-shakedown"

######################################################################
# Command: harbinger - stand up the guarded cold-walk clone and hand off
#
# The verb is a pure local constructor. It disposes of any prior rig by RENAME —
# seposing it to a timestamped sibling, never deleting — and reaches the public
# repository only for an anonymous read. It ends by printing the operator's launch
# line and the stranger prompt; the walk itself is the operator's, not harbinger's.
rblm_harbinger() {
  buc_doc_brief "Harbinger the cold-agent onboarding shakedown - stand up a guarded disposable clone of promoted public main and print the launch line and stranger prompt"
  buc_doc_shown || return 0

  mkdir -p "${BURD_TEMP_DIR}" || buc_die_now "Failed to create temp directory"
  local -r z_temp_prefix="${BURD_TEMP_DIR}/rblm_harbinger_"

  # Where we are. The rig's location derives from this tree's root, so its identity is
  # the anchor for every derived path below — and the guard that the seposition move
  # can never hit it.
  local -r z_toplevel_temp="${z_temp_prefix}toplevel.txt"
  git rev-parse --show-toplevel > "${z_toplevel_temp}" || buc_die_now "git rev-parse --show-toplevel failed — harbinger must run inside a git repository"
  local z_toplevel
  z_toplevel=$(<"${z_toplevel_temp}") || buc_die_now "Failed to read: ${z_toplevel_temp}"
  test -n "${z_toplevel}" || buc_die_now "git returned an empty repository root"
  case "${z_toplevel}" in
    /*) ;;
    *)  buc_die_now "Repository root is not an absolute path: ${z_toplevel}" ;;
  esac

  local -r z_infield="${z_toplevel%/*}"
  test -n "${z_infield}" || buc_die_now "Repository root has no parent directory: ${z_toplevel}"
  local -r z_estate="${z_infield%/*}"
  test -n "${z_estate}" || buc_die_now "Repository root's parent has no parent directory: ${z_infield}"
  case "${z_estate}" in
    /*) ;;
    *)  buc_die_now "Repository root's grandparent is not an absolute path: '${z_estate}' (from ${z_toplevel})" ;;
  esac

  # The walk date, which completes the rig's basename and dates the findings memo.
  local -r z_walkdate_temp="${z_temp_prefix}walkdate.txt"
  date +%Y%m%d > "${z_walkdate_temp}" || buc_die_now "date failed"
  local z_walkdate
  z_walkdate=$(<"${z_walkdate_temp}") || buc_die_now "Failed to read: ${z_walkdate_temp}"
  test -n "${z_walkdate}" || buc_die_now "walk date resolved empty"

  local -r z_target_dir="${z_estate}/${RBLM_harbinger_infield}/${RBLM_harbinger_dirname}_${z_walkdate}"

  # Rig-move guard. Harbinger moves the target aside wholesale, so it must never be the
  # maintainer tree; that is asserted before anything moves. The disposal is a RENAME,
  # not a delete — the prior rig is seposed to a timestamped sibling, recoverable — so
  # nothing here can destroy data even if a guard were wrong.
  test "${z_target_dir}" != "${z_toplevel}" \
    || buc_die_now "Refusing to move '${z_target_dir}' — it is this repository's own root"

  local -r z_clone_dir="${z_target_dir}/${RBLM_harbinger_clone_subdir}"

  # The dated findings-memo path — absolute, and a sibling of the clone so it survives
  # the clone's discard. The prompt below hands this exact path to the walker; the
  # maintainer reviews it and commits it into the studbook after the walk.
  local -r z_memo_path="${z_target_dir}/memo-${z_walkdate}-${RBLM_harbinger_memo_slug}.md"

  # The seposed sibling for any prior rig — a second-grained stamp so two runs in
  # one day cannot collide. An existing rig is renamed here, never deleted.
  local -r z_sepose_stamp_temp="${z_temp_prefix}sepose_stamp.txt"
  date +%Y%m%d-%H%M%S > "${z_sepose_stamp_temp}" || buc_die_now "date failed"
  local z_sepose_stamp
  z_sepose_stamp=$(<"${z_sepose_stamp_temp}") || buc_die_now "Failed to read: ${z_sepose_stamp_temp}"
  test -n "${z_sepose_stamp}" || buc_die_now "seposition timestamp resolved empty"
  local -r z_seposed_dir="${z_target_dir}.seposed-${z_sepose_stamp}"

  buh_section "Marshal Harbinger — cold-agent onboarding shakedown"
  buh_line "  Maintainer tree:   ${z_toplevel}"
  buh_line "  Cold-walk rig:     ${z_target_dir}  (prior seposed, fresh clone)"
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
  buh_line "  A same-day prior rig is SEPOSED: renamed to a timestamped sibling, never deleted;"
  buh_line "  sweep old ${RBLM_harbinger_dirname}_* rigs and their .seposed-* siblings whenever you like."
  buh_e
  buc_require "Sepose any existing rig and stand up a fresh cold-walk clone at ${z_target_dir}?" "harbinger"

  buc_step "Seposing any existing rig"
  if test -e "${z_target_dir}"; then
    test ! -e "${z_seposed_dir}" || buc_die_now "Seposition target already exists: ${z_seposed_dir}"
    mv "${z_target_dir}" "${z_seposed_dir}" || buc_die_now "Failed to sepose the existing rig: ${z_target_dir} -> ${z_seposed_dir}"
    buh_line "  Seposed prior rig: ${z_seposed_dir}"
  else
    buh_line "  No prior rig to sepose"
  fi
  mkdir -p "${z_target_dir}" || buc_die_now "Failed to create the rig: ${z_target_dir}"

  buc_step "Cloning the promoted public repository"
  git clone "${RBLM_harbinger_public_url}" "${z_clone_dir}" || buc_die_now "Failed to clone the public repository — is ${RBLM_harbinger_public_url} reachable and public?"

  # The clone must carry the promoted content, not an empty or wrong tree. The README
  # is the public face and the stranger's first read; its absence means the clone is
  # not what the walk expects, and the walk would start on nothing.
  test -f "${z_clone_dir}/README.md" \
    || buc_die_now "The clone carries no README.md — the promoted public tree is empty or wrong; refusing to hand the walker a tree with no onboarding face"

  # Sever the origin. From here the clone names no remote, so nothing it does can
  # reach the public repository. The materialization is already complete, so the
  # sever costs the walk nothing.
  buc_step "Severing the clone from its origin"
  git -C "${z_clone_dir}" remote remove origin || buc_die_now "Failed to sever the clone's origin"

  # Pin the hooks path to the clone's own, so a global config relocating hooks cannot
  # leave the refusal hook standing where git never reads it. A relative pin resolves
  # against the clone's root only where .git is a directory, as a clone's is.
  buc_step "Pinning the clone's hooks path to its own hooks directory"
  test -d "${z_clone_dir}/.git" \
    || buc_die_now "The clone stands no .git directory, so a relative hooks path would reach nothing: ${z_clone_dir}"
  git -C "${z_clone_dir}" config --local core.hooksPath "${RBLM_harbinger_hooks_path}" \
    || buc_die_now "Failed to pin the clone's hooks path"

  # Redirect every public-host push to a destination that cannot stand. A push to an
  # explicit URL needs no remote and --no-verify skips the hook; this rule still answers
  # it, the station's credentials notwithstanding.
  buc_step "Redirecting every public-host push to ${RBLM_harbinger_refused_destination}"
  local z_i=0
  for z_i in "${!RBLM_harbinger_public_prefixes[@]}"; do
    git -C "${z_clone_dir}" config --local --add "url.${RBLM_harbinger_refused_destination}.pushInsteadOf" "${RBLM_harbinger_public_prefixes[${z_i}]}" \
      || buc_die_now "Failed to redirect pushes from ${RBLM_harbinger_public_prefixes[${z_i}]}"
  done

  # Read the remotes back rather than trusting the sever's exit: a remote a station's
  # global config defines survives the removal of origin.
  local -r z_remotes_temp="${z_temp_prefix}remotes.txt"
  git -C "${z_clone_dir}" remote > "${z_remotes_temp}" || buc_die_now "Failed to list the clone's remotes"
  local z_remotes
  z_remotes=$(<"${z_remotes_temp}") || buc_die_now "Failed to read: ${z_remotes_temp}"
  test -z "${z_remotes}" || buc_die_now "The severed clone still names remote(s): ${z_remotes}"

  # Install the pre-push refusal. Belt to the sever's suspenders: even if the walk (or
  # a helpful agent) re-adds a remote, every push dies here. Written with a loud reason
  # so the refusal is legible, not a silent non-zero. printf (a builtin) writes the hook
  # line by line — BCG bars multi-line heredocs. Whatever stands at the hook path is
  # removed first, so the write never follows a symlink.
  buc_step "Installing the pre-push refusal hook"
  local -r z_hook="${z_clone_dir}/.git/hooks/pre-push"
  rm -f -- "${z_hook}" || buc_die_now "Failed to clear the pre-push hook path: ${z_hook}"
  printf '%s\n' \
    '#!/bin/sh' \
    'echo "coldwalk clone: pushing is disabled — this is a throwaway shakedown rig and must never push" >&2' \
    'exit 1' \
    > "${z_hook}" \
    || buc_die_now "Failed to write the pre-push hook: ${z_hook}"
  chmod +x "${z_hook}" || buc_die_now "Failed to make the pre-push hook executable"

  # Cut the throwaway walk branch. The consumer flow commits as it kludges; the walker
  # works here, and the clone's own default branch stays untouched as the pristine
  # reference to diff the walk against.
  buc_step "Cutting the throwaway walk branch ${RBLM_harbinger_walk_branch}"
  git -C "${z_clone_dir}" checkout -b "${RBLM_harbinger_walk_branch}" || buc_die_now "Failed to cut the walk branch"

  # The clone's HEAD, shown so the operator can match it to the promoted commit.
  local -r z_head_temp="${z_temp_prefix}head.txt"
  git -C "${z_clone_dir}" rev-parse HEAD > "${z_head_temp}" || buc_die_now "Failed to read the clone's HEAD"
  local z_head
  z_head=$(<"${z_head_temp}") || buc_die_now "Failed to read: ${z_head_temp}"
  test -n "${z_head}" || buc_die_now "The clone's HEAD resolved empty"

  # The launch line's path in POSIX single-quote form, an embedded quote spelled '\''
  # — the operator pastes the line into zsh or bash, and both read it so.
  local z_rest="${z_clone_dir}"
  local z_launch_dir=""
  while :; do
    case "${z_rest}" in
      *\'*)
        z_launch_dir="${z_launch_dir}${z_rest%%\'*}'\\''"
        z_rest="${z_rest#*\'}"
        ;;
      *)
        z_launch_dir="${z_launch_dir}${z_rest}"
        break
        ;;
    esac
  done
  z_launch_dir="'${z_launch_dir}'"

  buh_e
  buh_line "  Clone HEAD: ${z_head} — match it to the promoted commit before the walk."
  buh_e
  buh_line "  Rig ready. Two steps, by your hand:"
  buh_e
  buh_line "  1. Launch a cold session in the clone (NEW terminal):"
  buh_e
  buc_bare "        (cd ${z_launch_dir} && claude --model sonnet --permission-mode auto)"
  buh_e
  buh_line "  2. Paste the stranger prompt below:"
  buh_e

  # Emit the stranger prompt verbatim — printf (a builtin) straight to stderr, no yelp
  # formatter (so nothing in the body is reinterpreted as a display marker) and no file
  # left in the rig. Lines are literal single-quoted args; only the memo-path line is
  # double-quoted so it expands, so the walker knows where to log its findings. BCG bars
  # multi-line heredocs — printf line-by-line is the compliant emit.
  printf '%s\n' \
    '----------------------------------------------------------------------' \
    'You are trying Recipe Bottle for the first time. You cloned it because you want to' \
    'use it to build and run hardened, provenance-checked container images, and you are' \
    'starting completely cold: everything you know about this project must come from the' \
    'documentation in this repository. Do not reach for outside knowledge of how it' \
    '"really" works — if a step is unclear from the docs in front of you, that unclearness' \
    'is exactly what you are here to find.' \
    '' \
    'Your operator is a human at the keyboard with you. You drive; they grant permission' \
    'prompts, and they own everything involving money, accounts, and credentials.' \
    '' \
    'Walk the onboarding in four beats, in order:' \
    '' \
    '  1. Local setup. Read the README, find the onboarding entry point, and follow it to a' \
    '     first working local image build and a charged crucible you can shell into.' \
    '' \
    '  2. Cloud setup. The docs will reach a point that needs a Google Cloud account, a' \
    '     billing-backed project, and an identity provider. This is your operator'\''s block,' \
    '     not yours. Read the relevant guide, explain each step, and hand the keyboard to' \
    '     your operator for every account, billing, and credential action. STOP and hand off' \
    '     the moment the docs reach creating accounts or paid infrastructure.' \
    '' \
    '  3. Adversarial suite. Once cloud setup is done, charge the tadmor crucible and run its' \
    '     adversarial security suite, confirming the containment holds under attack.' \
    '' \
    '  4. Airgap build. Finally, walk the moriah airgap cloud build to its provenance' \
    '     comparison.' \
    '' \
    'Hard boundaries, always:' \
    '  - Create no accounts and spend no money on your own. Those are your operator'\''s, at the' \
    '    gates above.' \
    '  - Push nowhere. This clone is a throwaway; treat every remote as off-limits.' \
    '  - Never stop, quench, or delete any workload, container, or cloud resource you did not' \
    '    start yourself.' \
    '' \
    'Keep a findings log the whole way. Every time a doc is unclear, a command fails, a step' \
    'assumes knowledge you do not have, or you have to guess — write it down, with the exact' \
    'command and what you expected versus what happened. Put the log here, and update it as' \
    'you go:' \
    '' \
    "  ${z_memo_path}" \
    '' \
    'Begin by reading the README, then find the onboarding entry point and take the first' \
    'beat.' \
    '----------------------------------------------------------------------' \
    >&2 || buc_die_now "Failed to emit the stranger prompt"

  buh_e
  buh_line "  When the walk is done, review ${z_memo_path##*/} in the rig, commit it into"
  buh_line "  the studbook (jjqs_studbook/) as memo-${z_walkdate}-${RBLM_harbinger_memo_slug}.md, then discard the rig."
  buh_e
  buc_success "Harbinger ready — guarded cold-walk clone stood up, launch line and stranger prompt printed"
}

# eof
