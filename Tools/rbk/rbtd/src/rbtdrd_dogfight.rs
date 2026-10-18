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
// Single case, ordain → summon → resolved-base check → run → abjure, threaded
// through one body. The busybox vessel is consumerless — no nameplate holds its
// hallmark, so there is no committed regime file to carry the ephemeral hallmark
// across a case boundary. The hallmark therefore lives as a local across the
// steps, the same structural choice rbtcgp_hallmark_lifecycle makes for the same
// reason. This fixture IS hallmark_lifecycle with the registry-inventory
// middle (audit/rekon) swapped for summon + a bare container-runtime run.
//
// The resolved-base check reads the rbi_resolved_base_1 label off the summoned
// image and fails loud if it diverges from the vessel's committed base — the
// regression guard for the conjure resolved-base provenance feature
// (RBr_b4e). It reuses this fixture's existing ordain rather than spending a
// second cloud build on a separate case.

use std::path::Path;
use std::process::Command;

use crate::rbtcee_case;
use crate::rbtccc_crucible::rbtccc_with_ctx;
use crate::rbtcgp_patrol::{
    rbtcgp_docker_config_label, rbtcgp_docker_inspect, RBTCGP_ARK_BASENAME_IMAGE,
    RBTCGP_BUSYBOX_VESSEL_DIR,
};
use crate::rbtcee_engine::{rbtcee_Case, rbtcee_Disposition, rbtcee_Fixture, rbtcee_Tariff, rbtcee_Verdict};
use crate::rbtcei_invocation::{
    rbtcei_gar_ref_fact, rbtcei_invoke_or_fail, rbtcei_ordain_capture_full, rbtcei_Context,
    RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP,
};
use crate::rbtdgc_consts::{RBTDGC_ABJURE_HALLMARK, RBTDGC_RBRV_FILE, RBTDGC_SUMMON_HALLMARK};

/// Container runtime for the bare executability proof. Hardcoded to docker;
/// podman is deferred to the Director-governed runtime-regime decision that
/// rides with ₣BS. This single named site is the future swap point — do NOT
/// add a regime field here now.
const RBTCGD_RUNTIME: &str = "docker";

/// Degenerate command proving the summoned image is runnable. busybox's
/// default cmd is `sh`; passing an explicit `true` yields a clean exit-0
/// executability proof without spawning an interactive shell.
const RBTCGD_PROOF_CMD: &str = "true";

// ── Runtime run helper ───────────────────────────────────────

/// Run `<runtime> run --rm <image_ref> <cmd>` and return Ok on exit 0. The
/// single site naming the container runtime — see RBTCGD_RUNTIME.
fn rbtcgd_runtime_run(image_ref: &str, cmd: &str, dir: &Path) -> Result<(), String> {
    let output = Command::new(RBTCGD_RUNTIME)
        .args(["run", "--rm", image_ref, cmd])
        .output()
        .map_err(|e| format!("{} run exec failed: {}", RBTCGD_RUNTIME, e))?;
    let _ = std::fs::write(dir.join("run-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("run-stderr.txt"), &output.stderr);
    if !output.status.success() {
        return Err(format!(
            "{} run --rm {} {} exited {}: {}",
            RBTCGD_RUNTIME,
            image_ref,
            cmd,
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

// ── Committed-config reader ──────────────────────────────────

/// Read one shell-assignment field from a vessel's rbrv.env, value with
/// surrounding double-quotes stripped. The committed-config source the
/// resolved-base divergence check compares the emitted label against.
fn rbtcgd_vessel_field(
    ctx: &rbtcei_Context,
    vessel_dir: &str,
    key: &str,
) -> Result<String, String> {
    let path = ctx.rbtcei_project_root().join(vessel_dir).join(RBTDGC_RBRV_FILE);
    let body = std::fs::read_to_string(&path)
        .map_err(|e| format!("read {}: {}", path.display(), e))?;
    let prefix = format!("{}=", key);
    for line in body.lines() {
        if let Some(value) = line.trim().strip_prefix(&prefix) {
            return Ok(value.trim().trim_matches('"').to_owned());
        }
    }
    Err(format!("{} not found in {}", key, path.display()))
}

// ── Case ─────────────────────────────────────────────────────

fn rbtcgd_build_run_lifecycle(dir: &Path) -> rbtcee_Verdict {
    rbtccc_with_ctx(|ctx| zrbtcgd_build_run_lifecycle_impl(ctx, dir))
}

fn zrbtcgd_build_run_lifecycle_impl(ctx: &mut rbtcei_Context, dir: &Path) -> rbtcee_Verdict {
    let vessel_dir = RBTCGP_BUSYBOX_VESSEL_DIR;
    if !ctx.rbtcei_project_root().join(vessel_dir).is_dir() {
        return rbtcee_Verdict::Fail(format!("vessel directory not found: {}", vessel_dir));
    }

    // Ordain (conjure-mode): build busybox into the standing depot, capturing
    // the hallmark plus the gar_root/ark_stem facts needed to name the
    // locally-pulled image after summon.
    let (hallmark, gar_root, ark_stem) =
        match rbtcei_ordain_capture_full(ctx, dir, vessel_dir, &[], "01-ordain") {
            Ok(facts) => facts,
            Err(v) => return v,
        };

    // The image ref summon pulls locally: <gar_root>/<ark_stem>/image:<hallmark>.
    // Same construction onboarding's conjure verification tail uses.
    let image_ref = rbtcei_gar_ref_fact(&gar_root, &ark_stem, RBTCGP_ARK_BASENAME_IMAGE, &hallmark);
    let _ = std::fs::write(dir.join("02-image-ref.txt"), &image_ref);

    // Summon: retriever pulls the hallmark's arks locally. Confirm the image
    // ark is resolvable before attempting to run it.
    let _ = std::fs::write(dir.join("03-summon.txt"), "summoning");
    if let Err(v) = rbtcei_invoke_or_fail(
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
    if !rbtcgp_docker_inspect(&image_ref) {
        return rbtcee_Verdict::Fail(format!(
            "summon: image ark not local after pull: {}",
            image_ref
        ));
    }

    // Resolved-base provenance regression: the summoned image must carry the
    // rbi_resolved_base_1 label naming the vessel's committed base (busybox is a
    // pass-through, slot-1 vessel), pinned by a well-formed sha256. The label
    // rides the consumer image config byte-identically into the signed attest
    // image (RBr_b4e), so reading it off the locally summoned consumer image is
    // the same value without a registry round-trip.
    let _ = std::fs::write(dir.join("03-resolved-base.txt"), "checking rbi_resolved_base_1");
    let label = match rbtcgp_docker_config_label(&image_ref, "rbi_resolved_base_1") {
        Ok(v) => v,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!(
                "resolved-base: docker inspect label failed: {}",
                e
            ))
        }
    };
    let _ = std::fs::write(dir.join("03-resolved-base-label.txt"), &label);
    if label.is_empty() {
        return rbtcee_Verdict::Fail(format!(
            "resolved-base: rbi_resolved_base_1 absent on {} — write-side (rbgjb03/rbgjb04) regression",
            image_ref
        ));
    }
    let (ref_portion, digest) = match label.split_once("@sha256:") {
        Some(pair) => pair,
        None => {
            return rbtcee_Verdict::Fail(format!(
                "resolved-base: label not @sha256:-pinned: {}",
                label
            ))
        }
    };
    if digest.len() != 64 || !digest.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return rbtcee_Verdict::Fail(format!(
            "resolved-base: malformed sha256 digest in label: {}",
            label
        ));
    }
    // The ref-portion must equal the committed RBRV_IMAGE_1_ORIGIN with its tag
    // stripped — the exact transform rbgjb03 applies (${origin%:*}). A mismatch is
    // a provenance lie: the label names a base the vessel never committed.
    let committed_origin = match rbtcgd_vessel_field(ctx, vessel_dir, "RBRV_IMAGE_1_ORIGIN") {
        Ok(v) => v,
        Err(e) => return rbtcee_Verdict::Fail(format!("resolved-base: {}", e)),
    };
    let expected_ref = committed_origin
        .rsplit_once(':')
        .map(|(base, _)| base)
        .unwrap_or(committed_origin.as_str());
    if ref_portion != expected_ref {
        return rbtcee_Verdict::Fail(format!(
            "resolved-base DIVERGENCE: label base '{}' != committed '{}' (RBRV_IMAGE_1_ORIGIN='{}')",
            ref_portion, expected_ref, committed_origin
        ));
    }

    // Bare run — the executability proof. No crucible: a plain
    // `<runtime> run --rm <ref> true` exiting 0 proves the summoned artifact
    // is runnable.
    let _ = std::fs::write(dir.join("04-run.txt"), "running degenerate command");
    if let Err(e) = rbtcgd_runtime_run(&image_ref, RBTCGD_PROOF_CMD, dir) {
        return rbtcee_Verdict::Fail(format!("bare run executability proof: {}", e));
    }

    // Abjure — remove the hallmark's arks, restoring the standing depot to its
    // pre-run inventory. BURE_CONFIRM skipped for non-interactive teardown.
    let _ = std::fs::write(dir.join("05-abjure.txt"), "abjuring");
    if let Err(v) = rbtcei_invoke_or_fail(
        ctx,
        "abjure",
        &hallmark,
        RBTDGC_ABJURE_HALLMARK,
        &[&hallmark],
        &[(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)],
        dir,
        "05-abjure",
    ) {
        return v;
    }

    let _ = std::fs::write(dir.join("06-passed.txt"), "passed");
    rbtcee_Verdict::Pass
}

// ── Section registry ─────────────────────────────────────────

pub static RBTCGD_CASES_DOGFIGHT: &[rbtcee_Case] = &[rbtcee_case!(rbtcgd_build_run_lifecycle)];

pub static RBTCGD_FIXTURE_DOGFIGHT: rbtcee_Fixture = rbtcee_Fixture {
    name: crate::rbtcem_manifest::RBTCEM_FIXTURE_DOGFIGHT,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCGD_CASES_DOGFIGHT,
    credless: false,
    tariff: rbtcee_Tariff { min_secs: Some(60), max_secs: Some(1800), invocations: None },
};
