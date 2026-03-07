// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Rail command - reorder Paces within a Heat
//!
//! Supports two modes:
//! - Order mode: replace entire sequence with provided order array
//! - Move mode: relocate a single pace using positioning flags

use clap::Args;
use std::path::PathBuf;

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrg_gallops::jjrg_Gallops as Gallops;

/// Arguments for jjx_rail command
#[derive(Args, Debug)]
pub struct jjrrl_RailArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Legacy positional args - rejected at runtime
    #[arg(trailing_var_arg = true)]
    pub order: Vec<String>,

    // Move mode arguments

    /// Coronet to relocate within order - triggers move mode
    #[arg(long, short = 'm')]
    pub r#move: Option<String>,

    /// Move before specified Coronet
    #[arg(long, conflicts_with_all = ["after", "first", "last"])]
    pub before: Option<String>,

    /// Move after specified Coronet
    #[arg(long, conflicts_with_all = ["before", "first", "last"])]
    pub after: Option<String>,

    /// Move to beginning of order
    #[arg(long, conflicts_with_all = ["before", "after", "last"])]
    pub first: bool,

    /// Move to end of order
    #[arg(long, conflicts_with_all = ["before", "after", "first"])]
    pub last: bool,
}

/// Execute rail command - reorder Paces within a Heat
pub fn jjrrl_run_rail(args: jjrrl_RailArgs) -> (i32, String) {
    use std::fmt::Write;
    use crate::jjrg_gallops::jjrg_RailArgs as LibRailArgs;
    use crate::jjrn_notch::{jjrn_HeatAction, jjrn_format_heat_message};

    let mut buf = String::new();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            jjbuf!(buf, "jjx_rail: error: {}", e);
            return (1, buf);
        }
    };

    let order: Vec<String> = if args.order.len() == 1 && args.order[0].starts_with('[') {
        match serde_json::from_str(&args.order[0]) {
            Ok(v) => v,
            Err(_) => args.order.clone(),
        }
    } else {
        args.order.clone()
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            jjbuf!(buf, "jjx_rail: error loading Gallops: {}", e);
            return (1, buf);
        }
    };

    let firemark = args.firemark.clone();
    let move_coronet = args.r#move.clone();
    let move_before = args.before.clone();
    let move_after = args.after.clone();
    let move_first = args.first;
    let move_last = args.last;

    // Capture old order for no-op detection
    let fm = Firemark::jjrf_parse(&firemark).expect("rail given invalid firemark");
    let old_order: Vec<String> = gallops.heats.get(&fm.jjrf_display())
        .map(|h| h.order.clone())
        .unwrap_or_default();

    let rail_args = LibRailArgs {
        firemark: args.firemark,
        order,
        move_coronet: args.r#move,
        before: args.before,
        after: args.after,
        first: args.first,
        last: args.last,
    };

    match gallops.jjrg_rail(rail_args) {
        Ok(new_order) => {
            // No-op detection: if order unchanged, exit 0 with message
            if new_order == old_order {
                let position = if move_first { "first" }
                    else if move_last { "last" }
                    else if move_before.is_some() { "requested position" }
                    else if move_after.is_some() { "requested position" }
                    else { "requested position" };
                let coronet_display = move_coronet.as_deref().unwrap_or("pace");
                jjbuf!(buf, "jjx_rail: no change — {} is already {}", coronet_display, position);
                for coronet in new_order {
                    let _ = writeln!(buf, "{}", coronet);
                }
                return (0, buf);
            }

            // Compute descriptive subject for commit message
            let subject = if let Some(ref moved) = move_coronet {
                // Move mode: describe where the pace was moved
                let target = if move_first { "to first".to_string() }
                    else if move_last { "to last".to_string() }
                    else if let Some(ref b) = move_before { format!("before {}", b) }
                    else if let Some(ref a) = move_after { format!("after {}", a) }
                    else { "???".to_string() };
                format!("moved {} {}", moved, target)
            } else {
                // Order mode: list the new order
                format!("order: {}", new_order.join(", "))
            };

            let message = jjrn_format_heat_message(&fm, jjrn_HeatAction::Rail, &subject);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    jjbuf!(buf, "jjx_rail: committed {}", &hash[..8]);
                }
                Err(e) => {
                    jjbuf!(buf, "jjx_rail: error: {}", e);
                    return (1, buf);
                }
            }

            for coronet in new_order {
                let _ = writeln!(buf, "{}", coronet);
            }
            (0, buf)
        }
        Err(e) => {
            jjbuf!(buf, "jjx_rail: error: {}", e);
            (1, buf)
        }
    }
    // lock released here
}
