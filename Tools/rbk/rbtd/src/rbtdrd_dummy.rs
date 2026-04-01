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
// RBTDRD — dummy test cases for framework validation
// Placeholder cases exercising all verdict paths. Replaced by real
// test cases in later paces when container interaction is wired up.

use std::path::Path;

use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Section, rbtdre_Verdict};

fn rbtdrd_pass(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Pass
}

fn rbtdrd_pass_with_trace(dir: &Path) -> rbtdre_Verdict {
    let _ = std::fs::write(dir.join("output.txt"), "case-specific output data\n");
    rbtdre_Verdict::Pass
}

fn rbtdrd_skip(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Skip("no container runtime available".to_string())
}

fn rbtdrd_fail(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Fail("assertion: expected 42, got 0".to_string())
}

pub static RBTDRD_SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
    name: "framework-validation",
    cases: &[
        rbtdre_Case {
            name: "dummy-pass",
            func: rbtdrd_pass,
        },
        rbtdre_Case {
            name: "dummy-pass-with-trace",
            func: rbtdrd_pass_with_trace,
        },
        rbtdre_Case {
            name: "dummy-skip",
            func: rbtdrd_skip,
        },
        rbtdre_Case {
            name: "dummy-fail",
            func: rbtdrd_fail,
        },
    ],
}];
