// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Rein command - Parse git history for steeplechase entries
//!
//! Implements jjx_rein: Extract complete steeplechase history for a Heat.
//! Steeplechase entries are stored in git commit messages and parsed to show work tracking.

use clap::Args;
use crate::jjrs_steeplechase::{jjrs_ReinArgs as LibReinArgs, jjrs_run as lib_run};

/// Arguments for jjx_rein command
#[derive(Args, Debug)]
pub struct jjrrn_ReinArgs {
    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Maximum entries to return
    #[arg(long, default_value = "50")]
    pub limit: usize,
}

/// Run the rein command - display steeplechase history
pub fn jjrrn_run_rein(args: jjrrn_ReinArgs) -> i32 {
    let rein_args = LibReinArgs {
        firemark: args.firemark,
        limit: args.limit,
    };

    lib_run(rein_args)
}
