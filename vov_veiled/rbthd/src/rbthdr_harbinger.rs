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
// RBTHDR — harbinger: the standalone stranger rig against promoted public main
// (RBSHH). Stands up the guarded coldwalk rig from the anonymous HTTPS clone of
// the promoted public repository — the exact face a real stranger lands on —
// and hands off the launch line and stranger prompt. No gate: unlike essai,
// harbinger touches no maintainer-tree state and gives nothing to prove first.
//
// Rig construction is the one implementation shared with essai's rig stage
// (rbthdr_rig, RBSHC "Worker, never authority"); this module supplies only the
// clone source that distinguishes the two callers.

use std::process::ExitCode;

use crate::rbthdr_log;
use crate::rbthdr_repo;
use crate::rbthdr_rig;

/// The promoted public repository, cloned anonymously over HTTPS — the same
/// face a real stranger lands on. Distinct from rbthdr_expede's SSH
/// RBTHDR_BASE_URL, which is the cut's read-only base remote, not a clone
/// target.
const RBTHDR_HARBINGER_PUBLIC_URL: &str = "https://github.com/scaleinv/recipebottle.git";

/// Conduct the harbinger command. Fatal (exit 1) on any deficit; ExitCode::SUCCESS
/// only when a walk-ready rig stands against a clone of promoted public main.
pub fn conduct() -> ExitCode {
    rbthdr_log::section("Hierophant Harbinger — the stranger rig against promoted public main (RBSHH)");
    rbthdr_log::line("Clone promoted public main, guard it, hand off the walk. Zero remote acts.");

    let top = rbthdr_repo::toplevel();
    let parent = rbthdr_repo::parent(&top);
    rbthdr_log::line(&format!("Maintainer tree: {}", top.display()));

    let rig = rbthdr_rig::stand_up(&parent, &top, RBTHDR_HARBINGER_PUBLIC_URL, &top);
    rbthdr_rig::emit_handoff(&rig);

    rbthdr_log::blank();
    rbthdr_log::success("Harbinger rig stood up — guarded, disposable, push-incapable (RBSHH completion).");

    ExitCode::SUCCESS
}
