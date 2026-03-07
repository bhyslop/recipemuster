// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Nominate command - create a new Heat
//!
//! This module provides the Args struct and handler for the jjx_nominate command.

use std::fmt::Write;
use std::path::PathBuf;
use std::collections::BTreeMap;

use crate::jjrf_favor::{jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_NominateArgs as LibNominateArgs};
use crate::jjrc_core::jjrc_timestamp_from_env;
use crate::jjrn_notch::{jjrn_HeatAction as HeatAction, jjrn_format_heat_message as format_heat_message};

/// Arguments for jjx_nominate command
#[derive(clap::Args, Debug)]
pub struct jjrx_NominateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Kebab-case display name for the Heat
    #[arg(long, short = 's')]
    pub silks: String,
}

/// Handler for jjx_nominate command
pub fn jjrx_run_nominate(args: jjrx_NominateArgs) -> (i32, String) {
    use std::path::Path;
    let mut buf = String::new();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            jjbuf!(buf, "jjx_nominate: error: {}", e);
            return (1, buf);
        }
    };

    let mut gallops = if args.file.exists() {
        match Gallops::jjrg_load(&args.file) {
            Ok(g) => g,
            Err(e) => {
                jjbuf!(buf, "jjx_nominate: error loading Gallops: {}", e);
                return (1, buf);
            }
        }
    } else {
        if let Some(parent) = args.file.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                jjbuf!(buf, "jjx_nominate: error creating directory: {}", e);
                return (1, buf);
            }
        }
        Gallops {
            next_heat_seed: "AA".to_string(),
            heat_order: vec![],
            heats: BTreeMap::new(),
        }
    };

    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    let silks = args.silks.clone();
    let nominate_args = LibNominateArgs {
        silks: args.silks,
        created: jjrc_timestamp_from_env(),
    };

    match gallops.jjrg_nominate(nominate_args, base_path) {
        Ok(result) => {
            let fm = Firemark::jjrf_parse(&result.firemark).expect("nominate returned invalid firemark");
            let message = format_heat_message(&fm, HeatAction::Nominate, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    jjbuf!(buf, "jjx_nominate: committed {}", &hash[..8]);
                }
                Err(e) => {
                    jjbuf!(buf, "jjx_nominate: error: {}", e);
                    return (1, buf);
                }
            }

            let _ = writeln!(buf, "{}", result.firemark);
            (0, buf)
        }
        Err(e) => {
            jjbuf!(buf, "jjx_nominate: error: {}", e);
            (1, buf)
        }
    }
    // lock released here
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jjrg_gallops::*;
    use crate::jjtu_testdir::JjkTestDir;
    use std::collections::BTreeMap;

    #[test]
    fn test_nominate_appends_to_heat_order() {
        let td = JjkTestDir::new("jjk_test_nominate_appends_to_heat_order");

        let mut gallops = jjrg_Gallops {
            next_heat_seed: "AA".to_string(),
            heat_order: vec![],
            heats: BTreeMap::new(),
        };
        let args = jjrg_NominateArgs {
            silks: "test-heat".to_string(),
            created: "260101".to_string(),
        };
        let result = gallops.jjrg_nominate(args, td.path()).unwrap();
        assert!(gallops.heat_order.contains(&result.firemark),
            "heat_order should contain the new firemark after nominate");
    }
}
