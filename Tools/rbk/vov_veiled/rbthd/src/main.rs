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
//   rbthd essai      — the reversible repair lap (RBSHE). Built.
//   rbthd harbinger  — the stranger rig against promoted public main (RBSHH). Built.
//   rbthd ostend     — the guided irreversible reveal (RBSHO). Later pace.
//
// Launched only via the withheld tabtargets (tt/rbthw-*); a direct invocation
// still resolves the working repository from cwd, which the launcher normalizes
// to the maintainer repo root.

#![allow(non_camel_case_types)]
#![deny(warnings)]

use std::process::ExitCode;

/// The essai subcommand token.
const RBTHD_CMD_ESSAI: &str = "essai";

/// The harbinger subcommand token.
const RBTHD_CMD_HARBINGER: &str = "harbinger";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some(RBTHD_CMD_ESSAI) => rbthd::rbthdr_essai::conduct(),
        Some(RBTHD_CMD_HARBINGER) => rbthd::rbthdr_harbinger::conduct(),
        Some(other) => rbthd::rbthdr_fatal!(
            "hierophant: unknown command '{}' — usage: rbthd {}|{}",
            other, RBTHD_CMD_ESSAI, RBTHD_CMD_HARBINGER
        ),
        None => rbthd::rbthdr_fatal!(
            "hierophant: no command — usage: rbthd {}|{}\n\
             launch via the withheld tabtargets: tt/rbthw-e.Essai.sh, tt/rbthw-h.Harbinger.sh",
            RBTHD_CMD_ESSAI, RBTHD_CMD_HARBINGER
        ),
    }
}
