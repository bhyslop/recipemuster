// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
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
// Tests run via `tt/rbw-tt.Test.sh`, which routes through BUK dispatch, whose
// dispatch file names BURD_TEMP_DIR under ../temp-buk/. Direct `cargo test`
// invocations from outside BUK dispatch are not a supported workflow and
// fail loudly here rather than silently leaking scratch under /tmp.

use std::path::PathBuf;

use crate::rbtcei_invocation::RBTCEI_BURD_TEMP_DIR_KEY;
use crate::rbtcep_platform::rbtcep_path_from_dispatch;

/// Return the scratch root for test tempdirs, read from the dispatch file the
/// pointer names. Panics where the dispatch names none — tests must be
/// launched via the BUK tabtarget so artifacts land under temp-buk and survive
/// reboot.
pub(crate) fn rbtteh_scratch_root() -> PathBuf {
    rbtcep_path_from_dispatch(RBTCEI_BURD_TEMP_DIR_KEY).unwrap_or_else(|e| {
        panic!("rbtdth: {} — run tests via `tt/rbw-tt.Test.sh`", e)
    })
}

/// Make a fresh, uniquely-named scratch dir under the test scratch root. The
/// name carries the label plus pid and nanos so concurrent and repeat runs
/// never collide; any stale dir of the same name is precleaned first. Panics
/// on a create failure — a test that cannot stage scratch must fail loud.
pub(crate) fn rbtteh_make_scratch(label: &str) -> PathBuf {
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = rbtteh_scratch_root().join(format!("scratch-{}-{}-{}", label, pid, nanos));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("rbtdth: create scratch dir");
    dir
}
