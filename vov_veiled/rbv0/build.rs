// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>

//! The hierophant's build script: it compiles in the position the kennel's
//! converge states, so the binary can say what it was struck from when the
//! kennel asks it `--exergue` (the reader contract its app collar owes).
//!
//! A BUILD THAT CANNOT STATE THE POSITION DIES HERE. There is no "unknown" to
//! fall back on: a binary answering the kennel's question with text that means
//! nothing would be believed. The pattern is the kennel's hello exemplar's.

// EVERY DECLARATION STANDS INSIDE `main`: a file cargo names carries no prefix
// of its own.
fn main() {
    /// The variable the kennel's converge states into the build. Its home is the
    /// converge, `BKCH_COLLAR_POSITION_VAR` in `Tools/bkk/bk0/src/bkch_heel.rs`;
    /// it is spelled here because a build script links nothing. Cargo is told to
    /// watch it, so a commit touching the collar's roots rebuilds the binary
    /// rather than leaving it claiming a position it no longer holds.
    const Z_SEAT_VAR: &str = "BKCH_COLLAR_POSITION";

    println!("cargo:rerun-if-env-changed={}", Z_SEAT_VAR);

    let seat = match std::env::var(Z_SEAT_VAR) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => panic!(
            "{} is unset, so this build cannot state the position it was taken at. \
             The hierophant is built through the kennel's doors, which state it: \
             tt/rbvw-b.Build.sh for the app, tt/rbvw-t.Test.sh for the suite",
            Z_SEAT_VAR, //
        ),
    };

    // RE-EMITTED UNDER THE NAME IT ARRIVED UNDER, so `env!` reads what cargo set
    // for this compile and never whatever the ambient environment held.
    println!("cargo:rustc-env={}={}", Z_SEAT_VAR, seat.trim());
}

// eof
