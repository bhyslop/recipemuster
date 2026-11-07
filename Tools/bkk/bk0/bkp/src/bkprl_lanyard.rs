// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
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

//! The catch at the top of a main.
//!
//! A PANIC THAT REACHES A MAIN ENDS THE PROCESS BROKE, ON EXIT 1. Left to the
//! runtime, a process whose main thread panics exits 101, and 101 is a code the
//! estate's own precision band mints for a gate of its own, so a caller reading
//! an estate binary's status could not tell a panic from a ruling. A main that
//! runs its body through `bkprl_run` ends on the standard library's failure code
//! instead: the imprecise death every caller already reads as broke, the tool
//! having failed before ruling.
//!
//! THE STANDARD HOOK STILL SPEAKS, AND NOTHING HERE INSTALLS A HOOK. The hook
//! runs at the panic, before the unwind reaches this catch, so the message and
//! its location print exactly as they would have; the catch changes only the
//! code the process ends on. A hook that exited would end the process from
//! inside every catch standing deeper than the main — a server catching each
//! handler's panic so it can answer the call regardless, a guard whose drop
//! releases a lock on the unwind — before any of them could do its work. So the
//! exit is answered here, at the top, and never deeper.
//!
//! A CATCH INSIDE THE BODY STANDS. A body that catches its own unwind and
//! re-raises it by `resume_unwind` re-raises it into this catch, which answers it
//! as it answers any other: the re-raise skips the hook and nothing else.
//!
//! The body's own end passes through as `std::process::Termination` reports it,
//! so a body returning unit succeeds and a body returning an `ExitCode` hands its
//! code on unchanged.

use std::panic::UnwindSafe;
use std::process::{
    ExitCode,
    Termination, //
};

/// Run a main's body, answering a panic that reaches it as broke.
pub fn bkprl_run<T: Termination>(body: impl FnOnce() -> T + UnwindSafe) -> ExitCode {
    match std::panic::catch_unwind(body) {
        Ok(end) => end.report(),
        Err(_) => ExitCode::FAILURE,
    }
}

// eof
