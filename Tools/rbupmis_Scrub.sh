#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
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

# Scrub output from `podman machine init` for comparison

# In some environments there are null characters, and the hint
#    at the end is a function of the podman machine name: bad
#    for assuring vm environment is what we expect.

set -euo pipefail

outfile="$1"
match="To start your machine run:"

# Ensure output file is empty
> "$outfile"

while IFS= read -r line || [[ -n "$line" ]]; do
  # Strip nulls from line
  clean_line="${line//$'\x00'/}"

  # Stop writing at match
  [[ "$clean_line" == "$match" ]] && break

  echo "$clean_line" >> "$outfile"
done

# eof
