// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK - Job Jockey Kit
//!
//! Rust backend for Job Jockey initiative management.
//! Implements Gallops JSON operations as vvx jjx_* subcommands.
//!
//! This crate is compiled into vvr when the jjk feature is enabled.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

#[macro_use]
pub mod jjrc_core;
pub mod jjrf_favor;
pub mod jjrt_types;
pub mod jjrt_v3_types;
pub mod jjrv_validate;
pub mod jjri_io;
pub mod jjro_ops;
pub mod jjru_util;
pub mod jjrz_gazette;
pub mod jjrg_gallops;
pub mod jjrn_notch;
pub mod jjrnm_markers;
pub mod jjrq_query;
pub mod jjrp_print;
pub mod jjrs_steeplechase;
pub mod jjrdk_diskcheck;
pub mod jjrfr_farrier;
pub mod jjrfg_plaingit;
pub mod jjrrf_refit;
pub mod jjrvb_blotter;
pub mod jjrvg_guidon;
pub mod jjrds_spine;
pub mod jjrdm_muck;
pub mod jjrdc_cashier;
// Per-command modules (jjrxx_command pattern)
pub mod jjrch_chalk;
pub mod jjrcu_curry;
pub mod jjrdr_draft;
pub mod jjrfu_furlough;
pub mod jjrgc_get_coronets;
pub mod jjrgs_get_spec;
pub mod jjrld_landing;
pub mod jjrlg_legatio;
pub mod jjrmt_mount;
pub mod jjrmu_muster;
pub mod jjrnc_notch;
pub mod jjrno_nominate;
pub mod jjrpd_parade;
pub mod jjrrl_rail;
pub mod jjrrn_rein;
pub mod jjrrs_restring;
pub mod jjrrt_retire;
pub mod jjrsc_scout;
pub mod jjrsl_slate;
pub mod jjrtl_tally;
pub mod jjrvl_validate;
pub mod jjrwp_wrap;
pub mod jjrm_mcp;

#[cfg(test)]
mod jjts_steeplechase;

#[cfg(test)]
mod jjtf_favor;

#[cfg(test)]
mod jjtn_notch;

#[cfg(test)]
mod jjtnc_notch;

#[cfg(test)]
mod jjtg_gallops;

#[cfg(test)]
mod jjtc_core;

#[cfg(test)]
mod jjtq_query;

#[cfg(test)]
mod jjtcu_curry;

#[cfg(test)]
mod jjtrs_restring;

#[cfg(test)]
mod jjtsc_scout;

#[cfg(test)]
mod jjtfu_furlough;

#[cfg(test)]
mod jjtrl_rail;

#[cfg(test)]
mod jjtrn_rein;

#[cfg(test)]
mod jjtpd_parade;

#[cfg(test)]
mod jjtnm_markers;

#[cfg(test)]
mod jjtu_testdir;

#[cfg(test)]
mod jjtz_gazette;

#[cfg(test)]
mod jjtm_mcp;

#[cfg(test)]
mod jjti_io;

#[cfg(test)]
mod jjtfr_farrier;

#[cfg(test)]
mod jjtfg_plaingit;

#[cfg(test)]
mod jjtrf_refit;

#[cfg(test)]
mod jjtvb_blotter;

#[cfg(test)]
mod jjtds_spine;

#[cfg(test)]
mod jjtdm_muck;

#[cfg(test)]
mod jjtvg_guidon;

#[cfg(test)]
mod jjtdc_cashier;

// Re-export commonly used types (with RCG prefixes)
pub use jjrf_favor::{jjrf_Coronet, jjrf_Firemark, jjrf_Incipit, jjrf_Pensum};
pub use jjrn_notch::{jjrn_ChalkMarker, jjrn_HeatAction, jjrn_format_notch_prefix, jjrn_format_heat_message, jjrn_format_heat_discussion};
pub use jjrs_steeplechase::{jjrs_ReinArgs, jjrs_SteeplechaseEntry, jjrs_get_entries};

// Re-export new I/O routines
pub use jjri_io::{jjdr_load, jjdr_save, jjdr_ValidatedGallops, jjri_persist, jjri_consign, jjri_commit_refusal, jjri_size_interdictum};

// Re-export the reprieve mechanism (schema-migration tolerance: rivet, probe, status, transform)
pub use jjri_io::{jjdz_probe, jjdz_Status, jjdz_write_forward, JJDZ_LABEL_REPRIEVE, JJDZ_RIVET_REPRIEVE};
