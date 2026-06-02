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
// RBTDTH — shared test helpers
//
// Tests run via `tt/rbw-tt.Test.sh`, which routes through BUK dispatch and
// exports BURD_TEMP_DIR pointing under ../temp-buk/. Direct `cargo test`
// invocations from outside BUK dispatch are not a supported workflow and
// fail loudly here rather than silently leaking scratch under /tmp.

use std::path::PathBuf;

use crate::rbtdri_invocation::RBTDRI_BURD_TEMP_DIR_KEY;
use crate::rbtdrx_platform::rbtdrx_posix_to_native;

/// Return the scratch root for test tempdirs. Panics if BURD_TEMP_DIR is
/// unset — tests must be launched via the BUK tabtarget so artifacts land
/// under temp-buk and survive reboot.
pub(crate) fn rbtdth_scratch_root() -> PathBuf {
    match std::env::var(RBTDRI_BURD_TEMP_DIR_KEY) {
        Ok(v) if !v.is_empty() => rbtdrx_posix_to_native(&v)
            .unwrap_or_else(|e| panic!("rbtdth: cannot nativize {}: {}", RBTDRI_BURD_TEMP_DIR_KEY, e)),
        _ => panic!(
            "rbtdth: {} is not set — run tests via `tt/rbw-tt.Test.sh`",
            RBTDRI_BURD_TEMP_DIR_KEY
        ),
    }
}
