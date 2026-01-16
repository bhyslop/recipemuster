//! JJK - Job Jockey Kit
//!
//! Rust backend for Job Jockey initiative management.
//! Implements Gallops JSON operations as vvx jjx_* subcommands.
//!
//! This crate is compiled into vvr when the jjk feature is enabled.

pub mod jjrc_core;
pub mod jjrf_favor;
pub mod jjrg_gallops;
pub mod jjrn_notch;
pub mod jjrq_query;
pub mod jjrs_steeplechase;
pub mod jjrx_cli;

// Re-export commonly used types
pub use jjrf_favor::{Coronet, Firemark};
pub use jjrn_notch::{ChalkMarker, HeatAction, format_notch_prefix, format_chalk_message, format_heat_message, format_heat_discussion};
pub use jjrq_query::{MusterArgs, SaddleArgs, ParadeArgs, RetireArgs};
pub use jjrs_steeplechase::{ReinArgs, SteeplechaseEntry, get_entries as get_steeplechase_entries};

// Re-export CLI dispatch function
pub use jjrx_cli::{dispatch, is_jjk_command};
