#!/bin/bash
# z-launcher.sh — universal tabtarget trampoline.
#
# Every tt/*.sh dispatches through here:
#   exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <sprue> "${0##*/}" "${@}"
#
# Two responsibilities:
#   1. Normalize cwd to repo root so every workbench starts from a
#      deterministic directory regardless of where the user invoked the
#      tabtarget.
#   2. Resolve the moorings launcher named by <sprue> and exec it, forwarding
#      the tabtarget basename and user args unchanged. The downstream launcher
#      stub / bul_launch / bud_dispatch chain sees exactly the argument shape
#      it saw before this trampoline existed.
#
# Sprue contract: a tabtarget passes a minted moorings-launcher *sprue* as the
# first positional, never a bare workbench-id (a bare id would conflate the
# hyphenated colophon universe with launcher dispatch). Form
# {owner}ml_{launcher-id}. The owner prefix is ownership-semantic
# (rbml_ = RBK-authored, buml_ = BUK-hosted infrastructure), NOT a location
# selector — every launcher co-locates, so the launcher-id is recovered by
# stripping the *ml_ prefix (${1#*ml_}) and dispatch stays a single literal.
#
#   Valid sprue   launcher-id        Valid sprue   launcher-id
#   -----------   -----------        -----------   -----------
#   rbml_rbw      rbw                buml_cmw      cmw
#   rbml_rbtw     rbtw               buml_vow      vow
#   buml_buw      buw                buml_vvw      vvw
#   buml_jjw      jjw                buml_vslw     vslw
#   buml_apcw     apcw               buml_study    study
#
# No-log behavior is NOT a launcher selection: it rides the BURD_NO_LOG env
# var the tabtarget exports ahead of dispatch (bul_launcher skips the BURS
# station load under it). The former separate nolog launcher was collapsed.
#
# The moorings-launchers path is hardcoded as a literal below; the *ml_ strip
# survives unchanged. CLAUDE.md registration of the sprue universe is a later
# pace.

set -u

# Resolve own directory to an absolute path before any chdir.
z_dir="${BASH_SOURCE[0]%/*}"
case "${z_dir}" in
  /*) ;;
  *)  z_dir="${PWD}/${z_dir}" ;;
esac

z_sprue="${1:-}"
test -n "${z_sprue}" || { echo "z-launcher: no sprue given" >&2; exit 1; }

# Project-intimate config-dir anchor. z-launcher is the SOLE file that knows
# where THIS project keeps its moorings/config dir (.buk, rbmm_moorings, …).
# The shared kit (bul_launcher, bubc) consumes BURD_CONFIG_DIR exported below
# rather than hardcoding a name — so one kit serves every consumer.
z_moorings_dir="rbmm_moorings"

# Recover the launcher-id by stripping the *ml_ ownership prefix.
z_launcher_id="${z_sprue#*ml_}"
z_launcher="${z_dir}/../${z_moorings_dir}/rbml_launchers/launcher.${z_launcher_id}_workbench.sh"

# Fail loud on a mistyped sprue rather than dispatching silently to nothing.
test -f "${z_launcher}" || {
  echo "z-launcher: no launcher for sprue '${z_sprue}' (looked for ${z_launcher})" >&2
  exit 1
}

# Normalize cwd to repo root for the dispatched workbench.
cd -P "${z_dir}/.." || { echo "z-launcher: cannot cd to repo root" >&2; exit 1; }

# Hand the config-dir location to the shared launcher (absolute, cd-proof).
export BURD_CONFIG_DIR="${PWD}/${z_moorings_dir}"

# Preserve the BURD_LAUNCHER regime contract (required by burd_regime,
# consumed by every post-dispatch zburd_enforce). Repo-relative form matches
# the value the tabtarget exported before the trampoline.
export BURD_LAUNCHER="${z_moorings_dir}/rbml_launchers/launcher.${z_launcher_id}_workbench.sh"

# Forward everything after the sprue: tabtarget basename + user args.
exec "${z_launcher}" "${@:2}"
