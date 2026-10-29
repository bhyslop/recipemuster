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
// RBTDRY — the environment census through rb's own doors: what each process of
// a nested chain of rb's doors holds exported, read at every level, and the
// refusal that keeps it so.
//
// Two fixtures. `environment-quiet` is the reveille member. Its chain case
// plants a made-up control exported on an `rbw-tc` child running the relay case
// of `quiet-hush`; the relay records its own exported names and
// drives `rbw-tc` again into the leaf case, which records its own. The
// control's value names the directory every level records into, so the
// readings land beside the case that asserts them, and a reading that misses
// the control has read nothing. Its refused case plants a config-regime name
// with a valid value exported on a child door and asserts the enrollment gate's
// code and the exported-name rule in the refusal.
//
// `quiet-hush` is a member of no suite: its two cases are levels of
// the chain, meaningful only where the chain reaches them.

use std::path::{Path, PathBuf};

use crate::rbtccc_crucible::rbtccc_with_ctx;
use crate::rbtcee_case;
use crate::rbtcee_engine::{
    rbtcee_Case,
    rbtcee_Disposition,
    rbtcee_Fixture,
    rbtcee_Tariff,
    rbtcee_Verdict,
};
use crate::rbtcei_invocation::{rbtcei_invoke_global, rbtcei_InvokeResult};
use crate::rbtcep_platform::{rbtcep_native_to_posix, rbtcep_posix_to_native};
use crate::rbtgeb_breviary::RBTGEB_BUSX_ENROLL_DENTIL;
use crate::rbtgec_consts::RBTGEC_THEURGE_CASE;

pub const RBTDRY_NAME_ENVIRONMENT_QUIET: &str = "environment-quiet";
pub const RBTDRY_NAME_QUIET_HUSH: &str = "quiet-hush";

/// The control planted exported at the top of the chain: a made-up name,
/// matching no governed word and no family the estate seats. Its value names
/// the directory every level records into.
const ZRBTDRY_CONTROL: &str = "QUOBRY_PLANTED";

/// The hand-offs a process below a CLI holds (BUr_q2m).
const ZRBTDRY_HANDED: &[&str] = &["BURD_BURD_FILE", "BURD_BUK_DIR"];

/// The environment regime's family: ambient at every level, stripped by nothing.
const ZRBTDRY_AMBIENT: &str = "BURE_";

/// The families a name of the kit, rb's regimes or the census wears.
const ZRBTDRY_FAMILIES: &[&str] = &[
    "BUC_",
    "BURC_",
    "BURD_",
    "BURE_",
    "BURS_",
    "BURV_",
    "BURX_",
    "BUZ_",
    "RBRD_",
    "RBRF_",
    "RBRM_",
    "RBRN_",
    "RBRO_",
    "RBRP_",
    "RBRR_",
    "RBRV_",
    "RBRW_",
    "VOM_",
];

/// The planted refusal: a config-regime name carrying a valid value, so the
/// code says the exported-name rule fired rather than any check of the value.
const ZRBTDRY_REFUSED_NAME: &str = "BURC_TABTARGET_DIR";
const ZRBTDRY_REFUSED_VALUE: &str = "tt";

/// The rule the refusal names, as the kit's line spells it.
const ZRBTDRY_RULE: &str = "exported-name rule";

/// The two levels beneath the chain case, each recording under its own name.
const ZRBTDRY_LEVEL_RELAY: &str = "relay";
const ZRBTDRY_LEVEL_LEAF: &str = "leaf";

/// The hush cases each level runs, by the single-case runner's name for them.
const ZRBTDRY_CASE_RELAY: &str = "rbtdry_hush_relay";
const ZRBTDRY_CASE_LEAF: &str = "rbtdry_hush_leaf";

// ── Readings ─────────────────────────────────────────────────

/// The file a level records its exported names into.
fn zrbtdry_reading_path(dir: &Path, level: &str) -> PathBuf {
    dir.join(format!("exported-{}.txt", level))
}

/// Record this process's exported names, one per line, into the directory the
/// control names. A level the control does not reach refuses: it was run
/// outside the chain, and a reading there proves nothing.
fn zrbtdry_record(level: &str) -> Result<(), String> {
    let pointer = std::env::var(ZRBTDRY_CONTROL).map_err(|_| {
        format!(
            "{} is not in this level's environment — the hush levels run only beneath {}'s chain",
            ZRBTDRY_CONTROL, RBTDRY_NAME_ENVIRONMENT_QUIET
        )
    })?;
    let dir = rbtcep_posix_to_native(&pointer)?;
    let mut names: Vec<String> = std::env::vars_os()
        .map(|(name, _)| name.to_string_lossy().into_owned())
        .collect();
    names.sort();
    let path = zrbtdry_reading_path(&dir, level);
    std::fs::write(&path, format!("{}\n", names.join("\n")))
        .map_err(|e| format!("cannot write the reading {}: {}", path.display(), e))
}

/// The names a level recorded.
fn zrbtdry_reading(dir: &Path, level: &str) -> Result<Vec<String>, String> {
    let path = zrbtdry_reading_path(dir, level);
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("nothing was recorded at {}: {}", path.display(), e))?;
    Ok(text.lines().filter(|l| !l.is_empty()).map(str::to_string).collect())
}

/// A reading holds the planted control and the hand-offs, and no other name of
/// the kit, rb's regimes or the census save the environment regime's.
fn zrbtdry_quiet(level: &str, names: &[String]) -> Result<(), String> {
    if !names.iter().any(|n| n == ZRBTDRY_CONTROL) {
        return Err(format!("the {} reading misses the planted control: {:?}", level, names));
    }
    for held in ZRBTDRY_HANDED {
        if !names.iter().any(|n| n == held) {
            return Err(format!("the {} reading does not hold {}: {:?}", level, held, names));
        }
    }
    let stray: Vec<&String> = names
        .iter()
        .filter(|n| ZRBTDRY_FAMILIES.iter().any(|f| n.starts_with(f)))
        .filter(|n| !n.starts_with(ZRBTDRY_AMBIENT) && !ZRBTDRY_HANDED.contains(&n.as_str()))
        .collect();
    if !stray.is_empty() {
        return Err(format!("the {} reading holds {:?} exported", level, stray));
    }
    Ok(())
}

// ── Child drive ──────────────────────────────────────────────

/// Drive one hush case through the single-case runner, `rbw-tc`, with the
/// extra names planted exported on that door.
fn zrbtdry_drive(case: &str, planted: &[(&str, &str)]) -> Result<rbtcei_InvokeResult, String> {
    let mut driven: Option<Result<rbtcei_InvokeResult, String>> = None;
    rbtccc_with_ctx(|ctx| {
        driven = Some(rbtcei_invoke_global(
            ctx,
            RBTGEC_THEURGE_CASE,
            &[RBTDRY_NAME_QUIET_HUSH, case],
            planted,
        ));
        rbtcee_Verdict::Pass
    });
    driven.unwrap_or_else(|| Err("the hush drive did not run".to_string()))
}

/// What a drive said, both streams, for a refusal that names a miss.
fn zrbtdry_said(result: &rbtcei_InvokeResult) -> String {
    format!("--- stdout ---\n{}\n--- stderr ---\n{}", result.stdout, result.stderr)
}

/// The directory a case's levels record into, made fresh beneath the case.
fn zrbtdry_readings_dir(dir: &Path) -> Result<PathBuf, String> {
    let readings = dir.join("readings");
    std::fs::create_dir_all(&readings)
        .map_err(|e| format!("cannot create {}: {}", readings.display(), e))?;
    Ok(readings)
}

fn zrbtdry_verdict(outcome: Result<(), String>) -> rbtcee_Verdict {
    match outcome {
        Ok(()) => rbtcee_Verdict::Pass,
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

// ── environment-quiet cases ──────────────────────────────────

/// EVERY LEVEL OF A NESTED CHAIN OF RB'S DOORS HOLDS ITS HAND-OFFS AND NOTHING
/// ELSE OF THE KIT: the relay and the leaf each read their own exported names,
/// and each reading strikes the control planted at the top.
fn rbtdry_eq_chain(dir: &Path) -> rbtcee_Verdict {
    zrbtdry_verdict((|| {
        let readings = zrbtdry_readings_dir(dir)?;
        let pointer = rbtcep_native_to_posix(&readings);
        let result = zrbtdry_drive(ZRBTDRY_CASE_RELAY, &[(ZRBTDRY_CONTROL, &pointer)])?;
        let _ = std::fs::write(dir.join("chain-said.txt"), zrbtdry_said(&result));
        if result.exit_code != 0 {
            return Err(format!(
                "the relay level exited {}\n{}",
                result.exit_code,
                zrbtdry_said(&result)
            ));
        }
        for level in [ZRBTDRY_LEVEL_RELAY, ZRBTDRY_LEVEL_LEAF] {
            zrbtdry_quiet(level, &zrbtdry_reading(&readings, level)?)?;
        }
        Ok(())
    })())
}

/// A CONFIG-REGIME NAME ARRIVING EXPORTED IS REFUSED at the door it reaches,
/// under the enrollment gate's code, the refusal naming the exported rule.
fn rbtdry_eq_refused(dir: &Path) -> rbtcee_Verdict {
    zrbtdry_verdict((|| {
        let readings = zrbtdry_readings_dir(dir)?;
        let pointer = rbtcep_native_to_posix(&readings);
        let result = zrbtdry_drive(
            ZRBTDRY_CASE_LEAF,
            &[(ZRBTDRY_CONTROL, &pointer), (ZRBTDRY_REFUSED_NAME, ZRBTDRY_REFUSED_VALUE)],
        )?;
        let said = zrbtdry_said(&result);
        let _ = std::fs::write(dir.join("refused-said.txt"), &said);
        if result.exit_code != RBTGEB_BUSX_ENROLL_DENTIL {
            return Err(format!(
                "{} planted exported: expected exit {}, got {}\n{}",
                ZRBTDRY_REFUSED_NAME, RBTGEB_BUSX_ENROLL_DENTIL, result.exit_code, said
            ));
        }
        for needle in [ZRBTDRY_RULE, ZRBTDRY_REFUSED_NAME] {
            if !said.contains(needle) {
                return Err(format!("the refusal does not name '{}'\n{}", needle, said));
            }
        }
        Ok(())
    })())
}

// ── quiet-hush cases ────────────────────────────

/// The relay level: record, then drive the leaf through rb's own door.
fn rbtdry_hush_relay(_dir: &Path) -> rbtcee_Verdict {
    zrbtdry_verdict((|| {
        zrbtdry_record(ZRBTDRY_LEVEL_RELAY)?;
        let result = zrbtdry_drive(ZRBTDRY_CASE_LEAF, &[])?;
        if result.exit_code != 0 {
            return Err(format!(
                "the leaf level exited {}\n{}",
                result.exit_code,
                zrbtdry_said(&result)
            ));
        }
        Ok(())
    })())
}

/// The leaf level: record.
fn rbtdry_hush_leaf(_dir: &Path) -> rbtcee_Verdict {
    zrbtdry_verdict(zrbtdry_record(ZRBTDRY_LEVEL_LEAF))
}

// ── Case arrays and fixture statics ──────────────────────────

pub static RBTDRY_CASES_ENVIRONMENT_QUIET: &[rbtcee_Case] = &[
    rbtcee_case!(rbtdry_eq_chain),
    rbtcee_case!(rbtdry_eq_refused),
];

pub static RBTDRY_CASES_QUIET_HUSH: &[rbtcee_Case] = &[
    rbtcee_case!(rbtdry_hush_relay),
    rbtcee_case!(rbtdry_hush_leaf),
];

/// credless: a reveille member, so the guard rides every child launch — inert on
/// the rbw-tc chain, which mints no tokens.
pub static RBTDRY_FIXTURE_ENVIRONMENT_QUIET: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTDRY_NAME_ENVIRONMENT_QUIET,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRY_CASES_ENVIRONMENT_QUIET,
    credless: true,
    tariff: rbtcee_Tariff { min_secs: None, max_secs: None, invocations: Some(2) },
};

/// Reached only through the chain, one case at a time, so it carries no tariff.
pub static RBTDRY_FIXTURE_QUIET_HUSH: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTDRY_NAME_QUIET_HUSH,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRY_CASES_QUIET_HUSH,
    credless: true,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
