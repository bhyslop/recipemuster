// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCCH — harbinger: the standalone stranger walk against promoted public main
// (RBSHH). Stands up the guarded coldwalk temenos from the anonymous HTTPS clone of
// the promoted public repository — the exact face a real stranger lands on —
// and hands off the launch line and stranger prompt. No gate: unlike essai,
// harbinger touches no maintainer-tree state and gives nothing to prove first.
//
// Temenos construction is the one implementation shared with essai's temenos stage
// (rbvceg_temenos, RBSHC "Worker, never authority"); this module supplies only the
// clone source that distinguishes the two callers.

use std::process::ExitCode;

use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvceg_temenos;

/// The promoted public repository, cloned anonymously over HTTPS — the same
/// face a real stranger lands on. Distinct from rbvcex_expede's SSH
/// RBVCEX_BASE_URL, which is the cut's read-only base remote, not a clone
/// target.
const RBVCCH_HARBINGER_PUBLIC_URL: &str = "https://github.com/scaleinv/recipebottle.git";

/// Conduct the harbinger command. Fatal (exit 1) on any deficit; ExitCode::SUCCESS
/// only when a walk-ready temenos stands against a clone of promoted public main.
pub fn rbvcch_harbinger_conduct() -> ExitCode {
    rbvcel_log::rbvcel_section("Hierophant Harbinger — the stranger walk against promoted public main (RBSHH)");
    rbvcel_log::rbvcel_line("Clone promoted public main, guard it, hand off the walk. Zero remote acts.");

    let top = rbvcep_repo::rbvcep_toplevel();
    let parent = rbvcep_repo::rbvcep_parent(&top);
    rbvcel_log::rbvcel_line(&format!("Maintainer tree: {}", top.display()));

    let temenos = rbvceg_temenos::rbvceg_stand_up(&parent, &top, RBVCCH_HARBINGER_PUBLIC_URL, &top);
    rbvceg_temenos::rbvceg_emit_handoff(&temenos);

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_success("Harbinger temenos stood up — guarded, disposable, push-incapable (RBSHH completion).");

    ExitCode::SUCCESS
}
