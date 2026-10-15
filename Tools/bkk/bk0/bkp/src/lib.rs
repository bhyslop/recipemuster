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

//! Pastern — the estate's floor crate.
//!
//! Every jj-sired Rust crate stands on this one, so it carries what they would
//! otherwise each roll by hand: a safe surface several crates need, stated once,
//! knowing nothing of which crate is asking.
//!
//! STD-ONLY, BY DECLARATION. The manifest's `[dependencies]` stands empty and
//! stays so: a crate every other crate links puts its closure beneath all of
//! them, so the floor takes nothing it would hand on. A surface that needs a
//! registry crate is not a floor surface.
//!
//! ONE SAFE SURFACE PER MODULE. Each module states its own law in its own doc,
//! and this one states only what they share. The regime reader
//! (`bkprc_catena`) is one such surface, and the program writers
//! (`bkprs_scrivener`) are another.
//!
//! IT STANDS BESIDE THE KENNEL rather than inside it. The kennel is its first
//! consumer and not its owner, so nothing here knows what a collar is.
//!
//! Open source and delivered in the kennel's own parcel, which is why the
//! license header stands on every file.

#![deny(warnings)]
#![allow(non_camel_case_types)]

pub mod bkprc_catena;
pub mod bkprs_scrivener;

#[cfg(test)]
mod bkptc_catena;
#[cfg(test)]
mod bkpts_scrivener;

// eof
