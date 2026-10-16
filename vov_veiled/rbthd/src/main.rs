// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHD Hierophant — binary entry point.
//
// Subcommands, one per ceremony (RBSHC "The command seam"):
//   rbthd essai              — the reversible repair lap (RBSHE).
//   rbthd docimasy [rehearse] — the reveal's reversible proving act (RBSHD).
//   rbthd ostend   [rehearse] — the reveal's irreversible showing (RBSHO).
//   rbthd harbinger          — the stranger rig against promoted public main (RBSHH).
//
// docimasy and ostend take an optional trailing "rehearse" token, proving
// their reversible stages against the real private narthex without
// spending credential/gauntlet cost or crossing the irreversible line.
//
// Launched only via the withheld tabtargets (tt/rbthw-*); a direct invocation
// still resolves the working repository from cwd, which the launcher normalizes
// to the maintainer repo root.

#![allow(non_camel_case_types)]
#![deny(warnings)]

use std::process::ExitCode;

/// The essai subcommand token.
const RBTHD_CMD_ESSAI: &str = "essai";

/// The docimasy subcommand token.
const RBTHD_CMD_DOCIMASY: &str = "docimasy";

/// The ostend subcommand token.
const RBTHD_CMD_OSTEND: &str = "ostend";

/// The harbinger subcommand token.
const RBTHD_CMD_HARBINGER: &str = "harbinger";

/// The optional trailing rehearse-mode token for docimasy and ostend.
const RBTHD_ARG_REHEARSE: &str = "rehearse";

/// Whether argv[2] names the rehearse token — the ceremony's dangerous-seam
/// mode switch, so unlike the unknown-command match below (which already
/// rejects loud), this must reject just as loud rather than default toward
/// real mode. A typo'd or garbled token, or a trailing extra argument, is
/// fatal, never silently read as "no rehearse" (RCG Interface Contamination
/// Discipline: reject non-canonical input, never default toward it).
fn zrbthd_rehearse(command: &str, args: &[String]) -> bool {
    match args.get(2).map(|s| s.as_str()) {
        None => false,
        Some(RBTHD_ARG_REHEARSE) if args.get(3).is_none() => true,
        Some(RBTHD_ARG_REHEARSE) => rbthd::rbthdr_fatal!(
            "hierophant {} {}: unexpected extra argument '{}'",
            command, RBTHD_ARG_REHEARSE, args[3]
        ),
        Some(other) => rbthd::rbthdr_fatal!(
            "hierophant {}: unrecognized argument '{}' — usage: rbthd {} [{}]",
            command, other, command, RBTHD_ARG_REHEARSE
        ),
    }
}

/// The flag the kennel's election asks a standing binary before it launches or
/// borrows one. Its home is `BKCE_EXERGUE_FLAG` in the kennel's election
/// (`Tools/bkk/bk0/src/bkce_election.rs`); spelled here because this crate links
/// no kennel.
const RBTHD_FLAG_EXERGUE: &str = "--exergue";

/// Answer the kennel's `--exergue` with the position this binary was struck
/// from: what the kennel's converge stated to the build script, which handed it
/// to this compile.
///
/// THE ONE STDOUT LINE THE HIEROPHANT WRITES, and it stands here rather than in
/// RBTHDR for the reason RBTHDR gives for leaving stdout clear: this is the
/// machine-read channel, the kennel reading the answer's position-shaped lines
/// from stdout, so the answer is the bare position and nothing else.
fn zrbthd_answer_exergue() -> ExitCode {
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
        Some(RBTHD_FLAG_EXERGUE) => zrbthd_answer_exergue(),
        Some(RBTHD_CMD_ESSAI) => rbthd::rbthdr_essai::rbthdr_essai_conduct(),
        Some(RBTHD_CMD_DOCIMASY) => rbthd::rbthdr_docimasy::rbthdr_docimasy_conduct(zrbthd_rehearse(RBTHD_CMD_DOCIMASY, &args)),
        Some(RBTHD_CMD_OSTEND) => rbthd::rbthdr_ostend::rbthdr_ostend_conduct(zrbthd_rehearse(RBTHD_CMD_OSTEND, &args)),
        Some(RBTHD_CMD_HARBINGER) => rbthd::rbthdr_harbinger::rbthdr_harbinger_conduct(),
        Some(other) => rbthd::rbthdr_fatal!(
            "hierophant: unknown command '{}' — usage: rbthd {}|{} [{}]|{} [{}]|{}",
            other, RBTHD_CMD_ESSAI, RBTHD_CMD_DOCIMASY, RBTHD_ARG_REHEARSE, RBTHD_CMD_OSTEND, RBTHD_ARG_REHEARSE, RBTHD_CMD_HARBINGER
        ),
        None => rbthd::rbthdr_fatal!(
            "hierophant: no command — usage: rbthd {}|{} [{}]|{} [{}]|{}\n\
             launch via the withheld tabtargets: tt/rbthw-e.Essai.sh, tt/rbthw-d.Docimasy.sh, tt/rbthw-o.Ostend.sh, tt/rbthw-h.Harbinger.sh",
            RBTHD_CMD_ESSAI, RBTHD_CMD_DOCIMASY, RBTHD_ARG_REHEARSE, RBTHD_CMD_OSTEND, RBTHD_ARG_REHEARSE, RBTHD_CMD_HARBINGER
        ),
    }
}
