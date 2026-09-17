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

//! What the kennel knows about its own making.
//!
//! This is the answer to a question only the artifact can be asked. An mtime
//! comparison answers "should I rebuild before I exec", which is a launcher's
//! question about a file it is about to run; a running process asking whether it
//! is still current can only carry the answer with it
//! (BKSNC-Kennelcraft.adoc "Currency by git position").
//!
//! ONE POSITION, NOT TWO, AND IT IS THE SEAT'S. A seat-built binary has exactly
//! one consumer — the whistle that built it — asking exactly one question: is
//! what stands current with what this seat holds. The trunk-counterpart stamp
//! `Tools/buk/bue_exergue.sh` strikes cannot answer it, and not by accident: that
//! module reads LANDINGS and never HEAD, deliberately, so a commit made at a
//! billet never enters its walk. A whistle comparing landed stamps would sit
//! still while the source moved. The landed reading is what a DELIVERED binary
//! carries, struck at release for a reader that holds a record of landings rather
//! than the repository; it arrives when such a reader exists, and none does at
//! MVP, the kit being delivered as source. The seat side of the comparison
//! BKSNC "The Binary Election" describes is what this carries.
//!
//! THE POSITION IS WALKED OVER THE ELECTION, never bare HEAD. What stales an
//! artifact is a change to what it is made from, which is what the election
//! states; bare HEAD would stale this binary on every commit anywhere in the
//! repository and force a relink of something that did not change.
//!
//! THREE FACTS, ONE SOURCE. All of them come from `build.rs`, which is the only
//! place cargo's own choices are observable — and a build that cannot state all
//! three dies there. There is no sentinel value and no "unknown": an unstamped
//! kennel would answer questions about itself with text that means nothing.

/// The version of the rustc that actually compiled this binary, observed rather
/// than requested.
pub const BKCS_COMPILER: &str = env!("BKCS_COMPILER");

/// The channel the pin file asked for at the moment of the build.
pub const BKCS_PIN: &str = env!("BKCS_PIN");

/// The seat's newest first-parent commit touching an elected root, as of the
/// build.
pub const BKCS_SEAT: &str = env!("BKCS_SEAT");

/// Everything the kennel can say about its own making.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcs_Position {
    /// The position this build was taken at.
    pub seat: &'static str,
    /// The rustc that ran.
    pub compiler: &'static str,
    /// The channel the pin named.
    pub pin: &'static str,
}

/// Read this binary's own position. Constant by construction — every field was
/// fixed when the binary was struck, which is the whole property that makes the
/// reading trustworthy.
pub fn bkcs_position() -> bkcs_Position {
    bkcs_Position {
        seat: BKCS_SEAT,
        compiler: BKCS_COMPILER,
        pin: BKCS_PIN,
    }
}

impl bkcs_Position {
    /// The self-report, one fact per line.
    ///
    /// The pin and the compiler stand adjacent deliberately: the request and the
    /// answer read together, so a reader can see for themselves whether the
    /// channel that was asked for is the one that ran, rather than taking the
    /// kennel's word that it agreed.
    pub fn bkcs_render(&self) -> String {
        let mut said = String::new();
        said.push_str(&format!("seat      {}\n", self.seat));
        said.push_str(&format!("pin       {}\n", self.pin));
        said.push_str(&format!("compiler  {}", self.compiler));
        said
    }
}

// eof
