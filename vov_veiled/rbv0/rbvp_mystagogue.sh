#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBVP - the browser profile Recipe Bottle's windowed walks open on, veiled.
# The station holds the name in the browser's own list of its profiles, typed
# there by a person at setup; this file, named for it, and its one constant are
# its second seat, and no other code spells it.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBVP_SOURCED:-}" || buc_die_now "Module rbvp multiply sourced - check sourcing hierarchy"
ZRBVP_SOURCED=1

# The profile's name, exactly as the station's browser lists it
readonly RBVP_profile="rbvp_mystagogue"

# eof
