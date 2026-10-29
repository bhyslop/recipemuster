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
# TEMPLATE — copy to `tt/buw-SI.StationInit.sh` in your own repository, fill the
# one «...» substitution below, and delete these three lines.
#
# buw-SI.StationInit — write this machine's station regime file.
#
# A STANDALONE SCRIPT, and the one tabtarget that does not dispatch: every other
# door loads the station regime before it runs, so the door that CREATES that
# regime cannot be one of them. It sources the config regime and the field
# template and no kit module, and runs on a repository where nothing else works
# yet.
#
# The field list and the defaults are read from the bash utility kit's own
# `burs_template.sh`, so this script cannot drift from what the regime validator
# demands. The one field it names is the username, which it fills from the
# local account in place of the template's display placeholder.
#
# One station file serves every repository of an infield, so a file that
# already stands is left as it is. Pass --overwrite to replace it deliberately.

set -euo pipefail

zbpt_station_die() {
  echo "FATAL: ${1}" >&2 || exit 1
  exit 1
}

z_script_dir="${BASH_SOURCE[0]%/*}"

test "$#" -le 1 || zbpt_station_die "too many arguments (only --overwrite is taken)"
z_overwrite="${1:-}"
test -z "${z_overwrite}" || test "${z_overwrite}" = "--overwrite" \
  || zbpt_station_die "unknown argument: ${z_overwrite} (only --overwrite is taken)"

# Your repository's moorings directory — the same name your tt/z-launcher.sh
# carries.
z_moorings_dir="«your-moorings-dirname»"

test -f "${z_script_dir}/../${z_moorings_dir}/burc.env" \
  || zbpt_station_die "burc.env not found: ${z_script_dir}/../${z_moorings_dir}/burc.env"

source "${z_script_dir}/../${z_moorings_dir}/burc.env" \
  || zbpt_station_die "cannot source the config regime ${z_script_dir}/../${z_moorings_dir}/burc.env"
test -n "${BURC_STATION_FILE:-}" \
  || zbpt_station_die "BURC_STATION_FILE not set in ${z_script_dir}/../${z_moorings_dir}/burc.env"
test -n "${BURC_TOOLS_DIR:-}" \
  || zbpt_station_die "BURC_TOOLS_DIR not set in ${z_script_dir}/../${z_moorings_dir}/burc.env"

# The station path resolves from the REPOSITORY ROOT, not from burc.env.
z_project_root="${z_script_dir}/.."
z_station_path="${z_project_root}/${BURC_STATION_FILE}"

z_verb="created"
if test -f "${z_station_path}"; then
  z_verb="overwritten"
fi
if test "${z_verb}" = "overwritten" && test -z "${z_overwrite}"; then
  echo "Station regime left standing: ${z_station_path} (pass --overwrite to replace it)" \
    || zbpt_station_die "cannot report the standing station regime"
  exit 0
fi

mkdir -p "${z_station_path%/*}" || zbpt_station_die "cannot make the directory of ${z_station_path}"

source "${z_project_root}/${BURC_TOOLS_DIR}/buk/burs_template.sh" \
  || zbpt_station_die "cannot source the station field template ${z_project_root}/${BURC_TOOLS_DIR}/buk/burs_template.sh"

# Each value is written as bash's sourcing reads it back: a bare token where it
# is one, double quotes around one carrying anything else, and a refusal where
# it carries a character neither form holds. The whole file is composed before
# any of it is written, so a refusal leaves a standing station regime as it was.
z_newline=$'\n'
z_station_text=""
for z_i in "${!ZBURS_TEMPLATE_NAMES[@]}"; do
  z_name="${ZBURS_TEMPLATE_NAMES[${z_i}]}"
  z_value="${ZBURS_TEMPLATE_VALUES[${z_i}]}"
  if test "${z_name}" = "BURS_USER"; then
    z_value="${USER:-}"
    test -n "${z_value}" || zbpt_station_die "USER is not set, so the local username cannot be named"
  fi
  case "${z_value}" in
    *'"'*|*"'"*|*'\'*|*'$'*|*'`'*|*"${z_newline}"*)
      zbpt_station_die "${z_name} carries a quote, a backslash, a dollar, a backtick or a newline, which no station regime line can carry: ${z_value}"
      ;;
  esac
  z_line="${z_name}=${z_value}"
  case "${z_value}" in
    *[!ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_./:,@%+=-]*)
      z_line="${z_name}=\"${z_value}\""
      ;;
  esac
  z_station_text="${z_station_text}# ${ZBURS_TEMPLATE_COMMENTS[${z_i}]}${z_newline}${z_line}${z_newline}"
done

printf '%s' "${z_station_text}" > "${z_station_path}" || zbpt_station_die "cannot write ${z_station_path}"

echo "Station regime ${z_verb}: ${z_station_path}" \
  || zbpt_station_die "cannot report the station regime written"
