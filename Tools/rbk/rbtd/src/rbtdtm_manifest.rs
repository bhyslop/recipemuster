// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTDTM — tests for colophon manifest verification

use super::rbtdrm_manifest::*;

#[test]
fn rbtdtm_accepts_valid_manifest() {
    let manifest = format!(
        "rbw-PL rbw-gPI {} {} {} {} {} rbw-Qf",
        RBTDRM_COLOPHON_CHARGE,
        RBTDRM_COLOPHON_QUENCH,
        RBTDRM_COLOPHON_WRIT,
        RBTDRM_COLOPHON_FIAT,
        RBTDRM_COLOPHON_BARK,
    );
    assert!(rbtdrm_verify(&manifest).is_ok());
}

#[test]
fn rbtdtm_rejects_missing_colophon() {
    let manifest = "rbw-PL rbw-gPI rbw-cC rbw-Qf";
    let err = rbtdrm_verify(manifest).unwrap_err();
    assert!(err.contains(RBTDRM_COLOPHON_QUENCH));
}

#[test]
fn rbtdtm_rejects_empty_manifest() {
    let err = rbtdrm_verify("").unwrap_err();
    assert!(err.contains(RBTDRM_COLOPHON_CHARGE));
}

#[test]
fn rbtdtm_no_partial_match() {
    let manifest = "rbw-cCC rbw-cQQ";
    let err = rbtdrm_verify(manifest).unwrap_err();
    assert!(err.contains(RBTDRM_COLOPHON_CHARGE));
}
