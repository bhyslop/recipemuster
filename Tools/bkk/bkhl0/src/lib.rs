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

//! The hayloft tool's library: every act the binary performs, each taking its
//! clock's day, its read-back check and its folders as parameters.
//!
//! The binary reads the clock and the regime files and calls this surface; a
//! hurdle composes a day and a folder tree of its own and calls the same one,
//! so nothing a hurdle proves rides a seam the binary does not take.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

pub mod bkhlrd_day;
pub mod bkhlrg_ground;
pub mod bkhlrb_bale;
pub mod bkhlru_unbale;
pub mod bkhlrc_cli;

#[cfg(test)]
mod bkhlty_yard;
#[cfg(test)]
mod bkhltd_day;
#[cfg(test)]
mod bkhltg_ground;
#[cfg(test)]
mod bkhltb_bale;
#[cfg(test)]
mod bkhltu_unbale;

// eof
