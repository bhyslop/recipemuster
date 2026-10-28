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
//
// The clone source is the elected world's (RBSHC "The election"): in the real
// world the promoted public repository, cloned anonymously over HTTPS — the
// face a real stranger lands on; in the practice world the private stand-in,
// which cannot be read anonymously and is cloned over SSH, the fidelity gap
// the cosmology accepts.

use std::process::ExitCode;

use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvceg_temenos;
use crate::rbvcew_world;

/// Conduct the harbinger command in the elected world. Fatal (exit 1) on any
/// deficit; ExitCode::SUCCESS only when a walk-ready temenos stands against a
/// clone of promoted public main.
pub fn rbvcch_harbinger_conduct(world: rbvcew_world::rbvcew_World) -> ExitCode {
    rbvcel_log::rbvcel_elect(world);
    let bundle = rbvcew_world::rbvcew_bundle(world);
    rbvcel_log::rbvcel_section("Hierophant Harbinger — the stranger walk against promoted public main (RBSHH)");
    rbvcel_log::rbvcel_line("Clone promoted public main, guard it, hand off the walk. Zero remote acts.");

    let top = rbvcep_repo::rbvcep_toplevel();
    rbvcel_log::rbvcel_line(&format!("Maintainer tree: {}", top.display()));

    let temenos = rbvceg_temenos::rbvceg_stand_up(
        &rbvceg_temenos::rbvceg_coldwalk_parent(&top),
        &top,
        bundle.harbinger_url,
        &top,
        bundle,
    );
    rbvceg_temenos::rbvceg_emit_handoff(&temenos);

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_success("Harbinger temenos stood up — guarded, disposable, push-incapable (RBSHH completion).");

    ExitCode::SUCCESS
}
