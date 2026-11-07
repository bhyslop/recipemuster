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

//! The catch's own hurdle.
//!
//! THE CATCH IS PROVEN ON A PROCESS, because the code a panic ends a process on
//! is the runtime's to assign and no call inside this one can read it. The child
//! is this crate's example program, which cargo builds beside the test executable
//! when it builds the tests, its real main running its body through the catch.

use std::path::{
    Path,
    PathBuf, //
};
use std::process::Command;

/// The word the child is told to panic with. A made-up word, so nothing the
/// hurdle plants or asserts spells a word the estate governs.
const ZBKPTL_PLANTED: &str = "quillometric";

/// The child's path: cargo lays examples in the profile directory's `examples`,
/// beside the `deps` directory the test executable stands in. The child's name
/// is the manifest's, whose `[[example]]` table is its one home.
fn zbkptl_child() -> PathBuf {
    let exe = std::env::current_exe().expect("the hurdle reads its own executable's path");
    let profile = exe.parent().and_then(Path::parent).unwrap_or_else(|| {
        panic!(
            "the test executable at {} stands under no profile directory",
            exe.display(), //
        )
    });
    profile.join("examples").join(format!(
        "bkpxl_lanyard{}",
        std::env::consts::EXE_SUFFIX, //
    ))
}

#[test]
fn bkptl_a_panic_through_the_catch_exits_one_and_the_hook_still_speaks() {
    let child = zbkptl_child();
    assert!(
        child.exists(),
        "no child at {}: cargo builds the example beside the tests",
        child.display(), //
    );

    let panicked = Command::new(&child)
        .arg(ZBKPTL_PLANTED)
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "spawning {}: {}",
                child.display(),
                err, //
            )
        });
    let said = String::from_utf8_lossy(&panicked.stderr);
    assert_eq!(
        panicked.status.code(),
        Some(1),
        "a panic through the catch ends on 1: {}",
        said, //
    );
    assert!(
        said.contains(ZBKPTL_PLANTED),
        "the standard hook still prints the planted message: {}",
        said, //
    );

    let returned = Command::new(&child).output().unwrap_or_else(|err| {
        panic!(
            "spawning {}: {}",
            child.display(),
            err, //
        )
    });
    assert_eq!(
        returned.status.code(),
        Some(0),
        "a body that returns ends as it returned: {}",
        String::from_utf8_lossy(&returned.stderr), //
    );
}

// eof
