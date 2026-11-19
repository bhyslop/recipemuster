#!/bin/bash
#
# buw-SI.StationInit — Create default station regime file
#
# Standalone script (no launcher/dispatch) — BUD requires the station
# regime to exist, so this cannot go through dispatch.
#
# Reads BURC_STATION_FILE from rbmm_moorings/burc.env, creates the directory,
# writes a complete burs.env from the shared template (Tools/buk/burs_template.sh)
# — every required BURS field, each preceded by its purpose comment.
#
# Idempotent: overwrites any existing station file.

set -euo pipefail

z_script_dir="${BASH_SOURCE[0]%/*}"
z_burc="${z_script_dir}/../rbmm_moorings/burc.env"

test -f "${z_burc}" || { echo "FATAL: rbmm_moorings/burc.env not found: ${z_burc}" >&2; exit 1; }

z_station_file="$(grep '^BURC_STATION_FILE=' "${z_burc}" | cut -d= -f2)"
test -n "${z_station_file}" || { echo "FATAL: BURC_STATION_FILE not set in ${z_burc}" >&2; exit 1; }

# Resolve relative to project root (rbmm_moorings parent)
z_project_root="${z_script_dir}/.."
z_station_path="${z_project_root}/${z_station_file}"

mkdir -p "$(dirname "${z_station_path}")"

# shellcheck source=../Tools/buk/burs_template.sh
source "${z_script_dir}/../Tools/buk/burs_template.sh"

: > "${z_station_path}"
for z_i in "${!ZBURS_TEMPLATE_NAMES[@]}"; do
  z_name="${ZBURS_TEMPLATE_NAMES[${z_i}]}"
  z_value="${ZBURS_TEMPLATE_VALUES[${z_i}]}"
  # BURS_USER gets the real local username, not the display placeholder.
  # Subshell $(whoami) permitted: BUK environment not available in bootstrap tabtarget.
  test "${z_name}" = "BURS_USER" && z_value="$(whoami)"
  printf '# %s\n' "${ZBURS_TEMPLATE_COMMENTS[${z_i}]}" >> "${z_station_path}"
  printf '%s=%s\n' "${z_name}" "${z_value}" >> "${z_station_path}"
done

echo "Station regime created: ${z_station_path}"
