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
// RBTDRV — the patrol: bare cloud-service fixtures (no crucible charge/quench).
//
// The credentialed GCP lifecycle fixtures (hallmark, lode, reliquary, wsl,
// podvm, foedus, batch-vouch, access-probe, terrier scaffold/atomicity,
// chaining-livery), each a single case that drives a live cloud lifecycle and
// cleans up after itself. Also homes the shared ark/GAR vocabulary and docker
// inspection helpers consumed here and by rbtdrd_dogfight / rbtdro_onboarding.

use std::path::Path;
use std::process::{Command, Stdio};

use crate::case;
use crate::rbtdrc_crucible::rbtdrc_with_ctx;
use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Disposition, rbtdre_Fixture, rbtdre_Verdict};
use crate::rbtdri_invocation::{
    rbtdri_Context, rbtdri_InvokeResult, rbtdri_gar_ref_categorical, rbtdri_invoke_global,
    rbtdri_invoke_or_fail,
    rbtdri_ordain_capture, rbtdri_read_burv_fact, rbtdri_read_burv_facts_multi,
    RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP,
    RBTDRI_BURE_TWEAK_NAME_KEY, RBTDRI_BURE_TWEAK_VALUE_KEY,
};
use crate::rbtdgc_consts::{
    RBTDGC_ABJURE_HALLMARK, RBTDGC_ACCOUNT_DIRECTOR, RBTDGC_ACCOUNT_GOVERNOR, RBTDGC_ACCOUNT_PAYOR,
    RBTDGC_ACCOUNT_RETRIEVER, RBTDGC_AFFIANCE_MANOR, RBTDGC_AUDIT_HALLMARKS,
    RBTDGC_AUGUR_LODE, RBTDGC_BANISH_LODE, RBTDGC_CHECK_AVOWAL, RBTDGC_CHECK_MANTLE,
    RBTDGC_CHECK_PAYOR, RBTDGC_CONCLAVE_RELIQUARY, RBTDGC_DESCRY_FOEDUS,
    RBTDGC_DIVINE_LODES, RBTDGC_ENSCONCE_BOLE, RBTDGC_FACT_EXT_FOEDUS_HEALTH, RBTDGC_FEOFF_BOLE,
    RBTDGC_IMMURE_PODVM, RBTDGC_INSTATE_FOEDUS,
    RBTDGC_JETTISON_HALLMARK_IMAGE, RBTDGC_JETTISON_IMAGE, RBTDGC_JILT_MANOR, RBTDGC_LIST_IMAGES,
    RBTDGC_RBRR_FILE, RBTDGC_REKON_HALLMARK, RBTDGC_TALLY_HALLMARKS,
    RBTDGC_TERRIER_PROOF, RBTDGC_TERRIER_SCAFFOLD, RBTDGC_TWEAK_REGIME_POISON, RBTDGC_UNDERPIN_WSL,
    RBTDGC_VOUCH_HALLMARKS,
};
use crate::rbtdrm_manifest::rbtdrm_credential_check_colophon;

// ── Bare fixtures owned by rbtdrc (no charge/quench) ─────────

pub static RBTDRV_FIXTURE_HALLMARK_LIFECYCLE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_HALLMARK_LIFECYCLE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_HALLMARK_LIFECYCLE,
    credless: false,
};

pub static RBTDRV_FIXTURE_LODE_LIFECYCLE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_LODE_LIFECYCLE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_LODE_LIFECYCLE,
    credless: false,
};

pub static RBTDRV_FIXTURE_RELIQUARY_LIFECYCLE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_RELIQUARY_LIFECYCLE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_RELIQUARY_LIFECYCLE,
    credless: false,
};

pub static RBTDRV_FIXTURE_WSL_LIFECYCLE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_WSL_LIFECYCLE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_WSL_LIFECYCLE,
    credless: false,
};

pub static RBTDRV_FIXTURE_PODVM_LIFECYCLE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_PODVM_LIFECYCLE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_PODVM_LIFECYCLE,
    credless: false,
};

pub static RBTDRV_FIXTURE_FOEDUS_LIFECYCLE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_FOEDUS_LIFECYCLE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_FOEDUS_LIFECYCLE,
    credless: false,
};

pub static RBTDRV_FIXTURE_FOEDUS_REUSE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_FOEDUS_REUSE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_FOEDUS_REUSE,
    credless: false,
};

pub static RBTDRV_FIXTURE_BATCH_VOUCH: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_BATCH_VOUCH,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_BATCH_VOUCH,
    credless: false,
};

pub static RBTDRV_FIXTURE_ACCESS_PROBE: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_ACCESS_PROBE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_ACCESS_PROBE,
    credless: false,
};

pub static RBTDRV_FIXTURE_TERRIER_SCAFFOLD: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_TERRIER_SCAFFOLD,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_TERRIER_SCAFFOLD,
    credless: false,
};

pub static RBTDRV_FIXTURE_TERRIER_ATOMICITY: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_TERRIER_ATOMICITY,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_TERRIER_ATOMICITY,
    credless: false,
};

// Chaining-fact livery — the cloud sibling of the local chaining-fact band
// matrix. A bare cloud fixture (no crucible): the single case self-contains its
// reset baseline and best-effort cleanup (banish-if-present, body below), so
// setup/teardown stay None — the single-case runner reads a setup hook as
// "crucible fixture, verify it is charged", which this fixture is not.
pub static RBTDRV_FIXTURE_CHAINING_LIVERY: rbtdre_Fixture = rbtdre_Fixture {
    name: crate::rbtdrm_manifest::RBTDRM_FIXTURE_CHAINING_LIVERY,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRV_CASES_CHAINING_LIVERY,
    credless: false,
};

// ── Hallmark / ark vocabulary and docker helpers ─────────────

/// Ark basenames — matching rbgc_constants.sh RBGC_ARK_BASENAME_* values.
pub(crate) const RBTDRV_ARK_BASENAME_IMAGE: &str = "image";
pub(crate) const RBTDRV_ARK_BASENAME_VOUCH: &str = "vouch";
pub(crate) const RBTDRV_ARK_BASENAME_ABOUT: &str = "about";
pub(crate) const RBTDRV_ARK_BASENAME_ATTEST: &str = "attest";
pub(crate) const RBTDRV_ARK_BASENAME_POUCH: &str = "pouch";

/// GAR categorical namespace literal — matches RBGC_GAR_CATEGORY_HALLMARKS.
/// Used to build wrest locators (paths within a GAR repo, prefix-free).
pub(crate) const RBTDRV_GAR_CATEGORY_HALLMARKS: &str = "rbi_hm";

/// Docker wrapper: inspect image (returns true if exists).
pub(crate) fn rbtdrv_docker_inspect(image_ref: &str) -> bool {
    Command::new("docker")
        .args(["inspect", image_ref])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Docker wrapper: remove images.
pub(crate) fn rbtdrv_docker_rmi(refs: &[&str]) -> Result<(), String> {
    let status = Command::new("docker")
        .arg("rmi")
        .args(refs)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("docker rmi exec failed: {}", e))?;
    if !status.success() {
        return Err(format!("docker rmi exited {}", status.code().unwrap_or(-1)));
    }
    Ok(())
}

/// Parse a rekon stdout line for a given basename and return whether the
/// EXISTS column reads "yes". Returns false if the basename row is absent.
/// Rekon prints rows of `  <basename>  <yes|no>  <path-or-(absent)>`.
pub(crate) fn rbtdrv_rekon_basename_yes(stdout: &str, basename: &str) -> bool {
    for line in stdout.lines() {
        let mut fields = line.split_whitespace();
        if fields.next() == Some(basename) {
            return fields.next() == Some("yes");
        }
    }
    false
}

/// Docker wrapper: capture RootFS layer DiffIDs as a JSON array string.
/// Layer DiffIDs are SHA256s of uncompressed layer file content — byte-preserved
/// across registry round-trips even when manifest envelope normalizes (e.g.,
/// multi-arch index → single-platform manifest). Robust round-trip fingerprint.
pub(crate) fn rbtdrv_docker_layers_capture(image_ref: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .args(["inspect", "--format={{json .RootFS.Layers}}", image_ref])
        .output()
        .map_err(|e| format!("docker inspect exec failed: {}", e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(format!(
            "docker inspect {} exited {}: {}",
            image_ref,
            output.status.code().unwrap_or(-1),
            stderr
        ));
    }
    let layers = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if layers.is_empty() || layers == "null" {
        return Err(format!("docker inspect {} returned empty layers", image_ref));
    }
    Ok(layers)
}

/// Docker wrapper: read one image config label's value via inspect's Go-template
/// `index`. Returns the value; an absent key yields an empty string (every
/// conjure image carries hallmark/git.* labels, so `.Config.Labels` is never
/// nil and `index` cannot fault on it). Used to read the rbi_resolved_base_n
/// provenance labels off a summoned consumer image, whose config is
/// byte-identical to the signed attest image's (RBr_b4e).
pub(crate) fn rbtdrv_docker_config_label(image_ref: &str, label_key: &str) -> Result<String, String> {
    let fmt = format!("--format={{{{index .Config.Labels \"{}\"}}}}", label_key);
    let output = Command::new("docker")
        .args(["inspect", &fmt, image_ref])
        .output()
        .map_err(|e| format!("docker inspect exec failed: {}", e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(format!(
            "docker inspect {} exited {}: {}",
            image_ref,
            output.status.code().unwrap_or(-1),
            stderr
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

// Hallmark-lifecycle fixture — round-trip ark inventory across the
// ordain/abjure boundary on a conjure-mode hallmark. Verifies that abjure
// removes every ark basename (image, about, vouch, attest, pouch) without
// collateral damage to other hallmarks in the registry.
//
// Sequence:
//   1. Audit hallmarks → capture baseline.
//   2. Ordain rbev-busybox in conjure mode → capture new hallmark.
//   3. Audit hallmarks → assert baseline ∪ {new_hallmark}.
//   4. Rekon new_hallmark → assert all five basenames yes.
//   5. Abjure new_hallmark.
//   6. Rekon new_hallmark → assert all five basenames not yes.
//   7. Audit hallmarks → assert == baseline (no collateral damage).
//
// rbev-busybox is the load-bearing vessel — small, fast, conjure-mode
// (full ark inventory). Also referenced by rbtdrv_batch_vouch_lifecycle.

pub(crate) const RBTDRV_BUSYBOX_VESSEL_DIR: &str = concat!(crate::rbtd_vessels_dir!(), "/rbev-busybox");

/// All five ark basenames produced by a conjure-mode hallmark.
const ZRBTDRV_ARK_BASENAMES_ALL: &[&str] = &[
    RBTDRV_ARK_BASENAME_IMAGE,
    RBTDRV_ARK_BASENAME_ABOUT,
    RBTDRV_ARK_BASENAME_VOUCH,
    RBTDRV_ARK_BASENAME_ATTEST,
    RBTDRV_ARK_BASENAME_POUCH,
];

/// Multi-fact extension emitted by `rbw-iah` (rbfl_audit_hallmarks): one
/// `<hallmark>.audit-hallmark` file per discovered hallmark. Mirrors
/// rbcc_constants.sh RBCC_fact_ext_audit_hallmark.
const RBTDRV_FACT_EXT_AUDIT_HALLMARK: &str = "audit-hallmark";

/// Single-form chaining fact emitted host-side by `rbw-lE` (rbld_ensconce): the
/// captured Lode touchmark. The derived-pull base-anchor election reads it at
/// conjure; the provenance envelope lives only in GAR (:rbi_vouch), never
/// host-side. Mirrors rbgc_constants.sh RBF_FACT_LODE_TOUCHMARK.
const RBTDRV_FACT_LODE_TOUCHMARK: &str = "rbf_fact_lode_touchmark";

/// Bole-Lode member tags asserted by augur. Mirror rbgc_constants.sh
/// RBGC_LODE_TAG_BOLE / RBGC_LODE_TAG_VOUCH / RBGC_LODE_TAG_DIGEST_PREFIX.
const RBTDRV_LODE_TAG_BOLE: &str = "rbi_bole";
const RBTDRV_LODE_TAG_VOUCH: &str = "rbi_vouch";
const RBTDRV_LODE_TAG_DIGEST_PREFIX: &str = "rbi_sha256-";

/// Envelope-decode markers asserted by augur — values that live *inside* the
/// decoded :rbi_vouch envelope (the trust_grade field and a member's
/// verification field), never in a bare tag listing. Their presence in augur's
/// output is the load-bearing proof that augur decoded the envelope, not merely
/// enumerated tags as divine's retired inspect branch did. Mirror
/// rbgc_constants.sh RBGC_LODE_TRUST_VERIFIED and the rbgjl0* "oci-digest"
/// verification literal.
const RBTDRV_LODE_TRUST_VERIFIED: &str = "verified-against-published";
const RBTDRV_LODE_VERIFICATION_OCI: &str = "oci-digest";

/// Reliquary-Lode member tags asserted by divine inspect — a representative
/// pair of the build-tool cohort (one Google-hosted, one third-party). Compose
/// rbgc_constants.sh RBGC_LODE_TAG_SPRUE with the cohort tool names.
const RBTDRV_RELIQUARY_TAG_GCLOUD: &str = "rbi_gcloud";
const RBTDRV_RELIQUARY_TAG_GCRANE: &str = "rbi_gcrane";

/// GAR Lode package-root — the raw path the type-blind image verbs (rbw-il /
/// rbw-iJ) address a Lode by: rbi_ld/<touchmark>. Mirrors rbgc_constants.sh
/// RBGC_GAR_CATEGORY_LODES.
const RBTDRV_LODES_ROOT: &str = "rbi_ld";

/// Wsl-Lode member tag asserted by divine inspect — the single opaque rootfs
/// blob. Mirrors rbgc_constants.sh RBGC_LODE_TAG_ROOTFS.
const RBTDRV_LODE_TAG_ROOTFS: &str = "rbi_rootfs";

/// Underpin version arguments — the wsl substrate release + point the fixture
/// captures. Declarative version intent (no FQIN); the host assembles the cdimage
/// URL and the cloud step discovers + GPG-verifies the checksum (RBSLU).
const RBTDRV_WSL_RELEASE: &str = "24.04";
const RBTDRV_WSL_POINT: &str = "4";

/// BURE_TWEAK signal recognized by rbld_ensconce (rbldb_bole.sh) to pin the Lode
/// stamp, driving two captures onto one touchmark so the cloud-side collision
/// guard's idempotent/collision branches fire. Mirror: rbldb_bole.sh
/// `z_ensconce_stamp_tweak_name` — same literal. Carries the buo tweak sprue,
/// enforced by BURE.
const RBTDRV_ENSCONCE_STAMP_TWEAK_NAME: &str = "buorb_ensconce_stamp";

/// Debian-base vessel — a DIFFERENT upstream base than busybox, so ensconcing it
/// onto a busybox touchmark trips the collision guard's different-digest branch.
/// Carries the same yoked reliquary as busybox, so host-side tool resolution
/// succeeds and the failure lands cloud-side at the guard, not host-side.
const RBTDRV_DEB_VESSEL_DIR: &str = concat!(crate::rbtd_vessels_dir!(), "/rbev-sentry-deb-tether");

fn rbtdrv_hallmark_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let vessel_dir = RBTDRV_BUSYBOX_VESSEL_DIR;
        if !ctx.project_root().join(vessel_dir).is_dir() {
            return rbtdre_Verdict::Fail(format!("vessel directory not found: {}", vessel_dir));
        }

        // Step 1: baseline audit.
        let _ = std::fs::write(dir.join("01-audit-baseline.txt"), "auditing baseline");
        let baseline_audit = match rbtdri_invoke_global(ctx, RBTDGC_AUDIT_HALLMARKS, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("baseline audit failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("baseline audit invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("01-audit-baseline-stdout.txt"), &baseline_audit.stdout);
        let baseline = match rbtdri_read_burv_facts_multi(&baseline_audit, RBTDRV_FACT_EXT_AUDIT_HALLMARK) {
            Ok(v) => v,
            Err(e) => return rbtdre_Verdict::Fail(format!("read baseline audit facts: {}", e)),
        };
        let _ = std::fs::write(dir.join("01-baseline-parsed.txt"), baseline.join("\n"));

        // Step 2: ordain.
        let hallmark = match rbtdri_ordain_capture(ctx, dir, vessel_dir, &[], "02-ordain") {
            Ok(h) => h,
            Err(v) => return v,
        };

        // Step 3: audit shows new hallmark added.
        let _ = std::fs::write(dir.join("03-audit-after-ordain.txt"), "auditing after ordain");
        let after_ordain_audit = match rbtdri_invoke_global(ctx, RBTDGC_AUDIT_HALLMARKS, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("post-ordain audit failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("post-ordain audit invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("03-audit-after-ordain-stdout.txt"), &after_ordain_audit.stdout);
        let after_ordain = match rbtdri_read_burv_facts_multi(&after_ordain_audit, RBTDRV_FACT_EXT_AUDIT_HALLMARK) {
            Ok(v) => v,
            Err(e) => return rbtdre_Verdict::Fail(format!("read post-ordain audit facts: {}", e)),
        };
        let mut expected_after_ordain = baseline.clone();
        expected_after_ordain.push(hallmark.clone());
        expected_after_ordain.sort();
        if after_ordain != expected_after_ordain {
            return rbtdre_Verdict::Fail(format!(
                "post-ordain audit mismatch:\n  expected (baseline + new): {:?}\n  got: {:?}",
                expected_after_ordain, after_ordain
            ));
        }

        // Step 4: rekon shows all five ark basenames present.
        let _ = std::fs::write(dir.join("04-rekon-after-ordain.txt"), "rekoning after ordain");
        let rekon_after_ordain = match rbtdri_invoke_global(ctx, RBTDGC_REKON_HALLMARK, &[&hallmark], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("post-ordain rekon failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("post-ordain rekon invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("04-rekon-after-ordain-stdout.txt"), &rekon_after_ordain.stdout);
        for basename in ZRBTDRV_ARK_BASENAMES_ALL {
            if !rbtdrv_rekon_basename_yes(&rekon_after_ordain.stdout, basename) {
                return rbtdre_Verdict::Fail(format!(
                    "post-ordain rekon: basename '{}' not marked yes\nstdout:\n{}",
                    basename, rekon_after_ordain.stdout
                ));
            }
        }

        // Step 5: abjure.
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

        // Step 6: rekon for the abjured hallmark must exit non-zero — the
        // Unix exit contract is the assertion (rekon's display text is not
        // normative). Stdout captured for diagnostic value only; never read
        // for assertions.
        let _ = std::fs::write(dir.join("06-rekon-after-abjure.txt"), "rekoning after abjure");
        match rbtdri_invoke_global(ctx, RBTDGC_REKON_HALLMARK, &[&hallmark], &[]) {
            Ok(r) if r.exit_code != 0 => {
                let _ = std::fs::write(dir.join("06-rekon-after-abjure-stdout.txt"), &r.stdout);
            }
            Ok(r) => {
                let _ = std::fs::write(dir.join("06-rekon-after-abjure-stdout.txt"), &r.stdout);
                return rbtdre_Verdict::Fail(format!(
                    "post-abjure rekon: expected non-zero exit, got success (exit 0)\nstdout:\n{}",
                    r.stdout
                ));
            }
            Err(e) => return rbtdre_Verdict::Fail(format!("post-abjure rekon invocation: {}", e)),
        }

        // Step 7: final audit — registry restored to baseline.
        let _ = std::fs::write(dir.join("07-audit-final.txt"), "auditing final");
        let final_audit = match rbtdri_invoke_global(ctx, RBTDGC_AUDIT_HALLMARKS, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("final audit failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("final audit invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("07-audit-final-stdout.txt"), &final_audit.stdout);
        let final_state = match rbtdri_read_burv_facts_multi(&final_audit, RBTDRV_FACT_EXT_AUDIT_HALLMARK) {
            Ok(v) => v,
            Err(e) => return rbtdre_Verdict::Fail(format!("read final audit facts: {}", e)),
        };
        if final_state != baseline {
            return rbtdre_Verdict::Fail(format!(
                "final audit mismatch — abjure did not restore baseline:\n  baseline: {:?}\n  final: {:?}",
                baseline, final_state
            ));
        }

        let _ = std::fs::write(dir.join("08-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_HALLMARK_LIFECYCLE: &[rbtdre_Case] = &[case!(rbtdrv_hallmark_lifecycle)];


// ── Lode round-trip shared blocks ────────────────────────────
// The four Lode round-trip fixtures (lode/reliquary/wsl/podvm-lifecycle) share a
// byte-near-identical capture -> read-touchmark -> divine-contains -> augur ->
// [member-jettison] -> banish -> final-divine skeleton. These helpers home the
// three four-site invariant blocks plus the two-site member-jettison block. The
// six load-bearing per-kind differences stay inline at the call sites by design:
// the capture verb+args, the augur member-tag sets, the trust grade, lode's
// literal-HEAD-commit envelope assertion, podvm's refresh+cohort-count
// sub-sequence and trust-posture prose, and the jettison step's reliquary+podvm-
// only presence.

/// Read the bare Lode touchmark fact from a capture invocation and stamp it to
/// the case scratch dir. The host-side capture handoff is identical across every
/// Lode kind; only the capture result differs. Ok(touchmark) to continue,
/// Err(Fail) to short-circuit on a missing/empty fact.
fn zrbtdrv_read_touchmark(
    result: &rbtdri_InvokeResult,
    dir: &Path,
) -> Result<String, rbtdre_Verdict> {
    let touchmark = rbtdri_read_burv_fact(result, RBTDRV_FACT_LODE_TOUCHMARK)
        .map_err(|e| rbtdre_Verdict::Fail(format!("read touchmark fact: {}", e)))?;
    let _ = std::fs::write(dir.join("02-touchmark.txt"), &touchmark);
    Ok(touchmark)
}

/// Divine-enumerate the Lodes and confirm the just-captured touchmark appears.
/// `verb_label` preserves the per-kind Fail-message diagnostic (ensconce /
/// conclave / underpin / immure). Returns the divine stdout on success so a kind
/// can layer extra inline assertions (podvm's cohort-count) on the same output.
fn zrbtdrv_divine_contains(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    touchmark: &str,
    verb_label: &str,
) -> Result<String, rbtdre_Verdict> {
    let after = match rbtdri_invoke_global(ctx, RBTDGC_DIVINE_LODES, &[], &[]) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return Err(rbtdre_Verdict::Fail(format!("post-{} divine failed (exit {})\n{}", verb_label, r.exit_code, r.stderr))),
        Err(e) => return Err(rbtdre_Verdict::Fail(format!("post-{} divine invocation: {}", verb_label, e))),
    };
    let _ = std::fs::write(dir.join("03-divine-after.txt"), &after.stdout);
    if !after.stdout.contains(touchmark) {
        return Err(rbtdre_Verdict::Fail(format!(
            "post-{} divine missing touchmark {}\nstdout:\n{}",
            verb_label, touchmark, after.stdout
        )));
    }
    Ok(after.stdout)
}

/// Banish the whole Lode (confirm-skip) and confirm a final divine no longer
/// shows the touchmark — the registry-restored bookend, byte-identical across
/// every Lode kind. Some(Fail) to short-circuit, None to continue.
fn zrbtdrv_banish_and_verify_gone(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    touchmark: &str,
) -> Option<rbtdre_Verdict> {
    let _ = std::fs::write(dir.join("05-banish.txt"), "banishing");
    match rbtdri_invoke_global(
        ctx,
        RBTDGC_BANISH_LODE,
        &[touchmark],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
    ) {
        Ok(r) if r.exit_code == 0 => {}
        Ok(r) => return Some(rbtdre_Verdict::Fail(format!("banish failed (exit {})\n{}", r.exit_code, r.stderr))),
        Err(e) => return Some(rbtdre_Verdict::Fail(format!("banish invocation: {}", e))),
    }
    let final_divine = match rbtdri_invoke_global(ctx, RBTDGC_DIVINE_LODES, &[], &[]) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return Some(rbtdre_Verdict::Fail(format!("final divine failed (exit {})\n{}", r.exit_code, r.stderr))),
        Err(e) => return Some(rbtdre_Verdict::Fail(format!("final divine invocation: {}", e))),
    };
    let _ = std::fs::write(dir.join("06-divine-final.txt"), &final_divine.stdout);
    if final_divine.stdout.contains(touchmark) {
        return Some(rbtdre_Verdict::Fail(format!(
            "final divine still shows banished touchmark {} — banish did not restore baseline\nstdout:\n{}",
            touchmark, final_divine.stdout
        )));
    }
    None
}

/// Member-grain jettison proof for the multi-member Lode kinds (reliquary +
/// podvm): raw-list the cohort and assert both tags present, jettison the victim
/// tag via the type-blind raw verb, then re-list and assert the victim gone while
/// the survivor remains. Emits the 04b/04c/04d scratch files. Some(Fail) to
/// short-circuit, None to continue.
fn zrbtdrv_member_jettison_proof(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    touchmark: &str,
    victim_tag: &str,
    survivor_tag: &str,
) -> Option<rbtdre_Verdict> {
    let lode_path = format!("{}/{}", RBTDRV_LODES_ROOT, touchmark);
    let member_ref = format!("{}:{}", lode_path, victim_tag);

    let pre_list = match rbtdri_invoke_global(ctx, RBTDGC_LIST_IMAGES, &[&lode_path], &[]) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return Some(rbtdre_Verdict::Fail(format!("pre-jettison list failed (exit {})\n{}", r.exit_code, r.stderr))),
        Err(e) => return Some(rbtdre_Verdict::Fail(format!("pre-jettison list invocation: {}", e))),
    };
    let _ = std::fs::write(dir.join("04b-list-before-jettison.txt"), &pre_list.stdout);
    for member in &[survivor_tag, victim_tag] {
        if !pre_list.stdout.contains(member) {
            return Some(rbtdre_Verdict::Fail(format!(
                "pre-jettison raw list missing member tag '{}'\nstdout:\n{}",
                member, pre_list.stdout
            )));
        }
    }

    let _ = std::fs::write(dir.join("04c-jettison-member.txt"), &member_ref);
    match rbtdri_invoke_global(
        ctx,
        RBTDGC_JETTISON_IMAGE,
        &[&member_ref],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
    ) {
        Ok(r) if r.exit_code == 0 => {}
        Ok(r) => return Some(rbtdre_Verdict::Fail(format!("member jettison failed (exit {})\n{}", r.exit_code, r.stderr))),
        Err(e) => return Some(rbtdre_Verdict::Fail(format!("member jettison invocation: {}", e))),
    }

    let post_list = match rbtdri_invoke_global(ctx, RBTDGC_LIST_IMAGES, &[&lode_path], &[]) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return Some(rbtdre_Verdict::Fail(format!("post-jettison list failed (exit {})\n{}", r.exit_code, r.stderr))),
        Err(e) => return Some(rbtdre_Verdict::Fail(format!("post-jettison list invocation: {}", e))),
    };
    let _ = std::fs::write(dir.join("04d-list-after-jettison.txt"), &post_list.stdout);
    if post_list.stdout.contains(victim_tag) {
        return Some(rbtdre_Verdict::Fail(format!(
            "post-jettison list still shows jettisoned member '{}' — member-grain delete failed\nstdout:\n{}",
            victim_tag, post_list.stdout
        )));
    }
    if !post_list.stdout.contains(survivor_tag) {
        return Some(rbtdre_Verdict::Fail(format!(
            "post-jettison list missing sibling member '{}' — jettison damaged the Lode\nstdout:\n{}",
            survivor_tag, post_list.stdout
        )));
    }
    None
}

// Lode-lifecycle fixture — fetched-side base capture against live GAR. Single
// self-contained round-trip: ensconce the busybox base into a fresh rbi_ld
// Lode, divine-enumerate to confirm it appears, augur to confirm the member
// tags AND the decoded :rbi_vouch envelope rode in, banish the whole Lode, then
// divine-enumerate to confirm the registry is restored. Parallel to hallmark-lifecycle
// on the made side. Requires a reliquary yoked on the busybox vessel (same
// precondition hallmark-lifecycle's ordain carries).
fn rbtdrv_lode_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let vessel_dir = RBTDRV_BUSYBOX_VESSEL_DIR;
        if !ctx.project_root().join(vessel_dir).is_dir() {
            return rbtdre_Verdict::Fail(format!("vessel directory not found: {}", vessel_dir));
        }

        // Step 1: ensconce the busybox base into a fresh Lode.
        let _ = std::fs::write(dir.join("01-ensconce.txt"), "ensconcing busybox base");
        let ensconce = match rbtdri_invoke_global(ctx, RBTDGC_ENSCONCE_BOLE, &[vessel_dir], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("ensconce failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("ensconce invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("01-ensconce-stdout.txt"), &ensconce.stdout);

        // The host-side capture handoff is the bare touchmark fact.
        let touchmark = match zrbtdrv_read_touchmark(&ensconce, dir) {
            Ok(t) => t,
            Err(v) => return v,
        };

        // Step 2: divine enumerate shows the new Lode.
        if let Err(v) = zrbtdrv_divine_contains(ctx, dir, &touchmark, "ensconce") {
            return v;
        }

        // Step 3: augur inspects the single Lode — member tags AND the decoded
        // :rbi_vouch envelope. This is the explicit augur-decode case: beyond the
        // member tags (which the retired divine inspect branch also listed), it
        // asserts the envelope's own fields surfaced — the trust grade and a
        // member's verification — proving augur read vouch.json, not merely
        // enumerated tags.
        let augur = match rbtdri_invoke_global(ctx, RBTDGC_AUGUR_LODE, &[&touchmark], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("augur failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("augur invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("04-augur.txt"), &augur.stdout);
        for member in &[RBTDRV_LODE_TAG_BOLE, RBTDRV_LODE_TAG_VOUCH, RBTDRV_LODE_TAG_DIGEST_PREFIX] {
            if !augur.stdout.contains(member) {
                return rbtdre_Verdict::Fail(format!(
                    "augur missing member tag '{}'\nstdout:\n{}",
                    member, augur.stdout
                ));
            }
        }
        // Envelope-decode assertions — the new logic. These markers live inside
        // vouch.json (trust_grade, a member's verification), never in a tag list.
        for field in &[RBTDRV_LODE_TRUST_VERIFIED, RBTDRV_LODE_VERIFICATION_OCI] {
            if !augur.stdout.contains(field) {
                return rbtdre_Verdict::Fail(format!(
                    "augur did not decode :rbi_vouch envelope — missing '{}'\nstdout:\n{}",
                    field, augur.stdout
                ));
            }
        }
        // The envelope also carries the dispatching HEAD commit (rblv_git_commit,
        // spine-injected substitution spliced at the shared vouch-push step).
        // Assert the literal hash, not mere field presence — proving the value
        // survived host -> substitution -> splice -> GAR -> augur decode. HEAD
        // cannot have moved since dispatch: ensconce gated a clean tree and this
        // fixture commits nothing.
        let head = match Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(ctx.project_root())
            .output()
        {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout).trim().to_string()
            }
            Ok(out) => {
                return rbtdre_Verdict::Fail(format!(
                    "git rev-parse HEAD failed (exit {}): {}",
                    out.status.code().unwrap_or(-1),
                    String::from_utf8_lossy(&out.stderr).trim()
                ))
            }
            Err(e) => return rbtdre_Verdict::Fail(format!("git rev-parse invocation: {}", e)),
        };
        if !augur.stdout.contains(&head) {
            return rbtdre_Verdict::Fail(format!(
                "augur envelope missing dispatching commit {}\nstdout:\n{}",
                head, augur.stdout
            ));
        }

        // Step 4: banish the whole Lode, then confirm the registry is restored.
        if let Some(v) = zrbtdrv_banish_and_verify_gone(ctx, dir, &touchmark) {
            return v;
        }

        let _ = std::fs::write(dir.join("07-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

// Lode-collision case — exercises the cloud-side touchmark collision guard
// (rbgjl01-ensconce-capture.sh). The guard cannot fire under natural minting:
// each ensconce mints a fresh second-grained stamp, so two CLI captures land on
// distinct touchmarks. We pin the stamp via the buo tweak channel
// (RBTDRV_ENSCONCE_STAMP_TWEAK_NAME) to drive both captures onto ONE touchmark.
//
// Sequence: (1) ensconce busybox naturally -> mint touchmark S, read it back;
// (2) ensconce busybox pinned to S -> identical digest, guard's idempotent
// branch, exit 0; (3) ensconce debian pinned to S -> different digest under the
// same touchmark, guard's collision branch, host exit non-zero; (4) banish S.
//
// The collision verdict rests on the HOST EXIT CODE: the guard's "touchmark
// collision" message lands in Cloud Logging (CLOUD_LOGGING_ONLY), not host
// stdout, but a cloud build FAILURE propagates to a non-zero rbw-lE exit
// (rbfcb_host.sh: status != SUCCESS -> buc_die). The idempotent step (2) is
// the positive control: the identical pipeline on the same pinned touchmark
// SUCCEEDS for the same base, so step (3)'s failure isolates to the differing
// digest — the collision branch — not debian-specific infra. Both vessels carry
// the same yoked reliquary, so host-side tool resolution is identical.
fn rbtdrv_lode_collision(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let busybox_dir = RBTDRV_BUSYBOX_VESSEL_DIR;
        let deb_dir = RBTDRV_DEB_VESSEL_DIR;
        for vd in &[busybox_dir, deb_dir] {
            if !ctx.project_root().join(vd).is_dir() {
                return rbtdre_Verdict::Fail(format!("vessel directory not found: {}", vd));
            }
        }

        // Step 1: ensconce busybox naturally; read back the minted touchmark S.
        let _ = std::fs::write(dir.join("01-ensconce-fresh.txt"), "ensconcing busybox (fresh, natural mint)");
        let fresh = match rbtdri_invoke_global(ctx, RBTDGC_ENSCONCE_BOLE, &[busybox_dir], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("fresh ensconce failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("fresh ensconce invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("01-ensconce-fresh-stdout.txt"), &fresh.stdout);
        let touchmark = match rbtdri_read_burv_fact(&fresh, RBTDRV_FACT_LODE_TOUCHMARK) {
            Ok(v) => v,
            Err(e) => return rbtdre_Verdict::Fail(format!("read touchmark fact: {}", e)),
        };
        let _ = std::fs::write(dir.join("02-touchmark.txt"), &touchmark);

        let pin = &[
            ("BURE_TWEAK_NAME", RBTDRV_ENSCONCE_STAMP_TWEAK_NAME),
            ("BURE_TWEAK_VALUE", touchmark.as_str()),
        ];

        // Step 2 (positive control): ensconce busybox pinned to S — identical
        // digest under the same touchmark — guard's idempotent branch — must PASS.
        let _ = std::fs::write(dir.join("03-ensconce-idempotent.txt"), "ensconcing busybox pinned to S (identical digest)");
        let idem = match rbtdri_invoke_global(ctx, RBTDGC_ENSCONCE_BOLE, &[busybox_dir], pin) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!(
                "idempotent ensconce (same base, same touchmark) should pass but failed (exit {})\n{}",
                r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("idempotent ensconce invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("03-ensconce-idempotent-stdout.txt"), &idem.stdout);

        // Step 3: ensconce debian pinned to S — different digest under the same
        // touchmark — guard's collision branch — host exit must be non-zero.
        let _ = std::fs::write(dir.join("04-ensconce-collision.txt"), "ensconcing debian pinned to S (different digest -> collision)");
        let collision = match rbtdri_invoke_global(ctx, RBTDGC_ENSCONCE_BOLE, &[deb_dir], pin) {
            Ok(r) => r,
            Err(e) => return rbtdre_Verdict::Fail(format!("collision ensconce invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("04-ensconce-collision-stdout.txt"), &collision.stdout);
        let _ = std::fs::write(dir.join("04-ensconce-collision-stderr.txt"), &collision.stderr);
        if collision.exit_code == 0 {
            return rbtdre_Verdict::Fail(format!(
                "collision ensconce (different base, same touchmark {}) should fail loud but exited 0\nstdout:\n{}",
                touchmark, collision.stdout));
        }

        // Step 4: banish S — cleanup (removes the busybox Lode steps 1-2 left;
        // the collision step wrote nothing, dying before the GAR copy).
        let _ = std::fs::write(dir.join("05-banish.txt"), "banishing");
        match rbtdri_invoke_global(
            ctx,
            RBTDGC_BANISH_LODE,
            &[&touchmark],
            &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
        ) {
            Ok(r) if r.exit_code == 0 => {}
            Ok(r) => return rbtdre_Verdict::Fail(format!("banish failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("banish invocation: {}", e)),
        }

        // Step 5: divine enumerate no longer shows S — registry restored.
        let final_divine = match rbtdri_invoke_global(ctx, RBTDGC_DIVINE_LODES, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("final divine failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("final divine invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("06-divine-final.txt"), &final_divine.stdout);
        if final_divine.stdout.contains(&touchmark) {
            return rbtdre_Verdict::Fail(format!(
                "final divine still shows banished touchmark {} — cleanup failed\nstdout:\n{}",
                touchmark, final_divine.stdout));
        }

        let _ = std::fs::write(dir.join("07-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_LODE_LIFECYCLE: &[rbtdre_Case] = &[
    case!(rbtdrv_lode_lifecycle),
    case!(rbtdrv_lode_collision),
];


// Chaining-fact livery fixture — the cloud sibling of the local chaining-fact
// band matrix (rbtdrh_chain.rs). That matrix proves the chain LINKS' rejection
// bands by hand-SEEDING a synthetic touchmark into previous/; it can only
// simulate the producer. This fixture proves the GENUINE producer->consumer
// succession end-to-end: a real bole ensconce captures a base into live GAR and
// hands its touchmark forward as a chaining fact, chain_next_invoke wires that
// capture's BURV root into the following feoff, and the real feoff reads the
// chained fact from previous/ and elects the base anchor. It catches drift the
// synthetic matrix cannot — between what a live ensconce WRITES to current/ and
// what a live feoff READS from previous/.
//
// Distinct from onboarding-sequence's tracked-vessel ensconce->feoff (the same
// chain against a committed forge vessel, gauntlet-tier): this rides PICKET tier
// and feoffs a STAGED TEMP vessel resolved by path, touching no tracked config
// and committing nothing (band-matrix discipline, rbtdrh_chain.rs the model).
// feoff itself makes no GAR call — it composes the locator from the decoded
// touchmark (RBSDF) — so the live ensconce is what makes the chained touchmark
// real; the registry confirmation is conjure's at a later build, not feoff's.
//
// The touchmark is pinned to a fixed bole-shaped value via the ensconce-stamp
// tweak (the lode-collision precedent), giving the reset a stable banish handle:
// a prior crashed run leaves a Lode at exactly this touchmark, which would trip
// the cloud collision guard on the next ensconce. The case OPENS by re-
// establishing the absent-Lode baseline (load-bearing) and CLOSES by banishing
// best-effort regardless of verdict, so a mid-case failure still cleans up and a
// crash is recovered by the next run's opening reset. Both go through one
// divine-then-banish helper, because banish dies on an absent Lode (rbld_banish:
// "nothing to banish"). The reset lives in the case body, not a setup hook: the
// single-case runner reserves a setup hook for crucible charge, and this fixture
// charges no crucible.

/// Fixed bole touchmark this fixture pins via the ensconce-stamp tweak. The shape
/// is the band matrix's synthetic bole-seed shape — RBGC_LODE_KIND_BOLE 'b' + a
/// 12-digit YYMMDDHHMMSS stamp (cf. rbtdrh_chain.rs RBTDRH_BOLE_TOUCHMARK) — but
/// a deterministic value rather than a clock mint, so setup has a stable handle
/// and the elected anchor is predictable. ensconce takes BURE_TWEAK_VALUE
/// verbatim as the stamp (rbldb_bole.sh), so this becomes the captured Lode's
/// touchmark and the chained fact the feoff consumes.
const RBTDRV_LIVERY_TOUCHMARK: &str = "b260623000000";

/// The staged temp vessel's rbrv.env — one populated base ORIGIN slot, which is
/// all feoff needs to locate the slot whose ANCHOR it elects, and NO ANCHOR line,
/// so an RBRV_IMAGE_1_ANCHOR= present after feoff proves the write fired (a no-op
/// would leave the file ORIGIN-only). feoff never reads this origin's value;
/// busybox is the real yoked vessel the live ensconce actually captures from.
const RBTDRV_LIVERY_VESSEL_RBRV: &str = "RBRV_IMAGE_1_ORIGIN=docker.io/library/debian:bookworm\n";

/// Reset the fixture's pinned Lode to absent (idempotent). divine-then-banish:
/// banish dies on an absent Lode, so probe presence first and banish only when
/// the pinned touchmark is live. Shared by setup (load-bearing baseline) and
/// teardown (best-effort cleanup).
fn zrbtdrv_chaining_livery_reset(ctx: &mut rbtdri_Context) -> Result<(), String> {
    let divine = rbtdri_invoke_global(ctx, RBTDGC_DIVINE_LODES, &[], &[])
        .map_err(|e| format!("livery reset divine invocation: {}", e))?;
    if divine.exit_code != 0 {
        return Err(format!(
            "livery reset divine failed (exit {})\n{}",
            divine.exit_code, divine.stderr
        ));
    }
    if !divine.stdout.contains(RBTDRV_LIVERY_TOUCHMARK) {
        // Pinned Lode already absent — clean baseline, nothing to banish.
        return Ok(());
    }
    match rbtdri_invoke_global(
        ctx,
        RBTDGC_BANISH_LODE,
        &[RBTDRV_LIVERY_TOUCHMARK],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
    ) {
        Ok(r) if r.exit_code == 0 => Ok(()),
        Ok(r) => Err(format!(
            "livery reset banish failed (exit {})\n{}",
            r.exit_code, r.stderr
        )),
        Err(e) => Err(format!("livery reset banish invocation: {}", e)),
    }
}

/// The case: OPEN by re-establishing the absent-Lode baseline (load-bearing —
/// the cloud collision guard would trip on a leaked prior Lode at the pinned
/// touchmark), run the real ensconce->chain->feoff succession, then CLOSE by
/// banishing best-effort regardless of verdict so a mid-case failure still
/// cleans up. The reset is in the body, not a setup hook, because the single-case
/// runner reserves a setup hook for crucible charge and this fixture charges none.
fn rbtdrv_chaining_livery(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        if let Err(e) = zrbtdrv_chaining_livery_reset(ctx) {
            return rbtdre_Verdict::Fail(format!("baseline reset (banish-if-present): {}", e));
        }
        let verdict = zrbtdrv_chaining_livery_body(ctx, dir);
        // Best-effort cleanup, regardless of verdict — banish the Lode the body
        // captured. A crash that skips this is recovered by the next run's opening
        // reset (the pinned touchmark is the stable handle).
        if let Err(e) = zrbtdrv_chaining_livery_reset(ctx) {
            crate::rbtdrg_error_now!("chaining-livery cleanup banish: {}", e);
        }
        verdict
    })
}

/// The real producer->consumer succession, lifted out of the case wrapper so the
/// reset/cleanup bookend can frame it. Takes ctx directly (the wrapper already
/// holds the thread-local borrow — a nested rbtdrc_with_ctx would double-borrow).
fn zrbtdrv_chaining_livery_body(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let busybox_dir = RBTDRV_BUSYBOX_VESSEL_DIR;
    if !ctx.project_root().join(busybox_dir).is_dir() {
        return rbtdre_Verdict::Fail(format!("vessel directory not found: {}", busybox_dir));
    }

    // Step 1: real bole ensconce of the busybox base, pinned to the fixed
    // touchmark via the ensconce-stamp tweak. ensconce captures into live GAR
    // and emits the touchmark chaining fact to current/ — the real producer.
    let _ = std::fs::write(dir.join("01-ensconce.txt"), "ensconcing busybox base, pinned");
    let pin = &[
        (RBTDRI_BURE_TWEAK_NAME_KEY, RBTDRV_ENSCONCE_STAMP_TWEAK_NAME),
        (RBTDRI_BURE_TWEAK_VALUE_KEY, RBTDRV_LIVERY_TOUCHMARK),
    ];
    let ensconce = match rbtdri_invoke_global(ctx, RBTDGC_ENSCONCE_BOLE, &[busybox_dir], pin) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return rbtdre_Verdict::Fail(format!("ensconce failed (exit {})\n{}", r.exit_code, r.stderr)),
        Err(e) => return rbtdre_Verdict::Fail(format!("ensconce invocation: {}", e)),
    };
    let _ = std::fs::write(dir.join("01-ensconce-stdout.txt"), &ensconce.stdout);

    // The producer's handoff is the bare touchmark fact in current/. Read it
    // now, BEFORE the chained feoff promotes current/ into previous/ (where
    // feoff reads it but this read no longer would).
    let touchmark = match rbtdri_read_burv_fact(&ensconce, RBTDRV_FACT_LODE_TOUCHMARK) {
        Ok(v) => v,
        Err(e) => return rbtdre_Verdict::Fail(format!("read touchmark fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("02-touchmark.txt"), &touchmark);

    // The real touchmark must be the pinned value (ensconce honored the stamp
    // and round-tripped it through the fact) AND carry the band matrix's
    // synthetic bole-seed shape: 'b' + 12 digits (cf. rbtdrh_chain.rs
    // RBTDRH_BOLE_TOUCHMARK) — the proof that synthetic seed is faithful to
    // what a live ensconce emits and a live feoff consumes.
    if touchmark != RBTDRV_LIVERY_TOUCHMARK {
        return rbtdre_Verdict::Fail(format!(
            "ensconce emitted touchmark '{}', expected the pinned '{}' (stamp tweak not honored?)",
            touchmark, RBTDRV_LIVERY_TOUCHMARK
        ));
    }
    let shape_ok = touchmark.len() == 13
        && touchmark.starts_with('b')
        && touchmark[1..].chars().all(|c| c.is_ascii_digit());
    if !shape_ok {
        return rbtdre_Verdict::Fail(format!(
            "real ensconce touchmark '{}' is not the bole-seed shape ('b' + 12 digits)",
            touchmark
        ));
    }

    // Step 2: stage a temp vessel and chain feoff off the ensconce. The chain
    // makes feoff reuse the ensconce's BURV root, so bud promotes the touchmark
    // from current/ into feoff's previous/ — the operator's shared ../output-buk
    // flow, restored for exactly this pair (rbtdri chain_next). feoff resolves
    // the vessel by PATH, so its rbrv.env rewrite lands in the case temp dir — no
    // tracked config is touched. No express touchmark is passed, so feoff MUST
    // take the value from the chain or die loud.
    let vessel_dir = dir.join("vessel");
    if let Err(e) = std::fs::create_dir_all(&vessel_dir) {
        return rbtdre_Verdict::Fail(format!("stage vessel dir: {}", e));
    }
    let rbrv = vessel_dir.join("rbrv.env");
    if let Err(e) = std::fs::write(&rbrv, RBTDRV_LIVERY_VESSEL_RBRV) {
        return rbtdre_Verdict::Fail(format!("stage rbrv.env: {}", e));
    }
    let vessel_posix = crate::rbtdrx_platform::rbtdrx_native_to_posix(&vessel_dir);

    ctx.chain_next_invoke();
    let _ = std::fs::write(dir.join("03-feoff.txt"), "feoffing temp vessel off chained touchmark");
    let feoff = match rbtdri_invoke_global(
        ctx,
        RBTDGC_FEOFF_BOLE,
        &[vessel_posix.as_str()],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
    ) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return rbtdre_Verdict::Fail(format!(
            "feoff failed (exit {}) — the ensconce->feoff chain may not have carried the touchmark\n{}",
            r.exit_code, r.stderr
        )),
        Err(e) => return rbtdre_Verdict::Fail(format!("feoff invocation: {}", e)),
    };
    let _ = std::fs::write(dir.join("03-feoff-stdout.txt"), &feoff.stdout);

    // Step 3: the temp vessel's elected anchor must bear the REAL chained
    // touchmark's bole locator. The staged rbrv.env carried no ANCHOR line, so
    // an RBRV_IMAGE_1_ANCHOR= bearing '<touchmark>:rbi_bole' proves both that
    // feoff wrote (no no-op) and that it elected the touchmark the live ensconce
    // handed forward through the chain. Read the config file, never a printout
    // scrape.
    let content = std::fs::read_to_string(&rbrv).unwrap_or_default();
    let _ = std::fs::write(dir.join("04-vessel-rbrv.env"), &content);
    let bole_locator = format!("{}:{}", touchmark, RBTDRV_LODE_TAG_BOLE);
    if content.contains("RBRV_IMAGE_1_ANCHOR=") && content.contains(&bole_locator) {
        let _ = std::fs::write(dir.join("05-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "feoff did not elect the chained bole locator '{}' into the temp vessel; rbrv.env:\n{}",
            bole_locator, content
        ))
    }
}

pub static RBTDRV_CASES_CHAINING_LIVERY: &[rbtdre_Case] = &[case!(rbtdrv_chaining_livery)];


// Reliquary-lifecycle fixture — fetched-side cohort capture against live GAR.
// Single self-contained round-trip: conclave the build-tool cohort into a fresh
// rbi_ld Lode, divine-enumerate to confirm it appears, divine-inspect to confirm
// the member tags + vouch envelope rode in, member-grain jettison one member tag
// via the type-blind raw verbs (rbw-il enumerate, rbw-iJ delete) and confirm the
// member is gone while a sibling survives, banish the whole Lode, then divine-
// enumerate to confirm the registry is restored. The reliquary kind's N-member
// cohort analogue of lode-lifecycle's single-image bole round-trip, and the home
// of the per-member-delete assertion the multi-member kinds need (the podvm
// fixture builds on it). Conclave captures a fixed tool cohort, so it needs no
// vessel precondition.
fn rbtdrv_reliquary_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // Step 1: conclave the build-tool cohort into a fresh Lode.
        let _ = std::fs::write(dir.join("01-conclave.txt"), "conclaving build-tool cohort");
        let conclave = match rbtdri_invoke_global(ctx, RBTDGC_CONCLAVE_RELIQUARY, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("conclave failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("conclave invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("01-conclave-stdout.txt"), &conclave.stdout);

        // The host-side capture handoff is the bare touchmark fact.
        let touchmark = match zrbtdrv_read_touchmark(&conclave, dir) {
            Ok(t) => t,
            Err(v) => return v,
        };

        // Step 2: divine enumerate shows the new Lode.
        if let Err(v) = zrbtdrv_divine_contains(ctx, dir, &touchmark, "conclave") {
            return v;
        }

        // Step 3: augur inspects the cohort Lode — member tags AND the decoded
        // :rbi_vouch envelope. The trust-grade assertion proves augur decodes the
        // N-member (cardinality-N) envelope, not just the bole singleton's.
        let augur = match rbtdri_invoke_global(ctx, RBTDGC_AUGUR_LODE, &[&touchmark], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("augur failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("augur invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("04-augur.txt"), &augur.stdout);
        for member in &[RBTDRV_RELIQUARY_TAG_GCLOUD, RBTDRV_RELIQUARY_TAG_GCRANE, RBTDRV_LODE_TAG_VOUCH] {
            if !augur.stdout.contains(member) {
                return rbtdre_Verdict::Fail(format!(
                    "augur missing member tag '{}'\nstdout:\n{}",
                    member, augur.stdout
                ));
            }
        }
        if !augur.stdout.contains(RBTDRV_LODE_TRUST_VERIFIED) {
            return rbtdre_Verdict::Fail(format!(
                "augur did not decode cohort :rbi_vouch envelope — missing trust grade '{}'\nstdout:\n{}",
                RBTDRV_LODE_TRUST_VERIFIED, augur.stdout
            ));
        }

        // Step 3.5: member-grain jettison via the type-blind raw verbs — delete
        // one member tag and prove it gone while a sibling survives.
        if let Some(v) = zrbtdrv_member_jettison_proof(
            ctx,
            dir,
            &touchmark,
            RBTDRV_RELIQUARY_TAG_GCRANE,
            RBTDRV_RELIQUARY_TAG_GCLOUD,
        ) {
            return v;
        }

        // Step 4: banish the whole Lode, then confirm the registry is restored.
        if let Some(v) = zrbtdrv_banish_and_verify_gone(ctx, dir, &touchmark) {
            return v;
        }

        let _ = std::fs::write(dir.join("07-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_RELIQUARY_LIFECYCLE: &[rbtdre_Case] = &[case!(rbtdrv_reliquary_lifecycle)];


// Shared payor-credential probe-and-gate preamble for the four credentialed-service
// fixtures below. The probe is identical; the verdict on a non-green probe is
// policy-split:
//   Skip — the terrier pair: auto-suite members, so an absent credential is
//     suite-passenger protection (terse, exit-code-only message).
//   Fail — the foedus pair: operator-invoked only (never a passenger), so an
//     absent credential fails the run and dumps the probe's stdout/stderr verbatim.
// The policy carries the whole per-policy verdict template, not just a Skip|Fail
// flag, precisely because the Fail side's stdout/stderr dump has no Skip analogue.
enum zrbtdrv_PayorGatePolicy {
    Skip,
    Fail,
}

/// Probe the payor credential; return None when green (caller proceeds), or
/// Some(verdict) when the gate trips. `fixture` is interpolated into the message
/// (pass the RBTDRM_FIXTURE_* constant), so each call reproduces its prior
/// open-coded verdict byte-for-byte.
fn zrbtdrv_payor_gate(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    fixture: &str,
    policy: zrbtdrv_PayorGatePolicy,
) -> Option<rbtdre_Verdict> {
    let _ = std::fs::write(dir.join("01-payor-probe.txt"), "probing payor credential");
    match rbtdri_invoke_global(ctx, RBTDGC_CHECK_PAYOR, &[], &[]) {
        Ok(r) if r.exit_code == 0 => None,
        Ok(r) => Some(match policy {
            zrbtdrv_PayorGatePolicy::Skip => rbtdre_Verdict::Skip(format!(
                "payor credential not reachable (exit {}) — {} requires service credentials",
                r.exit_code, fixture
            )),
            zrbtdrv_PayorGatePolicy::Fail => rbtdre_Verdict::Fail(format!(
                "payor credential probe not green (exit {}) — {} is operator-invoked \
                 and requires a live payor credential; this is a failure of the run, not a skip\n\
                 stdout:\n{}\nstderr:\n{}",
                r.exit_code, fixture, r.stdout, r.stderr
            )),
        }),
        Err(e) => Some(match policy {
            zrbtdrv_PayorGatePolicy::Skip => rbtdre_Verdict::Skip(format!(
                "payor credential probe could not run ({}) — {} requires service credentials",
                e, fixture
            )),
            zrbtdrv_PayorGatePolicy::Fail => {
                rbtdre_Verdict::Fail(format!("payor probe invocation: {}", e))
            }
        }),
    }
}


// Foedus-lifecycle fixture — federation IdP-trust round-trip against the live org.
// The reliquary-lifecycle shape (single self-contained case, no charge/quench)
// applied to the affiance→jilt create/destroy round-trip: probe the payor
// credential, affiance the manor onto a fresh throwaway workforce pool, jilt it to
// the soft-deleted terminal, then re-jilt to prove the idempotent no-op. Codifies
// the manual proof the create-shape fix was found by.
//
// Quota-touching by nature — a genuine create cannot reuse a soft-deleted id, and
// soft-deleted pools count against the 100-per-org cap for ~30 days
// (workforce-pool-constraints memo) — so this fixture is operator-invoked only:
// registered for discovery, a member of no auto-suite.

/// RBRF field the throwaway-pool override targets through the regime-poison seam.
const RBTDRV_RBRF_POOL_VAR: &str = "RBRF_WORKFORCE_POOL_ID";

/// Drive the affiance→jilt→re-jilt round-trip on `pool_id`, asserting each
/// terminal banner. Split from the case so the case can run a best-effort cleanup
/// jilt on any failure (the round-trip's own jilt may not have been reached).
fn zrbtdrv_foedus_roundtrip(ctx: &mut rbtdri_Context, dir: &Path, pool_id: &str) -> rbtdre_Verdict {
    // Payor credential precondition — Fail, not Skip (never a suite passenger).
    if let Some(v) = zrbtdrv_payor_gate(
        ctx, dir, crate::rbtdrm_manifest::RBTDRM_FIXTURE_FOEDUS_LIFECYCLE, zrbtdrv_PayorGatePolicy::Fail,
    ) {
        return v;
    }

    // The throwaway pool id rides the regime-poison seam: RBRF_WORKFORCE_POOL_ID
    // carries the RBRF_ enroll-scope prefix, so the tweak rewrites that one field
    // at regime kindle and both affiance and jilt target the throwaway pool. Only
    // the pool id is overridden — the provider is created beneath the fresh pool
    // and cascades on jilt.
    let poison = format!("{}={}", RBTDRV_RBRF_POOL_VAR, pool_id);

    // Step 1: affiance the manor onto the fresh pool; assert the create banners.
    let affiance = match rbtdri_invoke_global(ctx, RBTDGC_AFFIANCE_MANOR, &[], &[
        (RBTDRI_BURE_TWEAK_NAME_KEY, RBTDGC_TWEAK_REGIME_POISON),
        (RBTDRI_BURE_TWEAK_VALUE_KEY, poison.as_str()),
    ]) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return rbtdre_Verdict::Fail(format!("affiance failed (exit {})\n{}", r.exit_code, r.stderr)),
        Err(e) => return rbtdre_Verdict::Fail(format!("affiance invocation: {}", e)),
    };
    let affiance_out = format!("{}\n{}", affiance.stdout, affiance.stderr);
    let _ = std::fs::write(dir.join("02-affiance.txt"), &affiance_out);
    // The create banner (not the already-present path) proves the seam overrode
    // the regime pool with the throwaway id.
    let created_banner = format!("Workforce pool {} created", pool_id);
    if !affiance_out.contains(&created_banner) {
        return rbtdre_Verdict::Fail(format!(
            "affiance did not create the throwaway pool — missing banner '{}'\n{}",
            created_banner, affiance_out
        ));
    }
    let affianced_banner = format!("Manor affianced: pool={}", pool_id);
    if !affiance_out.contains(&affianced_banner) {
        return rbtdre_Verdict::Fail(format!(
            "affiance did not reach the affianced terminal — missing banner '{}'\n{}",
            affianced_banner, affiance_out
        ));
    }

    // Step 2: jilt the pool — live dissolution to the DELETED (soft-delete) terminal.
    let jilt = match rbtdri_invoke_global(ctx, RBTDGC_JILT_MANOR, &[], &[
        (RBTDRI_BURE_TWEAK_NAME_KEY, RBTDGC_TWEAK_REGIME_POISON),
        (RBTDRI_BURE_TWEAK_VALUE_KEY, poison.as_str()),
        (RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP),
    ]) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return rbtdre_Verdict::Fail(format!("jilt failed (exit {})\n{}", r.exit_code, r.stderr)),
        Err(e) => return rbtdre_Verdict::Fail(format!("jilt invocation: {}", e)),
    };
    let jilt_out = format!("{}\n{}", jilt.stdout, jilt.stderr);
    let _ = std::fs::write(dir.join("03-jilt.txt"), &jilt_out);
    let dissolved_banner = format!("Manor jilted: workforce pool {} dissolved", pool_id);
    if !jilt_out.contains(&dissolved_banner) {
        return rbtdre_Verdict::Fail(format!(
            "jilt did not reach the dissolved terminal — missing banner '{}'\n{}",
            dissolved_banner, jilt_out
        ));
    }

    // Step 3: re-jilt the soft-deleted pool — the idempotent no-op. Either no-op
    // branch (already-soft-deleted or absent) names the pool and tags "(no-op)".
    let rejilt = match rbtdri_invoke_global(ctx, RBTDGC_JILT_MANOR, &[], &[
        (RBTDRI_BURE_TWEAK_NAME_KEY, RBTDGC_TWEAK_REGIME_POISON),
        (RBTDRI_BURE_TWEAK_VALUE_KEY, poison.as_str()),
        (RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP),
    ]) {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => return rbtdre_Verdict::Fail(format!("re-jilt failed (exit {})\n{}", r.exit_code, r.stderr)),
        Err(e) => return rbtdre_Verdict::Fail(format!("re-jilt invocation: {}", e)),
    };
    let rejilt_out = format!("{}\n{}", rejilt.stdout, rejilt.stderr);
    let _ = std::fs::write(dir.join("04-rejilt.txt"), &rejilt_out);
    if !(rejilt_out.contains(pool_id) && rejilt_out.contains("no-op")) {
        return rbtdre_Verdict::Fail(format!(
            "re-jilt was not the idempotent no-op — expected an 'already … (no-op)' banner naming {}\n{}",
            pool_id, rejilt_out
        ));
    }

    let _ = std::fs::write(dir.join("05-passed.txt"), "passed");
    rbtdre_Verdict::Pass
}

fn rbtdrv_foedus_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // A unique throwaway id every run: a genuine create cannot reuse a
        // soft-deleted id, and millis-since-epoch stays within the regime's
        // [a-z0-9-]{4,32} regex while staying unique across back-to-back runs.
        let pool_id = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => format!("foedus-{}", d.as_millis()),
            Err(e) => return rbtdre_Verdict::Fail(format!("system clock before epoch: {}", e)),
        };
        let _ = std::fs::write(dir.join("00-pool-id.txt"), &pool_id);

        let verdict = zrbtdrv_foedus_roundtrip(ctx, dir, &pool_id);

        // Cleanup safety net: if the round-trip failed after affiance created the
        // pool, a leaked LIVE pool counts against the org cap as active (worse than
        // soft-deleted). Jilt is idempotent (no-op on absent/already-deleted), so a
        // best-effort pass soft-deletes any leak. Result ignored — the round-trip
        // verdict stands.
        if matches!(verdict, rbtdre_Verdict::Fail(_)) {
            let poison = format!("{}={}", RBTDRV_RBRF_POOL_VAR, pool_id);
            let _ = rbtdri_invoke_global(ctx, RBTDGC_JILT_MANOR, &[], &[
                (RBTDRI_BURE_TWEAK_NAME_KEY, RBTDGC_TWEAK_REGIME_POISON),
                (RBTDRI_BURE_TWEAK_VALUE_KEY, poison.as_str()),
                (RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP),
            ]);
        }
        verdict
    })
}

pub static RBTDRV_CASES_FOEDUS_LIFECYCLE: &[rbtdre_Case] = &[case!(rbtdrv_foedus_lifecycle)];


// Foedus-reuse fixture — the standing-freehold REUSE credential leg. Unlike the
// quota-touching lifecycle round-trip above, this reuses the REAL standing foedus
// cap-flat (no regime-poison, no throwaway pool) and is quota-neutral on the reuse
// path. Composes the two new atoms (descry, instate) with the credential heal
// (avow + don), the branch (reuse-if-valid-else-affiance) living here at the
// fixture call site, never folded into a fat verb.

/// The standing-freehold REUSE credential leg: descry the active foedus, reuse it
/// cap-flat when healthy (affiance only on a check failure), re-point the selector
/// (instate), then heal the credentials — avow the sitting, don each mantle. The
/// release ladders (skirmish/dogfight/blockade) assume this readiness step but no
/// fixture established it; operator-invoked (human-present avow, live dons), a
/// member of no suite, the payor gate fails loud (never a passenger).
fn rbtdrv_foedus_reuse(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // Payor credential precondition — Fail, not Skip (never a suite passenger).
        if let Some(v) = zrbtdrv_payor_gate(
            ctx, dir, crate::rbtdrm_manifest::RBTDRM_FIXTURE_FOEDUS_REUSE, zrbtdrv_PayorGatePolicy::Fail,
        ) {
            return v;
        }

        // The standing foedus the manor authenticates against — the committed
        // active selector, read rather than hardcoded so the leg follows the
        // regime (degenerate today: one standing foedus).
        let root = ctx.project_root().to_path_buf();
        let rbrr = root.join(RBTDGC_RBRR_FILE);
        let foedus = match crate::rbtdrk_freehold::rbtdrk_read_env_value(&rbrr, "RBRR_ACTIVE_FOEDUS") {
            Some(f) if !f.trim().is_empty() => f.trim().to_string(),
            _ => return rbtdre_Verdict::Fail(format!(
                "RBRR_ACTIVE_FOEDUS blank or absent in {} — no standing foedus to reuse",
                rbrr.display()
            )),
        };
        let _ = std::fs::write(dir.join("00-foedus.txt"), &foedus);

        // Descry the standing foedus — probe its workforce-pool health. A clean
        // probe exits 0 and reports its verdict via the foedus-health fact; only an
        // unresolvable name or broken read rejects (descry's own band).
        let descry = match rbtdri_invoke_global(ctx, RBTDGC_DESCRY_FOEDUS, &[foedus.as_str()], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!(
                "descry {} errored (exit {}) — could not determine pool health\n{}",
                foedus, r.exit_code, r.stderr
            )),
            Err(e) => return rbtdre_Verdict::Fail(format!("descry invocation: {}", e)),
        };
        let _ = std::fs::write(
            dir.join("02-descry.txt"),
            format!("{}\n{}", descry.stdout, descry.stderr),
        );
        let fact_name = format!("{}.{}", foedus, RBTDGC_FACT_EXT_FOEDUS_HEALTH);
        let health = match rbtdri_read_burv_fact(&descry, &fact_name) {
            Ok(s) => s.trim().to_string(),
            Err(e) => {
                return rbtdre_Verdict::Fail(format!("descry wrote no {} health fact: {}", fact_name, e))
            }
        };

        // Reuse-or-establish: the branch lives HERE (the verbs stay atomic). Reuse
        // the standing foedus cap-flat when healthy — no affiance, no pool churn;
        // affiance fires ONLY on a descry deficit (the rebuild-on-check-failure arm).
        // The verdict token "healthy" is descry's (rbof_descry / RBCC_fact_ext_foedus_health).
        if health == "healthy" {
            let _ = std::fs::write(
                dir.join("03-decision.txt"),
                format!("reused {} (healthy, cap-flat)", foedus),
            );
        } else {
            let _ = std::fs::write(
                dir.join("03-decision.txt"),
                format!("affiance {} (descry verdict '{}')", foedus, health),
            );
            match rbtdri_invoke_global(ctx, RBTDGC_AFFIANCE_MANOR, &[], &[]) {
                Ok(r) if r.exit_code == 0 => {}
                Ok(r) => return rbtdre_Verdict::Fail(format!(
                    "affiance (on descry deficit '{}') exit {}\n{}",
                    health, r.exit_code, r.stderr
                )),
                Err(e) => return rbtdre_Verdict::Fail(format!("affiance invocation: {}", e)),
            }
        }

        // Instate — re-point the active-foedus selector at the standing foedus.
        // Idempotent on the already-active one (rewrite to the same value, no diff).
        match rbtdri_invoke_global(ctx, RBTDGC_INSTATE_FOEDUS, &[foedus.as_str()], &[]) {
            Ok(r) if r.exit_code == 0 => {}
            Ok(r) => return rbtdre_Verdict::Fail(format!(
                "instate {} exit {}\n{}", foedus, r.exit_code, r.stderr
            )),
            Err(e) => return rbtdre_Verdict::Fail(format!("instate invocation: {}", e)),
        }
        let _ = std::fs::write(dir.join("04-instate.txt"), format!("instated {}", foedus));

        // Credential heal — avow opens or reuses the sitting (one human click at
        // suite head); the mantle dons then ride the cached federated token.
        match rbtdri_invoke_global(ctx, RBTDGC_CHECK_AVOWAL, &[], &[]) {
            Ok(r) if r.exit_code == 0 => {}
            Ok(r) => return rbtdre_Verdict::Fail(format!(
                "avow exit {} — open a sitting with rbw-acf (one device-flow click), or launch \
                 from a terminal so the prompt can surface\n{}", r.exit_code, r.stderr
            )),
            Err(e) => return rbtdre_Verdict::Fail(format!("avow invocation: {}", e)),
        }
        let _ = std::fs::write(dir.join("05-avow.txt"), "avowed");

        // Don each mantle and reach Artifact Registry (rbw-acm) — proves the
        // standing freehold's mantle credentials are LIVE (the assertion the
        // release ladders previously made in prose). The durable admission (gird/
        // brevet) is freehold-establish's; a don failure here means the freehold is
        // not seated for the subject — run freehold-establish first.
        for mantle in [RBTDGC_ACCOUNT_GOVERNOR, RBTDGC_ACCOUNT_DIRECTOR, RBTDGC_ACCOUNT_RETRIEVER] {
            match rbtdri_invoke_global(ctx, RBTDGC_CHECK_MANTLE, &[mantle], &[]) {
                Ok(r) if r.exit_code == 0 => {}
                Ok(r) => return rbtdre_Verdict::Fail(format!(
                    "don {} exit {} — mantle credential not healed (is the freehold seated for the \
                     subject? run freehold-establish first)\n{}", mantle, r.exit_code, r.stderr
                )),
                Err(e) => return rbtdre_Verdict::Fail(format!("don {} invocation: {}", mantle, e)),
            }
            let _ = std::fs::write(dir.join(format!("06-don-{}.txt", mantle)), "donned");
        }

        let _ = std::fs::write(dir.join("07-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_FOEDUS_REUSE: &[rbtdre_Case] = &[case!(rbtdrv_foedus_reuse)];


// Terrier-scaffold fixture — interim terrier-provision proof against live GCP.
// Probes the payor credential and self-skips when it is unreachable (suite-
// passenger protection), then runs the rbw-dt scaffold twice. The first run
// provisions: the verb ensures the payor-project terrier bucket, destroys-then-
// creates the polity managed folder, grants folder-scoped write + bucket-level
// read to the depot's governor mantle, and verifies all of it via a getIamPolicy
// read-back that dies fatally on any absent piece — so exit 0 IS the bucket +
// per-polity folder + write/read-IAM assertion the pace requires (a getIamPolicy
// check, not impersonation-enforcement; donning the mantle to prove own-folder-
// only belongs to the admission/foedus paces). The second run proves the reset is
// idempotent: destroy-then-create at folder grain reaches the same clean state and
// the same read-back passes again. Payor-credentialed and cross-project to the
// governor mantle, so a levied freehold absent the mantle is a real failure.
fn rbtdrv_terrier_scaffold(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // Self-skip gate: a service fixture stays green on a machine with no GCP
        // credentials by skipping, not failing, when the payor probe is not green.
        if let Some(v) = zrbtdrv_payor_gate(
            ctx, dir, crate::rbtdrm_manifest::RBTDRM_FIXTURE_TERRIER_SCAFFOLD, zrbtdrv_PayorGatePolicy::Skip,
        ) {
            return v;
        }

        // First run — provision. The scaffold's getIamPolicy read-back is the
        // bucket + per-polity folder + write/read-IAM assertion; exit 0 is that proof.
        let provision = match rbtdri_invoke_global(ctx, RBTDGC_TERRIER_SCAFFOLD, &[], &[]) {
            Ok(r) => r,
            Err(e) => {
                return rbtdre_Verdict::Fail(format!(
                    "terrier scaffold (provision) invocation: {}",
                    e
                ))
            }
        };
        let provision_out = format!("{}\n{}", provision.stdout, provision.stderr);
        let _ = std::fs::write(dir.join("02-provision.txt"), &provision_out);
        if provision.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "terrier scaffold (provision) exit {} — bucket / folder / IAM not stood up\n{}",
                provision.exit_code, provision_out
            ));
        }

        // Second run — idempotent reset. Destroy-then-create at folder grain must
        // reach the same clean state, and the same read-back verify must pass again.
        let reset = match rbtdri_invoke_global(ctx, RBTDGC_TERRIER_SCAFFOLD, &[], &[]) {
            Ok(r) => r,
            Err(e) => {
                return rbtdre_Verdict::Fail(format!(
                    "terrier scaffold (reset) invocation: {}",
                    e
                ))
            }
        };
        let reset_out = format!("{}\n{}", reset.stdout, reset.stderr);
        let _ = std::fs::write(dir.join("03-reset.txt"), &reset_out);
        if reset.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "terrier scaffold (idempotent reset) exit {} — re-run did not reach the same clean state\n{}",
                reset.exit_code, reset_out
            ));
        }

        let _ = std::fs::write(dir.join("04-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_TERRIER_SCAFFOLD: &[rbtdre_Case] = &[case!(rbtdrv_terrier_scaffold)];


// Terrier-atomicity fixture — the muniment sub-operation proof against live GCP.
// Probes the payor credential and self-skips when unreachable (suite-passenger
// protection). Charges the terrier via the rbw-dt scaffold (so the bucket + polity
// folder exist), then runs the rbw-dT proof: it engrosses a synthetic muniment,
// re-engrosses to assert the 412-on-conflict idempotency, peruses it present,
// expunges, re-expunges to assert the 404 idempotency, and peruses it gone. The
// proof self-asserts and dies on any deviation, so its exit 0 IS the atomicity
// assertion the pace requires. Payor-credentialed — project-owner read/write proves
// the GCS precondition mechanics; mantle-scoped write enforcement is the admission
// paces' to prove, not this one's.
fn rbtdrv_terrier_atomicity(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // Self-skip gate: stay green on a machine with no GCP credentials.
        if let Some(v) = zrbtdrv_payor_gate(
            ctx, dir, crate::rbtdrm_manifest::RBTDRM_FIXTURE_TERRIER_ATOMICITY, zrbtdrv_PayorGatePolicy::Skip,
        ) {
            return v;
        }

        // Charge the terrier — the proof needs a provisioned bucket + polity folder.
        let charge = match rbtdri_invoke_global(ctx, RBTDGC_TERRIER_SCAFFOLD, &[], &[]) {
            Ok(r) => r,
            Err(e) => {
                return rbtdre_Verdict::Fail(format!("terrier scaffold (charge) invocation: {}", e))
            }
        };
        let charge_out = format!("{}\n{}", charge.stdout, charge.stderr);
        let _ = std::fs::write(dir.join("02-charge.txt"), &charge_out);
        if charge.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "terrier scaffold (charge) exit {} — terrier not provisioned for the proof\n{}",
                charge.exit_code, charge_out
            ));
        }

        // Run the muniment-atomicity proof — exit 0 is the engross/expunge/peruse
        // round-trip plus the 412/404 idempotency assertions (the verb dies on any
        // deviation).
        let proof = match rbtdri_invoke_global(ctx, RBTDGC_TERRIER_PROOF, &[], &[]) {
            Ok(r) => r,
            Err(e) => return rbtdre_Verdict::Fail(format!("terrier proof invocation: {}", e)),
        };
        let proof_out = format!("{}\n{}", proof.stdout, proof.stderr);
        let _ = std::fs::write(dir.join("03-proof.txt"), &proof_out);
        if proof.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "terrier proof exit {} — engross/expunge/peruse atomicity not proven\n{}",
                proof.exit_code, proof_out
            ));
        }

        let _ = std::fs::write(dir.join("04-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_TERRIER_ATOMICITY: &[rbtdre_Case] = &[case!(rbtdrv_terrier_atomicity)];


// Wsl-lifecycle fixture — fetched-side rootfs capture against live GAR. Single
// self-contained round-trip: underpin a vendor-published Ubuntu rootfs into a
// fresh rbi_ld Lode, divine-enumerate to confirm it appears, divine-inspect to
// confirm the opaque-rootfs member tag + vouch envelope rode in, banish the whole
// Lode, then divine-enumerate to confirm the registry is restored. The wsl kind's
// structural-outlier analogue of lode-lifecycle: its capture is curl + GPG-verify
// + opaque-blob wrap, not a registry pull. Underpin takes the substrate version
// as arguments (release point), so it needs no vessel precondition. Consumption
// (wsl --import) is deferred — this stops at the registry, no host in the loop.
fn rbtdrv_wsl_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // Step 1: underpin the pinned Ubuntu rootfs version into a fresh Lode.
        let _ = std::fs::write(dir.join("01-underpin.txt"), "underpinning wsl rootfs");
        let underpin = match rbtdri_invoke_global(
            ctx,
            RBTDGC_UNDERPIN_WSL,
            &[RBTDRV_WSL_RELEASE, RBTDRV_WSL_POINT],
            &[],
        ) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("underpin failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("underpin invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("01-underpin-stdout.txt"), &underpin.stdout);

        // The host-side capture handoff is the bare touchmark fact.
        let touchmark = match zrbtdrv_read_touchmark(&underpin, dir) {
            Ok(t) => t,
            Err(v) => return v,
        };

        // Step 2: divine enumerate shows the new Lode.
        if let Err(v) = zrbtdrv_divine_contains(ctx, dir, &touchmark, "underpin") {
            return v;
        }

        // Step 3: augur inspects the rootfs Lode — member tags AND the decoded
        // :rbi_vouch envelope (the rootfs singleton).
        let augur = match rbtdri_invoke_global(ctx, RBTDGC_AUGUR_LODE, &[&touchmark], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("augur failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("augur invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("04-augur.txt"), &augur.stdout);
        for member in &[RBTDRV_LODE_TAG_ROOTFS, RBTDRV_LODE_TAG_VOUCH] {
            if !augur.stdout.contains(member) {
                return rbtdre_Verdict::Fail(format!(
                    "augur missing member tag '{}'\nstdout:\n{}",
                    member, augur.stdout
                ));
            }
        }

        // Step 4: banish the whole Lode, then confirm the registry is restored.
        if let Some(v) = zrbtdrv_banish_and_verify_gone(ctx, dir, &touchmark) {
            return v;
        }

        let _ = std::fs::write(dir.join("07-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_WSL_LIFECYCLE: &[rbtdre_Case] = &[case!(rbtdrv_wsl_lifecycle)];


// Podvm-lifecycle fixture — fetched-side podvm disk-leaf capture against live GAR. Single
// self-contained round-trip: immure a quay.io/podman/machine-os-wsl family into a fresh
// rbi_ld Lode, divine-enumerate to confirm it appears as a cohort, augur to confirm the
// two member tags (rbi_wsl-x86_64, rbi_wsl-aarch64) + decoded :rbi_vouch envelope rode
// in at recorded-at-acquisition grade, jettison one member tag via the type-blind raw
// image verb proving per-member delete, banish the whole Lode, then divine-enumerate to
// confirm the registry is restored. Structural analogue of both reliquary-lifecycle
// (multi-member cohort + member-jettison case) and wsl-lifecycle (opaque-blob capture).

/// Podvm-wsl kind argument — family brand passed to immure.
const RBTDRV_PODVM_FAMILY: &str = "podvm-wsl";
/// Podvm version tag — the quay.io family index version to capture.
const RBTDRV_PODVM_VERSION: &str = "5.6";

/// Podvm-wsl member tags asserted by augur. Compose RBGC_LODE_TAG_SPRUE ("rbi_")
/// with the selection leaf names from rbgc_constants.sh RBGC_LODE_PODVM_WSL_SELECTION.
const RBTDRV_PODVM_TAG_WSL_X86: &str = "rbi_wsl-x86_64";
const RBTDRV_PODVM_TAG_WSL_AARCH: &str = "rbi_wsl-aarch64";

/// Trust grade for the recorded-at-acquisition envelope — mirrors rbgc_constants.sh
/// RBGC_LODE_TRUST_RECORDED. The podvm upstream offers no durable checksum, so RB
/// attests only the digest observed at capture.
const RBTDRV_LODE_TRUST_RECORDED: &str = "recorded-at-acquisition";

/// Honest-posture text fragment emitted by augur for recorded-at-acquisition grade
/// (rbldl_lifecycle.sh RBGC_LODE_TRUST_RECORDED branch). Proves augur rendered the
/// trust-posture section, not just the envelope header. Matches a stable substring
/// of the fixed prose rather than the full multi-line block.
const RBTDRV_PODVM_TRUST_POSTURE_FRAGMENT: &str = "attests only the digest observed at capture";

fn rbtdrv_podvm_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // Step 1: immure the podvm-wsl family + version into a fresh Lode.
        let _ = std::fs::write(dir.join("01-immure.txt"), "immuring podvm-wsl disk leaves");
        let immure = match rbtdri_invoke_global(
            ctx,
            RBTDGC_IMMURE_PODVM,
            &[RBTDRV_PODVM_FAMILY, RBTDRV_PODVM_VERSION],
            &[],
        ) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("immure failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("immure invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("01-immure-stdout.txt"), &immure.stdout);

        // The host-side capture handoff is the bare touchmark fact.
        let touchmark = match zrbtdrv_read_touchmark(&immure, dir) {
            Ok(t) => t,
            Err(v) => return v,
        };

        // Step 2: divine enumerate shows the new Lode as a cohort.
        let after = match zrbtdrv_divine_contains(ctx, dir, &touchmark, "immure") {
            Ok(s) => s,
            Err(v) => return v,
        };
        // Cohort display asserts the member count column, not a specific digest.
        if !after.contains("cohort: 2 members") {
            return rbtdre_Verdict::Fail(format!(
                "post-immure divine row for {} missing '(cohort: 2 members)'\nstdout:\n{}",
                touchmark, after
            ));
        }

        // Step 2.5: REFRESH the same Lode at its locked version. The wsl family's
        // production curation IS the full 2-leaf set, so this refresh adds no new
        // member — it is the all-preserved / convergent path, and that is what the
        // recurring suite proves: refresh reuses the existing touchmark (no new Lode),
        // derives the locked version from the envelope (it takes no version argument),
        // re-reads the GAR member tags as the source of truth, preserves both originals
        // verbatim, and re-authors :rbi_vouch. The widen-adds-a-member path is the
        // one-time native gate (full 8-leaf machine-os, multi-GB), deliberately kept
        // OUT of the recurring suite; see rbgc_constants RBGC_LODE_PODVM_NATIVE_SELECTION.
        let _ = std::fs::write(dir.join("03b-refresh.txt"), "refreshing podvm-wsl Lode (all-preserved)");
        match rbtdri_invoke_global(
            ctx,
            RBTDGC_IMMURE_PODVM,
            &[RBTDRV_PODVM_FAMILY, "--refresh", &touchmark],
            &[],
        ) {
            Ok(r) if r.exit_code == 0 => {}
            Ok(r) => return rbtdre_Verdict::Fail(format!("refresh immure failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("refresh immure invocation: {}", e)),
        }
        // Post-refresh divine: the SAME touchmark, still a 2-member cohort — refresh
        // reused the Lode and preserved both originals (no new Lode, no membership drift).
        let after_refresh = match rbtdri_invoke_global(ctx, RBTDGC_DIVINE_LODES, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("post-refresh divine failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("post-refresh divine invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("03c-divine-after-refresh.txt"), &after_refresh.stdout);
        if !after_refresh.stdout.contains(&touchmark) {
            return rbtdre_Verdict::Fail(format!(
                "post-refresh divine missing touchmark {} — refresh lost the Lode\nstdout:\n{}",
                touchmark, after_refresh.stdout
            ));
        }
        if !after_refresh.stdout.contains("cohort: 2 members") {
            return rbtdre_Verdict::Fail(format!(
                "post-refresh divine row for {} not '(cohort: 2 members)' — refresh changed membership\nstdout:\n{}",
                touchmark, after_refresh.stdout
            ));
        }

        // Step 3: augur inspects the podvm Lode — member tags AND the decoded
        // (this augur now also validates the refresh PRESERVED both original members)
        // :rbi_vouch envelope. The trust-grade assertion proves augur decoded the
        // recorded-at-acquisition envelope (distinct from the verified grade the bole
        // and reliquary kinds carry); the posture-fragment assertion proves the honest-
        // posture prose block was rendered (struct proof, not digest assertion).
        let augur = match rbtdri_invoke_global(ctx, RBTDGC_AUGUR_LODE, &[&touchmark], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("augur failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("augur invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("04-augur.txt"), &augur.stdout);
        // Member-tag presence (structural: these are the two WSL disk-leaf kinds).
        for member in &[RBTDRV_PODVM_TAG_WSL_X86, RBTDRV_PODVM_TAG_WSL_AARCH, RBTDRV_LODE_TAG_VOUCH] {
            if !augur.stdout.contains(member) {
                return rbtdre_Verdict::Fail(format!(
                    "augur missing member tag '{}'\nstdout:\n{}",
                    member, augur.stdout
                ));
            }
        }
        // Trust-grade assertion — recorded-at-acquisition, not verified-against-published.
        // Never assert specific digest hex: the upstream rotates and the digest changes.
        if !augur.stdout.contains(RBTDRV_LODE_TRUST_RECORDED) {
            return rbtdre_Verdict::Fail(format!(
                "augur did not decode podvm :rbi_vouch envelope — missing trust grade '{}'\nstdout:\n{}",
                RBTDRV_LODE_TRUST_RECORDED, augur.stdout
            ));
        }
        // Kind field assertion — proves the envelope names the podvm-wsl kind.
        if !augur.stdout.contains(RBTDRV_PODVM_FAMILY) {
            return rbtdre_Verdict::Fail(format!(
                "augur envelope missing kind '{}'\nstdout:\n{}",
                RBTDRV_PODVM_FAMILY, augur.stdout
            ));
        }
        // Honest-posture prose block — proves the recorded-at-acquisition branch ran.
        if !augur.stdout.contains(RBTDRV_PODVM_TRUST_POSTURE_FRAGMENT) {
            return rbtdre_Verdict::Fail(format!(
                "augur trust-posture prose missing expected fragment '{}'\nstdout:\n{}",
                RBTDRV_PODVM_TRUST_POSTURE_FRAGMENT, augur.stdout
            ));
        }

        // Step 3.5: per-member jettison via the type-blind raw verbs — delete one
        // member tag and prove it gone while a sibling survives.
        if let Some(v) = zrbtdrv_member_jettison_proof(
            ctx,
            dir,
            &touchmark,
            RBTDRV_PODVM_TAG_WSL_AARCH,
            RBTDRV_PODVM_TAG_WSL_X86,
        ) {
            return v;
        }

        // Step 4: banish the whole Lode, then confirm the registry is restored.
        if let Some(v) = zrbtdrv_banish_and_verify_gone(ctx, dir, &touchmark) {
            return v;
        }

        let _ = std::fs::write(dir.join("07-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_PODVM_LIFECYCLE: &[rbtdre_Case] = &[case!(rbtdrv_podvm_lifecycle)];


// Batch-vouch fixture — exercises rbfv_batch_vouch's two-pass pending→vouched
// transition. Single self-contained lifecycle: ordain conjure, jettison the
// vouch ark to plant a pending hallmark, tally to confirm pending, batch_vouch
// to fill the gap, tally to confirm vouched, abjure.

/// Locate a hallmark's row in tally stdout and return its health column.
/// Tally rows have shape `  <hallmark>  <health>  <basenames...>`.
fn rbtdrv_tally_health(stdout: &str, hallmark: &str) -> Option<String> {
    for line in stdout.lines() {
        let mut fields = line.split_whitespace();
        if fields.next() == Some(hallmark) {
            return fields.next().map(str::to_string);
        }
    }
    None
}

fn rbtdrv_batch_vouch_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let vessel_dir = RBTDRV_BUSYBOX_VESSEL_DIR;
        if !ctx.project_root().join(vessel_dir).is_dir() {
            return rbtdre_Verdict::Fail(format!("vessel directory not found: {}", vessel_dir));
        }

        let hallmark = match rbtdri_ordain_capture(ctx, dir, vessel_dir, &[], "01-ordain") {
            Ok(h) => h,
            Err(v) => return v,
        };

        // Plant pending state: jettison the vouch ark, leaving image+about.
        let _ = std::fs::write(dir.join("02-plant-jettison.txt"), "jettisoning vouch");
        let jettison_locator = rbtdri_gar_ref_categorical(
            RBTDRV_GAR_CATEGORY_HALLMARKS,
            RBTDRV_ARK_BASENAME_VOUCH,
            &hallmark,
        );
        match rbtdri_invoke_global(ctx, RBTDGC_JETTISON_HALLMARK_IMAGE, &[&jettison_locator], &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)]) {
            Ok(r) if r.exit_code == 0 => {}
            Ok(r) => return rbtdre_Verdict::Fail(format!("plant jettison failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("plant jettison invocation: {}", e)),
        }

        // Tally — expect pending classification on our hallmark.
        let _ = std::fs::write(dir.join("03-tally-pending.txt"), "tallying for pending");
        let tally_pending = match rbtdri_invoke_global(ctx, RBTDGC_TALLY_HALLMARKS, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("tally (pending) failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("tally (pending) invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("03-tally-pending-stdout.txt"), &tally_pending.stdout);
        match rbtdrv_tally_health(&tally_pending.stdout, &hallmark) {
            Some(h) if h == "pending" => {}
            Some(h) => return rbtdre_Verdict::Fail(format!(
                "tally: expected health 'pending' for {}, got '{}'\nstdout:\n{}",
                hallmark, h, tally_pending.stdout
            )),
            None => return rbtdre_Verdict::Fail(format!(
                "tally: hallmark {} not found in tally output\nstdout:\n{}",
                hallmark, tally_pending.stdout
            )),
        }

        // Batch vouch — should detect the pending hallmark and re-create vouch.
        let _ = std::fs::write(dir.join("04-batch-vouch.txt"), "running batch vouch");
        match rbtdri_invoke_global(ctx, RBTDGC_VOUCH_HALLMARKS, &[], &[]) {
            Ok(r) if r.exit_code == 0 => {}
            Ok(r) => return rbtdre_Verdict::Fail(format!("batch_vouch failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("batch_vouch invocation: {}", e)),
        }

        // Tally — expect vouched after batch_vouch.
        let _ = std::fs::write(dir.join("05-tally-vouched.txt"), "tallying for vouched");
        let tally_vouched = match rbtdri_invoke_global(ctx, RBTDGC_TALLY_HALLMARKS, &[], &[]) {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => return rbtdre_Verdict::Fail(format!("tally (vouched) failed (exit {})\n{}", r.exit_code, r.stderr)),
            Err(e) => return rbtdre_Verdict::Fail(format!("tally (vouched) invocation: {}", e)),
        };
        let _ = std::fs::write(dir.join("05-tally-vouched-stdout.txt"), &tally_vouched.stdout);
        match rbtdrv_tally_health(&tally_vouched.stdout, &hallmark) {
            Some(h) if h == "vouched" => {}
            Some(h) => return rbtdre_Verdict::Fail(format!(
                "tally: expected health 'vouched' for {}, got '{}'\nstdout:\n{}",
                hallmark, h, tally_vouched.stdout
            )),
            None => return rbtdre_Verdict::Fail(format!(
                "tally: hallmark {} not found in tally output\nstdout:\n{}",
                hallmark, tally_vouched.stdout
            )),
        }

        if let Err(v) = rbtdri_invoke_or_fail(
            ctx,
            "abjure",
            &hallmark,
            RBTDGC_ABJURE_HALLMARK,
            &[&hallmark],
            &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
            dir,
            "06-abjure",
        ) {
            return v;
        }

        let _ = std::fs::write(dir.join("07-passed.txt"), "passed");
        rbtdre_Verdict::Pass
    })
}

pub static RBTDRV_CASES_BATCH_VOUCH: &[rbtdre_Case] = &[case!(rbtdrv_batch_vouch_lifecycle)];

// ── Access probe cases (bare fixture, imprint-scoped) ────────

/// Invoke a credential access-probe tabtarget by role, check exit code.
fn rbtdrv_access_probe_role(ctx: &mut rbtdri_Context, role: &str, dir: &Path) -> rbtdre_Verdict {
    let colophon = match rbtdrm_credential_check_colophon(role) {
        Some(c) => c,
        None => return rbtdre_Verdict::Fail(format!("unknown credential role: {}", role)),
    };
    let result = match rbtdri_invoke_global(ctx, colophon, &[], &[]) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("{} probe invocation: {}", role, e)),
    };
    let _ = std::fs::write(dir.join("probe-stdout.txt"), &result.stdout);
    let _ = std::fs::write(dir.join("probe-stderr.txt"), &result.stderr);

    if result.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "{} probe exited {}\n{}",
            role, result.exit_code, result.stderr
        ));
    }
    rbtdre_Verdict::Pass
}

fn rbtdrv_oauth_payor(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrv_access_probe_role(ctx, RBTDGC_ACCOUNT_PAYOR, dir))
}

pub static RBTDRV_CASES_ACCESS_PROBE: &[rbtdre_Case] = &[
    case!(rbtdrv_oauth_payor),
];
