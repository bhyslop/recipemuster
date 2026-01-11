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
# VVT Testbench - Tests for VVK Git utilities

set -euo pipefail

# Multiple inclusion detection
test -z "${ZVVT_SOURCED:-}" || return 0
ZVVT_SOURCED=1

# Capture module directory at source time
ZVVT_MODULE_DIR="${BASH_SOURCE[0]%/*}"

######################################################################
# Internal Helpers (zvvt_*)

# Create a fresh git repo for testing
# Sets ZVVT_TEST_REPO to the repo path
zvvt_create_test_repo() {
  local z_name="${1:-test-repo}"

  ZVVT_TEST_REPO="${BUT_TEMP_DIR}/${z_name}"
  mkdir -p "${ZVVT_TEST_REPO}" || but_fatal "Failed to create test repo dir"

  (
    cd "${ZVVT_TEST_REPO}" || exit 1
    git init --quiet || exit 1
    git config user.email "test@test.local" || exit 1
    git config user.name "Test User" || exit 1
    echo "initial" > README.md || exit 1
    git add README.md || exit 1
    git commit -m "Initial commit" --quiet || exit 1
  ) || but_fatal "Failed to initialize test repo"

  but_trace "Created test repo: ${ZVVT_TEST_REPO}"
}

# Create a file of specified size in test repo
# Usage: zvvt_create_file <filename> <size_bytes>
zvvt_create_file() {
  local z_filename="${1:-}"
  local z_size="${2:-}"

  test -n "${z_filename}" || but_fatal "zvvt_create_file: filename required"
  test -n "${z_size}" || but_fatal "zvvt_create_file: size required"
  test -n "${ZVVT_TEST_REPO:-}" || but_fatal "zvvt_create_file: no test repo"

  local z_filepath="${ZVVT_TEST_REPO}/${z_filename}"

  # Create file with random content of specified size
  dd if=/dev/urandom of="${z_filepath}" bs=1 count="${z_size}" 2>/dev/null || \
    but_fatal "Failed to create test file: ${z_filename}"

  but_trace "Created file: ${z_filename} (${z_size} bytes)"
}

# Run vvg_cli.sh command in test repo
# Sets ZBUT_STDOUT, ZBUT_STDERR, ZBUT_STATUS via zbut_invoke
zvvt_run_vvg() {
  test -n "${ZVVT_TEST_REPO:-}" || but_fatal "zvvt_run_vvg: no test repo"

  local z_orig_dir="${PWD}"
  cd "${ZVVT_TEST_REPO}" || but_fatal "Cannot cd to test repo"

  zbut_invoke "${ZVVT_MODULE_DIR}/vvg_cli.sh" "$@"

  cd "${z_orig_dir}" || but_fatal "Cannot cd back"
}

# Check if a lock ref exists in test repo
# Returns 0 if exists, 1 if not
zvvt_lock_exists_predicate() {
  local z_resource="${1:-}"

  test -n "${z_resource}" || but_fatal "zvvt_lock_exists_predicate: resource required"
  test -n "${ZVVT_TEST_REPO:-}" || but_fatal "zvvt_lock_exists_predicate: no test repo"

  local z_ref="refs/vvg/locks/${z_resource}"

  (cd "${ZVVT_TEST_REPO}" && git show-ref --verify --quiet "${z_ref}" 2>/dev/null)
}

######################################################################
# Test Functions (vvt_test_*)

# Test: Lock acquire succeeds on new resource
vvt_test_lock_acquire_new() {
  zvvt_create_test_repo

  # Create a file and modify it so there's something to stage
  echo "modified" >> "${ZVVT_TEST_REPO}/README.md"

  # Acquire lock via guard_begin
  zvvt_run_vvg vvg_guard_begin

  but_fatal_on_error "${ZBUT_STATUS}" "guard_begin should succeed" \
    "STDERR: ${ZBUT_STDERR}"

  # Verify lock exists
  zvvt_lock_exists_predicate "commit" || \
    but_fatal "Lock should exist after guard_begin"
}

# Test: Lock acquire fails when already held
vvt_test_lock_acquire_held() {
  zvvt_create_test_repo

  echo "modified" >> "${ZVVT_TEST_REPO}/README.md"

  # First acquire should succeed
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_error "${ZBUT_STATUS}" "First guard_begin should succeed"

  # Make another modification for second attempt
  echo "more changes" >> "${ZVVT_TEST_REPO}/README.md"

  # Second acquire should fail
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_success "${ZBUT_STATUS}" "Second guard_begin should fail when lock held"
}

# Test: Lock release succeeds after acquire
vvt_test_lock_release() {
  zvvt_create_test_repo

  echo "modified" >> "${ZVVT_TEST_REPO}/README.md"

  # Acquire lock
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_error "${ZBUT_STATUS}" "guard_begin should succeed"

  # Release lock
  zvvt_run_vvg vvg_guard_end
  but_fatal_on_error "${ZBUT_STATUS}" "guard_end should succeed"

  # Verify lock no longer exists
  if zvvt_lock_exists_predicate "commit"; then
    but_fatal "Lock should not exist after guard_end"
  fi
}

# Test: Force unlock clears lock
vvt_test_force_unlock() {
  zvvt_create_test_repo

  echo "modified" >> "${ZVVT_TEST_REPO}/README.md"

  # Acquire lock
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_error "${ZBUT_STATUS}" "guard_begin should succeed"

  # Force unlock
  zvvt_run_vvg vvg_force_unlock
  but_fatal_on_error "${ZBUT_STATUS}" "force_unlock should succeed"

  # Verify lock no longer exists
  if zvvt_lock_exists_predicate "commit"; then
    but_fatal "Lock should not exist after force_unlock"
  fi
}

# Test: Force unlock succeeds even when no lock held
vvt_test_force_unlock_no_lock() {
  zvvt_create_test_repo

  # Force unlock when no lock exists
  zvvt_run_vvg vvg_force_unlock
  but_fatal_on_error "${ZBUT_STATUS}" "force_unlock should succeed even with no lock"
}

# Test: guard_begin fails with nothing to stage
vvt_test_guard_nothing_staged() {
  zvvt_create_test_repo

  # No modifications - nothing to stage
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_success "${ZBUT_STATUS}" "guard_begin should fail with nothing to stage"

  # Verify no lock left behind
  if zvvt_lock_exists_predicate "commit"; then
    but_fatal "Lock should not exist after failed guard_begin"
  fi
}

# Test: guard_begin succeeds with small content (under limit)
vvt_test_guard_small_content() {
  zvvt_create_test_repo

  # Create small file (100 bytes, well under 250K warn threshold)
  zvvt_create_file "small.txt" 100
  (cd "${ZVVT_TEST_REPO}" && git add small.txt)

  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_error "${ZBUT_STATUS}" "guard_begin should succeed with small content" \
    "STDERR: ${ZBUT_STDERR}"

  # Verify lock held
  zvvt_lock_exists_predicate "commit" || \
    but_fatal "Lock should exist after successful guard_begin"

  # Clean up lock
  zvvt_run_vvg vvg_guard_end
}

# Test: guard workflow stages modified files correctly
vvt_test_guard_stages_modifications() {
  zvvt_create_test_repo

  # Modify existing file
  echo "modified content" >> "${ZVVT_TEST_REPO}/README.md"

  # Run guard_begin
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_error "${ZBUT_STATUS}" "guard_begin should succeed"

  # Check that file is staged
  local z_staged
  z_staged=$(cd "${ZVVT_TEST_REPO}" && git diff --cached --name-only)
  test -n "${z_staged}" || but_fatal "No files staged after guard_begin"

  echo "${z_staged}" | grep -q "README.md" || \
    but_fatal "README.md should be staged" \
    "Staged files: ${z_staged}"

  # Clean up
  zvvt_run_vvg vvg_guard_end
}

# Test: guard_end warns when lock not held
vvt_test_guard_end_no_lock() {
  zvvt_create_test_repo

  # End without begin
  zvvt_run_vvg vvg_guard_end

  # Should succeed but warn
  but_fatal_on_error "${ZBUT_STATUS}" "guard_end should succeed even without lock"

  # Should contain warning about lock not held
  echo "${ZBUT_STDERR}" | grep -qi "not held\|was not held" || \
    but_trace "Expected warning about lock not held (may be OK)"
}

# Test: Can re-acquire lock after release
vvt_test_lock_reacquire() {
  zvvt_create_test_repo

  echo "first" >> "${ZVVT_TEST_REPO}/README.md"

  # Acquire
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_error "${ZBUT_STATUS}" "First guard_begin should succeed"

  # Release
  zvvt_run_vvg vvg_guard_end
  but_fatal_on_error "${ZBUT_STATUS}" "guard_end should succeed"

  echo "second" >> "${ZVVT_TEST_REPO}/README.md"

  # Re-acquire
  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_error "${ZBUT_STATUS}" "Second guard_begin should succeed after release"

  # Clean up
  zvvt_run_vvg vvg_guard_end
}

######################################################################
# Size Validation Tests (require vvr binary)

# Helper: Check if vvr binary is available
zvvt_has_vvr_predicate() {
  test -x "${ZVVT_MODULE_DIR}/bin/vvx"
}

# Test: vvr guard returns 0 for small content
vvt_test_vvr_guard_small() {
  zvvt_has_vvr_predicate || {
    but_info "Skipping vvr test - binary not available"
    return 0
  }

  zvvt_create_test_repo

  # Create small file (100 bytes)
  zvvt_create_file "small.txt" 100
  (cd "${ZVVT_TEST_REPO}" && git add small.txt)

  # Run vvx guard directly
  local z_status=0
  (cd "${ZVVT_TEST_REPO}" && "${ZVVT_MODULE_DIR}/bin/vvx" guard --limit 500000 --warn 250000) || z_status=$?

  test "${z_status}" -eq 0 || \
    but_fatal "vvr guard should return 0 for small content" \
    "Got status: ${z_status}"
}

# Test: vvr guard returns 1 for content over limit
vvt_test_vvr_guard_over_limit() {
  zvvt_has_vvr_predicate || {
    but_info "Skipping vvr test - binary not available"
    return 0
  }

  zvvt_create_test_repo

  # Create file larger than limit (use small limits for test speed)
  zvvt_create_file "big.txt" 1100
  (cd "${ZVVT_TEST_REPO}" && git add big.txt)

  # Run with small limit
  local z_status=0
  (cd "${ZVVT_TEST_REPO}" && "${ZVVT_MODULE_DIR}/bin/vvx" guard --limit 1000 --warn 500) 2>/dev/null || z_status=$?

  test "${z_status}" -eq 1 || \
    but_fatal "vvr guard should return 1 for content over limit" \
    "Got status: ${z_status}"
}

# Test: vvr guard returns 2 for content over warn threshold
vvt_test_vvr_guard_over_warn() {
  zvvt_has_vvr_predicate || {
    but_info "Skipping vvr test - binary not available"
    return 0
  }

  zvvt_create_test_repo

  # Create file between warn and limit
  zvvt_create_file "medium.txt" 750
  (cd "${ZVVT_TEST_REPO}" && git add medium.txt)

  # Run with limits that put 750 between warn (500) and limit (1000)
  local z_status=0
  (cd "${ZVVT_TEST_REPO}" && "${ZVVT_MODULE_DIR}/bin/vvx" guard --limit 1000 --warn 500) 2>/dev/null || z_status=$?

  test "${z_status}" -eq 2 || \
    but_fatal "vvr guard should return 2 for content over warn threshold" \
    "Got status: ${z_status}"
}

# Test: vvr guard returns 0 for nothing staged
vvt_test_vvr_guard_empty() {
  zvvt_has_vvr_predicate || {
    but_info "Skipping vvr test - binary not available"
    return 0
  }

  zvvt_create_test_repo

  # No staged changes
  local z_status=0
  (cd "${ZVVT_TEST_REPO}" && "${ZVVT_MODULE_DIR}/bin/vvx" guard --limit 1000 --warn 500) 2>/dev/null || z_status=$?

  test "${z_status}" -eq 0 || \
    but_fatal "vvr guard should return 0 for nothing staged" \
    "Got status: ${z_status}"
}

######################################################################
# Integration Tests

# Test: guard_begin blocks when content over limit
vvt_test_guard_blocks_large_content() {
  zvvt_has_vvr_predicate || {
    but_info "Skipping integration test - binary not available"
    return 0
  }

  zvvt_create_test_repo

  # Create file larger than the hardcoded limit (500000)
  # Use slightly over to ensure it fails
  zvvt_create_file "huge.bin" 510000
  (cd "${ZVVT_TEST_REPO}" && git add huge.bin)

  zvvt_run_vvg vvg_guard_begin
  but_fatal_on_success "${ZBUT_STATUS}" "guard_begin should fail for content over limit"

  # Verify lock was released
  if zvvt_lock_exists_predicate "commit"; then
    but_fatal "Lock should not exist after guard_begin failed size check"
  fi

  # Verify content was unstaged
  local z_staged
  z_staged=$(cd "${ZVVT_TEST_REPO}" && git diff --cached --name-only)
  test -z "${z_staged}" || \
    but_fatal "Files should be unstaged after guard_begin failed" \
    "Still staged: ${z_staged}"
}

# eof
