#!/bin/bash
# vvg_git.sh - VVK Git Utilities
#
# Git-based locking for multi-agent coordination.
# Uses git refs for atomic lock operations.
#
# Usage:
#   source Tools/vvk/vvg_git.sh
#   vvg_lock_acquire "resource" || die "Lock held"
#   # ... do work ...
#   vvg_lock_release "resource"
#
# Refs namespace: refs/vvg/locks/*

# Guard against multiple inclusion
[[ -n "${ZVVG_INCLUDED:-}" ]] && return 0
ZVVG_INCLUDED=1

# -----------------------------------------------------------------------------
# Configuration
# -----------------------------------------------------------------------------

VVG_LOCK_PREFIX="refs/vvg/locks"

# -----------------------------------------------------------------------------
# Lock Operations
# -----------------------------------------------------------------------------

# Acquire a lock on a resource
# Returns 0 on success, 1 if lock already held
vvg_lock_acquire() {
    local resource="$1"
    local ref="$VVG_LOCK_PREFIX/$resource"

    if [[ -z "$resource" ]]; then
        echo "vvg_lock_acquire: resource name required" >&2
        return 1
    fi

    # Create a dummy commit hash to store in the ref
    # Using the null hash check pattern
    local lock_value
    lock_value=$(git rev-parse HEAD 2>/dev/null || echo "0000000000000000000000000000000000000000")

    # Try to create the ref - fails if it already exists
    if git update-ref "$ref" "$lock_value" "" 2>/dev/null; then
        return 0
    else
        echo "vvg_lock_acquire: lock '$resource' already held" >&2
        return 1
    fi
}

# Release a lock on a resource
# Returns 0 on success, 1 if lock not held
vvg_lock_release() {
    local resource="$1"
    local ref="$VVG_LOCK_PREFIX/$resource"

    if [[ -z "$resource" ]]; then
        echo "vvg_lock_release: resource name required" >&2
        return 1
    fi

    # Delete the ref
    if git update-ref -d "$ref" 2>/dev/null; then
        return 0
    else
        echo "vvg_lock_release: lock '$resource' not held" >&2
        return 1
    fi
}

# Break a lock unconditionally (human recovery)
# Always returns 0
vvg_lock_break() {
    local resource="$1"
    local ref="$VVG_LOCK_PREFIX/$resource"

    if [[ -z "$resource" ]]; then
        echo "vvg_lock_break: resource name required" >&2
        return 1
    fi

    # Force delete - ignore errors
    git update-ref -d "$ref" 2>/dev/null || true
    echo "vvg_lock_break: lock '$resource' cleared"
    return 0
}

# Check if a lock is held
# Returns 0 if held, 1 if not held
vvg_lock_check() {
    local resource="$1"
    local ref="$VVG_LOCK_PREFIX/$resource"

    if [[ -z "$resource" ]]; then
        echo "vvg_lock_check: resource name required" >&2
        return 1
    fi

    if git show-ref --verify --quiet "$ref" 2>/dev/null; then
        return 0
    else
        return 1
    fi
}

# List all held locks
vvg_lock_list() {
    git for-each-ref --format='%(refname:short)' "$VVG_LOCK_PREFIX/" 2>/dev/null | \
        sed "s|^$VVG_LOCK_PREFIX/||"
}

# -----------------------------------------------------------------------------
# Guard function (pre-commit size check)
# -----------------------------------------------------------------------------

# Run pre-commit guard via vvr binary
# Returns exit code from vvr guard
vvg_guard() {
    local limit="${VVG_SIZE_LIMIT:-500000}"
    local warn="${VVG_WARN_LIMIT:-250000}"

    # Find vvx relative to this script
    local script_dir
    script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    local vvx="$script_dir/bin/vvx"

    if [[ ! -x "$vvx" ]]; then
        echo "vvg_guard: vvx not found at $vvx" >&2
        return 1
    fi

    "$vvx" guard --limit "$limit" --warn "$warn"
}
