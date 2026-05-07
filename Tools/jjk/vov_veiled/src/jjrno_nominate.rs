// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Nominate command - create a new Heat
//!
//! This module provides the Args struct and handler for the jjx_nominate command.

use std::path::PathBuf;
use std::collections::BTreeMap;

use vvc::{vvco_out, vvco_err, vvco_Output};

// Command name constant — RCG String Boundary Discipline
const JJRNO_CMD_NAME_CREATE: &str = "jjx_create";

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
    let cn = JJRNO_CMD_NAME_CREATE;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let mut gallops = if args.file.exists() {
        match Gallops::jjrg_load(&args.file) {
            Ok(g) => g,
            Err(e) => {
                vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
                return (1, output.vvco_finish());
            }
        }
    } else {
        if let Some(parent) = args.file.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                vvco_err!(output, "{}: error creating directory: {}", cn, e);
                return (1, output.vvco_finish());
            }
        }
        Gallops {
            schema_version: Some(4),
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

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, vvc::VVCG_SIZE_LIMIT, &mut output) {
                Ok(hash) => {
                    vvco_out!(output, "{}: committed {}", cn, &hash[..8]);
                }
                Err(e) => {
                    vvco_err!(output, "{}: error: {}", cn, e);
                    return (1, output.vvco_finish());
                }
            }

            vvco_out!(output, "{}", result.firemark);
            (0, output.vvco_finish())
        }
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}

#[cfg(test)]
mod tests {
    use crate::jjrg_gallops::*;
    use crate::jjtu_testdir::JjkTestDir;
    use std::collections::BTreeMap;

    #[test]
    fn test_nominate_appends_to_heat_order() {
        let td = JjkTestDir::new("jjk_test_nominate_appends_to_heat_order");

        let mut gallops = jjrg_Gallops {
            schema_version: Some(4),
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
