// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVM Hierophant — binary entry point.
//
// Subcommands, one per ceremony (RBSHC "The command seam"), each taking the
// world it conducts in as its one argument, with no default (RBSHC "The
// election" — the agora or the palaestra):
//   rbv essai     <world> — the reversible repair lap (RBSHE).
//   rbv docimasy  <world> — the reveal's reversible proving act (RBSHD).
//   rbv ostend    <world> — the reveal's irreversible showing (RBSHO).
//   rbv harbinger <world> — the stranger walk against promoted public main (RBSHH).
//
// Beside them one station chore, never a ceremony act (RBSHC "The election"):
//   rbv encamp <world> <home> — stand up a practice seat beneath an absolute
//   practice home; the palaestra alone (rbvces_seat).
//
// The world is judged by the library's own parser (rbvcew_world), where the
// self-proofs reach it; this file only hands it argv and fatals on a refusal.
//
// Launched only via the withheld tabtargets (tt/rbvw-*, one per command and
// world, the world riding the imprint); a direct invocation still resolves
// the working repository from cwd, which the launcher normalizes to the
// maintainer repo root.

#![allow(non_camel_case_types)]
#![deny(warnings)]

use std::process::ExitCode;

use rbv::rbvcew_world;
use rbv::rbvgeb_breviary;

/// The essai subcommand token.
const RBVM_CMD_ESSAI: &str = "essai";

/// The docimasy subcommand token.
const RBVM_CMD_DOCIMASY: &str = "docimasy";

/// The ostend subcommand token.
const RBVM_CMD_OSTEND: &str = "ostend";

/// The harbinger subcommand token.
const RBVM_CMD_HARBINGER: &str = "harbinger";

/// The practice seat's subcommand token — a chore beside the ceremony, not a
/// ceremony act: it encamps a practice seat.
const RBVM_CMD_ENCAMP: &str = "encamp";

/// The world argv[2] elects for `command` — the ceremony's dangerous-seam
/// input. The library's parser refuses a bare invocation, an unknown word and
/// a trailing extra argument; each refusal is fatal here, never defaulted
/// toward either world (RCG Interface Contamination Discipline: reject
/// non-canonical input, never default toward it).
fn zrbvm_world(command: &str, args: &[String]) -> rbvcew_world::rbvcew_World {
    let rest = args.get(2..).unwrap_or(&[]);
    rbvcew_world::rbvcew_elect(command, rest).unwrap_or_else(|e| {
        rbv::rbvcel_fatal!(
            "{}",
            e, //
        )
    })
}

/// The practice home argv[3] names under the world argv[2] elects, for the
/// practice seat. The library's parser refuses anything but the palaestra, a
/// missing or relative home and a trailing extra argument; each refusal is
/// fatal here.
fn zrbvm_encamp_home(args: &[String]) -> std::path::PathBuf {
    let rest = args.get(2..).unwrap_or(&[]);
    rbv::rbvces_seat::rbvces_elect(RBVM_CMD_ENCAMP, rest).unwrap_or_else(|e| {
        rbv::rbvcel_fatal!(
            "{}",
            e, //
        )
    })
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
    match writeln!(
        stdout,
        "{}",
        env!("BKCH_COLLAR_POSITION"), //
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some(RBVM_FLAG_EXERGUE) => zrbvm_answer_exergue(),
        Some(RBVM_CMD_ESSAI) => rbv::rbvcce_essai::rbvcce_essai_conduct(zrbvm_world(RBVM_CMD_ESSAI, &args)),
        Some(RBVM_CMD_DOCIMASY) => rbv::rbvccd_docimasy::rbvccd_docimasy_conduct(zrbvm_world(RBVM_CMD_DOCIMASY, &args)),
        Some(RBVM_CMD_OSTEND) => rbv::rbvcco_ostend::rbvcco_ostend_conduct(zrbvm_world(RBVM_CMD_OSTEND, &args)),
        Some(RBVM_CMD_HARBINGER) => rbv::rbvcch_harbinger::rbvcch_harbinger_conduct(zrbvm_world(RBVM_CMD_HARBINGER, &args)),
        Some(RBVM_CMD_ENCAMP) => rbv::rbvces_seat::rbvces_seat_conduct(zrbvm_encamp_home(&args)),
        Some(other) => rbv::rbvcel_fatal!(
            "{}: unknown verb '{}' — usage: rbv {}|{}|{}|{} {}|{}, or rbv {} {} <home>",
            rbvgeb_breviary::RBVGEB_HIEROPHANT_BASE, other,
            RBVM_CMD_ESSAI, RBVM_CMD_DOCIMASY, RBVM_CMD_OSTEND, RBVM_CMD_HARBINGER,
            rbvgeb_breviary::RBVGEB_AGORA_BASE, rbvgeb_breviary::RBVGEB_PALAESTRA_BASE,
            RBVM_CMD_ENCAMP, rbvgeb_breviary::RBVGEB_PALAESTRA_BASE, //
        ),
        None => rbv::rbvcel_fatal!(
            "{}: no verb — usage: rbv {}|{}|{}|{} {}|{}, or rbv {} {} <home>\n\
             launch via the withheld colophons rbvw-e, rbvw-d, rbvw-o and rbvw-h, each imprinted with its world, and rbvw-p imprinted with the palaestra",
            rbvgeb_breviary::RBVGEB_HIEROPHANT_BASE,
            RBVM_CMD_ESSAI, RBVM_CMD_DOCIMASY, RBVM_CMD_OSTEND, RBVM_CMD_HARBINGER,
            rbvgeb_breviary::RBVGEB_AGORA_BASE, rbvgeb_breviary::RBVGEB_PALAESTRA_BASE,
            RBVM_CMD_ENCAMP, rbvgeb_breviary::RBVGEB_PALAESTRA_BASE, //
        ),
    }
}
