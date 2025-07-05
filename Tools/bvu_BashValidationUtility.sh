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
# Bash Validation Utility Library
# Compatible with Bash 3.2 (e.g., macOS default shell)

# Global context variable
ZBVU_CONTEXT=""

# Set validation context for error messages
bvu_context() {
    ZBVU_CONTEXT="$1"
}

# Internal error handler
zbvu_die() {
    echo "ERROR: [$ZBVU_CONTEXT] $*" >&2
    exit 1
}

# Validate and return a string
bvu_string() {
    local value="$1"
    local min="${2:-1}"
    local max="${3:-255}"
    
    # Check not empty
    test -n "$value" || zbvu_die "string must not be empty"
    
    # Check length constraints
    local len=${#value}
    test $len -ge $min || zbvu_die "string must be at least $min chars, got '$value' ($len)"
    test $len -le $max || zbvu_die "string must be no more than $max chars, got '$value' ($len)"
    
    echo "$value"
}

# Validate and return a cross-context name (system-safe identifier)
bvu_xname() {
    local value="$1"
    local min="${2:-2}"
    local max="${3:-64}"
    
    # Check not empty
    test -n "$value" || zbvu_die "xname must not be empty"
    
    # Check length constraints
    local len=${#value}
    test $len -ge $min || zbvu_die "xname must be at least $min chars, got '$value' ($len)"
    test $len -le $max || zbvu_die "xname must be no more than $max chars, got '$value' ($len)"
    
    # Must start with letter and contain only allowed chars
    echo "$value" | grep -qE '^[a-zA-Z][a-zA-Z0-9_-]*$' || \
        zbvu_die "xname must start with letter and contain only letters, numbers, underscore, hyphen, got '$value'"
    
    echo "$value"
}

# Validate and return a Fully Qualified Image Name
bvu_fqin() {
    local value="$1"
    local min="${2:-1}"
    local max="${3:-512}"
    
    # Check not empty
    test -n "$value" || zbvu_die "fqin must not be empty"
    
    # Check length constraints
    local len=${#value}
    test $len -ge $min || zbvu_die "fqin must be at least $min chars, got '$value' ($len)"
    test $len -le $max || zbvu_die "fqin must be no more than $max chars, got '$value' ($len)"
    
    # Allow letters, numbers, dots, hyphens, underscores, forward slashes, colons
    echo "$value" | grep -qE '^[a-zA-Z0-9][a-zA-Z0-9:._/-]*$' || \
        zbvu_die "fqin must start with letter/number and contain only letters, numbers, colons, dots, underscores, hyphens, forward slashes, got '$value'"
    
    echo "$value"
}

# eof

