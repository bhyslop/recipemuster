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
// RBTD Theurge — crucible test orchestrator for Recipe Bottle

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

/// Single source of truth for the moorings directory name, mirroring bash
/// RBCC_moorings_dir across the language boundary. Defined as a macro, not a
/// const, so bundled path constants can compose from it at compile time:
/// `concat!` rejects a const *identifier* (an opaque name by expansion time)
/// but eagerly expands a `macro_rules!` invocation, consuming the literal
/// token it produces. Zero dependency, zero runtime cost, no textual
/// repetition — every moorings path derives from this one literal.
#[macro_export]
macro_rules! rbtd_moorings_dir {
    () => {
        "rbmm_moorings"
    };
}

/// Vessels directory, composed from the moorings dir (single-sources both the
/// `rbmm_moorings` root and the `rbmv_vessels` subdir).
#[macro_export]
macro_rules! rbtd_vessels_dir {
    () => {
        concat!($crate::rbtd_moorings_dir!(), "/rbmv_vessels")
    };
}

/// Runtime `&str` form for `Path::join` sites that take a value, not a literal.
pub const RBTD_MOORINGS_DIR: &str = rbtd_moorings_dir!();

pub mod rbtdrb_probe;
pub mod rbtdrc_crucible;
pub mod rbtdrd_dogfight;
pub mod rbtdre_engine;
pub mod rbtdrf_fast;
pub mod rbtdrf_handbook;
pub mod rbtdrg_log;
pub mod rbtdri_invocation;
pub mod rbtdrk_canonical;
pub mod rbtdrl_calibrant;
pub mod rbtdrm_manifest;
pub mod rbtdro_onboarding;
pub mod rbtdrp_pristine;
pub mod rbtdrx_platform;

#[cfg(test)]
mod rbtdth_helpers;
#[cfg(test)]
mod rbtdtb_probe;
#[cfg(test)]
mod rbtdte_engine;
#[cfg(test)]
mod rbtdti_invocation;
#[cfg(test)]
mod rbtdtk_canonical;
#[cfg(test)]
mod rbtdtl_calibrant;
#[cfg(test)]
mod rbtdtm_manifest;
#[cfg(test)]
mod rbtdto_onboarding;
#[cfg(test)]
mod rbtdtp_pristine;
#[cfg(test)]
mod rbtdtx_platform;
