// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK - Job Jockey Kit
//!
//! Rust backend for Job Jockey initiative management.
//! Implements Gallops JSON operations as vvx jjx_* subcommands.
//!
//! This crate is compiled into vvr when the jjk feature is enabled.

#![allow(non_camel_case_types)]
#![allow(private_interfaces)]
#![deny(unused_variables)]

pub mod jjrc_core;
pub mod jjrf_favor;
pub mod jjrt_types;
pub mod jjrv_validate;
pub mod jjri_io;
pub mod jjro_ops;
pub mod jjru_util;
pub mod jjrg_gallops;
pub mod jjrn_notch;
pub mod jjrnm_markers;
pub mod jjrq_query;
pub mod jjrp_print;
pub mod jjrs_steeplechase;
pub mod jjrx_cli;

// Per-command modules (jjrxx_command pattern)
pub mod jjrch_chalk;
pub mod jjrcu_curry;
pub mod jjrdr_draft;
pub mod jjrfu_furlough;
pub mod jjrgc_get_coronets;
pub mod jjrgl_garland;
pub mod jjrgs_get_spec;
pub mod jjrld_landing;
pub mod jjrmu_muster;
pub mod jjrnc_notch;
pub mod jjrno_nominate;
pub mod jjrpd_parade;
pub mod jjrrl_rail;
pub mod jjrrn_rein;
pub mod jjrrs_restring;
pub mod jjrrt_retire;
pub mod jjrsc_scout;
pub mod jjrsd_saddle;
pub mod jjrsl_slate;
pub mod jjrtl_tally;
pub mod jjrvl_validate;
pub mod jjrwp_wrap;

#[cfg(test)]
mod jjts_steeplechase;

#[cfg(test)]
mod jjtf_favor;

#[cfg(test)]
mod jjtn_notch;

#[cfg(test)]
mod jjtg_gallops;

#[cfg(test)]
mod jjtc_core;

#[cfg(test)]
mod jjtq_query;

#[cfg(test)]
mod jjtcu_curry;

#[cfg(test)]
mod jjtgl_garland;

#[cfg(test)]
mod jjtrs_restring;

#[cfg(test)]
mod jjtsc_scout;

#[cfg(test)]
mod jjtfu_furlough;

#[cfg(test)]
mod jjtrn_rein;

#[cfg(test)]
mod jjtpd_parade;

#[cfg(test)]
mod jjtnm_markers;

// Re-export commonly used types (with RCG prefixes)
pub use jjrf_favor::{jjrf_Coronet, jjrf_Firemark};
pub use jjrn_notch::{jjrn_ChalkMarker, jjrn_HeatAction, jjrn_format_notch_prefix, jjrn_format_chalk_message, jjrn_format_heat_message, jjrn_format_heat_discussion};
pub use jjrs_steeplechase::{jjrs_ReinArgs, jjrs_SteeplechaseEntry, jjrs_get_entries};

// Re-export new I/O routines
pub use jjri_io::{jjdr_load, jjdr_save, jjdr_ValidatedGallops, jjri_persist};

// Re-export CLI dispatch function
pub use jjrx_cli::{jjrx_dispatch, jjrx_is_jjk_command};
