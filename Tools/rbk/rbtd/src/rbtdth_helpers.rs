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

use crate::rbtdra_almanac::rbtdra_lookup_fixture;
use crate::rbtdre_engine::rbtdre_Disposition;
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

/// Make a fresh, uniquely-named scratch dir under the test scratch root. The
/// name carries the label plus pid and nanos so concurrent and repeat runs
/// never collide; any stale dir of the same name is precleaned first. Panics
/// on a create failure — a test that cannot stage scratch must fail loud.
pub(crate) fn rbtdth_make_scratch(label: &str) -> PathBuf {
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = rbtdth_scratch_root().join(format!("rbtd-test-{}-{}-{}", label, pid, nanos));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("rbtdth: create scratch dir");
    dir
}

/// Assert a registered fixture carries the expected disposition. The
/// disposition is a parameter, never baked in — a fixture's tag is its own
/// fact, and callers asserting different tags (Independent vs StateProgressing)
/// share this one lookup.
pub(crate) fn rbtdth_assert_disposition(fixture: &str, expected: rbtdre_Disposition) {
    let fix = rbtdra_lookup_fixture(fixture)
        .unwrap_or_else(|| panic!("fixture '{}' not registered", fixture));
    assert_eq!(
        fix.disposition, expected,
        "{}: expected disposition {:?}, got {:?}",
        fixture, expected, fix.disposition
    );
}

/// Assert a registered fixture has exactly `expected_len` cases, and that some
/// case name contains each needle. Pass an empty needle slice to pin the count
/// alone.
pub(crate) fn rbtdth_assert_cases(fixture: &str, expected_len: usize, needles: &[&str]) {
    let fix = rbtdra_lookup_fixture(fixture)
        .unwrap_or_else(|| panic!("fixture '{}' not registered", fixture));
    assert_eq!(
        fix.cases.len(),
        expected_len,
        "{}: expected {} cases, got {}",
        fixture,
        expected_len,
        fix.cases.len()
    );
    let names: Vec<&str> = fix.cases.iter().map(|c| c.name).collect();
    for needle in needles {
        assert!(
            names.iter().any(|n| n.contains(*needle)),
            "{}: no case name contains '{}'; cases are {:?}",
            fixture,
            needle,
            names
        );
    }
}
