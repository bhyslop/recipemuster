// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
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
// their reversible stages against the real private quarantine without
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

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some(RBTHD_CMD_ESSAI) => rbthd::rbthdr_essai::conduct(),
        Some(RBTHD_CMD_DOCIMASY) => rbthd::rbthdr_docimasy::conduct(zrbthd_rehearse(RBTHD_CMD_DOCIMASY, &args)),
        Some(RBTHD_CMD_OSTEND) => rbthd::rbthdr_ostend::conduct(zrbthd_rehearse(RBTHD_CMD_OSTEND, &args)),
        Some(RBTHD_CMD_HARBINGER) => rbthd::rbthdr_harbinger::conduct(),
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
