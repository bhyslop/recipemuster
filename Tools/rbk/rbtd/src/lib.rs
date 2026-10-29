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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTD Theurge — crucible test orchestrator for Recipe Bottle

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

#[path = "rbtdgb_breviary.rs"]
pub mod rbtgeb_breviary;
#[path = "rbtdgc_consts.rs"]
pub mod rbtgec_consts;
#[path = "rbtdra_almanac.rs"]
pub mod rbtcea_almanac;
#[path = "rbtdrb_probe.rs"]
pub mod rbtccp_probe;
#[path = "rbtdrc_crucible.rs"]
pub mod rbtccc_crucible;
#[path = "rbtdrd_dogfight.rs"]
pub mod rbtcgd_dogfight;
#[path = "rbtdre_engine.rs"]
pub mod rbtcee_engine;
#[path = "rbtdrf_fast.rs"]
pub mod rbtchf_fast;
#[path = "rbtdrf_handbook.rs"]
pub mod rbtchh_handbook;
#[path = "rbtdrg_log.rs"]
pub mod rbtcel_log;
#[path = "rbtdrh_chain.rs"]
pub mod rbtcgh_chain;
#[path = "rbtdri_invocation.rs"]
pub mod rbtcei_invocation;
#[path = "rbtdrj_touchstone.rs"]
pub mod rbtcct_touchstone;
#[path = "rbtdrk_depot.rs"]
pub mod rbtcgk_depot;
#[path = "rbtdrk_freehold.rs"]
pub mod rbtcgf_freehold;
#[path = "rbtdrl_calibrant.rs"]
pub mod rbtchl_calibrant;
#[path = "rbtdrm_manifest.rs"]
pub mod rbtcem_manifest;
#[path = "rbtdrn_conformance.rs"]
pub mod rbtchn_conformance;
#[path = "rbtdro_onboarding.rs"]
pub mod rbtcgo_onboarding;
#[path = "rbtdrp_attest.rs"]
pub mod rbtcga_attest;
#[path = "rbtdrp_lifecycle.rs"]
pub mod rbtcgl_lifecycle;
#[path = "rbtdrq_damnatio.rs"]
pub mod rbtchd_damnatio;
#[path = "rbtdrq_pyx.rs"]
pub mod rbtchx_pyx;
#[path = "rbtdrs_poison.rs"]
pub mod rbtchs_poison;
#[path = "rbtdru_bash.rs"]
pub mod rbtchb_bash;
#[path = "rbtdru_cupel.rs"]
pub mod rbtchc_cupel;
#[path = "rbtdru_python.rs"]
pub mod rbtchy_python;
#[path = "rbtdrv_patrol.rs"]
pub mod rbtcgp_patrol;
#[path = "rbtdrw_dowse.rs"]
pub mod rbtccd_dowse;
#[path = "rbtdrx_platform.rs"]
pub mod rbtcep_platform;
#[path = "rbtdry_quiet.rs"]
pub mod rbtdry_quiet;

#[cfg(test)]
#[path = "rbtdth_helpers.rs"]
mod rbtteh_helpers;
#[cfg(test)]
#[path = "rbtdtb_probe.rs"]
mod rbttcp_probe;
#[cfg(test)]
#[path = "rbtdtc_crucible.rs"]
mod rbttcc_crucible;
#[cfg(test)]
#[path = "rbtdte_engine.rs"]
mod rbttee_engine;
#[cfg(test)]
#[path = "rbtdti_invocation.rs"]
mod rbttei_invocation;
#[cfg(test)]
#[path = "rbtdtk_freehold.rs"]
mod rbttgf_freehold;
#[cfg(test)]
#[path = "rbtdtl_calibrant.rs"]
mod rbtthl_calibrant;
#[cfg(test)]
#[path = "rbtdtm_manifest.rs"]
mod rbttem_manifest;
#[cfg(test)]
#[path = "rbtdto_onboarding.rs"]
mod rbttgo_onboarding;
#[cfg(test)]
#[path = "rbtdtu_cupel.rs"]
mod rbtthc_cupel;
#[cfg(test)]
#[path = "rbtdtw_dowse.rs"]
mod rbttcd_dowse;
#[cfg(test)]
#[path = "rbtdtx_platform.rs"]
mod rbttep_platform;
