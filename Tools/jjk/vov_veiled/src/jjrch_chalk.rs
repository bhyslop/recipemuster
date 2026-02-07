// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK chalk command - empty commits marking steeplechase events
//!
//! The chalk command creates empty commits with special formatting for tracking
//! steeplechase events like approach, wrap, fly, or heat-level discussions.

use crate::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark};
use crate::jjrn_notch::{jjrn_ChalkMarker as ChalkMarker, jjrn_format_chalk_message, jjrn_format_heat_discussion};

/// Arguments for jjx_chalk command
#[derive(clap::Args, Debug)]
pub struct jjrx_ChalkArgs {
    /// Identity: Coronet (pace-level) or Firemark (heat-level discussion only)
    pub identity: String,

    /// Marker type: A(pproach), W(rap), F(ly), d(iscussion)
    #[arg(long)]
    pub marker: String,

    /// Marker description text
    #[arg(long)]
    pub description: String,
}

/// Run the chalk command - create steeplechase marker commit
pub fn jjrx_run_chalk(args: jjrx_ChalkArgs) -> i32 {
    let marker = match ChalkMarker::jjrn_parse(&args.marker) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("jjx_chalk: error: {}", e);
            return 1;
        }
    };

    // Try parsing as Coronet first (5 base64 chars), then as Firemark (2 base64 chars)
    let identity = args.identity.strip_prefix('₢').or_else(|| args.identity.strip_prefix('₣')).unwrap_or(&args.identity);

    let message = if identity.len() == 5 {
        // Coronet - pace-level chalk
        let coronet = match Coronet::jjrf_parse(&args.identity) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_chalk: error: {}", e);
                return 1;
            }
        };
        jjrn_format_chalk_message(&coronet, marker, &args.description)
    } else if identity.len() == 2 {
        // Firemark - heat-level (discussion, session)
        if marker.jjrn_requires_pace() {
            eprintln!("jjx_chalk: error: {} marker requires a Coronet (pace identity), not a Firemark", marker.jjrn_as_str());
            return 1;
        }
        let firemark = match Firemark::jjrf_parse(&args.identity) {
            Ok(fm) => fm,
            Err(e) => {
                eprintln!("jjx_chalk: error: {}", e);
                return 1;
            }
        };

        jjrn_format_heat_discussion(&firemark, &args.description)
    } else {
        eprintln!("jjx_chalk: error: identity must be Coronet (5 chars) or Firemark (2 chars), got {} chars", identity.len());
        return 1;
    };

    let commit_args = vvc::vvcc_CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    vvc::commit(&commit_args)
}
