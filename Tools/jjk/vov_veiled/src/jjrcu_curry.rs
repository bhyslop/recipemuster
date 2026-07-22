// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Paddock read — the jjx_paddock reader body.
//!
//! The curry module name is historical: curry is the *write* operation, and
//! it once lived here as a stdin-driven setter mode. The writer is now the
//! jjx_curry command (jjrm_mcp CURRY arm over jjrg_curry_apply); what remains
//! here is the read half, jjx_paddock.

use std::path::PathBuf;
use vvc::{vvco_err, vvco_Output};
use crate::jjrf_favor::{jjrf_Firemark};
use crate::jjrz_gazette::{jjrz_Gazette, jjrz_Slug};

const JJRCU_CMD_NAME_PADDOCK: &str = "jjx_paddock";

/// Arguments for the jjx_paddock reader
#[derive(clap::Args, Debug)]
pub struct jjrcu_CurryArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,
}

/// Handler for the jjx_paddock reader — read a Heat's paddock content into
/// the gazette.
///
/// Read-only. The write path does not live here: a paddock revision is the
/// jjx_curry command, applied in-memory via jjrg_curry_apply and committed
/// through the shared dispatch lifecycle (jjrm_mcp CURRY arm), so it folds
/// into one commit with any batched reslates/slates.
pub fn jjrcu_run_curry(args: jjrcu_CurryArgs, gazette: &mut jjrz_Gazette, studbook_root: &std::path::Path) -> (i32, String) {
    let cn = JJRCU_CMD_NAME_PADDOCK;

    let mut output = vvco_Output::buffer();

    // Parse firemark
    let firemark = match jjrf_Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Getter mode: display paddock content
    let gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let firemark_key = firemark.jjrf_display();
    if !gallops.heats.contains_key(&firemark_key) {
        vvco_err!(output, "{}: error: Heat '{}' not found", cn, firemark_key);
        return (1, output.vvco_finish());
    }

    let paddock_path = crate::jjri_io::jjri_paddock_file(studbook_root, firemark.jjrf_as_str());
    let paddock_content = match std::fs::read_to_string(&paddock_path) {
        Ok(c) => c,
        Err(e) => {
            vvco_err!(output, "{}: error reading paddock: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Add gazette notice for downstream consumption
    gazette.jjrz_add(jjrz_Slug::Paddock, &firemark_key, &paddock_content).ok();

    (0, output.vvco_finish())
}
