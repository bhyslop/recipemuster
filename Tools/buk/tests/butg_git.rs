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
//! The main-checkout creed: a linked worktree refuses, the main checkout passes.
//!
//! WHY THE SEAT COMPOSES ITS OWN REPOSITORY PAIR. The reading under test is git's
//! own — the git directory against the common directory — and it can only be
//! driven from a repository that stands beside a linked worktree of itself. The
//! composed seat is not that, so the coordinator makes one: a repository with a
//! single empty commit, and a worktree added from it. Nothing here reads the
//! estate's own repository, whose shape at a given station is exactly the variable
//! the creed exists to notice.
//!
//! THE TWO ARMS ARE ONE COORDINATOR DRIVEN TWICE, the tree the creed is entered
//! from being the only thing that varies. That is what makes the passing arm the
//! control for the refusing one: a creed that refused everything would satisfy the
//! second and fail the first.

#![deny(warnings)]

use buk::buth_hurdle::buth_Bench;

/// The band member the creed raises, spelled as `bubc_constants.sh` spells it.
///
/// NAMED HERE AND NOT COMPUTED, on the band hurdles' own ground: a number derived
/// from the band's base would go green against a band that had moved.
const BUTG_WORKTREE: i32 = 121;

/// The modules the creed needs: the band tinder, the yelp module the diagnostic
/// renders through, the console library carrying the rejection door, and the git
/// module itself.
const BUTG_MODULES: &[&str] = &[
    "bubc_constants.sh",
    "buym_yelp.sh",
    "buc_command.sh",
    "bug_git.sh",
];

/// A coordinator standing the repository pair, then entering the creed from the
/// tree named. The tree is written into the body rather than handed as an
/// argument, the dispatch reading its positional arguments as its own.
///
/// The setup is idempotent so either arm may be driven first and a seat driven
/// twice does not trip over itself. The identity rides on the commit's own
/// command line, so a station with none configured still stands the pair.
fn butg_body(enter: &str) -> String {
    format!("\
buc_context \"bujg\"
test -d hippodrome || {{
  git init --quiet hippodrome
  git -C hippodrome -c user.name=bujg -c user.email=bujg@example.invalid \\
    commit --quiet --allow-empty -m \"bujg: the one commit\"
  git -C hippodrome worktree add --quiet ../billet
}}
cd \"{enter}\"
bug_require_main_checkout_creed \"the rationale the caller supplies\"
echo \"the-creed-passed\"
",
        enter = enter
    )
}

#[test]
fn butg_a_linked_worktree_refuses_naming_the_main_checkout() {
    let said = buth_Bench::buth_seat("substrate-main-checkout-linked", BUTG_MODULES, &butg_body("billet"))
        .buth_drive(&[]);

    said.buth_took(BUTG_WORKTREE)
        .buth_carries("linked worktree")
        .buth_carries("main checkout at ")
        .buth_carries("/hippodrome")
        .buth_carries("the rationale the caller supplies")
        .buth_lacks("the-creed-passed");
}

#[test]
fn butg_the_main_checkout_passes() {
    buth_Bench::buth_seat("substrate-main-checkout-main", BUTG_MODULES, &butg_body("hippodrome"))
        .buth_drive(&[])
        .buth_thrived()
        .buth_carries("the-creed-passed");
}
// eof
