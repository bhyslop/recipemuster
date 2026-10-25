// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVM Hierophant — binary entry point.
//
// Subcommands, one per ceremony (RBSHC "The command seam"):
//   rbv essai              — the reversible repair lap (RBSHE).
//   rbv docimasy [rehearse] — the reveal's reversible proving act (RBSHD).
//   rbv ostend   [rehearse] — the reveal's irreversible showing (RBSHO).
//   rbv harbinger          — the stranger walk against promoted public main (RBSHH).
//
// docimasy and ostend take an optional trailing "rehearse" token, proving
// their reversible stages against the real private narthex without
// spending credential/gauntlet cost or crossing the irreversible line.
//
// Launched only via the withheld tabtargets (tt/rbvw-*); a direct invocation
// still resolves the working repository from cwd, which the launcher normalizes
// to the maintainer repo root.

#![allow(non_camel_case_types)]
#![deny(warnings)]

use std::process::ExitCode;

/// The essai subcommand token.
const RBVM_CMD_ESSAI: &str = "essai";

/// The docimasy subcommand token.
const RBVM_CMD_DOCIMASY: &str = "docimasy";

/// The ostend subcommand token.
const RBVM_CMD_OSTEND: &str = "ostend";

/// The harbinger subcommand token.
const RBVM_CMD_HARBINGER: &str = "harbinger";

/// The optional trailing rehearse-mode token for docimasy and ostend.
const RBVM_ARG_REHEARSE: &str = "rehearse";

/// Whether argv[2] names the rehearse token — the ceremony's dangerous-seam
/// mode switch, so unlike the unknown-command match below (which already
/// rejects loud), this must reject just as loud rather than default toward
/// real mode. A typo'd or garbled token, or a trailing extra argument, is
/// fatal, never silently read as "no rehearse" (RCG Interface Contamination
/// Discipline: reject non-canonical input, never default toward it).
fn zrbvm_rehearse(command: &str, args: &[String]) -> bool {
    match args.get(2).map(|s| s.as_str()) {
        None => false,
        Some(RBVM_ARG_REHEARSE) if args.get(3).is_none() => true,
        Some(RBVM_ARG_REHEARSE) => rbv::rbvcel_fatal!(
            "hierophant {} {}: unexpected extra argument '{}'",
            command, RBVM_ARG_REHEARSE, args[3]
        ),
        Some(other) => rbv::rbvcel_fatal!(
            "hierophant {}: unrecognized argument '{}' — usage: rbv {} [{}]",
            command, other, command, RBVM_ARG_REHEARSE
        ),
    }
}

/// The flag the kennel's election asks a standing binary before it launches or
/// borrows one. Its home is `BKCE_EXERGUE_FLAG` in the kennel's election
/// (`Tools/bkk/bk0/src/bkce_election.rs`); spelled here because this crate links
/// no kennel.
const RBVM_FLAG_EXERGUE: &str = "--exergue";

/// Answer the kennel's `--exergue` with the position this binary was struck
/// from: what the kennel's converge stated to the build script, which handed it
/// to this compile.
///
/// THE ONE STDOUT LINE THE HIEROPHANT WRITES, and it stands here rather than in
/// RBVCEL for the reason RBVCEL gives for leaving stdout clear: this is the
/// machine-read channel, the kennel reading the answer's position-shaped lines
/// from stdout, so the answer is the bare position and nothing else.
fn zrbvm_answer_exergue() -> ExitCode {
    use std::io::Write;
    let mut stdout = std::io::stdout().lock();
    match writeln!(stdout, "{}", env!("BKCH_COLLAR_POSITION")) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some(RBVM_FLAG_EXERGUE) => zrbvm_answer_exergue(),
        Some(RBVM_CMD_ESSAI) => rbv::rbvcce_essai::rbvcce_essai_conduct(),
        Some(RBVM_CMD_DOCIMASY) => rbv::rbvccd_docimasy::rbvccd_docimasy_conduct(zrbvm_rehearse(RBVM_CMD_DOCIMASY, &args)),
        Some(RBVM_CMD_OSTEND) => rbv::rbvcco_ostend::rbvcco_ostend_conduct(zrbvm_rehearse(RBVM_CMD_OSTEND, &args)),
        Some(RBVM_CMD_HARBINGER) => rbv::rbvcch_harbinger::rbvcch_harbinger_conduct(),
        Some(other) => rbv::rbvcel_fatal!(
            "hierophant: unknown command '{}' — usage: rbv {}|{} [{}]|{} [{}]|{}",
            other, RBVM_CMD_ESSAI, RBVM_CMD_DOCIMASY, RBVM_ARG_REHEARSE, RBVM_CMD_OSTEND, RBVM_ARG_REHEARSE, RBVM_CMD_HARBINGER
        ),
        None => rbv::rbvcel_fatal!(
            "hierophant: no command — usage: rbv {}|{} [{}]|{} [{}]|{}\n\
             launch via the withheld tabtargets: tt/rbvw-e.Essai.sh, tt/rbvw-d.Docimasy.sh, tt/rbvw-o.Ostend.sh, tt/rbvw-h.Harbinger.sh",
            RBVM_CMD_ESSAI, RBVM_CMD_DOCIMASY, RBVM_ARG_REHEARSE, RBVM_CMD_OSTEND, RBVM_ARG_REHEARSE, RBVM_CMD_HARBINGER
        ),
    }
}
