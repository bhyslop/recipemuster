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

//! The catch hurdle's child (`bkptl_lanyard`): a real main running its body
//! through the catch, which panics with its first argument where one is given
//! and returns where none is.

fn main() -> std::process::ExitCode {
    bkp::bkprl_lanyard::bkprl_run(|| {
        if let Some(word) = std::env::args().nth(1) {
            panic!("{}", word);
        }
    })
}

// eof
