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

//! The hello exemplar's suite: the greeting names this crate and this binary,
//! and the position the binary answers is an object name.

use super::bkera_hello::{bkera_greeting, bkera_position};

#[test]
fn bkert_the_position_is_an_object_name() {
    let position = bkera_position();
    assert_eq!(position.len(), 40, "{position}");
    assert!(position.chars().all(|c| c.is_ascii_hexdigit()), "{position}");
}

#[test]
fn bkert_the_greeting_names_this_crate() {
    let greeting = bkera_greeting().expect("the greeting composes");
    assert!(greeting.contains(env!("CARGO_PKG_NAME")), "{greeting}");
    assert!(greeting.contains(env!("CARGO_PKG_VERSION")), "{greeting}");
}

#[test]
fn bkert_the_greeting_names_this_binary() {
    let here = std::env::current_exe().expect("the running binary's path resolves");
    let greeting = bkera_greeting().expect("the greeting composes");
    assert!(greeting.contains(&here.display().to_string()), "{greeting}");
}

// eof
