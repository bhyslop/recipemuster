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

//! The parcel engine.
//!
//! A forge collects the kits a target declares into a staging directory and
//! issues its record; a target lays the parcel that results. Both halves live here, so
//! any repository carrying this kit can mint a parcel as well as receive one.
//!
//! The forge's caller supplies what only it can know: the kits its forge may
//! distribute, and the behest the install will need — the signets the Claude
//! assets wear, the canonical binary a taken kit carries, and a seat's precinct
//! and endonym — which the release writes into the parcel. The install and the
//! vacate read that behest from the parcel and take nothing from a caller, so
//! this kit's own binary, `bpx`, drives them for any forge. Nothing in the
//! library writes to a console; every operation returns what it did for its
//! caller to report, and the binary is the one place that reports it.

#![deny(warnings)]
#![allow(non_camel_case_types)]

#[path = "bpe0/bpee_emplace.rs"]
pub mod bpee_emplace;
#[path = "bpe0/bper_release.rs"]
pub mod bper_release;

// Re-export release types and functions
pub use bper_release::bper_off_trunk;
pub use bper_release::{
    bper_CollectResult,
    bper_TamgaResult,
    bper_collect,
    bper_issue_tamga,
    bper_tamga_json,
    bper_target_name, //
};
pub use bper_release::{
    bper_Kit,
    bper_Roster, //
};
pub use bper_release::{
    bper_RegistryEntry,
    bper_registry_json, //
};
pub use bper_release::{
    bper_git_tracked_files,
    bper_git_untracked_files, //
};

// Re-export emplace and vacate types and functions: each reads its parcel's
// behest, so no caller hands an install or a vacate inputs of its own.
pub use bpee_emplace::{
    bpee_Behest,
    bpee_Binary,
    bpee_Signets, //
};
pub use bpee_emplace::{
    bpee_BurcEnv,
    bpee_parse_burc, //
};
pub use bpee_emplace::{
    bpee_EmplaceResult,
    bpee_Freight,
    bpee_emplace_parcel,
    bpee_installed_tamga_json, //
};
pub use bpee_emplace::{
    bpee_VacateResult,
    bpee_vacate_parcel, //
};
