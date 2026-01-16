//! JJK - Job Jockey Kit
//!
//! Rust backend for Job Jockey initiative management.
//! Implements Gallops JSON operations as vvx jjx_* subcommands.
//!
//! This crate is compiled into vvr when the jjk feature is enabled.

#![allow(non_camel_case_types)]

pub mod jjrc_core;
pub mod jjrf_favor;
pub mod jjrg_gallops;
pub mod jjrn_notch;
pub mod jjrq_query;
pub mod jjrs_steeplechase;
pub mod jjrx_cli;

// Re-export commonly used types (with RCG prefixes)
pub use jjrf_favor::{jjrf_Coronet, jjrf_Firemark};
pub use jjrn_notch::{jjrn_ChalkMarker, jjrn_HeatAction, jjrn_format_notch_prefix, jjrn_format_chalk_message, jjrn_format_heat_message, jjrn_format_heat_discussion};
pub use jjrq_query::{jjrq_MusterArgs, jjrq_SaddleArgs, jjrq_ParadeArgs};
pub use jjrs_steeplechase::{jjrs_ReinArgs, jjrs_SteeplechaseEntry, jjrs_get_entries};

// Re-export CLI dispatch function
pub use jjrx_cli::{jjrx_dispatch, jjrx_is_jjk_command};
