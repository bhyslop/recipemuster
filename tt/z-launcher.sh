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
# The moorings-launchers path is hardcoded as a literal below (transitional —
# a later pace flips it to the post-rename layout; the *ml_ strip survives
# that flip unchanged). CLAUDE.md registration of the sprue universe is a
# later pace.

set -u

# Resolve own directory to an absolute path before any chdir.
z_dir="${BASH_SOURCE[0]%/*}"
case "${z_dir}" in
  /*) ;;
  *)  z_dir="${PWD}/${z_dir}" ;;
esac

z_sprue="${1:-}"
test -n "${z_sprue}" || { echo "z-launcher: no sprue given" >&2; exit 1; }

# Recover the launcher-id by stripping the *ml_ ownership prefix.
z_launcher_id="${z_sprue#*ml_}"
z_launcher="${z_dir}/../.buk/launcher.${z_launcher_id}_workbench.sh"

# Fail loud on a mistyped sprue rather than dispatching silently to nothing.
test -f "${z_launcher}" || {
  echo "z-launcher: no launcher for sprue '${z_sprue}' (looked for ${z_launcher})" >&2
  exit 1
}

# Normalize cwd to repo root for the dispatched workbench.
cd -P "${z_dir}/.." || { echo "z-launcher: cannot cd to repo root" >&2; exit 1; }

# Preserve the BURD_LAUNCHER regime contract (required by burd_regime,
# consumed by every post-dispatch zburd_enforce). Repo-relative form matches
# the value the tabtarget exported before the trampoline.
export BURD_LAUNCHER=".buk/launcher.${z_launcher_id}_workbench.sh"

# Forward everything after the sprue: tabtarget basename + user args.
exec "${z_launcher}" "${@:2}"
