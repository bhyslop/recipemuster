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
// RBTDRD — dogfight cloud-build viability fixture
//
// Proves the cloud-depot build-and-retrieve path yields a *runnable*
// artifact, with no crucible apparatus. Standing-depot scenario fixture in
// the operator-precondition family: it reuses a depot the operator has levied
// by hand (no levy, no unmake) and assumes director + retriever credentials
// already in place. It differs from the skirmish chain
// on the crucible axis — dogfight charges NO crucible. It proves only
// build → summon → run viability, not containment (the crucible's orthogonal
// concern).
//
// Single case, ordain → summon → run → abjure, threaded through one body. The
// busybox vessel is consumerless — no nameplate holds its hallmark, so there
// is no committed regime file to carry the ephemeral hallmark across a
// case boundary. The hallmark therefore lives as a local across the steps,
// the same structural choice rbtdrv_hallmark_lifecycle makes for the same
// reason. This fixture IS hallmark_lifecycle with the registry-inventory
// middle (audit/rekon) swapped for summon + a bare container-runtime run.

use std::path::Path;
use std::process::Command;

use crate::case;
use crate::rbtdrc_crucible::rbtdrc_with_ctx;
use crate::rbtdrv_patrol::{
    rbtdrv_docker_inspect, RBTDRV_ARK_BASENAME_IMAGE, RBTDRV_BUSYBOX_VESSEL_DIR,
};
use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Disposition, rbtdre_Fixture, rbtdre_Verdict};
use crate::rbtdri_invocation::{
    rbtdri_gar_ref_fact, rbtdri_invoke_or_fail, rbtdri_ordain_capture_full, rbtdri_Context,
    RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP,
};
use crate::rbtdgc_consts::{RBTDGC_ABJURE_HALLMARK, RBTDGC_SUMMON_HALLMARK};

/// Container runtime for the bare executability proof. Hardcoded to docker;
/// podman is deferred to the Director-governed runtime-regime decision that
/// rides with ₣BS. This single named site is the future swap point — do NOT
/// add a regime field here now.
const RBTDRD_RUNTIME: &str = "docker";

/// Degenerate command proving the summoned image is runnable. busybox's
/// default cmd is `sh`; passing an explicit `true` yields a clean exit-0
/// executability proof without spawning an interactive shell.
const RBTDRD_PROOF_CMD: &str = "true";

// ── Runtime run helper ───────────────────────────────────────

/// Run `<runtime> run --rm <image_ref> <cmd>` and return Ok on exit 0. The
/// single site naming the container runtime — see RBTDRD_RUNTIME.
fn rbtdrd_runtime_run(image_ref: &str, cmd: &str, dir: &Path) -> Result<(), String> {
    let output = Command::new(RBTDRD_RUNTIME)
        .args(["run", "--rm", image_ref, cmd])
        .output()
        .map_err(|e| format!("{} run exec failed: {}", RBTDRD_RUNTIME, e))?;
    let _ = std::fs::write(dir.join("run-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("run-stderr.txt"), &output.stderr);
    if !output.status.success() {
        return Err(format!(
            "{} run --rm {} {} exited {}: {}",
            RBTDRD_RUNTIME,
            image_ref,
            cmd,
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

// ── Case ─────────────────────────────────────────────────────

fn rbtdrd_build_run_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrd_build_run_lifecycle_impl(ctx, dir))
}

fn rbtdrd_build_run_lifecycle_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let vessel_dir = RBTDRV_BUSYBOX_VESSEL_DIR;
    if !ctx.project_root().join(vessel_dir).is_dir() {
        return rbtdre_Verdict::Fail(format!("vessel directory not found: {}", vessel_dir));
    }

    // Ordain (conjure-mode): build busybox into the standing depot, capturing
    // the hallmark plus the gar_root/ark_stem facts needed to name the
    // locally-pulled image after summon.
    let (hallmark, gar_root, ark_stem) =
        match rbtdri_ordain_capture_full(ctx, dir, vessel_dir, &[], "01-ordain") {
            Ok(facts) => facts,
            Err(v) => return v,
        };

    // The image ref summon pulls locally: <gar_root>/<ark_stem>/image:<hallmark>.
    // Same construction onboarding's conjure verification tail uses.
    let image_ref = rbtdri_gar_ref_fact(&gar_root, &ark_stem, RBTDRV_ARK_BASENAME_IMAGE, &hallmark);
    let _ = std::fs::write(dir.join("02-image-ref.txt"), &image_ref);

    // Summon: retriever pulls the hallmark's arks locally. Confirm the image
    // ark is resolvable before attempting to run it.
    let _ = std::fs::write(dir.join("03-summon.txt"), "summoning");
    if let Err(v) = rbtdri_invoke_or_fail(
        ctx,
        "summon",
        &hallmark,
        RBTDGC_SUMMON_HALLMARK,
        &[&hallmark],
        &[],
        dir,
        "03-summon",
    ) {
        return v;
    }
    if !rbtdrv_docker_inspect(&image_ref) {
        return rbtdre_Verdict::Fail(format!(
            "summon: image ark not local after pull: {}",
            image_ref
        ));
    }

    // Bare run — the executability proof. No crucible: a plain
    // `<runtime> run --rm <ref> true` exiting 0 proves the summoned artifact
    // is runnable.
    let _ = std::fs::write(dir.join("04-run.txt"), "running degenerate command");
    if let Err(e) = rbtdrd_runtime_run(&image_ref, RBTDRD_PROOF_CMD, dir) {
        return rbtdre_Verdict::Fail(format!("bare run executability proof: {}", e));
    }

    // Abjure — remove the hallmark's arks, restoring the standing depot to its
    // pre-run inventory. BURE_CONFIRM skipped for non-interactive teardown.
    let _ = std::fs::write(dir.join("05-abjure.txt"), "abjuring");
    if let Err(v) = rbtdri_invoke_or_fail(
        ctx,
        "abjure",
        &hallmark,
        RBTDGC_ABJURE_HALLMARK,
        &[&hallmark],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
        dir,
        "05-abjure",
    ) {
        return v;
    }

    let _ = std::fs::write(dir.join("06-passed.txt"), "passed");
    rbtdre_Verdict::Pass
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRD_CASES_DOGFIGHT: &[rbtdre_Case] = &[case!(rbtdrd_build_run_lifecycle)];

pub static RBTDRD_FIXTURE_DOGFIGHT: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_DOGFIGHT,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRD_CASES_DOGFIGHT,
    credless: false,
};
