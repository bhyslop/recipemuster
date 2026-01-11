#!/bin/bash
#
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
# VVK Git Utilities - Git-based locking and pre-commit guard

set -euo pipefail

# Multiple inclusion detection
test -z "${ZVVG_SOURCED:-}" || buc_die "Module vvg multiply sourced - check sourcing hierarchy"
ZVVG_SOURCED=1

# Capture module directory at source time
ZVVG_MODULE_DIR="${BASH_SOURCE[0]%/*}"

######################################################################
# Internal Functions (zvvg_*)

zvvg_kindle() {
  test -z "${ZVVG_KINDLED:-}" || buc_die "Module vvg already kindled"

  # Validate we're in a git repository
  git rev-parse --git-dir >/dev/null 2>&1 || buc_die "Not in a git repository"

  # Internal constants
  ZVVG_LOCK_PREFIX="refs/vvg/locks"
  ZVVG_VVX_PATH="${ZVVG_MODULE_DIR}/bin/vvx"
  ZVVG_COMMIT_RESOURCE="commit"

  # Size limits (hardcoded for now)
  ZVVG_SIZE_LIMIT=500000
  ZVVG_WARN_LIMIT=250000

  ZVVG_KINDLED=1
}

zvvg_sentinel() {
  test "${ZVVG_KINDLED:-}" = "1" || buc_die "Module vvg not kindled - call zvvg_kindle first"
}

# Internal: Acquire a lock on a resource
# Returns 0 on success, 1 if lock already held
zvvg_lock_acquire() {
  zvvg_sentinel

  local z_resource="${1:-}"

  test -n "${z_resource}" || buc_die "zvvg_lock_acquire: resource name required"

  local z_ref="${ZVVG_LOCK_PREFIX}/${z_resource}"

  buc_log_args "Acquiring lock: ${z_resource}"

  # Get current HEAD as lock value
  local z_lock_value
  z_lock_value=$(git rev-parse HEAD 2>/dev/null) || z_lock_value="0000000000000000000000000000000000000000"

  # Try to create the ref - fails if it already exists
  # The empty string as third arg means "only create if ref doesn't exist"
  if git update-ref "${z_ref}" "${z_lock_value}" "" 2>/dev/null; then
    buc_log_args "Lock acquired: ${z_resource}"
    return 0
  else
    buc_log_args "Lock already held: ${z_resource}"
    return 1
  fi
}

# Internal: Release a lock on a resource
# Returns 0 on success, 1 if lock not held
zvvg_lock_release() {
  zvvg_sentinel

  local z_resource="${1:-}"

  test -n "${z_resource}" || buc_die "zvvg_lock_release: resource name required"

  local z_ref="${ZVVG_LOCK_PREFIX}/${z_resource}"

  buc_log_args "Releasing lock: ${z_resource}"

  if git update-ref -d "${z_ref}" 2>/dev/null; then
    buc_log_args "Lock released: ${z_resource}"
    return 0
  else
    buc_log_args "Lock not held: ${z_resource}"
    return 1
  fi
}

# Internal: Stage modified/deleted files
# Returns 0 on success with files staged, 1 on failure or nothing to stage
zvvg_stage() {
  zvvg_sentinel

  buc_log_args "Staging modified/deleted files"

  if ! git add -u 2>/dev/null; then
    buc_log_args "git add -u failed"
    return 1
  fi

  # Check if anything is staged
  if git diff --cached --quiet 2>/dev/null; then
    buc_log_args "Nothing staged to commit"
    return 1
  fi

  buc_log_args "Files staged successfully"
  return 0
}

# Internal: Check staged content size
# Returns 0 if ok, 1 if over limit, 2 if over warn threshold
zvvg_check_size() {
  zvvg_sentinel

  buc_log_args "Validating staged content size (limit: ${ZVVG_SIZE_LIMIT}, warn: ${ZVVG_WARN_LIMIT})"

  test -x "${ZVVG_VVX_PATH}" || buc_die "vvx not found at ${ZVVG_VVX_PATH}"

  local z_result=0
  "${ZVVG_VVX_PATH}" guard --limit "${ZVVG_SIZE_LIMIT}" --warn "${ZVVG_WARN_LIMIT}" || z_result=$?

  return "${z_result}"
}

# Internal: Unstage all staged files
zvvg_unstage() {
  zvvg_sentinel

  buc_log_args "Unstaging files"
  git reset HEAD --quiet 2>/dev/null || true
}

######################################################################
# External Functions (vvg_*)

vvg_guard_begin() {
  zvvg_sentinel

  buc_doc_brief "Begin guarded commit: acquire lock, stage files, validate size"
  buc_doc_lines "On success: lock held, files staged, ready for commit agent"
  buc_doc_lines "On failure: lock released, files unstaged"
  buc_doc_shown || return 0

  buc_step "Beginning guarded commit"

  # Step 1: Acquire commit lock
  if ! zvvg_lock_acquire "${ZVVG_COMMIT_RESOURCE}"; then
    buc_die "Another commit in progress - lock held"
  fi

  # Step 2: Stage modified/deleted files (captures snapshot NOW)
  # This is the race condition fix: staging happens before agent dispatch
  if ! zvvg_stage; then
    zvvg_lock_release "${ZVVG_COMMIT_RESOURCE}" || true
    buc_die "Nothing to commit or staging failed"
  fi

  # Step 3: Run size validation on staged content
  local z_size_result=0
  zvvg_check_size || z_size_result=$?

  if test "${z_size_result}" -eq 1; then
    # Over limit - unstage and release lock
    buc_warn "Staged content exceeds size limit"
    zvvg_unstage
    zvvg_lock_release "${ZVVG_COMMIT_RESOURCE}" || true
    buc_die "Commit blocked: staged content too large"
  fi

  if test "${z_size_result}" -eq 2; then
    buc_warn "Staged content near size limit - proceed with caution"
  fi

  buc_success "Guard passed - lock held, files staged, ready for commit"
  return 0
}

vvg_guard_end() {
  zvvg_sentinel

  buc_doc_brief "End guarded commit: release the commit lock"
  buc_doc_lines "Called by background agent after successful commit"
  buc_doc_shown || return 0

  buc_step "Ending guarded commit"

  if ! zvvg_lock_release "${ZVVG_COMMIT_RESOURCE}"; then
    buc_warn "Lock was not held - may have been force-unlocked"
  fi

  buc_success "Commit lock released"
  return 0
}

vvg_force_unlock() {
  zvvg_sentinel

  buc_doc_brief "Force unlock: break commit lock unconditionally"
  buc_doc_lines "For human recovery when commit process fails or hangs"
  buc_doc_shown || return 0

  buc_step "Force unlocking commit"

  local z_ref="${ZVVG_LOCK_PREFIX}/${ZVVG_COMMIT_RESOURCE}"

  # Force delete - ignore errors
  git update-ref -d "${z_ref}" 2>/dev/null || true

  buc_success "Commit lock cleared"
  return 0
}

# eof
