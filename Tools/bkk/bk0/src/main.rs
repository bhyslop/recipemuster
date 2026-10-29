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

//! `bkx` — the kennel's door face.
//!
//! EVERY DOOR HAS AN ARM, AND FIVE OF THEM DO SOMETHING. *tattoo* reads one
//! collar and changes nothing; *mush* launches what one collar names, holding
//! the binary election inside it; *heel* converges one collar, the one act the
//! others refuse to perform; *muzzle* sweeps every collar the walk finds and
//! *derby* runs every suite among them, and *futurity* runs the part of that
//! set a pattern names.
//! The rest dispatch to a BLANK that refuses, naming the door and the work that
//! fills it.
//!
//! THE BLANKS ARE OPENED HERE RATHER THAN BY THE PACES THAT FILL THEM. The
//! doors of this rollout are built in parallel billets, and an arm each of them
//! opened beside its neighbours' is a merge conflict at every refit — so the
//! seam is opened once, empty, and each pace fills a blank instead of authoring
//! one. The same reasoning stands the empty modules beside them.
//!
//! A BLANK REFUSES AND NEVER SUCCEEDS QUIETLY. A door word reaching this binary
//! before its pace lands meets a sentence rather than being handed to cargo,
//! which would run something nobody asked for.
//!
//! THE TWO STANDING DOORS SHARE ONE READING AND PART ON WHAT THEY DO WITH IT.
//! Both resolve a collar and both gather every finding against it, because
//! validation is implicit in every door that reads one. Tattoo answers the
//! proclamation whatever the findings say and refuses only its exit; mush
//! refuses outright, having no answer to withhold and an act to stop.
//!
//! WHAT DOES ANSWER IS THE LEASH REGISTER: cargo's own grammar with the
//! discipline applied and nothing reinterpreted. It wears no tabtarget of its
//! own and is reached from inside another door's dispatch — today from bash,
//! by each cargo-invoking site the rollout repoints, and later by the collar
//! register's own verbs from inside this binary. That is why the dispatch below
//! reads a verb and hands it on without knowing what it means: reinterpreting a
//! caller's arguments is the one thing the register may not do.
//!
//! Bare, with no arguments at all, the kennel still reports its own making — the
//! one question that needs no collar.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

// Output discipline: all emission via bkco_*! — no direct println!/eprintln!
use bkk::bkco_output;
use bkk::{
    bkco_answer_now,
    bkco_error_now,
    bkco_fatal_now,
    bkco_info_now,
    bkco_trace_now,
    bkco_voice_now, //
};

use bkk::bkcc_record;
use bkk::bkcd_preflight;
use bkk::bkce_election;
use bkk::bkcf_guard;
use bkk::bkcg_gangline;
use bkk::bkch_heel;
use bkk::bkci_pipeline;
use bkk::bkcl_leash;
use bkk::bkcm_mush;
use bkk::bkcn_delouse;
use bkk::bkcq_kibble;
use bkk::bkcr_resolve;
use bkk::bkcs_stamp;
use bkk::bkct_tattoo;
use bkk::bkcv_voice;
use bkk::bkcx_python;
use bkk::bkcy_sweep;
use bkk::bkcz_muzzle;
use std::ffi::OsString;
use std::path::{
    Path,
    PathBuf, //
};

/// End the door barred: it read its subject and ruled against it on purpose.
///
/// EVERY DOOR SPEAKS THE FOUR VERDICTS EVERY DOOR SPEAKS, and the kennel declares
/// no exit code of its own. Sound exits zero. Broke is the standard library's
/// failure code, reached through `bkco_output::bkco_broke`. Barred and fenced are
/// the substrate's two verdict codes, which this crate holds by grant in its
/// generated constants and cites from there.
///
/// WHAT PARTED ONE REFUSAL FROM ANOTHER IS SAID, NEVER CODED. A caller acting on
/// the exit alone acts on the verdict, and a caller that must know which gate
/// fired reads the line the door printed, which every refusal prints before it
/// ends. A code per refusal was a contract per refusal, kept in step with every
/// caller that read one, and no caller read one.
fn zbkm_barred() -> ! {
    std::process::exit(bkk::bkg_breviary::BKG_BUSX_BARRED_PSEPHOS)
}

/// End the door fenced: it refused before reading any subject at all.
///
/// PARTED FROM BARRED because a caller reading barred acts on a verdict, and a
/// door that never reached its subject has given none: the door law, a program
/// the station lacks, a geography that does not derive, a weaned set that does
/// not conform, an artifact the election rules outrun and a walk's preflight all
/// refuse here, each ahead of the subject it stands guard over.
fn zbkm_fenced() -> ! {
    std::process::exit(bkk::bkg_breviary::BKG_BUSX_FENCED_PSEPHOS)
}

/// End a door on the exit of a program it relayed, answered as a verdict.
///
/// A RELAYED CODE IS CLASSIFIED, NEVER HANDED ON. Zero is sound, and a program
/// exiting barred or fenced is relayed as this door's own barred or fenced, so a
/// verdict the program spoke crosses the door unchanged. Any other code names no
/// verdict — a usage refusal, a panic, a tool's own failure numbering — and ends
/// the door broke, the line above saying what the program exited: handed on raw,
/// a code of 100 or above would read as a gate the band never minted.
fn zbkm_relayed(code: i32, ran: &str) -> ! {
    match code {
        0 => std::process::exit(0),
        bkk::bkg_breviary::BKG_BUSX_BARRED_PSEPHOS => zbkm_barred(),
        bkk::bkg_breviary::BKG_BUSX_FENCED_PSEPHOS => zbkm_fenced(),
        other => {
            bkco_error_now!(
                "{} exited {}, which names no verdict, so this door ends broke",
                ran,
                other, //
            );
            bkco_output::bkco_broke()
        }
    }
}

/// The doors of the collar register, each wearing its word at three seats — the
/// tabtarget's frontispiece, this subcommand, and the spoken word in the
/// conduct text. Six are the freeze's own; gangline, scoop, futurity and
/// delouse were slated after it.
///
/// THE WHOLE ROSTER IS DECLARED WHETHER OR NOT THE INTERIOR STANDS, which is
/// what lets every word take an arm below. A door absent from this list would
/// fall through to the leash register and be handed to cargo as a verb.
///
/// THE WORDS THEMSELVES HOME IN THE LIBRARY and are aliased here, which is what
/// lets a refusal composed in a library module name a door without spelling its
/// word a second time. The roster is `bkco_output`'s, beside the lookup that
/// resolves each word to the tabtarget that actually stands; three spellings of
/// `heel` stood across this tree before it, and the binary's copy was the one no
/// library module could reach.
const ZBKM_DOOR_MUSH: &str = bkco_output::BKCO_VERB_MUSH;
const ZBKM_DOOR_DERBY: &str = bkco_output::BKCO_VERB_DERBY;
const ZBKM_DOOR_FUTURITY: &str = bkco_output::BKCO_VERB_FUTURITY;
const ZBKM_DOOR_LINEUP: &str = bkco_output::BKCO_VERB_LINEUP;
const ZBKM_DOOR_TATTOO: &str = bkco_output::BKCO_VERB_TATTOO;
const ZBKM_DOOR_HEEL: &str = bkco_output::BKCO_VERB_HEEL;
const ZBKM_DOOR_MUZZLE: &str = bkco_output::BKCO_VERB_MUZZLE;
const ZBKM_DOOR_GANGLINE: &str = bkco_output::BKCO_VERB_GANGLINE;
const ZBKM_DOOR_SCOOP: &str = bkco_output::BKCO_VERB_SCOOP;
const ZBKM_DOOR_DELOUSE: &str = bkco_output::BKCO_VERB_DELOUSE;
const ZBKM_DOOR_BLEACH: &str = bkco_output::BKCO_VERB_BLEACH;

/// Rebed's binary half: every heel and strike the derby's preflight names, and
/// every manifest's registry closure, performed.
///
/// A WORD OF THE BINARY AND NO DOOR OF THE REGISTER. Its tabtarget reaches the
/// whistle's bash first, which lays the toolchains and converges the kennel and
/// then execs this word, so it takes an arm here for the reason every door does
/// — a word falling through to the leash register would reach cargo as a verb.
const ZBKM_REBED: &str = bkco_output::BKCO_VERB_REBED;

/// What fills each blank door, said as the work rather than as the pace that
/// carries it. A reader who typed a door needs to know what the door will DO
/// when it stands; the plan that gets it there is not their question, and a
/// pace identity in an error message would age into a dangling reference.

/// The converge a stale app election names.
///
/// IT IS THE KENNEL'S OWN DOOR AND NOT THE TREE'S. Which build door a given
/// crate keeps is a fact the kennel does not hold and must not learn — a collar
/// names a manifest and a target, never a door — so the act named is the one the
/// freeze reserves for exactly this, whose job is to build the launchable current
/// through the leash and install it at its residence.
/// It is spelled as the door rather than as a word, so the two cannot come to
/// disagree: whatever heel is called, this names it.
const ZBKM_CONVERGE: &str = ZBKM_DOOR_HEEL;

/// The geography this seat stands in, derived at the door face and threaded
/// from here.
///
/// GIT'S OWN ANSWER FROM INSIDE THE REPOSITORY THE DOOR ENTERED, and nothing
/// else: no config directory is read for it, no variable names it and no file
/// states it. Every door that consumes a delivered root calls this and passes
/// the value down the way it passes a repository, so a hurdle poses a geography
/// by standing a real linked worktree rather than by mutating the process every
/// other hurdle in the same runner is also standing in.
///
/// CALLED BY THE ARMS THAT CONSUME A DELIVERED ROOT AND BY NO OTHER, which is
/// the reading's laziness held where it is decided. The bare self-report above
/// returns before any door arm is reached, and it must: `bkcp_position.sh` runs
/// the kennel bare and treats any non-zero exit as no position, so a geography
/// refusal standing ahead of that report would counterfeit staleness at the
/// whistle and at the build module's gate. The derby's preflight is such an
/// arm — it holds the election over every app collar, behind the delouse and
/// ahead of the walk — and it takes the reading inside the derby arm, so the
/// bare self-report has already returned. Suites, the muzzle, lineup, gangline,
/// scoop and the leash register never call it.
///
/// A GEOGRAPHY THAT DOES NOT DERIVE FENCES THE DOOR, never a fallback to self:
/// a fallback would turn a worktree whose main worktree is gone into a quiet own
/// build, and the borrow candidate would simply never be consulted.
fn zbkm_geography(repository: &Path) -> bkce_election::bkce_Geography {
    match bkce_election::bkce_geography(repository) {
        Ok(geography) => geography,
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            zbkm_fenced();
        }
    }
}

/// The families the substrate speaks across an exec, each a prefix every name
/// of one wears.
const ZBKM_KIT_FAMILIES: &[&str] = &[
    "BUC_",
    "BURC_",
    "BURD_",
    "BURE_",
    "BURS_",
    "BURV_",
    "BURX_",
    "BUZ_", //
];

/// The environment regime's family, which every process holds as the operator's
/// ambient.
const ZBKM_AMBIENT_FAMILY: &str = "BURE_";

/// The two hand-offs a door holds beside the environment regime: the pointer,
/// and the kit directory.
const ZBKM_HANDOFFS: &[&str] = &[
    bkcr_resolve::BKCR_DISPATCH_FILE_VAR,
    "BURD_BUK_DIR", //
];

/// The environment regime's test-seam pair, which no door of the kennel honors.
const ZBKM_SEAM: &[&str] = &[
    "BURE_TWEAK_NAME",
    "BURE_TWEAK_VALUE", //
];

/// The kennel's start: no name of the substrate stands in its environment save
/// the environment regime and the two hand-offs, and no test seam is set.
///
/// THE EXPORTED-NAME RULE OF THE SUBSTRATE'S SCOPE SENTINEL, applied where the
/// kennel stands: it is the one program of the estate that stands where a door
/// stands, so it reads its environment as a door's entry does. A dispatch value
/// arriving here crossed by inheritance rather than through the file the pointer
/// names, and a seam set here would ride silently into a door that reads none.
///
/// FENCED, AHEAD OF ANY SUBJECT: after the bare self-report, which needs no
/// environment and whose probe must not counterfeit staleness, and before the
/// dispatch is read.
fn zbkm_inherited() {
    let mut held: Vec<(String, bool)> = std::env::vars_os()
        .map(|(name, value)| (name.to_string_lossy().into_owned(), !value.is_empty()))
        .collect();
    held.sort();

    for (name, set) in &held {
        if !ZBKM_KIT_FAMILIES
            .iter()
            .any(|family| name.starts_with(family))
        {
            continue;
        }
        if *set && ZBKM_SEAM.contains(&name.as_str()) {
            bkco_error_now!(
                "{} is set at the kennel's start, and no door of the kennel honors a test seam: unset it",
                name, //
            );
            zbkm_fenced();
        }
        if name.starts_with(ZBKM_AMBIENT_FAMILY) || ZBKM_HANDOFFS.contains(&name.as_str()) {
            continue;
        }
        bkco_error_now!(
            "{} stands in the kennel's environment at its start, and no value of the substrate crosses an exec by inheritance save the environment regime, {} and {} (the exported-name rule)",
            name,
            ZBKM_HANDOFFS[0],
            ZBKM_HANDOFFS[1], //
        );
        zbkm_fenced();
    }
}

/// The dispatch this door runs under, its weaned set proven whole before any
/// door reads it.
///
/// TAKEN AT THE DOOR FACE AND THREADED FROM HERE, the geography's own shape: the
/// file the pointer names is read once at this edge and every reading below
/// takes its values as a value (`bkcr_dispatched`). A pointer absent, or naming
/// no readable file, is a broken launch contract and ends the door broke.
///
/// A SET THAT DOES NOT CONFORM FENCES THE DOOR before any door arm is
/// reached — a copy drifted from its brood, a name two kits' broods both
/// declare, or an instance carrying other than one family file. Fail-fast by
/// ruling, never a report: the first nonconformance refuses.
///
/// AFTER THE BARE SELF-REPORT, which returns before any door arm and must: a
/// position probe is not a door, and a dispatch or set refusal standing ahead of
/// it would counterfeit staleness at the whistle.
fn zbkm_dispatched(repository: &Path) -> bkcr_resolve::bkcr_Dispatch {
    let dispatch = match bkcr_resolve::bkcr_dispatched() {
        Ok(dispatch) => dispatch,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if let Err(err) = bkcr_resolve::bkcr_weaned_audit(repository, dispatch.bkcr_weaned()) {
        bkco_error_now!(
            "{}",
            err, //
        );
        zbkm_fenced();
    }

    dispatch
}

/// The process's own entry: the door face, run so that broke can reach it.
///
/// BROKE ARRIVES HERE AS AN UNWIND and leaves as the standard library's failure
/// code, the one road the pinned channel offers a door that meets broke at any
/// depth (`bkco_output::bkco_Broke`). Every other end of a door exits where the
/// door decides it, sound at zero and barred or fenced at the codes the generated
/// constants carry, so this match sees a door's return only from the bare
/// self-report.
///
/// A GENUINE PANIC IS NOT BROKE and is not answered as one. Its payload is
/// handed back to the unwind it came from, which ends the process as a panic
/// does, its hook having already said where.
fn main() -> std::process::ExitCode {
    match std::panic::catch_unwind(zbkm_main) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(payload) if payload.is::<bkco_output::bkco_Broke>() => std::process::ExitCode::FAILURE,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// The door face: the door law, then the door the first argument names.
fn zbkm_main() {
    let repository = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => bkco_fatal_now!(
            "could not read the working directory: {}",
            err, //
        ),
    };

    zbkm_door_law(&repository);

    let arguments: Vec<OsString> = std::env::args_os().skip(1).collect();

    if arguments.is_empty() {
        let position = bkcs_stamp::bkcs_position();
        for line in position.bkcs_render().lines() {
            bkco_info_now!(
                "{}",
                line, //
            );
        }
        return;
    }

    // ONE ARM PER DOOR, whether or not its interior stands, and the register
    // catches only what is no door at all. A word that fell through to the
    // register would reach cargo as a verb, which is the one outcome a door
    // still being built may never produce.
    let door = arguments[0].to_string_lossy().into_owned();
    let rest = &arguments[1..];

    zbkm_inherited();

    let dispatch = zbkm_dispatched(&repository);

    match door.as_str() {
        ZBKM_DOOR_MUSH => zbkm_mush(&repository, &dispatch, rest),
        ZBKM_DOOR_TATTOO => zbkm_tattoo(&repository, &dispatch, rest),
        ZBKM_DOOR_MUZZLE => zbkm_muzzle(&repository, &dispatch, rest),
        ZBKM_DOOR_DERBY => zbkm_derby(&repository, &dispatch, rest),
        ZBKM_DOOR_FUTURITY => zbkm_futurity(&repository, &dispatch, rest),
        ZBKM_DOOR_LINEUP => zbkm_lineup(&repository, &dispatch, rest),
        ZBKM_DOOR_HEEL => zbkm_heel(&repository, &dispatch, rest),
        ZBKM_DOOR_GANGLINE => zbkm_gangline(&repository, &dispatch, rest),
        ZBKM_DOOR_SCOOP => zbkm_scoop(&repository, &dispatch, rest),
        ZBKM_DOOR_DELOUSE => zbkm_delouse(&repository, &dispatch, rest),
        ZBKM_DOOR_BLEACH => zbkm_bleach(&dispatch, rest),
        ZBKM_REBED => zbkm_rebed(&repository, &dispatch, rest),
        _ => zbkm_register(&repository, &dispatch, &arguments),
    }
}

/// *gangline* — re-derive one collar's lock, with the lock flag lifted.
///
/// THE DOOR LAW STANDS AHEAD OF THIS AS IT DOES AHEAD OF EVERY DOOR, and here
/// it is load-bearing twice over rather than merely uniform: the lock this door
/// writes is the only thing that may be dirty when the drive ends, so a tree
/// that was already dirty would leave the operator unable to tell the door's
/// own output from whatever else was standing. The refusal is what makes the
/// diff readable, and the sequence a caller lives is notch, gangline, notch.
///
/// IT TAKES ONE COLLAR AND NOT THE WALK. Re-deriving a lock is a deliberate act
/// over a manifest whose dependencies the operator just changed; sweeping every
/// collar would re-derive locks nobody asked about, which is the one shape a
/// door that lifts the lock flag may never take.
///
/// THE ARREARS READING DOES NOT WIDEN THAT, and the sentence above stays true of
/// what this door WRITES. What follows the write is a reading: every collar whose
/// closure reaches the manifest just re-derived, probed for whether its own lock
/// still answers, and each that does not NAMED — never written. The verdict is
/// red while any stands, so a caller who admitted a dependency is told the whole
/// debt at this door rather than one refusal at a time at whichever door meets it
/// next.
///
/// THE ROSTER IS GATHERED AHEAD OF THE WRITE, which is a sequencing choice and
/// not an accident: the gather refuses a register that does not conform, and a
/// refusal arriving AFTER the lift would leave a lock written into a tree the
/// door had then declined to finish reading.
fn zbkm_gangline(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    rest: &[OsString],
) -> ! {
    if rest.len() != 1 {
        bkco_fatal_now!(
            "'{}' takes exactly one collar as its imprint and was given {} — re-deriving a lock is \
             a deliberate act over one manifest, and this door does not sweep",
            ZBKM_DOOR_GANGLINE,
            rest.len(), //
        );
    }

    let name = rest[0].to_string_lossy().into_owned();

    // A PYTHON COLLAR IS RE-DERIVED BY THIS DOOR TOO, and its arm stands FIRST
    // on heel's own precedent: the family is decided by where the name stands
    // and never by a flag, because a caller typing a name knows what it names
    // and a door that made them say twice would be asking them to restate the
    // tree.
    if let Ok(resolved) = bkcx_python::bkcx_resolve(repository, dispatch, &name) {
        zbkm_gangline_python(repository, dispatch, resolved);
    }

    // THE REFUSAL ACCOUNTS FOR BOTH COLLAR FAMILIES, on heel's ground: the rust
    // walk names the rust collars and is correct about what it says, but it
    // cannot say the name may have been a python collar's, that family's walk
    // not being its to take.
    let resolved = match bkcr_resolve::bkcr_resolve(repository, dispatch, &name) {
        Ok(resolved) => resolved,
        Err(err) => {
            let python = bkcx_python::bkcx_roster(repository, dispatch);
            if python.is_empty() {
                bkco_fatal_now!(
                    "{}",
                    err, //
                );
            }
            bkco_fatal_now!(
                "{}\n  and no python collar wears it either — that family carries: {}",
                err,
                python, //
            );
        }
    };

    let roster = zbkm_collars(repository, dispatch, "re-derived");

    bkco_info_now!(
        "re-deriving the lock of {} — the one act in which the leash lifts the lock flag",
        resolved.collar.name, //
    );

    match bkcg_gangline::bkcg_gangline(repository, dispatch, &resolved.collar) {
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            bkco_output::bkco_broke();
        }
        Ok(bkcg_gangline::bkcg_Verdict::Unmoved) => {
            bkco_answer_now!(
                "the lock already answered its manifest — nothing was written, and the tree is no \
                 dirtier than this door found it"
            );
        }
        Ok(bkcg_gangline::bkcg_Verdict::Moved) => {
            bkco_answer_now!(
                "the lock was re-derived and stands DIRTY for your notch — read its diff before \
                 you bank it, because this door writes a lock and never commits one"
            );
        }
    }

    zbkm_arrears(repository, dispatch, &resolved.collar, &roster);
}

/// Re-derive one python collar's lock: the other half of *gangline*.
///
/// SEPARATE FROM THE RUST ARM BECAUSE THE ACTS ANSWER TO DIFFERENT TOOLS, and
/// they share a door word for the reason the two heel arms do — an operator
/// re-deriving a lock does not care which resolver owns it.
///
/// THE ARREARS PROBE IS NOT TAKEN HERE, AND SAYING SO IS THE POINT. That reading
/// walks cargo's own closure to name every lock a re-derivation left behind, and
/// a python project stands in no cargo closure at all: probing rust manifests on
/// a python collar's account would be answering a question nobody asked, and
/// going quiet would let a reader take the rust arm's silence for this one's.
fn zbkm_gangline_python(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    resolved: bkcx_python::bkcx_Resolved,
) -> ! {
    let name = resolved.collar.name.clone();

    // AN ABSENT LOCK IS THE ONE FINDING THIS DOOR READS PAST, its act being the
    // repair; every other finding bars the write as it bars every door.
    let barring = resolved.bkcx_findings_but_lock();

    if !barring.is_empty() {
        bkco_error_now!(
            "the python collar '{}' at {} carries {} finding(s), and no door operates on a collar \
             that does not conform — least of all one that writes:",
            name,
            resolved.collar.instance.display(),
            barring.len(), //
        );
        for finding in barring {
            bkco_error_now!(
                "  {}",
                finding, //
            );
        }
        zbkm_barred();
    }

    if resolved.lockless {
        bkco_info_now!(
            "the project at {} carries no lock yet, so this drive authors its first",
            resolved.collar.bkcx_project().display(), //
        );
    }

    let seat = match bkcx_python::bkcx_seat(dispatch, &resolved.collar) {
        Ok(seat) => seat,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    bkco_info_now!(
        "re-deriving the lock of the python collar {} — the one act in which the leash lifts the \
         lock flag",
        name, //
    );

    match bkcg_gangline::bkcg_python(repository, dispatch, &resolved.collar, &seat) {
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            bkco_output::bkco_broke();
        }
        Ok(bkcg_gangline::bkcg_Verdict::Unmoved) => {
            bkco_answer_now!(
                "the lock already answered its manifest — nothing was written, and the tree is no \
                 dirtier than this door found it"
            );
        }
        Ok(bkcg_gangline::bkcg_Verdict::Moved) => {
            bkco_answer_now!(
                "the lock was re-derived and stands DIRTY for your notch — read its diff before \
                 you bank it, because this door writes a lock and never commits one"
            );
        }
    }

    bkco_info_now!(
        "no downstream lock was probed: the arrears reading walks cargo's closure, which a python \
         project stands in nowhere"
    );

    std::process::exit(0);
}

/// Name every lock the write left behind what it owes, and take the verdict.
///
/// THE READING IS TAKEN WHETHER THE LOCK MOVED OR NOT, deliberately. A manifest
/// whose own lock already answered can still stand upstream of a collar whose
/// does not — that is precisely the standing the trunk was in when a lock
/// re-derived one day earlier left the engine's consumer behind — so a reading
/// gated on `Moved` would go quiet in the one case worth reporting.
fn zbkm_arrears(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    written: &bkcr_resolve::bkcr_Collar,
    roster: &[bkcr_resolve::bkcr_Collar],
) -> ! {
    let (probed, arrears) = match bkcg_gangline::bkcg_arrears(repository, dispatch, written, roster)
    {
        Ok(reading) => reading,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    // THE COUNT RIDES BOTH VERDICTS, and on the green one it is the load-bearing
    // half. Finding nothing is this reading's ordinary answer, so a green saying
    // only that nothing was found cannot be told from a walk that reached
    // nothing — and `0 probed` is a coverage report, never a clean tree.
    if arrears.is_empty() {
        bkco_answer_now!(
            "{} downstream lock(s) probed, none behind — every collar whose closure reaches this \
             manifest still answers under the lock flag",
            probed, //
        );
        std::process::exit(0);
    }

    bkco_error_now!(
        "{} of {} downstream lock(s) stand behind what they owe — this door names them and writes \
         none:",
        arrears.len(),
        probed, //
    );

    // ONE DRIVE IS SPELLED AND THE REST ARE NAMED, because the door takes one
    // collar and the debt belongs to the manifest: any collar over it settles the
    // lock, and spelling several would read as several drives owed.
    for arrear in &arrears {
        // A manifest reaches this list only by a collar having declared it, so
        // the empty arm is unreachable — and it is written rather than asserted
        // away, because a reader owed a debt is owed the manifest even in a
        // shape that should not arise.
        match arrear.collars.split_first() {
            Some((drive, over)) => {
                bkco_error_now!(
                    "  {} no longer answers its manifest — drive: {} {}",
                    arrear.manifest,
                    ZBKM_DOOR_GANGLINE,
                    drive, //
                );

                if !over.is_empty() {
                    bkco_error_now!(
                        "    (the same manifest is also declared by: {})",
                        over.join(", "), //
                    );
                }
            }
            None => bkco_error_now!(
                "  {} no longer answers its manifest",
                arrear.manifest, //
            ),
        }
    }

    zbkm_barred();
}

/// The door law, applied. Every door refuses an uncommitted repository, this one
/// included, and the refusal names the remedy rather than merely reporting the
/// state — a reader who is told the tree is dirty and not what to do about it has
/// been handed a search rather than a remedy.
///
/// It stands ahead of the register as well as ahead of the report, and the
/// ordering is the law's whole reach: a leash invocation is a build or a test,
/// which is exactly the record that must map to a position.
///
/// The library face holds no such posture and must not: see the crate docs.
fn zbkm_door_law(repository: &PathBuf) {
    // THE FIRST GIT SPAWN OF EVERY DOOR INVOCATION STANDS HERE, which is why the
    // pin is proven here. The law reads the repository through git before any
    // door runs, so a station holding no git — or one below the floor the
    // kennel's own invocations need — meets a sentence naming the program and
    // the floor rather than a failure to read a tree, which is what it would
    // otherwise look like. The verdict is cached for the process, so the four
    // further git spawns a door may make cost nothing.
    if let Err(err) = bkcl_leash::bkcl_git_held() {
        bkco_error_now!(
            "{}",
            err, //
        );
        zbkm_fenced();
    }

    let standing = match bkcf_guard::bkcf_standing(repository) {
        Ok(standing) => standing,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if !standing.bkcf_clean() {
        bkco_error_now!(
            "{}",
            standing.bkcf_grievance(repository), //
        );
        zbkm_fenced();
    }

    // THE LYAM IS JUDGED WHERE CLEANLINESS IS, and after it: the listing it reads
    // is the tracked tree, which only a clean tree makes the same as the one on
    // disk. Standing here, it reaches every door ahead of any hurdle a course
    // would launch.
    let lyam = match bkcf_guard::bkcf_lyam(repository) {
        Ok(lyam) => lyam,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if !lyam.bkcf_clean() {
        bkco_error_now!(
            "{}",
            lyam.bkcf_grievance(repository), //
        );
        zbkm_fenced();
    }

    for include in &lyam.skipped {
        bkco_trace_now!(
            "include skipped, outside the repository: {}:{}: @{}",
            include.file,
            include.line,
            include.path, //
        );
    }
}

/// *tattoo* — read one collar, proclaim it, and report every finding.
///
/// THE PROCLAMATION IS ANSWERED WHATEVER THE FINDINGS SAY, and the ordering is
/// deliberate: a reader looking at a collar that does not conform needs to see
/// what it declares most, and a door that withheld the answer until the collar
/// was clean would be least useful exactly when it was most wanted. What the
/// findings change is the EXIT, never whether the question is answered.
///
/// It changes nothing on disk, which is what makes it the resolver's read-only
/// face.
fn zbkm_tattoo(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if rest.len() != 1 {
        bkco_fatal_now!(
            "'{}' takes exactly one collar as its imprint and was given {} — the frozen shape is \
             the verb, the collar, and then what the collar's kind owns, and this door's kind owns \
             nothing",
            ZBKM_DOOR_TATTOO,
            rest.len(), //
        );
    }

    let name = rest[0].to_string_lossy().into_owned();

    // A PYTHON COLLAR IS PROCLAIMED BY THIS DOOR TOO, its arm standing first on
    // heel's and gangline's precedent: the family is decided by where the name
    // stands and never by a flag.
    if let Ok(resolved) = bkcx_python::bkcx_resolve(repository, dispatch, &name) {
        zbkm_tattoo_python(repository, dispatch, resolved);
    }

    // THE REFUSAL ACCOUNTS FOR BOTH COLLAR FAMILIES, on gangline's ground: the
    // rust walk is correct about what it names, and cannot say the name may have
    // been a python collar's.
    let resolved = match bkcr_resolve::bkcr_resolve(repository, dispatch, &name) {
        Ok(resolved) => resolved,
        Err(err) => {
            let python = bkcx_python::bkcx_roster(repository, dispatch);
            if python.is_empty() {
                bkco_fatal_now!(
                    "{}",
                    err, //
                );
            }
            bkco_fatal_now!(
                "{}\n  and no python collar wears it either — that family carries: {}",
                err,
                python, //
            );
        }
    };

    // THE ELECTION IS READ HERE AND NOT IN THE RENDER, which is what keeps the
    // proclamation a pure function of a collar. It is a reading of a repository
    // and of a standing binary, and this door is the one holding a repository.
    //
    // A READ-ONLY DOOR MAY HOLD IT: the election spawns the standing binary to
    // ask what it was struck from and changes nothing on disk, which is the same
    // posture the whole door keeps.
    let geography = zbkm_geography(repository);

    let elected = match bkce_election::bkce_elected(repository, &geography, &resolved.collar) {
        Ok(elected) => elected,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    for line in bkct_tattoo::bkct_proclamation(&resolved.collar, elected, &geography).lines() {
        bkco_answer_now!(
            "{}",
            line, //
        );
    }

    if resolved.findings.is_empty() {
        bkco_info_now!(
            "the collar '{}' at {} conforms: 0 finding(s) stand against it",
            name,
            resolved.collar.instance.display(), //
        );
        std::process::exit(0);
    }

    bkco_error_now!(
        "the collar '{}' at {} carries {} finding(s), every one of them reported before anything \
         refuses:",
        name,
        resolved.collar.instance.display(),
        resolved.findings.len(), //
    );

    for finding in &resolved.findings {
        bkco_error_now!(
            "  {}",
            finding, //
        );
    }

    zbkm_barred();
}

/// Proclaim one python collar: the other half of *tattoo*.
///
/// THE ELECTION IS THE SEAT'S OWN, and that is the true answer rather than a
/// slot filled to keep the shape. A python launchable runs in the environment the
/// converge sited under this checkout's own loosebox, so nothing is borrowed and
/// no standing artifact exists for a vintage question to be asked about — the
/// fact a rust suite's election already states (`bkce_election::bkce_elected`).
fn zbkm_tattoo_python(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    resolved: bkcx_python::bkcx_Resolved,
) -> ! {
    let geography = zbkm_geography(repository);

    let proclaimed = bkct_tattoo::bkct_proclamation_python(
        &resolved.collar,
        bkce_election::BKCE_ELECTED_OWN,
        &geography, //
    );

    for line in proclaimed.lines() {
        bkco_answer_now!(
            "{}",
            line, //
        );
    }

    if resolved.findings.is_empty() {
        zbkm_tattoo_unhitched(repository, dispatch, &resolved.collar);

        bkco_info_now!(
            "the python collar '{}' at {} conforms: 0 finding(s) stand against it",
            resolved.collar.name,
            resolved.collar.instance.display(), //
        );
        std::process::exit(0);
    }

    bkco_error_now!(
        "the python collar '{}' at {} carries {} finding(s), every one of them reported before \
         anything refuses:",
        resolved.collar.name,
        resolved.collar.instance.display(),
        resolved.findings.len(), //
    );

    for finding in &resolved.findings {
        bkco_error_now!(
            "  {}",
            finding, //
        );
    }

    zbkm_barred();
}

/// Say whether a conforming python collar is unhitched on this station.
///
/// A VERDICT AND NO FINDING. The declaration is the project's own and the
/// collar is sound under it, so it moves no exit; it is said beside the clean
/// verdict, which a reader would otherwise take to mean the collar runs here.
/// A reading that could not be taken — uv unplaced, say — is said as such and
/// moves nothing either, the door's question being the collar's conformance.
fn zbkm_tattoo_unhitched(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_python::bkcx_Collar,
) {
    let read = bkcx_python::bkcx_seat(dispatch, collar)
        .and_then(|seat| bkcx_python::bkcx_unhitched_at(repository, dispatch, collar, &seat));

    match read {
        Ok(Some(unhitched)) => bkco_info_now!(
            "{}; its courses launch nothing here, and the declaration is no finding",
            unhitched.bkcx_said(collar), //
        ),
        Ok(None) => {}
        Err(err) => bkco_info_now!(
            "whether the project of the python collar '{}' declares this station out could not \
             be read: {}",
            collar.name,
            err, //
        ),
    }
}

/// *mush* — the launch seam. Launch the launchable one collar names.
///
/// THE DOOR'S OWN HELP, and it lists no option that names a binary choice
/// because there is none to list. Which binary answers is DERIVED from the
/// collar's spend class and the seat's own position, never declared: a flag
/// offering the choice would make a launch's most consequential decision its
/// least visible one, and would let a caller run a vintage the record could not
/// account for. The shape admits the verb, the collar, and what the collar's kind
/// owns — nothing else, at this door or any other.
const ZBKM_MUSH_HELP: &str = "mush <collar> [what the collar's kind owns]\n  \
     an app runs the binary the election names, its arguments passed through verbatim\n  \
     a suite runs under the runner the collar declares, through the leash\n  \
     a suite's arguments are selection patterns, matched in the kennel in the rust regex\n  \
     dialect against the runner's own listing; the runner is then handed exact names\n  \
     no flag rides the shape, and no option names which binary answers";

/// *mush* — launch what one collar names.
///
/// NEVER OPERATE ON AN INVALID COLLAR. Every finding is reported and then the
/// door refuses, ahead of anything being launched — which is validation being
/// implicit in every door rather than a step a caller elects. Tattoo answers the
/// proclamation anyway and refuses only the exit, because a reader looking at a
/// broken collar most needs to see what it declares; this door has no such
/// answer to withhold, and launching something a finding stands against is the
/// act the rule exists to stop.
fn zbkm_mush(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if rest.is_empty() {
        bkco_fatal_now!(
            "{}",
            ZBKM_MUSH_HELP, //
        );
    }

    let name = rest[0].to_string_lossy().into_owned();
    let carried = &rest[1..];

    // A PYTHON COLLAR IS LAUNCHED BY THIS DOOR TOO, and its arm parts HERE, at
    // the top, on heel's own precedent: the family is decided by WHERE THE NAME
    // STANDS and never by a flag, because a caller typing a name knows what it
    // names and a door that made them say twice would be asking them to restate
    // the tree.
    //
    // IT PARTS AHEAD OF THE RUST WALK RATHER THAN INSIDE IT, which is what the
    // two families being two TYPES forces and is honest about: a python collar
    // shares the genus's fields and none of its launch — no manifest, no
    // profile, no build shape, and a runner reached through a different leash
    // face entirely. A branch further down would be one function describing two
    // disciplines, which is the shape the leash's own uv composition already
    // refused.
    if let Ok(resolved) = bkcx_python::bkcx_resolve(repository, dispatch, &name) {
        zbkm_mush_python(repository, dispatch, resolved, &name, carried);
    }

    let resolved = zbkm_sound_collar(repository, dispatch, &name);

    if resolved.collar.bkcr_app() {
        zbkm_mush_app(repository, dispatch, &resolved.collar, &name, carried);
    }

    zbkm_mush_suite(repository, dispatch, &resolved.collar, &name, carried);
}

/// Launch what one PYTHON collar names: the third arm of *mush*.
///
/// NEVER OPERATE ON AN INVALID COLLAR, on the rust arm's own ground. The gate
/// stands ahead of the verify rather than behind it, because what the validation
/// refuses includes the very declarations the verify would read — a pin that is a
/// bare floor, a project missing its lock — so verifying past a finding would be
/// asking whether an environment answers declarations the kennel had already
/// judged unfit.
fn zbkm_mush_python(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    resolved: bkcx_python::bkcx_Resolved,
    name: &str,
    carried: &[OsString],
) -> ! {
    if !resolved.findings.is_empty() {
        bkco_error_now!(
            "the python collar '{}' at {} carries {} finding(s), and no door operates on a collar \
             that does not conform:",
            name,
            resolved.collar.instance.display(),
            resolved.findings.len(), //
        );
        for finding in &resolved.findings {
            bkco_error_now!(
                "  {}",
                finding, //
            );
        }
        zbkm_barred();
    }

    let collar = resolved.collar;

    let seat = match bkcx_python::bkcx_seat(dispatch, &collar) {
        Ok(seat) => seat,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    // THE VERIFY IS THE FIRST STEP AND IT REFUSES RATHER THAN CONVERGING, which
    // is the routine-download ruling's own shape: every door but the converge
    // asks whether the environment still answers its declarations and names the
    // converge where it does not. Its refusals already name that door, so nothing
    // here restates it.
    if let Err(err) = bkcx_python::bkcx_verify_at(repository, dispatch, &collar, &seat) {
        bkco_error_now!(
            "{}",
            err, //
        );
        zbkm_fenced();
    }

    if collar.bkcx_app() {
        zbkm_mush_python_app(dispatch, &collar, &seat, name, carried);
    }

    zbkm_mush_python_suite(repository, dispatch, &collar, &seat, name, carried);
}

/// The python app arm: run the entry point the collar names, under the venv the
/// kennel sited.
///
/// THE EXIT IS THE TENANT'S VERDICT, exactly as the rust app arm's is: a tenant
/// exiting sound, barred or fenced crosses the door as that verdict, and any
/// other code ends the door broke beside a line naming it (`zbkm_relayed`). The
/// tenant's output already went to the terminal, so what the kennel adds is the
/// verdict and never an opinion of the run.
///
/// REACHED BY ABSOLUTE PATH AND NEVER BY ACTIVATION. Nothing here puts the venv
/// on a `PATH` or exports the venv's prefix: the entry point stands at one place
/// the kennel itself decided, and it is spawned from there. The one variable it
/// is handed is the bytecode root (`bkcx_python::bkcx_app_bytecode`).
fn zbkm_mush_python_app(
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_python::bkcx_Collar,
    seat: &bkcx_python::bkcx_Seat,
    name: &str,
    carried: &[OsString],
) -> ! {
    let entry = bkcx_python::bkcx_entry(seat, collar);

    bkco_info_now!(
        "the python collar '{}' runs {} — the entry point its project declares, in the \
         environment the converge sited",
        name,
        entry.display(), //
    );

    // THE APP ARM WRITES A RECORD AND NOT A TRANSCRIPT, on the rust arm's own
    // ground: this arm hands the tenant THIS PROCESS'S STREAMS, an app's
    // arguments and output passing through verbatim, so the family says plainly
    // that what the tenant said went to the terminal.
    if let Err(err) = zbkm_python_app_record(
        dispatch,
        &bkcv_voice::bkcv_Record::bkcv_invocation(),
        &entry,
        name,
    ) {
        bkco_error_now!(
            "{}",
            err, //
        );
    }

    let (bytecode_var, bytecode_root) = bkcx_python::bkcx_app_bytecode(seat);

    let status = std::process::Command::new(&entry)
        .args(carried)
        .env(bytecode_var, bytecode_root)
        .status()
        .unwrap_or_else(|err| {
            bkco_fatal_now!(
                "no entry point answered at {}: {} — the byname names the key the project's \
                 scripts table declares, so either the project declares no such script or the \
                 environment was synced before it did",
                entry.display(),
                err, //
            )
        });

    match status.code() {
        Some(code) => zbkm_relayed(
            code,
            &format!(
                "the python collar '{}'",
                name, //
            ),
        ),
        None => bkco_fatal_now!(
            "a signal took {}",
            entry.display(), //
        ),
    }
}

/// Write the python app arm's record: what was run, and the plain statement that
/// the tenant's own output is not in here.
fn zbkm_python_app_record(
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    invocation: &str,
    entry: &Path,
    name: &str,
) -> Result<(), String> {
    let mut record = match bkcv_voice::bkcv_Record::bkcv_open(dispatch, invocation)? {
        Some(record) => record,
        None => return Ok(()),
    };

    record.bkcv_write(&format!(
        "the python collar '{}' runs {}",
        name,
        entry.display(), //
    ))?;
    record.bkcv_write(
        "the tenant holds the terminal: an app's arguments and output pass through verbatim, so \
         what it said went to the console and stands in no member of this family",
    )?;
    record.bkcv_close()
}

/// The python suite arm: list, match here, hand pytest exact node ids, run.
///
/// THE SAME FOUR STEPS THE RUST ARM DRIVES, and the dialect is the same one: a
/// pattern is a rust regular expression matched IN THE KENNEL against the
/// runner's own listing, and what crosses the boundary is a set of names.
///
/// NO MUZZLE RUNS HERE, AND THE ABSENCE IS A RULING. The muzzle is the rust
/// plane's lint step; the python guide's linter election is deliberately
/// deferred, so this arm records no muzzle marks rather than inventing a python
/// lint for the sake of symmetry.
///
/// THE BARE DRIVE IS UNTOUCHED BY THE PIPELINE, on the rust arm's own ground: a
/// suite asked for whole spawns no listing, the pipeline being what a NARROWING
/// costs.
fn zbkm_mush_python_suite(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_python::bkcx_Collar,
    seat: &bkcx_python::bkcx_Seat,
    name: &str,
    carried: &[OsString],
) -> ! {
    let (selection, narrowing) = if carried.is_empty() {
        (Vec::new(), None)
    } else {
        let (spelled, cut) =
            zbkm_python_narrowed(repository, dispatch, collar, seat, name, carried);
        (spelled, Some(cut))
    };

    let began = std::time::Instant::now();

    let (run, tally) =
        match bkcm_mush::bkcm_python_suite(repository, dispatch, collar, seat, &selection) {
            Ok(answered) => answered,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

    let taken = began.elapsed();

    bkco_trace_now!(
        "{} — {} case(s) counted",
        run.spelling,
        tally.ran, //
    );

    zbkm_python_hollow(name, &run, &tally);

    zbkm_record(dispatch, name, &run, &tally, narrowing.as_ref(), taken);

    zbkm_exit_as_ran(&run)
}

/// Drive the narrowing over a python collar: list, match, spell the node ids.
///
/// A PATTERN SET MATCHING NOTHING REFUSES AND SPAWNS NOTHING, which bites harder
/// here than at either rust runner. Handed a node id it cannot find, pytest
/// answers `ERROR: not found` and collects zero — so a kennel that passed the
/// selection through would land in the hollow gate and report a suite that never
/// ran as one that ran nothing, which is the same silence by a longer road. The
/// count is the kennel's own and the refusal stands ahead of the spawn.
fn zbkm_python_narrowed(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_python::bkcx_Collar,
    seat: &bkcx_python::bkcx_Seat,
    name: &str,
    carried: &[OsString],
) -> (Vec<OsString>, bkci_pipeline::bkci_Narrowing) {
    let patterns: Vec<String> = carried
        .iter()
        .map(|pattern| pattern.to_string_lossy().into_owned())
        .collect();

    let held = match bkcx_python::bkcx_recite(repository, dispatch, collar, seat) {
        Ok(held) => held,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let narrowing = match bkci_pipeline::bkci_choose(&held, &patterns) {
        Ok(narrowing) => narrowing,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if narrowing.chosen.is_empty() {
        bkco_error_now!(
            "the {} selection pattern(s) given to '{}' matched none of the {} hurdle(s) the \
             python collar '{}' holds. NOTHING WAS SPAWNED: a runner handed a selection that \
             matches nothing reports having run nothing, and a verdict for a suite that never ran \
             is the one answer a launch may never give. Drive '{} {}' to see the names the \
             patterns are matched against",
            patterns.len(),
            ZBKM_DOOR_MUSH,
            narrowing.held.len(),
            name,
            ZBKM_DOOR_LINEUP,
            name, //
        );
        zbkm_barred();
    }

    let selection =
        match bkci_pipeline::bkci_selection(collar.bkcx_field("BKRP_RUNNER"), &narrowing.chosen) {
            Ok(selection) => selection,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

    bkco_info_now!(
        "the python collar '{}' narrows: {}",
        name,
        narrowing.bkci_verdict(), //
    );

    (selection, narrowing)
}

/// The app arm: hold the election, then run what it named.
///
/// THE EXIT IS THE TENANT'S VERDICT. A tenant exiting sound, barred or fenced
/// crosses the door as that verdict, so a program that rules on what it reads —
/// the census tool on its subject road among them — is read by its caller exactly
/// as it would be run bare. Any other code names no verdict and ends the door
/// broke beside a line naming it (`zbkm_relayed`): a tenant's own numbering,
/// passed on raw, would be a contract the kennel never made.
fn zbkm_mush_app(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcr_resolve::bkcr_Collar,
    name: &str,
    carried: &[OsString],
) -> ! {
    let geography = zbkm_geography(repository);

    let verdict = match bkce_election::bkce_elect(repository, &geography, collar) {
        Ok(verdict) => verdict,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let Some(answers) = verdict.answers.clone() else {
        bkco_error_now!(
            "{}",
            verdict.bkce_grievance(name, ZBKM_CONVERGE), //
        );

        // A MUTE BINARY'S GRIEVANCE HAS ALREADY SAID THE ACT IS THE CRATE'S, and
        // a second line naming the converge as the act would undo it.
        if verdict.mute {
            zbkm_fenced();
        }

        bkco_error_now!(
            "'{} {}' is the converge, and running it is a deliberate act rather than something \
             this door does on your behalf",
            ZBKM_CONVERGE,
            name, //
        );
        zbkm_fenced();
    };

    // WHY THE ELECTION IS SAID OUT LOUD on an ordinary run. The whole property
    // of a derived election is that nobody performs the handoff, which is also
    // what makes it invisible: a seat that quietly stopped borrowing would look
    // exactly like one that never had. The verdict is one line and it rides
    // every launch, so the reading is always in front of whoever is watching.
    //
    // AND IT NAMES THE GEOGRAPHY IT READ, because the same binary in the same
    // tree is reached by more than one road: a door driven in a work billet
    // borrows from that billet's main worktree, and the same door driven in the
    // main worktree itself runs self-geography. Both roads are open on the
    // record, and this line is what makes them legible rather than silent.
    //
    // AND IT SAYS WHY THE BORROW DID NOT ANSWER, where it did not: an own build
    // elected over an identical borrow is a reading nobody could infer from the
    // word, and the election's plaint is the one sentence that states it.
    bkco_info_now!(
        "the collar '{}' elects {} under geography {} — running {}{}",
        name,
        verdict.elected,
        geography.bkce_stated(),
        answers.display(),
        verdict
            .plaint
            .as_deref()
            .map(|why| format!(
                "; {}",
                why, //
            ))
            .unwrap_or_default(), //
    );

    // THE APP ARM WRITES A RECORD AND NOT A TRANSCRIPT, which is a consequence
    // of the mode this door's tabtarget declares and is stated rather than left
    // to be discovered. Under the amanuensis mode the dispatch tees nothing, and
    // this arm hands the tenant THIS PROCESS'S STREAMS — the freeze's own shape,
    // an app's arguments and output passing through verbatim, which an
    // interactive tenant requires. So the family carries the invocation, the
    // position and the election, and says plainly that the tenant's own output
    // went to the terminal. All three members or none: a door that wrote nothing
    // here would leave a reader unable to tell a launch that happened from one
    // that never did.
    if let Err(err) = zbkm_app_record(
        dispatch,
        &bkcv_voice::bkcv_Record::bkcv_invocation(),
        &verdict,
        &geography,
        &answers,
        name,
    ) {
        bkco_error_now!(
            "{}",
            err, //
        );
    }

    let status = std::process::Command::new(&answers)
        .args(carried)
        .status()
        .unwrap_or_else(|err| {
            bkco_fatal_now!(
                "no binary answered at {}: {} — the residence names the directory a collar's \
                 binary stands in and the byname the file it wears there, so either the \
                 launchable was never built or the byname does not spell what stands",
                answers.display(),
                err, //
            )
        });

    match status.code() {
        Some(code) => zbkm_relayed(
            code,
            &format!(
                "the collar '{}'",
                name, //
            ),
        ),
        None => bkco_fatal_now!(
            "a signal took {}",
            answers.display(), //
        ),
    }
}

/// Write the app arm's record: what was asked, where it stood, and the plain
/// statement that the tenant's own output is not in here.
fn zbkm_app_record(
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    invocation: &str,
    verdict: &bkce_election::bkce_Verdict,
    geography: &bkce_election::bkce_Geography,
    answers: &Path,
    name: &str,
) -> Result<(), String> {
    let mut record = match bkcv_voice::bkcv_Record::bkcv_open(dispatch, invocation)? {
        Some(record) => record,
        None => return Ok(()),
    };

    record.bkcv_write(&format!(
        "the collar '{}' elects {} under geography {} — running {}{}",
        name,
        verdict.elected,
        geography.bkce_stated(),
        answers.display(),
        verdict
            .plaint
            .as_deref()
            .map(|why| format!(
                "; {}",
                why, //
            ))
            .unwrap_or_default(), //
    ))?;
    record.bkcv_write(
        "the tenant holds the terminal: an app's arguments and output pass through verbatim, so          what it said went to the console and stands in no member of this family",
    )?;
    record.bkcv_close()
}

/// The suite arm: compose from the collar and run under the declared runner.
///
/// SELECTION PATTERNS ARE MATCHED HERE, in the kennel, and the runner is handed
/// exact names. The frozen shape admits patterns after a suite collar and has the
/// kennel match them itself against the runner's own listing, so that one dialect
/// answers at every collar rather than each runner's own filter grammar.
///
/// THE BARE DRIVE IS UNTOUCHED BY THE PIPELINE. A suite asked for whole spawns
/// no listing and spells no selection: the pipeline is what a NARROWING costs,
/// and making every whole-suite drive pay a listing compile for it would be a
/// price nobody asked for. It is also what keeps the bare verdict line the one
/// it always was.
///
/// A PATTERN SET MATCHING NOTHING REFUSES AND SPAWNS NOTHING. Cargo's harness
/// reports a filter that matched nothing as a pass, so a kennel that handed the
/// selection through and trusted the answer would report a green for a suite it
/// never ran — which is the one outcome a launch may never produce.
fn zbkm_mush_suite(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcr_resolve::bkcr_Collar,
    name: &str,
    carried: &[OsString],
) -> ! {
    let launch = match bkcm_mush::bkcm_suite(repository, collar) {
        Ok(launch) => launch,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    // NOTHING IS SPAWNED UNTIL THE PROGRAM THAT WOULD BE SPAWNED IS THE ONE THE
    // KENNEL DECLARES. Asked here because this is where a nextest launch is
    // known to be one — a cargo-runner collar spawns no nextest, and refusing a
    // station over a program this run was never going to touch would be a
    // refusal about nothing. The composition already decided it; this reads the
    // decision rather than the collar.
    //
    // IT STANDS AHEAD OF THE NARROWING, which is where the ordering earns its
    // keep: the selection pipeline reaches the runner's own listing to match a
    // pattern against, so a narrowed launch has already spawned the program by
    // the time it is composed. Checked after, this would be reporting on a
    // program that had already run.
    if launch.nextest {
        match bkcl_leash::bkcl_kibbled(repository, dispatch, bkcm_mush::BKCM_VERB_NEXTEST) {
            // THE BINARY IS NAMED BEFORE IT IS SPAWNED, and the absolute path is
            // the whole of what the line is for. A program the kennel did not
            // build is the one thing in a launch that a reader cannot otherwise
            // account for — every other artifact is struck from source this seat
            // holds and answers for its own position — so the verdict says which
            // FILE answered rather than leaving it to be inferred from a name
            // that resolves differently on every station.
            //
            // AT THE INFO DIAL AND NOT THE TRACE, unlike the composed spelling
            // below it. The spelling is the door's own arrangement of flags,
            // which a reader wants when debugging the door; this is a fact about
            // what ran, which a reader wants whenever they are reading a verdict
            // at all.
            Ok(Some(residence)) => bkco_info_now!(
                "spawning {} from {} — a program this kennel did not build, reached at its \
                 kibble's own residence and never on the path",
                bkcm_mush::BKCM_VERB_NEXTEST,
                residence.display(), //
            ),
            Ok(None) => {}
            Err(err) => {
                bkco_error_now!(
                    "{}",
                    err, //
                );
                zbkm_fenced();
            }
        }
    }

    // THE MUZZLE RUNS FIRST, WHICH IS WHAT MAKES A GREEN COURSE A MUZZLED ONE.
    // It stands behind the station reading above because that one is a free read
    // of what is installed, and ahead of the narrowing below because the
    // narrowing SPAWNS: the pipeline asks the runner for its own listing, so a
    // muzzle asked after it would be barring calls in a crate the toolchain had
    // already been run over.
    let marks = match zbkm_muzzled(repository, dispatch, collar, name) {
        Some(marks) => marks,
        None => zbkm_barred(),
    };

    let (launch, narrowing) = if carried.is_empty() {
        (launch, None)
    } else {
        let (narrowed, cut) = zbkm_narrowed(repository, dispatch, collar, name, launch, carried);
        (narrowed, Some(cut))
    };

    bkco_trace_now!(
        "the collar '{}' runs its suite: {}",
        name,
        launch.bkcm_spelling(), //
    );

    // THE INVOCATION IS A DIAGNOSTIC AND NOT THE VOICE, so it moved onto the
    // dial: at 2 and above it is the kennel's own diagnostic, which is where the
    // frozen shape puts it, and the quiet flavor's console is the heartbeat and
    // the verdict alone.
    // THE CLOCK STARTS AT THE SPAWN AND NOT AT THE DOOR, so what the record
    // carries is the course's own time rather than the door's. Collar
    // resolution, validation and the listing a narrowing takes all stand ahead
    // of this line, and none of them is the thing a series of course times is
    // read to compare.
    let began = std::time::Instant::now();

    let (run, tally) = match bkcm_mush::bkcm_run(
        repository,
        dispatch,
        collar,
        &launch,
        collar.bkcr_field("BKRR_TONGUE"),
        &marks,
    ) {
        Ok(answered) => answered,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let taken = began.elapsed();

    bkco_trace_now!(
        "{} — {} case(s) counted",
        run.spelling,
        tally.ran, //
    );

    zbkm_hollow(name, &run, &tally);

    zbkm_record(dispatch, name, &run, &tally, narrowing.as_ref(), taken);

    zbkm_exit_as_ran(&run)
}

/// *lineup* — the no-run listing.
const ZBKM_LINEUP_HELP: &str = "lineup <collar>\n  \
     names the suite's hurdles as its runner names them, and runs nothing";

/// *lineup* — the pipeline's first step, said out loud and then stopped.
///
/// IT IS THE PIPELINE'S OWN LISTING AND NOT A SECOND ONE. A door that asked the
/// runner its own way would be free to name a different set than the narrowing
/// does, and the whole use of this door is to show a caller what their patterns
/// will be matched against.
///
/// NO RUN IS SPAWNED, which is the door's whole promise. It takes no pattern:
/// what a pattern would select is a question for the door that runs, and
/// answering it here would make this door a dry run rather than a listing.
fn zbkm_lineup(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if rest.len() != 1 {
        bkco_fatal_now!(
            "{}",
            ZBKM_LINEUP_HELP, //
        );
    }

    let name = rest[0].to_string_lossy().into_owned();

    // THE SAME FAMILY SPLIT MUSH HAS, on its own precedent (see zbkm_mush): the
    // family is decided by where the name stands, ahead of the rust walk, since
    // a python collar shares the genus's fields and none of its launch.
    if let Ok(resolved) = bkcx_python::bkcx_resolve(repository, dispatch, &name) {
        zbkm_lineup_python(repository, dispatch, resolved, &name);
    }

    let resolved = zbkm_sound_collar(repository, dispatch, &name);

    let listing = match bkci_pipeline::bkci_listing(repository, &resolved.collar) {
        Ok(listing) => listing,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let held = match bkci_pipeline::bkci_recite(repository, dispatch, &resolved.collar, &listing) {
        Ok(held) => held,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    for hurdle in &held {
        bkco_answer_now!(
            "{}",
            hurdle, //
        );
    }

    bkco_info_now!(
        "the collar '{}' holds {} hurdle(s); nothing ran",
        name,
        held.len(), //
    );
    std::process::exit(0);
}

/// Lineup's python arm: recite the collect-only listing `zbkm_mush_python`'s
/// suite arm already drives, and print it. NOTHING IS SPAWNED BUT THAT READING,
/// on lineup's own rust-arm ground: a listing door is what a narrowing costs,
/// never a second run.
fn zbkm_lineup_python(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    resolved: bkcx_python::bkcx_Resolved,
    name: &str,
) -> ! {
    if !resolved.findings.is_empty() {
        bkco_error_now!(
            "the python collar '{}' at {} carries {} finding(s), and no door operates on a collar \
             that does not conform:",
            name,
            resolved.collar.instance.display(),
            resolved.findings.len(), //
        );
        for finding in &resolved.findings {
            bkco_error_now!(
                "  {}",
                finding, //
            );
        }
        zbkm_barred();
    }

    let collar = resolved.collar;

    let seat = match bkcx_python::bkcx_seat(dispatch, &collar) {
        Ok(seat) => seat,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if let Err(err) = bkcx_python::bkcx_verify_at(repository, dispatch, &collar, &seat) {
        bkco_error_now!(
            "{}",
            err, //
        );
        zbkm_fenced();
    }

    let held = match bkcx_python::bkcx_recite(repository, dispatch, &collar, &seat) {
        Ok(held) => held,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    for hurdle in &held {
        bkco_answer_now!(
            "{}",
            hurdle, //
        );
    }

    bkco_info_now!(
        "the python collar '{}' holds {} hurdle(s); nothing ran",
        name,
        held.len(), //
    );
    std::process::exit(0);
}

/// Resolve a collar and refuse on any finding, which is validation being
/// implicit in every door rather than a step a caller elects. Shared by the
/// doors that operate on one collar, because a second spelling of this is a
/// second chance for one door to act on a collar another would have refused.
fn zbkm_sound_collar(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    name: &str,
) -> bkcr_resolve::bkcr_Resolved {
    let resolved = match bkcr_resolve::bkcr_resolve(repository, dispatch, name) {
        Ok(resolved) => resolved,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if !resolved.findings.is_empty() {
        bkco_error_now!(
            "the collar '{}' at {} carries {} finding(s), and no door operates on a collar that \
             does not conform:",
            name,
            resolved.collar.instance.display(),
            resolved.findings.len(), //
        );
        for finding in &resolved.findings {
            bkco_error_now!(
                "  {}",
                finding, //
            );
        }
        zbkm_barred();
    }

    resolved
}

/// Drive the three steps and hand back the launch with the selection spelled on
/// it: list, match, hand over exact names.
///
/// THE VERDICT NAMES THE NARROWING BEFORE THE RUN, not after, and it is the
/// kennel's own count rather than a reading of the runner's summary: a caller
/// who asked for two hurdles out of seventy is told that is what they are about
/// to get, whatever the run then says about them.
///
/// THE NARROWING RIDES BACK BESIDE THE LAUNCH because the record needs what the
/// launch cannot carry. A launch is what the leash will spell; the narrowing is
/// how many hurdles the suite HOLDS and which patterns cut them down, and both
/// are fields of the course record. Re-deriving them at the door would mean
/// listing the suite a second time, which is a second answer to a question this
/// step already answered.
fn zbkm_narrowed(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcr_resolve::bkcr_Collar,
    name: &str,
    launch: bkcm_mush::bkcm_Launch,
    carried: &[OsString],
) -> (bkcm_mush::bkcm_Launch, bkci_pipeline::bkci_Narrowing) {
    let patterns: Vec<String> = carried
        .iter()
        .map(|pattern| pattern.to_string_lossy().into_owned())
        .collect();

    let listing = match bkci_pipeline::bkci_listing(repository, collar) {
        Ok(listing) => listing,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let held = match bkci_pipeline::bkci_recite(repository, dispatch, collar, &listing) {
        Ok(held) => held,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let narrowing = match bkci_pipeline::bkci_choose(&held, &patterns) {
        Ok(narrowing) => narrowing,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if narrowing.chosen.is_empty() {
        bkco_error_now!(
            "the {} selection pattern(s) given to '{}' matched none of the {} hurdle(s) the \
             collar '{}' holds. NOTHING WAS SPAWNED: a runner handed a selection that matches \
             nothing reports a pass, and a green for a suite that never ran is the one answer a \
             launch may never give. Drive '{} {}' to see the names the patterns are matched \
             against",
            patterns.len(),
            ZBKM_DOOR_MUSH,
            narrowing.held.len(),
            name,
            ZBKM_DOOR_LINEUP,
            name, //
        );
        zbkm_barred();
    }

    let selection =
        match bkci_pipeline::bkci_selection(collar.bkcr_field("BKRR_RUNNER"), &narrowing.chosen) {
            Ok(selection) => selection,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

    bkco_info_now!(
        "the collar '{}' narrows: {}",
        name,
        narrowing.bkci_verdict(), //
    );

    let mut narrowed = launch;
    narrowed.rest.extend(selection);
    (narrowed, narrowing)
}

/// *muzzle* — the lint step, over every collar the walk finds.
///
/// IT TAKES NOTHING, which is the frozen shape's own answer for this door and
/// not a convenience. A muzzle is a statement about a tree rather than about a
/// crate: what it says is that no barred call stands anywhere the door reaches,
/// and a per-collar drive could never say that however many times it was run.
///
/// IT REPORTS AND NEVER BANKS. Nothing it writes stands anywhere, in the
/// tracked tree least of all, so this door drives twice in a row with nothing
/// to notch between — which is the door law being kept by a door rather than
/// excused for one. What carries a muzzle's currency is the
/// course that ran behind it, and no second record is kept.
///
/// THE ROSTER IS RETIRED INTO THE COLLARS. A crate enters the muzzle by wearing
/// a collar that names a lint list and leaves it by naming a different one, so
/// there is no roster anywhere for a crate to be silently absent from — which is
/// the enumeration law the sheaf sets, held at the one place the reach is now
/// stated.
///
/// EVERY COLLAR IS VALIDATED BEFORE ANY IS LINTED, and an invalid one refuses the
/// whole sweep naming each. Never operate on an invalid collar is the rule, and
/// a door that linted the sound collars and reported the rest would have acted on
/// half a tree while calling the answer whole.
fn zbkm_muzzle(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if !rest.is_empty() {
        bkco_fatal_now!(
            "'{}' takes nothing and was given {} argument(s) — it sweeps every collar the walk \
             finds, a muzzle being a statement about the tree rather than about one crate",
            ZBKM_DOOR_MUZZLE,
            rest.len(), //
        );
    }

    let collars = zbkm_collars(repository, dispatch, "linted");

    if collars.is_empty() {
        bkco_fatal_now!(
            "the walk over {} found no collar at all — a muzzle over nothing says nothing about \
             the tree",
            repository.display(), //
        );
    }

    let mut drives: Vec<(String, PathBuf, PathBuf)> = Vec::new();
    let mut refused = false;

    for collar in &collars {
        let manifest = repository.join(collar.bkcr_field("BKRR_MANIFEST"));
        let conf = match bkcz_muzzle::bkcz_conf_dir(repository, collar) {
            Ok(conf) => conf,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

        // ONE DRIVE PER LIST-AND-MANIFEST PAIR, REPORTED PER COLLAR. Two collars
        // over one crate — an app and its suite — declare one manifest between
        // them, and linting it twice would ask the identical question twice and
        // print two answers to it. What the report must carry is every collar,
        // because a collar is how a crate enters the muzzle at all.
        let already = drives
            .iter()
            .any(|(_, seen_manifest, seen_conf)| *seen_manifest == manifest && *seen_conf == conf);

        if already {
            bkco_info_now!(
                "muzzling {} — its crate and lint list are already asked about",
                collar.name, //
            );
            continue;
        }

        bkco_info_now!(
            "muzzling {} at {}",
            collar.name,
            collar.bkcr_field("BKRR_MUZZLE"), //
        );

        match bkcz_muzzle::bkcz_lint(repository, dispatch, collar) {
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
            Ok(bkcz_muzzle::bkcz_Verdict::Clean(_)) => {
                drives.push((collar.name.clone(), manifest, conf));
            }
            Ok(bkcz_muzzle::bkcz_Verdict::Unbuilt(report)) => {
                bkco_answer_now!(
                    "{}",
                    report.trim_end(), //
                );
                bkco_fatal_now!(
                    "the crate the collar '{}' declares did not compile — this is a build failure \
                     and not a muzzle finding",
                    collar.name, //
                );
            }
            Ok(bkcz_muzzle::bkcz_Verdict::Refused(report)) => {
                bkco_answer_now!(
                    "{}",
                    report.trim_end(), //
                );
                bkco_error_now!(
                    "the muzzle refused the collar '{}' — each finding above names the surface to \
                     adopt",
                    collar.name, //
                );
                refused = true;
            }
        }
    }

    if refused {
        zbkm_barred();
    }

    bkco_info_now!(
        "the muzzle finds nothing across {} collar(s)",
        collars.len(), //
    );

    std::process::exit(0);
}

/// *heel* — the converge. Build one collar's launchable current and install it
/// at the collar's residence.
///
/// THE ONE DOOR THAT CONVERGES. Every other door reports an outrun artifact and
/// stops, naming this act; here it is performed, by a caller who typed it. That
/// is the whole of what parts this door from the others, and it is why the act
/// is never folded into a launch: a build that happened because someone asked to
/// RUN something is a build nobody decided to do.
///
/// NEVER OPERATE ON AN INVALID COLLAR, the same rule the launch seam keeps and
/// for a sharper reason: this door WRITES. A finding against a collar whose
/// residence field is what it writes to is a finding about where the artifact
/// would land.
///
/// IT SAYS WHAT IT BUILT AND WHERE IT PUT IT, both, always. The install is
/// reported even where there was nothing to move, because a reader who cannot
/// tell an install that was unnecessary from one that was never attempted has
/// been handed a converge they have to go and verify.
fn zbkm_heel(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if rest.len() != 1 {
        bkco_fatal_now!(
            "'{}' takes exactly one collar as its imprint and was given {} — the frozen shape is \
             the verb, the collar, and then what the collar's kind owns, and this door's kind owns \
             nothing",
            ZBKM_DOOR_HEEL,
            rest.len(), //
        );
    }

    let name = rest[0].to_string_lossy().into_owned();

    // A KIBBLE IS CONVERGED BY THIS DOOR AND BY NO OTHER, so the name is offered
    // to that family FIRST and the collar walk answers only where no kibble
    // wears the name. Two families, one door, and the imprint says which by what
    // it names rather than by a flag — a caller typing a name knows what it
    // stands for, and a door that made them say twice would be asking them to
    // restate the tree.
    //
    // A NAME IN NEITHER FAMILY MEETS THE COLLAR REFUSAL, deliberately: the
    // collar roster is the larger one and the one a mistyped name is almost
    // always reaching for, so its listing is the more useful thing to hand back.
    // The kibble refusal carries its own roster and is reached by naming a
    // kibble that does not conform, which is a different mistake.
    if let Ok(resolved) = bkcq_kibble::bkcq_resolve(repository, dispatch.bkcr_weaned(), &name) {
        zbkm_heel_kibble(repository, dispatch, resolved);
    }

    // A PYTHON COLLAR IS CONVERGED BY THIS DOOR TOO, and its arm stands HERE —
    // behind the kibble, ahead of the sweep — for the kibble arm's own reason. A
    // python converge BUILDS NOTHING IN THIS TREE: the environment stands under
    // the checkout's loosebox and the interpreter under the station's tackroom,
    // so there are no intermediates for a sweep to be ahead of, and firing one
    // here would empty every rust yard in the tree as a side effect of building a
    // python environment.
    //
    // THE FAMILY IS DECIDED BY WHERE THE NAME STANDS and never by a flag, on the
    // kibble arm's precedent: a caller typing a name knows what it names, and a
    // door that made them say twice would be asking them to restate the tree.
    if let Ok(resolved) = bkcx_python::bkcx_resolve(repository, dispatch, &name) {
        zbkm_heel_python(repository, dispatch, resolved);
    }

    // THE YARD IS TAKEN AHEAD OF THE SWEEP and held until this door exits, so
    // a second heel in this checkout neither sweeps under this one's build nor
    // copies onto the residence beside it (`bkch_heel::bkch_Yard`).
    let _yard = zbkm_heel_yard(dispatch);

    // THE SWEEP STANDS AHEAD OF THE BUILD, which is where the forced clean it
    // replaces stood, and it is the only ordering that means anything: a yard
    // swept after a converge would remove the artifact the converge just struck.
    //
    // AND IT STANDS BEHIND THE KIBBLE ARM, which is the other half of the same
    // reasoning and was settled where the two landings met. A kibble converge
    // BUILDS NOTHING in this tree — it places a program the kennel did not make
    // — so there are no intermediates for a sweep to be ahead of, and firing one
    // there would empty every yard in the tree as a side effect of installing a
    // foreign artifact. It would also gate that install on every collar in the
    // walk conforming, which is a demand the act has no reason to make.
    zbkm_deloused(repository, dispatch);

    // THE REFUSAL ACCOUNTS FOR BOTH COLLAR FAMILIES, because this door now serves
    // both and a name that reached here was in neither. The rust walk names the
    // rust collars and is correct about what it says; what it cannot say is that
    // the caller may have meant a python collar, that family's walk not being its
    // to take. A refusal listing half the tree sends its reader looking for a
    // typo in the half they did not mean.
    let resolved = match bkcr_resolve::bkcr_resolve(repository, dispatch, &name) {
        Ok(resolved) => resolved,
        Err(err) => {
            let python = bkcx_python::bkcx_roster(repository, dispatch);
            if python.is_empty() {
                bkco_fatal_now!(
                    "{}",
                    err, //
                );
            }
            bkco_fatal_now!(
                "{}\n  and no python collar wears it either — that family carries: {}",
                err,
                python, //
            );
        }
    };

    if !resolved.findings.is_empty() {
        bkco_error_now!(
            "the collar '{}' at {} carries {} finding(s), and no door operates on a collar that \
             does not conform — least of all one that writes:",
            name,
            resolved.collar.instance.display(),
            resolved.findings.len(), //
        );
        for finding in &resolved.findings {
            bkco_error_now!(
                "  {}",
                finding, //
            );
        }
        zbkm_barred();
    }

    let geography = zbkm_geography(repository);

    let converge = match bkch_heel::bkch_compose(repository, &geography, &resolved.collar) {
        Ok(converge) => converge,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    bkco_info_now!(
        "the collar '{}' converges under geography {}: {} — its artifact lands at {}",
        name,
        geography.bkce_stated(),
        converge.bkch_spelling(),
        converge.struck.display(), //
    );

    // THE STRIKE IS NAMED BESIDE THE GEOGRAPHY, and this line is the whole of
    // what the kennel owes a STALE exergue. This door converges the crate; it
    // does not strike the file, and a binary it builds over an exergue recording
    // an older position is one the tenant's own fail-if-stale fence will refuse
    // afterward — correctly, and to an operator with no reason to connect the
    // two. Saying here which door writes that file is what makes the refusal
    // that follows read as expected rather than as a fresh mystery. The kennel
    // judges staleness nowhere: that fence already stands in the declaring
    // crate, and a second one here would be a second thing to keep in step.
    let exergues = resolved.collar.bkcr_exergue();
    if !exergues.is_empty() {
        bkco_info_now!(
            "its exergue(s) at {} are struck by {}, which this door does not run — a converge over \
             a stale exergue builds a stale binary, and the fence that catches that stands in the \
             crate rather than here",
            exergues.join(", "),
            resolved.collar.bkcr_strike().join(" "), //
        );
    }

    let (run, built) =
        match bkch_heel::bkch_build(repository, dispatch, &resolved.collar, &converge) {
            Ok(ran) => ran,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

    bkco_info_now!(
        "{}",
        run.spelling, //
    );

    // A BUILD THAT DID NOT LAND ENDS THE DOOR BROKE, never on the compiler's
    // code. The converge is an act and not a reading, so its failing rules on
    // nothing: what a caller reads here is that the act did not happen, and
    // which of cargo's many ways to fail was taken is what the voice's
    // diagnostic lines above are for.
    if !built {
        bkco_error_now!(
            "the collar '{}' did not build, so nothing was installed and {} stands as it did",
            name,
            converge.residence.display(), //
        );
        bkco_output::bkco_broke();
    }

    match bkch_heel::bkch_install(&converge) {
        Ok(true) => bkco_info_now!(
            "installed at {} — the collar's declared residence, and the only path this door wrote",
            converge.residence.display(), //
        ),
        Ok(false) => bkco_info_now!(
            "installed at {} — where cargo already leaves it, so nothing was copied",
            converge.residence.display(), //
        ),
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            bkco_output::bkco_broke();
        }
    }

    bkco_info_now!(
        "the collar '{}' is converged: built at {} and standing at {}",
        name,
        converge.struck.display(),
        converge.residence.display(), //
    );

    std::process::exit(0);
}

/// Take the checkout's yard for a heel, saying one line if another heel holds
/// it, and hand back the guard the caller keeps until it exits.
///
/// A LOOSEBOX THAT CANNOT BE READ TAKES NO YARD, and that is deliberate rather
/// than lenient: every arm that takes one reads the loosebox again for its own
/// act and refuses there in its own words, so taking the yard first must not
/// put a second refusal ahead of the one the door already gives.
fn zbkm_heel_yard(dispatch: &bkcr_resolve::bkcr_Dispatch) -> Option<bkch_heel::bkch_Yard> {
    let loosebox = bkcc_record::bkcc_loosebox(dispatch).ok()?;

    match bkch_heel::bkch_yard(&loosebox, |at| {
        bkco_info_now!(
            "another heel in this checkout holds its yard at {} — waiting for it to finish",
            at.display(), //
        )
    }) {
        Ok(yard) => Some(yard),
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    }
}

/// Converge one python collar: the third arm of *heel*, and the only write of a
/// checkout's loosebox in this whole binary.
///
/// IT SAYS WHAT IT PINNED AND WHERE IT PUT IT, both, always — the kibble arm's
/// own rule, and it bites harder here. A python environment stands OUTSIDE the
/// source tree by ruling, so a reader who cannot see the path in the output has
/// no way to guess it: there is no `.venv` beside the project to find.
///
/// THE FINDINGS GATE STANDS AHEAD OF THE CONVERGE, on the rust arm's ground and
/// for a sharper reason than it has. This door WRITES, and what the validation
/// refuses includes the very declarations the converge would build to — a pin
/// that is a bare floor, a project missing its lock, a uv table carrying a knob
/// the kennel states. Converging past a finding would build an environment to a
/// declaration the kennel had already judged unfit.
fn zbkm_heel_python(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    resolved: bkcx_python::bkcx_Resolved,
) -> ! {
    let name = resolved.collar.name.clone();

    // AN ABSENT LOCK DOES NOT BAR THE INSTALL STEP, which on a cold store is what
    // makes a first lock authorable; the converge stops after it and names the
    // lock-authoring door (`bkch_heel::bkch_python_at`). Every other finding bars.
    let barring = resolved.bkcx_findings_but_lock();

    if !barring.is_empty() {
        bkco_error_now!(
            "the python collar '{}' at {} carries {} finding(s), and no door operates on a collar \
             that does not conform — least of all one that writes:",
            name,
            resolved.collar.instance.display(),
            barring.len(), //
        );
        for finding in barring {
            bkco_error_now!(
                "  {}",
                finding, //
            );
        }
        zbkm_barred();
    }

    // THE YARD IS HELD THROUGH THE CONVERGE, on the rust arm's ground: two heels
    // of one collar in one checkout sync one environment.
    let _yard = zbkm_heel_yard(dispatch);

    let environed = match bkch_heel::bkch_python(repository, dispatch, &resolved.collar) {
        Ok(environed) => environed,
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            bkco_output::bkco_broke();
        }
    };

    bkco_info_now!(
        "{}",
        environed.installed, //
    );
    bkco_info_now!(
        "{}",
        environed.synced, //
    );

    bkco_info_now!(
        "the python collar '{}' is converged: the project at {} stands on CPython {} in the \
         environment at {}, synced from its lock",
        name,
        resolved.collar.bkcx_project().display(),
        environed.pinned,
        environed.environment.display(), //
    );

    std::process::exit(0);
}

/// Converge one kibble: the other half of *heel*, and the only write of the
/// tackroom in this whole binary.
///
/// SEPARATE FROM THE COLLAR ARM BECAUSE THE ACTS ARE SEPARATE. A collar's
/// converge builds source this repository holds and installs it at a residence
/// this repository names; a kibble's fetches foreign source, proves it against a
/// declared seal, and lands it in the station's shared store. They share a door
/// word because an operator converging something does not care which, and they
/// share nothing else — least of all a refusal, since the two send their reader
/// to different trees.
fn zbkm_heel_kibble(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    resolved: bkcq_kibble::bkcq_Resolved,
) -> ! {
    let kibble = resolved.kibble;

    if !resolved.findings.is_empty() {
        bkco_error_now!(
            "the kibble '{}' at {} carries {} finding(s), and no door operates on a kibble that \
             does not conform — least of all one that writes:",
            kibble.name,
            kibble.instance.display(),
            resolved.findings.len(), //
        );
        for finding in &resolved.findings {
            bkco_error_now!(
                "  {}",
                finding, //
            );
        }
        zbkm_barred();
    }

    // THE ROOT IS READ ONCE, HERE, and the converge is handed what this read, so
    // the residence the console names and the one the converge writes cannot be
    // two readings of the environment.
    let feedroom = match bkcq_kibble::bkcq_feedroom(dispatch) {
        Ok(feedroom) => feedroom,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let residence = kibble.bkcq_residence(&feedroom);

    bkco_info_now!(
        "the kibble '{}' declares {} {}, sealed {} — its residence is {}",
        kibble.name,
        kibble.bkcq_field("BKRK_PROGRAM"),
        kibble.bkcq_field("BKRK_VERSION"),
        kibble.bkcq_field("BKRK_SEAL"),
        residence.display(), //
    );

    match bkch_heel::bkch_kibble(repository, dispatch, &kibble, &feedroom) {
        Ok(bkch_heel::bkch_Kibbled::Current) => bkco_info_now!(
            "current at {} — the declared version already stands, so nothing was fetched and \
             nothing was built",
            residence.display(), //
        ),
        Ok(bkch_heel::bkch_Kibbled::Placed(from)) => bkco_info_now!(
            "fetched {}, digest proven, and placed at {}",
            from,
            residence.display(), //
        ),
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            bkco_output::bkco_broke();
        }
    }

    std::process::exit(0);
}

/// One course of a walk: a suite collar of either family, and for a python one
/// the seat its environment stands at.
///
/// TWO FAMILIES, ONE WALK. The derby runs every suite collar the weaned set
/// holds, and the set holds collars of both families; the two resolve through
/// different readers into different types, so a course names which it is rather
/// than a walk keeping one list per family and an order between them.
#[derive(Clone, Copy)]
enum zbkm_Course<'a> {
    Rust(&'a bkcr_resolve::bkcr_Collar),
    Python(&'a bkcx_python::bkcx_Collar, &'a bkcx_python::bkcx_Seat),
}

impl<'a> zbkm_Course<'a> {
    /// The collar name.
    fn zbkm_name(&self) -> &'a str {
        match *self {
            zbkm_Course::Rust(collar) => &collar.name,
            zbkm_Course::Python(collar, _) => &collar.name,
        }
    }

    /// The instance directory, which orders the walk.
    fn zbkm_instance(&self) -> &'a Path {
        match *self {
            zbkm_Course::Rust(collar) => &collar.instance,
            zbkm_Course::Python(collar, _) => &collar.instance,
        }
    }
}

/// A course a walk has composed and may run: what the launch seam spawns for it.
///
/// THE RUST COURSE CARRIES ITS MUZZLE'S MARKS AND THE PYTHON COURSE CARRIES NONE,
/// which is the python plane's ruling rather than an omission: the muzzle is the
/// rust plane's lint step, and the python guide's linter election is deferred.
enum zbkm_Bound<'a> {
    Rust {
        collar: &'a bkcr_resolve::bkcr_Collar,
        launch: bkcm_mush::bkcm_Launch,
        marks: Vec<String>,
    },
    Python {
        collar: &'a bkcx_python::bkcx_Collar,
        seat: &'a bkcx_python::bkcx_Seat,
        selection: Vec<OsString>,
    },
}

impl<'a> zbkm_Bound<'a> {
    /// The collar name.
    fn zbkm_name(&self) -> &'a str {
        match *self {
            zbkm_Bound::Rust { collar, .. } => &collar.name,
            zbkm_Bound::Python { collar, .. } => &collar.name,
        }
    }

    /// The runner the collar declares, which spells a selection.
    fn zbkm_runner(&self) -> &str {
        match self {
            zbkm_Bound::Rust { collar, .. } => collar.bkcr_field("BKRR_RUNNER"),
            zbkm_Bound::Python { collar, .. } => collar.bkcx_field("BKRP_RUNNER"),
        }
    }

    /// Hand the course the exact names a narrowing chose, spelled for its runner.
    fn zbkm_narrow(&mut self, chosen: Vec<OsString>) {
        match self {
            zbkm_Bound::Rust { launch, .. } => launch.rest.extend(chosen),
            zbkm_Bound::Python { selection, .. } => *selection = chosen,
        }
    }

    /// The invocation as the launch seam spells it, for the diagnostic position.
    fn zbkm_spelled(&self, repository: &Path) -> String {
        match self {
            zbkm_Bound::Rust { launch, .. } => launch.bkcm_spelling(),
            zbkm_Bound::Python {
                collar, selection, ..
            } => bkcx_python::bkcx_run_call(&bkcx_python::bkcx_at(repository, collar), selection)
                .iter()
                .map(|one| one.to_string_lossy().into_owned())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }

    /// The runner's own listing, read back as the names a pattern is matched
    /// against — the pipeline's first step for either family.
    fn zbkm_held(
        &self,
        repository: &Path,
        dispatch: &bkcr_resolve::bkcr_Dispatch,
    ) -> Result<Vec<String>, String> {
        match self {
            zbkm_Bound::Rust { collar, .. } => {
                let listing = bkci_pipeline::bkci_listing(repository, collar)?;
                bkci_pipeline::bkci_recite(repository, dispatch, collar, &listing)
            }
            zbkm_Bound::Python { collar, seat, .. } => {
                bkcx_python::bkcx_recite(repository, dispatch, collar, seat)
            }
        }
    }

    /// Run the course through the launch seam's own composition.
    fn zbkm_ran(
        &self,
        repository: &Path,
        dispatch: &bkcr_resolve::bkcr_Dispatch,
    ) -> Result<(bkcl_leash::bkcl_Run, bkcv_voice::bkcv_Tally), String> {
        match self {
            zbkm_Bound::Rust {
                collar,
                launch,
                marks,
            } => bkcm_mush::bkcm_run(
                repository,
                dispatch,
                collar,
                launch,
                collar.bkcr_field("BKRR_TONGUE"),
                marks,
            ),
            zbkm_Bound::Python {
                collar,
                seat,
                selection,
            } => bkcm_mush::bkcm_python_suite(repository, dispatch, collar, seat, selection),
        }
    }

    /// Whether the course ran nothing, read as its runner presents it, the
    /// course named red where it did.
    fn zbkm_hollow_read(&self, run: &bkcl_leash::bkcl_Run, tally: &bkcv_voice::bkcv_Tally) -> bool {
        match self {
            zbkm_Bound::Rust { collar, .. } => zbkm_hollowed(&collar.name, run, tally),
            zbkm_Bound::Python { collar, .. } => zbkm_python_hollowed(&collar.name, run, tally),
        }
    }
}

/// Compose what the launch seam spawns for one course, or `None` where the
/// muzzle refused it.
///
/// THE RUST COURSE IS MUZZLED HERE, at the launch seam's own position: behind
/// the composition and ahead of anything that compiles, which for the futurity
/// is the listing. A python course is bound with nothing to muzzle and nothing
/// selected yet.
fn zbkm_bind<'a>(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    course: zbkm_Course<'a>,
) -> Result<Option<zbkm_Bound<'a>>, String> {
    match course {
        zbkm_Course::Rust(collar) => {
            let launch = bkcm_mush::bkcm_suite(repository, collar)?;

            Ok(
                zbkm_muzzled(repository, dispatch, collar, &collar.name).map(|marks| {
                    zbkm_Bound::Rust {
                        collar,
                        launch,
                        marks,
                    }
                }),
            )
        }
        zbkm_Course::Python(collar, seat) => Ok(Some(zbkm_Bound::Python {
            collar,
            seat,
            selection: Vec::new(),
        })),
    }
}

/// Every python SUITE collar beside the seat its environment stands at.
///
/// SUITES ALONE, because a course is a suite: a python app's environment is the
/// project's, read through whichever suite collar stands on that project, and an
/// app collar with no suite beside it is no course of a walk. The seat is
/// composed once here, at the door face, and threaded to the preflight and the
/// launch both, so the two cannot read two environments.
fn zbkm_seated<'a>(
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    python: &'a [bkcx_python::bkcx_Collar],
) -> Vec<(&'a bkcx_python::bkcx_Collar, bkcx_python::bkcx_Seat)> {
    python
        .iter()
        .filter(|collar| !collar.bkcx_app())
        .map(|collar| match bkcx_python::bkcx_seat(dispatch, collar) {
            Ok(seat) => (collar, seat),
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        })
        .collect()
}

/// Every course a walk runs: the suite collars of both families, in the order
/// of their instance directories, so the walk reads the weaned set as one set.
fn zbkm_courses<'a>(
    collars: &'a [bkcr_resolve::bkcr_Collar],
    seated: &'a [(&'a bkcx_python::bkcx_Collar, bkcx_python::bkcx_Seat)],
) -> Vec<zbkm_Course<'a>> {
    let mut courses: Vec<zbkm_Course<'a>> = collars
        .iter()
        .filter(|collar| !collar.bkcr_app())
        .map(zbkm_Course::Rust)
        .chain(
            seated
                .iter()
                .map(|(collar, seat)| zbkm_Course::Python(collar, seat)),
        )
        .collect();

    courses.sort_by(|one, other| one.zbkm_instance().cmp(other.zbkm_instance()));

    courses
}

/// *derby* — every suite collar the walk finds, run in turn.
///
/// IT TAKES NOTHING, on the muzzle's own reasoning: a derby is a statement about
/// a TREE rather than about one suite, and a per-collar drive could never say
/// what this one says. Which suite runs a caller wants alone is the launch seam's
/// question, and it is one collar away.
///
/// THIN DISPATCH OVER THE LAUNCH SEAM, and deliberately so. Every course here is
/// composed and run by the very functions `mush` reaches, so a suite run under
/// derby and the same suite run under mush are the same invocation — a door that
/// composed its own would be a second place for the collar's declared shape to
/// be read, free to drift from the first.
///
/// EVERY SUITE COLLAR OF EITHER FAMILY IS A COURSE. A python course is launched
/// as the launch door launches it, takes no muzzle by the python plane's ruling,
/// and banks its record as every course does; its environment is the preflight's
/// to answer for, read by the launch door's own verify.
///
/// EVERY COLLAR IS VALIDATED BEFORE ANY COURSE RUNS, and an unsound one refuses
/// the whole walk naming each. Never operate on an invalid collar; a door that
/// ran the sound suites and reported the rest would have tested part of a tree
/// while calling the answer whole.
///
/// IT RUNS EVERY COURSE AND REPORTS EVERY RED (operator, 260929), reversing the
/// stop it once made on meeting a red course. A course that runs red and a
/// course the muzzle refuses are each named as they are met, the walk presses
/// on, and the set verdict names every one of them and bars. The tree is still
/// kept green: nothing reports green over a red, and one
/// line per red course is a repair list, where a stop hid every red behind the
/// first until that one was repaired.
fn zbkm_derby(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if !rest.is_empty() {
        bkco_fatal_now!(
            "'{}' takes nothing and was given {} argument(s) — it runs every suite collar the walk \
             finds, a derby being a statement about the tree rather than about one suite; one \
             suite alone is '{} <collar>'",
            ZBKM_DOOR_DERBY,
            rest.len(),
            ZBKM_DOOR_MUSH, //
        );
    }

    // THE SWEEP STANDS AHEAD OF THE WALK. A derby is a statement about a tree,
    // and a course run over artifacts the day's sweep was about to remove is a
    // green about a yard that no longer stands.
    zbkm_deloused(repository, dispatch);

    let (collars, python) = zbkm_yard(repository, dispatch, "run");
    let seated = zbkm_seated(dispatch, &python);

    // THE PREFLIGHT STANDS BEHIND THE SWEEP AND AHEAD OF EVERY COURSE, and both
    // halves of that are load-bearing. Ahead of the sweep it would pass over the
    // very residence the sweep is about to remove and call the tree ready;
    // behind the first course, the condition it reads has already presented as a
    // spawn failure inside a hurdle or as a run of failing cases about nothing,
    // which is the reading this exists to replace.
    let unhitched = zbkm_preflight(
        repository,
        dispatch,
        &collars,
        &seated,
        bkco_output::BKCO_COLOPHON_DERBY,
    );

    let mut courses = zbkm_courses(&collars, &seated);
    let walked = collars.len() + python.len();

    if courses.is_empty() {
        bkco_fatal_now!(
            "the walk over {} found {} collar(s) and no suite among them — a derby over nothing is \
             not a green tree",
            repository.display(),
            walked, //
        );
    }

    zbkm_unhitch(&mut courses, &unhitched);

    // THE WALK'S SIZE RIDES THE VERBOSE POSITION AND NOT THE DIAGNOSTIC ONE, and
    // the two are parted by what the frozen shape puts at each: above the verbose
    // flavor stands the kennel's own interior — the invocation it composed, the
    // election it computed — and how many courses the walk found is neither.
    // The quiet flavor carries the verdict alone, so this is said to a reader
    // who asked for more and to no other.
    if bkco_output::bkco_verbose() {
        bkco_info_now!(
            "the derby runs {} course(s), of the {} collar(s) the walk found",
            courses.len(),
            walked, //
        );
    }

    let mut red: Vec<&str> = Vec::new();
    let mut muzzled: Vec<&str> = Vec::new();

    for (run_so_far, course) in courses.iter().enumerate() {
        let name = course.zbkm_name();

        // EVERY RUST COURSE IS MUZZLED, and the step is the launch seam's own
        // rather than a second spelling of it. A derby is a statement about a
        // tree, so a course it cleared without the muzzle would be a green this
        // door has no right to report. A course it refuses is named where it is
        // met and the walk presses on, the muzzle's own findings already
        // standing above.
        let bound = match zbkm_bind(repository, dispatch, *course) {
            Ok(Some(bound)) => bound,
            Ok(None) => {
                muzzled.push(name);
                continue;
            }
            Err(err) => {
                zbkm_broken(
                    ZBKM_DOOR_DERBY,
                    &red,
                    &muzzled,
                    &zbkm_named(&courses[run_so_far..]),
                );
                bkco_fatal_now!(
                    "{}",
                    err, //
                )
            }
        };

        // THE INVOCATION IS A DIAGNOSTIC AND NOT THE VOICE, and it is said through
        // the macro that IS that position rather than through a dial read spelled
        // here: the trace throat is documented as the kennel's own diagnostics and
        // gates itself at 2, which is where the frozen shape puts a composed
        // invocation. The suite arm says the same fact the same way
        // (`zbkm_mush_suite`), so one door's composition cannot come to wear a
        // different level than the other's — and the dial stays read in the one
        // place the output module declares it.
        bkco_trace_now!(
            "course {} of {}: {} — {}",
            run_so_far + 1,
            courses.len(),
            name,
            bound.zbkm_spelled(repository), //
        );

        let began = std::time::Instant::now();

        let (run, tally) = match bound.zbkm_ran(repository, dispatch) {
            Ok(answered) => answered,
            Err(err) => {
                zbkm_broken(
                    ZBKM_DOOR_DERBY,
                    &red,
                    &muzzled,
                    &zbkm_named(&courses[run_so_far..]),
                );
                bkco_fatal_now!(
                    "{}",
                    err, //
                )
            }
        };

        let taken = began.elapsed();

        // A HOLLOW COURSE AND A GREEN ONE THAT COULD NOT BANK ITS RECORD ARE RED
        // COURSES OF THE WALK, each already named by its own line; they press on
        // as any red does rather than ending the walk under their own verdicts,
        // which stay the launch seam's for a single course.
        if bound.zbkm_hollow_read(&run, &tally) {
            red.push(name);
            continue;
        }

        // EVERY DERBY COURSE RAN WHOLE, so every record this door banks carries
        // a time. The door takes no pattern at all — a derby is a statement
        // about the tree rather than about one suite — so there is no narrowing
        // to hand on and none to withhold a time for.
        if !zbkm_banked(dispatch, name, &run, &tally, None, taken) {
            red.push(name);
            continue;
        }

        // NO SECOND VERDICT IS SPOKEN HERE. The launch seam's own `green:` line
        // already answered for this course on the voice channel; a second one
        // at the info dial repeated it — silently, at every verbosity, since
        // the macro that carried it is unconditional by nature.
        if run.bkcl_landed() {
            continue;
        }

        // THE COURSE IS NAMED WHERE IT IS MET, a signal's spelling among the
        // rest, and its exit is not passed on: the walk's verdict is its own.
        bkco_error_now!(
            "course {} is red: {} — {}",
            name,
            run.spelling,
            zbkm_red_said(&run, &tally), //
        );

        red.push(name);
    }

    zbkm_fallen(
        ZBKM_DOOR_DERBY,
        courses.len(),
        &red,
        &muzzled,
        &[],
        &unhitched,
    );

    // THE SET VERDICT SPEAKS AS A VERDICT, on the launch seam's own precedent:
    // it rides the voice channel rather than the info dial, so it stands in the
    // quiet flavor's console beside every course's own `green:` line rather
    // than behind a diagnostic position nobody armed.
    bkco_voice_now!(
        "the derby is green across {} course(s){}",
        courses.len(),
        zbkm_unhitched_said(&unhitched), //
    );

    std::process::exit(0);
}

/// *futurity* — the derby's set, narrowed by selection patterns across every
/// course at once, so a hurdle is found wherever it stands.
///
/// THE SELECTION PIPELINE AT THE SET'S GRAIN. Each course is listed, the
/// patterns are matched ONCE over the union of the listings, and each runner
/// is handed exact names from its own share of that match — the same three
/// steps mush takes over one course, held here over the yard. No pattern
/// reaches a runner's filter grammar at this grain any more than at that one,
/// and a python course's node ids stand in the union beside the rust courses'
/// test names, each family's listing taken by its own runner.
///
/// A UNION THE PATTERNS MATCH NOTHING IN BARS AND SPAWNS NO RUNNER, as the
/// pipeline bars at the launch seam, for the pipeline's own reason: a runner handed a
/// selection that matches nothing reports a pass. A course whose share is empty
/// is not launched and is NAMED in the verdict, so a narrowed walk never reads
/// as a whole one.
///
/// IT TAKES THE DERBY'S PREFLIGHT AND EACH COURSE'S MUZZLE, AND NEVER THE
/// DELOUSE. A narrowed drive is a session's act and not the yard being taken,
/// so the sweep's cadence stays the derby's and the converge's. Every rust course
/// is muzzled ahead of its listing, on the launch seam's own ordering: the
/// listing compiles the course, so a muzzle asked after it would bar calls in a
/// crate the toolchain had already been run over.
///
/// IT RUNS EVERY LAUNCHED COURSE AND REPORTS EVERY RED, as the derby does: a
/// course the muzzle refuses is named and never listed, and the set verdict
/// names every red course, every muzzle-refused course, and the courses the
/// patterns left idle.
///
/// IT TAKES ONE PATTERN OR MORE and refuses none as a pattern. Bare, it
/// refuses naming the derby: a futurity with nothing to narrow is the derby
/// without its sweep, which is a door nobody elected.
fn zbkm_futurity(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    rest: &[OsString],
) -> ! {
    if rest.is_empty() {
        bkco_fatal_now!(
            "'{}' takes one or more selection patterns and was given none — it narrows the set \
             the derby runs, and that set unnarrowed is '{}'",
            ZBKM_DOOR_FUTURITY,
            ZBKM_DOOR_DERBY, //
        );
    }

    let patterns: Vec<String> = rest
        .iter()
        .map(|pattern| pattern.to_string_lossy().into_owned())
        .collect();

    let (collars, python) = zbkm_yard(repository, dispatch, "run");
    let seated = zbkm_seated(dispatch, &python);

    let unhitched = zbkm_preflight(
        repository,
        dispatch,
        &collars,
        &seated,
        bkco_output::BKCO_COLOPHON_FUTURITY,
    );

    let mut courses = zbkm_courses(&collars, &seated);

    if courses.is_empty() {
        bkco_fatal_now!(
            "the walk over {} found {} collar(s) and no suite among them — a {} over nothing has \
             nothing to narrow",
            repository.display(),
            collars.len() + python.len(),
            ZBKM_DOOR_FUTURITY, //
        );
    }

    zbkm_unhitch(&mut courses, &unhitched);

    let mut entered: Vec<(zbkm_Bound, Vec<String>)> = Vec::new();
    let mut muzzled: Vec<&str> = Vec::new();

    for (index, course) in courses.iter().enumerate() {
        let unreached = zbkm_named(&courses[index..]);

        let bound = match zbkm_bind(repository, dispatch, *course) {
            Ok(Some(bound)) => bound,
            Ok(None) => {
                muzzled.push(course.zbkm_name());
                continue;
            }
            Err(err) => {
                zbkm_broken(ZBKM_DOOR_FUTURITY, &[], &muzzled, &unreached);
                bkco_fatal_now!(
                    "{}",
                    err, //
                )
            }
        };

        let held = match bound.zbkm_held(repository, dispatch) {
            Ok(held) => held,
            Err(err) => {
                zbkm_broken(ZBKM_DOOR_FUTURITY, &[], &muzzled, &unreached);
                bkco_fatal_now!(
                    "{}",
                    err, //
                )
            }
        };

        entered.push((bound, held));
    }

    // ONE MATCH OVER THE UNION, and each course's share read back out of it
    // rather than matched again: the narrowing is one answer about the yard, and
    // a per-course match would be as many answers as there are courses.
    let union: Vec<String> = entered
        .iter()
        .flat_map(|(_, held)| held.iter().cloned())
        .collect();

    let narrowing = match bkci_pipeline::bkci_choose(&union, &patterns) {
        Ok(narrowing) => narrowing,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    if narrowing.chosen.is_empty() {
        bkco_error_now!(
            "the {} selection pattern(s) given to '{}' matched none of the {} hurdle(s) the {} \
             course(s) hold. NOTHING WAS SPAWNED: a runner handed a selection that matches \
             nothing reports a pass, and a green for a suite that never ran is the one answer a \
             launch may never give. Drive '{} <collar>' to see the names the patterns are matched \
             against",
            patterns.len(),
            ZBKM_DOOR_FUTURITY,
            narrowing.held.len(),
            entered.len(),
            ZBKM_DOOR_LINEUP, //
        );

        // A COURSE THE MUZZLE REFUSED WAS NEVER LISTED, so the patterns were
        // never matched against it: the walk is red for that course whatever the
        // match found, and the set verdict says so beside the empty match.
        zbkm_fallen(
            ZBKM_DOOR_FUTURITY,
            courses.len(),
            &[],
            &muzzled,
            &[],
            &unhitched,
        );

        zbkm_barred();
    }

    let chosen: std::collections::HashSet<&str> =
        narrowing.chosen.iter().map(String::as_str).collect();

    let mut launched: Vec<(zbkm_Bound, bkci_pipeline::bkci_Narrowing)> = Vec::new();
    let mut idle: Vec<&str> = Vec::new();

    for (mut bound, held) in entered {
        let share: Vec<String> = held
            .iter()
            .filter(|hurdle| chosen.contains(hurdle.as_str()))
            .cloned()
            .collect();

        if share.is_empty() {
            idle.push(bound.zbkm_name());
            continue;
        }

        let selection = match bkci_pipeline::bkci_selection(bound.zbkm_runner(), &share) {
            Ok(selection) => selection,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

        bound.zbkm_narrow(selection);

        launched.push((
            bound,
            bkci_pipeline::bkci_Narrowing {
                held,
                chosen: share,
            },
        ));
    }

    if bkco_output::bkco_verbose() {
        bkco_info_now!(
            "the {} narrows: {} across {} course(s), launching {}",
            ZBKM_DOOR_FUTURITY,
            narrowing.bkci_verdict(),
            courses.len(),
            launched.len(), //
        );
    }

    let mut red: Vec<&str> = Vec::new();

    for (run_so_far, (bound, cut)) in launched.iter().enumerate() {
        let name = bound.zbkm_name();

        bkco_trace_now!(
            "course {} of {}: {} — {}",
            run_so_far + 1,
            launched.len(),
            name,
            bound.zbkm_spelled(repository), //
        );

        let (run, tally) = match bound.zbkm_ran(repository, dispatch) {
            Ok(answered) => answered,
            Err(err) => {
                let unreached: Vec<&str> = launched[run_so_far..]
                    .iter()
                    .map(|(later, _)| later.zbkm_name())
                    .collect();
                zbkm_broken(ZBKM_DOOR_FUTURITY, &red, &muzzled, &unreached);
                bkco_fatal_now!(
                    "{}",
                    err, //
                )
            }
        };

        if bound.zbkm_hollow_read(&run, &tally) {
            red.push(name);
            continue;
        }

        // A NARROWED COURSE CARRIES NO TIME, so the clock is not read: the
        // record is handed the course's own cut, and the writer withholds the
        // time for any course that carries one.
        if !zbkm_banked(
            dispatch,
            name,
            &run,
            &tally,
            Some(cut),
            std::time::Duration::ZERO,
        ) {
            red.push(name);
            continue;
        }

        if run.bkcl_landed() {
            continue;
        }

        bkco_error_now!(
            "course {} is red: {} — {}",
            name,
            run.spelling,
            zbkm_red_said(&run, &tally), //
        );

        red.push(name);
    }

    zbkm_fallen(
        ZBKM_DOOR_FUTURITY,
        launched.len(),
        &red,
        &muzzled,
        &idle,
        &unhitched,
    );

    if idle.is_empty() {
        bkco_voice_now!(
            "the {} is green across {} course(s), {}; every course held a match{}",
            ZBKM_DOOR_FUTURITY,
            launched.len(),
            narrowing.bkci_verdict(),
            zbkm_unhitched_said(&unhitched), //
        );
    } else {
        bkco_voice_now!(
            "the {} is green across {} course(s), {}; {} course(s) not launched, the patterns \
             matching none of their hurdles: {}{}",
            ZBKM_DOOR_FUTURITY,
            launched.len(),
            narrowing.bkci_verdict(),
            idle.len(),
            idle.join(", "),
            zbkm_unhitched_said(&unhitched), //
        );
    }

    std::process::exit(0);
}

/// Every precondition the tree's own declarations state, read before a walk is
/// spent proving the tree cannot be green.
///
/// THE ENVIRONMENT IS READ HERE AND NOWHERE BELOW, which is the split every
/// station fact in this kit takes: the reading is pure and takes the kibble root
/// as a value, and this one line at the door face consults the process. A
/// hurdle poses a root by composing a directory rather than by writing an
/// environment its parallel neighbours also stand in.
///
/// AN UNDECLARED STORE IS THE KIBBLE FAMILY'S OWN REFUSAL and is not restated
/// here: the reader composes the sentence in one place so the station meets what
/// a hurdle proves, and this door adds only the whistle a kennel reaching an
/// absent store was not launched through.
///
/// THE REFUSAL NAMES THE DOOR THAT WAS DRIVEN, which is why the colophon is the
/// caller's: the derby and the futurity read the same preconditions, and a
/// reader re-running once the named doors strike re-runs the door they typed.
///
/// IT ANSWERS THE UNHITCHED COURSES, which refuse nothing: the walk names them
/// beside the courses it runs and launches nothing of them.
fn zbkm_preflight(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collars: &[bkcr_resolve::bkcr_Collar],
    seated: &[(&bkcx_python::bkcx_Collar, bkcx_python::bkcx_Seat)],
    colophon: &str,
) -> Vec<(String, bkcx_python::bkcx_Unhitched)> {
    let ready = zbkm_ready(repository, dispatch, collars, seated);

    if ready.absent.is_empty() {
        return ready.unhitched;
    }

    bkco_error_now!(
        "{}",
        bkcd_preflight::bkcd_unready(
            &ready.absent,
            &bkco_output::bkco_door(dispatch.bkcr_tabtargets(), colophon),
            dispatch.bkcr_tabtargets(),
        ), //
    );

    zbkm_fenced();
}

/// The preflight's reading at the door face, refusing nothing: the walk's
/// preconditions and the unhitched courses, whatever stands.
///
/// SPLIT FROM THE REFUSAL so rebed reads the very roster the derby refuses
/// on, before and after its heels, and never a second account of it.
fn zbkm_ready(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collars: &[bkcr_resolve::bkcr_Collar],
    seated: &[(&bkcx_python::bkcx_Collar, bkcx_python::bkcx_Seat)],
) -> bkcd_preflight::bkcd_Ready {
    let feedroom = match bkcq_kibble::bkcq_feedroom(dispatch) {
        Ok(feedroom) => feedroom,
        Err(err) => bkco_fatal_now!(
            "{} — the tree's preconditions cannot be read without a kibble root: {}",
            err,
            bkco_output::bkco_door(
                dispatch.bkcr_tabtargets(),
                bkco_output::BKCO_COLOPHON_WHISTLE
            ), //
        ),
    };

    // THE GEOGRAPHY IS TAKEN HERE, inside the derby arm and behind the bare
    // self-report, because an app residence's currency is the election's
    // question and the election reads two residences under dispatch. It is
    // threaded down as a value beside the kibble root, for the same reason the
    // root is.
    let geography = zbkm_geography(repository);

    let python: Vec<(&bkcx_python::bkcx_Collar, &bkcx_python::bkcx_Seat)> = seated
        .iter()
        .map(|(collar, seat)| (*collar, seat))
        .collect();

    let ready = match bkcd_preflight::bkcd_preflight_at(
        repository,
        dispatch,
        &feedroom,
        &geography,
        collars,
        &python,
    ) {
        Ok(ready) => ready,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    ready
}

/// One act rebed performs for an absence the preflight names: a heel of a
/// collar, or a strike door the collar declares.
#[derive(Debug, Clone, PartialEq, Eq)]
enum zbkm_Supply {
    Heel(String),
    Strike(String),
}

/// *rebed*, the binary's half — every act the derby's preflight names that a
/// heel or a strike supplies, performed, every manifest's registry closure
/// fetched under its lock, and the preflight read again.
///
/// THE KENNEL'S ONE READINESS CONVERGE. A derby verifies and refuses; where it
/// refuses, it names this door and nothing else, and the release calls it ahead
/// of its builds. The roster of what to perform is the preflight's own reading
/// rather than a list this verb holds and lets drift. The toolchains it stands
/// on were laid by its bash half before the kennel could be built.
///
/// THE DAY'S SWEEP STANDS FIRST, on the build doors' own ground: a heel the
/// sweep then emptied would hand the derby the swept shape this verb was called
/// to cure. The sweep is the day's, so behind a build door that took it this is
/// a no-op.
///
/// THE REGISTRY CLOSURE IS FETCHED NEXT, one cargo fetch per manifest the
/// weaned set names, through the leash and so under its pin and its lock flag:
/// the derby compiles what that fetch laid, and a station whose registry was
/// emptied downloads here rather than inside a course.
///
/// EACH HEEL IS THE HEEL DOOR, spawned as this binary's own child with the
/// collar as its argument. The heel arms end the process they run in, which is
/// what a door does, and a second spelling of their bodies here would be a
/// second converge free to drift from the first. A python environment behind
/// its lock is a heel like any other, so the sync is the heel's.
///
/// EACH STRIKE IS THE TREE'S OWN DOOR, spawned from the repository root exactly
/// as the collar declares it, once per drive however many absences name it. The
/// kennel holds no idea of what a tree's build door does; it runs the door the
/// collar says supplies the absence, in the collar's order, and reads the exit.
/// A heel or strike that fails ends this verb on its verdict, relayed.
///
/// ONE ACT, THEN THE ROSTER READ AGAIN, because an act can uncover what it
/// stood in front of: an environment read only by its record while uv was
/// unplaced is read whole once uv is placed — and a project declaring this
/// station out of its environments is then seen unhitched rather than heeled
/// and refused, as it was when a whole round ran ahead of the re-read. An act is
/// performed once and never twice, so an absence the acts did not cure ends
/// the drive rather than looping on it.
///
/// WHAT NEITHER SUPPLIES REFUSES, named as the derby names it: a moved lock is
/// the operator's to re-derive and commit, and no tool commits.
fn zbkm_rebed(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if !rest.is_empty() {
        bkco_fatal_now!(
            "'{}' takes nothing and was given {} argument(s) — it converges whatever the derby's \
             preflight names across the whole tree, a heel of one collar being '{} <collar>'",
            ZBKM_REBED,
            rest.len(),
            ZBKM_DOOR_HEEL, //
        );
    }

    zbkm_deloused(repository, dispatch);

    let (collars, python) = zbkm_yard(repository, dispatch, "converged");
    let seated = zbkm_seated(dispatch, &python);

    let fetched = zbkm_fetched(repository, dispatch, &collars);

    let executable = match std::env::current_exe() {
        Ok(executable) => executable,
        Err(err) => {
            bkco_error_now!(
                "the running executable's own path could not be read, so no heel can be spawned: {}",
                err, //
            );
            bkco_output::bkco_broke();
        }
    };

    let mut performed: Vec<zbkm_Supply> = Vec::new();

    loop {
        let ready = zbkm_ready(repository, dispatch, &collars, &seated);

        let next = ready.absent.iter().find_map(|absence| {
            let acts: Vec<zbkm_Supply> = match &absence.heel {
                Some(name) => vec![zbkm_Supply::Heel(name.clone())],
                None => absence
                    .strike
                    .iter()
                    .cloned()
                    .map(zbkm_Supply::Strike)
                    .collect(),
            };

            acts.into_iter()
                .find(|act| !performed.contains(act))
                .map(|act| (absence.subject.clone(), act))
        });

        let Some((subject, act)) = next else {
            if !ready.absent.is_empty() {
                bkco_error_now!(
                    "{}",
                    bkcd_preflight::bkcd_unsupplied(
                        &ready.absent,
                        &bkco_output::bkco_call(
                            dispatch.bkcr_tabtargets(),
                            bkco_output::BKCO_VERB_REBED,
                            None
                        )
                    ), //
                );
                zbkm_fenced();
            }

            let heeled = performed
                .iter()
                .filter(|act| matches!(act, zbkm_Supply::Heel(_)))
                .count();
            bkco_info_now!(
                "the tree is ready for the derby: {} manifest(s) fetched, {} heel(s) and {} \
                 strike(s) performed",
                fetched,
                heeled,
                performed.len() - heeled, //
            );
            std::process::exit(0);
        };

        let ran = zbkm_supply_said(&act);
        bkco_info_now!(
            "supplying {}: {}",
            subject,
            ran, //
        );

        let spawned = match &act {
            zbkm_Supply::Heel(name) => std::process::Command::new(&executable)
                .arg(ZBKM_DOOR_HEEL)
                .arg(name)
                .status(),
            zbkm_Supply::Strike(door) => std::process::Command::new(repository.join(door))
                .current_dir(repository)
                .status(),
        };

        let status = match spawned {
            Ok(status) => status,
            Err(err) => {
                bkco_error_now!(
                    "'{}' could not be spawned: {}",
                    ran,
                    err, //
                );
                bkco_output::bkco_broke();
            }
        };

        match status.code() {
            Some(0) => performed.push(act),
            Some(code) => zbkm_relayed(code, &ran),
            None => {
                bkco_error_now!(
                    "'{}' ended on a signal and answered no exit",
                    ran, //
                );
                bkco_output::bkco_broke();
            }
        }
    }
}

/// One act of rebed's, said as it is performed.
fn zbkm_supply_said(act: &zbkm_Supply) -> String {
    match act {
        zbkm_Supply::Heel(name) => format!(
            "{} {}",
            ZBKM_DOOR_HEEL,
            name, //
        ),
        zbkm_Supply::Strike(door) => door.clone(),
    }
}

/// The cargo verb that lays a manifest's registry closure without building it.
const ZBKM_FETCH: &str = "fetch";

/// Every manifest the walk's rust collars name, its registry closure fetched
/// through the leash under its pin and its lock flag — answering how many.
///
/// ONE FETCH PER MANIFEST, however many collars stand over it: an app and its
/// suite name one manifest between them. A fetch that fails ends rebed on its
/// verdict, relayed, since a closure that cannot be laid is a derby that cannot
/// compile.
fn zbkm_fetched(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collars: &[bkcr_resolve::bkcr_Collar],
) -> usize {
    let mut manifests: Vec<PathBuf> = Vec::new();

    for collar in collars {
        let manifest = repository.join(collar.bkcr_field("BKRR_MANIFEST"));
        if manifests.contains(&manifest) {
            continue;
        }
        manifests.push(manifest);
    }

    for manifest in &manifests {
        bkco_info_now!(
            "fetching the registry closure of {}",
            manifest.display(), //
        );

        let run = match bkcl_leash::bkcl_drive(
            repository,
            dispatch,
            manifest,
            ZBKM_FETCH,
            std::iter::empty::<&str>(),
            &[],
        ) {
            Ok(run) => run,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

        match run.code {
            Some(0) => {}
            Some(code) => zbkm_relayed(code, &run.spelling),
            None => {
                bkco_error_now!(
                    "'{}' ended on a signal and answered no exit",
                    run.spelling, //
                );
                bkco_output::bkco_broke();
            }
        }
    }

    manifests.len()
}

/// *bleach* — what the kennel provisions on the station, emptied, on the
/// operator's word alone.
///
/// THE STATION COUNTERPART OF SCOOP. Scoop empties what a tree builds; this
/// empties what the station holds for every tree: the tackroom's rustup home
/// and cargo home, the kennel's quarter of the tackroom where the kibbles, the
/// managed interpreters and uv's cache stand, and this checkout's quarter of
/// its loosebox where the python environments stand. Rustup itself is never
/// touched — it stands in the station user's own home, outside every root
/// here. Rebed lays every one of them again.
///
/// NO DOOR CALLS IT AND NO ROAD RUNS IT. It is the operator's act, taken to
/// prove the road from cold, and a kennel door that emptied a store shared by
/// every clone on the station unasked would be a hazard to every other session.
///
/// EVERY ROOT IS SIGHTED BEFORE ANY IS REMOVED, on scoop's own law: a root that
/// does not stand strictly inside the tackroom the fence proved, or strictly
/// inside the loosebox, refuses the whole drive with nothing removed, and a
/// removal takes only what a sighting yielded.
fn zbkm_bleach(dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if !rest.is_empty() {
        bkco_fatal_now!(
            "'{}' takes nothing and was given {} argument(s) — it empties what the kennel \
             provisions on the station, the whole of it or none",
            ZBKM_DOOR_BLEACH,
            rest.len(), //
        );
    }

    let tackroom = match bkcl_leash::bkcl_fenced(dispatch) {
        Ok(tackroom) => tackroom,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let loosebox = match bkcc_record::bkcc_loosebox(dispatch) {
        Ok(loosebox) => loosebox,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let feedroom = match bkcq_kibble::bkcq_feedroom(dispatch) {
        Ok(feedroom) => feedroom,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let mut roots: Vec<PathBuf> = vec![
        bkcl_leash::bkcl_rustup_home(&tackroom),
        bkcl_leash::bkcl_cargo_home(&tackroom),
    ];
    roots.push(feedroom.bkcq_root().to_path_buf());
    roots.extend(bkcx_python::bkcx_provisions_at(&loosebox, &tackroom));

    let sighted: Vec<bkcy_sweep::bkcy_Sighting> = roots
        .into_iter()
        .map(|root| bkcy_sweep::bkcy_sight_provision(&tackroom, &loosebox, root))
        .collect();

    let beyond: Vec<&PathBuf> = sighted
        .iter()
        .filter_map(|sighting| match sighting {
            bkcy_sweep::bkcy_Sighting::Beyond(directory) => Some(directory),
            _ => None,
        })
        .collect();

    if !beyond.is_empty() {
        for directory in &beyond {
            bkco_error_now!(
                "{} stands inside neither the tackroom {} nor the loosebox {}, and nothing \
                 outside them is removed",
                directory.display(),
                tackroom.display(),
                loosebox.display(), //
            );
        }
        bkco_error_now!(
            "{} of {} root(s) name a directory this door may not remove, so nothing was {}",
            beyond.len(),
            sighted.len(),
            bkk::bkg_breviary::BKG_BLEACH_PAST, //
        );
        zbkm_barred();
    }

    let mut removed = 0usize;

    for sighting in &sighted {
        match bkcy_sweep::bkcy_remove(ZBKM_DOOR_BLEACH, sighting) {
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
            Ok(taken) if taken.stood => {
                removed += 1;
                bkco_info_now!(
                    "removed {}",
                    taken.directory.display(), //
                );
            }
            Ok(taken) => bkco_info_now!(
                "nothing stood at {}",
                taken.directory.display(), //
            ),
        }
    }

    bkco_info_now!(
        "the station is {}: {} of {} root(s) removed — {} lays every one of them again",
        bkk::bkg_breviary::BKG_BLEACH_PAST,
        removed,
        sighted.len(),
        bkco_output::bkco_call(
            dispatch.bkcr_tabtargets(),
            bkco_output::BKCO_VERB_REBED,
            None
        ), //
    );

    std::process::exit(0);
}

/// The delouse step the converging and testing doors run first, once a day, and
/// the one the delouse door hands a build door to run ahead of its own build.
///
/// IT STANDS WHERE THE FORCED CLEAN STOOD, and the difference is the whole
/// point: the build path discarded every intermediate before every compile, and
/// no commit ever explained why. What a periodic sweep is actually for — a yard
/// whose stale artifacts are cleared often enough to be worthless and rarely
/// enough to be cheap — is a cadence rather than a habit, so the cadence is
/// recorded and the act is skipped when it has already happened.
///
/// EVERY COLLAR IS VALIDATED BEFORE ANYTHING IS DELETED, and this door's own
/// walk is what validates them rather than the caller's. Heel resolves ONE
/// collar and derby resolves all of them, so a step trusting whichever the
/// caller happened to hold would delete a yard on the strength of one conforming
/// collar. The refusal lands with the yard untouched, which is the sight-then-
/// remove order the sweep makes structural one layer down.
///
/// THE LAUNCH SEAM NEVER REACHES IT. A build that happened because someone asked
/// to RUN something is a build nobody decided to do, and that is heel's own
/// reasoning about the converge held here about the sweep: mush runs one course
/// and decides nothing about the yard. A build door that DID decide to build
/// takes the sweep through its own door first, never through the seam.
///
/// IT REFUSES RATHER THAN PRESSING ON, at every failure this step can meet. A
/// delouse that could not read its day, could not sweep, or swept and could not
/// record it has left the yard in a state the next door cannot reason about, and
/// a build let through on top of that is exactly the silent staleness the
/// cadence exists to bound.
fn zbkm_deloused(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch) {
    let day = match bkcn_delouse::bkcn_day(dispatch) {
        Ok(day) => day,
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            bkco_output::bkco_broke();
        }
    };

    let seat = match bkcc_record::bkcc_loosebox(dispatch) {
        Ok(seat) => seat,
        Err(err) => {
            bkco_error_now!(
                "{}",
                err, //
            );
            bkco_output::bkco_broke();
        }
    };

    if bkcn_delouse::bkcn_read(&seat).as_deref() == Some(day.as_str()) {
        if bkco_output::bkco_verbose() {
            bkco_info_now!(
                "the yard was swept on {} and nothing is swept again today — the delouse keeps a \
                 day and not a habit",
                day, //
            );
        }
        return;
    }

    // THE RUNNING EXECUTABLE'S OWN PATH, so the loop below can spare whichever
    // directory holds it. Read once, ahead of every sighting, on the geography
    // reading's own precedent: a build that happened because someone asked to
    // RUN something is a build nobody decided to do, and a delouse that could
    // not name its own binary has no business judging what shelters it.
    let executable = match std::env::current_exe() {
        Ok(executable) => executable,
        Err(err) => {
            bkco_error_now!(
                "the running executable's own path could not be read, so the sweep cannot judge \
                 which directory shelters it: {}",
                err, //
            );
            bkco_output::bkco_broke();
        }
    };

    let collars = zbkm_collars(repository, dispatch, "swept");

    let mut sighted: Vec<(String, bkcy_sweep::bkcy_Sighting)> = Vec::new();
    let mut beyond = 0usize;

    for collar in &collars {
        match bkcy_sweep::bkcy_sight(repository, dispatch, Some(&seat), collar) {
            Err(err) => {
                bkco_error_now!(
                    "{}",
                    err, //
                );
                bkco_output::bkco_broke();
            }
            Ok(bkcy_sweep::bkcy_Sighting::Beyond(directory)) => {
                beyond += 1;
                bkco_error_now!(
                    "the collar '{}' builds into {}, which stands outside {} — nothing outside \
                     the work tree is removed, the toolchain store and the crate registry least \
                     of all",
                    collar.name,
                    directory.display(),
                    repository.display(), //
                );
            }
            Ok(within) => sighted.push((collar.name.clone(), within)),
        }
    }

    if beyond > 0 {
        bkco_error_now!(
            "{} of {} collar(s) name a build directory this door may not remove, so nothing was \
             swept and no day was recorded",
            beyond,
            collars.len(), //
        );
        bkco_output::bkco_broke();
    }

    if bkco_output::bkco_verbose() {
        bkco_info_now!(
            "the delouse takes the yard for {} — no sweep has landed here today",
            day, //
        );
    }

    let mut swept: Vec<PathBuf> = Vec::new();
    let mut removed = 0usize;

    for (name, sighting) in &sighted {
        // ONE REMOVAL PER DIRECTORY, REPORTED PER COLLAR, on scoop's own
        // reading: two collars over one crate — an app and its suite — name one
        // build directory between them, and the second would report an absent
        // directory as a yard that was already clean rather than as one this
        // drive had just emptied.
        let directory = match sighting {
            bkcy_sweep::bkcy_Sighting::Within(directory)
            | bkcy_sweep::bkcy_Sighting::Loosebox(directory) => directory.clone(),
            bkcy_sweep::bkcy_Sighting::Beyond(_) => continue,
        };

        if swept.contains(&directory) {
            continue;
        }

        // THE DELOUSE SPARES ITS OWN RESIDENCE, and only the delouse: scoop
        // calls `bkcy_remove` over every collar the operator names and must
        // keep doing so undiminished, so this exemption is authored here,
        // once, rather than in the sweep module both doors share. Recognition
        // is the running executable's own filesystem ancestry and never a
        // collar's name.
        if bkcy_sweep::bkcy_shelters(&directory, &executable) {
            if bkco_output::bkco_verbose() {
                bkco_info_now!(
                    "{} — spared {}, which holds the kennel performing this sweep",
                    name,
                    directory.display(), //
                );
            }
            swept.push(directory);
            continue;
        }

        match bkcy_sweep::bkcy_remove(name, sighting) {
            Err(err) => {
                bkco_error_now!(
                    "{}",
                    err, //
                );
                bkco_output::bkco_broke();
            }
            Ok(taken) => {
                if taken.stood {
                    removed += 1;
                    if bkco_output::bkco_verbose() {
                        bkco_info_now!(
                            "{} — removed {}",
                            taken.collar,
                            taken.directory.display(), //
                        );
                    }
                } else if bkco_output::bkco_verbose() {
                    bkco_info_now!(
                        "{} — nothing stood at {}",
                        taken.collar,
                        taken.directory.display(), //
                    );
                }
                swept.push(directory);
            }
        }
    }

    // THE DAY IS WRITTEN LAST, so a crash anywhere above costs a second sweep
    // rather than a skipped one.
    if let Err(err) = bkcn_delouse::bkcn_write(&seat, &day) {
        bkco_error_now!(
            "{}",
            err, //
        );
        bkco_output::bkco_broke();
    }

    // THE ONE TALLY LINE THE QUIET FLAVOR KEEPS, so a caller sees that a sweep
    // happened even with the per-collar detail behind the verbose position; it
    // rides the voice channel rather than the info dial for the same reason
    // the derby's own set verdict does.
    bkco_voice_now!(
        "the yard is swept for {}: {} build director(ies) removed across {} collar(s)",
        day,
        removed,
        collars.len(), //
    );
}

/// The muzzle step every suite course runs first, at both doors that run one.
///
/// A GREEN COURSE IS A MUZZLED COURSE, and that is what puts this step INSIDE
/// the course rather than beside it. The muzzle
/// keeps no record of its own, so nothing anywhere says a tree was clean at a
/// position except the course that ran behind this step; a door that ran hurdles
/// without it would report a green the muzzle never cleared.
///
/// THE CLEAN STREAM GOES TO THE RECORD AND NEVER TO THE CONSOLE. The cap leaves
/// every lint this door does not name standing as a warning, so a clean drive
/// still says things worth keeping — and the console belongs to the voice, whose
/// bounded output is the frozen shape's and not a seat for a second program's
/// compiler chatter. The lines ride back rather than being written here because
/// the log family opens truncating and the voice opens it: a record written
/// ahead of the run would be erased by the run's own open.
///
/// WHAT FOLLOWS A REFUSAL IS THE CALLER'S, which is why a refusal answers `None`
/// rather than exiting. The doors answer it differently — mush has nothing
/// further to say, and the walks step past the refused course and name it in
/// their set verdict.
fn zbkm_muzzled(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &bkcr_resolve::bkcr_Collar,
    name: &str,
) -> Option<Vec<String>> {
    match bkcz_muzzle::bkcz_lint(repository, dispatch, collar) {
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
        Ok(bkcz_muzzle::bkcz_Verdict::Clean(report)) => {
            let mut marks = vec![format!(
                "muzzled: {} at {}",
                name,
                collar.bkcr_field("BKRR_MUZZLE"), //
            )];
            marks.extend(report.lines().map(str::to_string));
            Some(marks)
        }
        Ok(bkcz_muzzle::bkcz_Verdict::Unbuilt(report)) => {
            bkco_answer_now!(
                "{}",
                report.trim_end(), //
            );
            bkco_fatal_now!(
                "the crate the collar '{}' declares did not compile — this is a build failure and \
                 not a muzzle finding",
                name, //
            );
        }
        Ok(bkcz_muzzle::bkcz_Verdict::Refused(report)) => {
            bkco_answer_now!(
                "{}",
                report.trim_end(), //
            );
            bkco_error_now!(
                "the muzzle refused the collar '{}' before any hurdle ran — each finding above \
                 names the surface to adopt",
                name, //
            );
            None
        }
    }
}

/// The set verdict of a walk that ran through: every red course, every course
/// the muzzle refused, and — for the futurity — every course the patterns left
/// idle, then the walk bars. Returns only when nothing fell.
///
/// SHARED BY BOTH WALKS, because the derby and the futurity take one behavior
/// and a reader owed the courses that went red is owed them the same way by
/// either. Each course was already named where it was met; this is the repair
/// list gathered at the end, so it stands last in the console whatever scrolled
/// past above it.
///
/// THE RED AND THE REFUSED ARE SAID APART, and the idle and the unhitched apart
/// from both, because they are different facts about the walk: a red course ran
/// and failed, a refused one never ran a hurdle, an idle one held no match, and
/// an unhitched one stands on a station its project declares itself out of.
fn zbkm_fallen(
    door: &str,
    ran: usize,
    red: &[&str],
    muzzled: &[&str],
    idle: &[&str],
    unhitched: &[(String, bkcx_python::bkcx_Unhitched)],
) {
    if red.is_empty() && muzzled.is_empty() {
        return;
    }

    bkco_error_now!(
        "the {} is red across {} course(s): {} red, {} refused by the muzzle",
        door,
        ran + muzzled.len(),
        red.len(),
        muzzled.len(), //
    );

    if !red.is_empty() {
        bkco_error_now!(
            "{} course(s) went red: {}",
            red.len(),
            red.join(", "), //
        );
    }

    if !muzzled.is_empty() {
        bkco_error_now!(
            "{} course(s) refused by the muzzle before any hurdle ran: {}",
            muzzled.len(),
            muzzled.join(", "), //
        );
    }

    if !idle.is_empty() {
        bkco_error_now!(
            "{} course(s) not launched, the patterns matching none of their hurdles: {}",
            idle.len(),
            idle.join(", "), //
        );
    }

    if !unhitched.is_empty() {
        bkco_error_now!(
            "{}",
            zbkm_unhitched_listed(unhitched), //
        );
    }

    zbkm_barred();
}

/// Take the unhitched courses out of a walk: the preflight read their projects'
/// declarations, and nothing of them is launched, listed or recorded.
fn zbkm_unhitch(
    courses: &mut Vec<zbkm_Course<'_>>,
    unhitched: &[(String, bkcx_python::bkcx_Unhitched)],
) {
    courses.retain(|course| !unhitched.iter().any(|(name, _)| name == course.zbkm_name()));
}

/// The unhitched courses as one sentence: each collar beside its project's own
/// words.
fn zbkm_unhitched_listed(unhitched: &[(String, bkcx_python::bkcx_Unhitched)]) -> String {
    let named: Vec<String> = unhitched
        .iter()
        .map(|(name, declared)| {
            format!(
                "{} ({})",
                name,
                declared.environments, //
            )
        })
        .collect();

    format!(
        "{} course(s) unhitched, their projects declaring environments this station stands \
         outside: {}",
        unhitched.len(),
        named.join(", "), //
    )
}

/// The clause a green set verdict carries where courses stood unhitched, and
/// nothing where none did.
///
/// AN UNHITCHED COURSE MOVES NO EXIT, so it rides the green verdict rather than
/// a line of its own: a reader of the set verdict sees every course the walk
/// stepped past, and why, in the line that says the walk was green.
fn zbkm_unhitched_said(unhitched: &[(String, bkcx_python::bkcx_Unhitched)]) -> String {
    if unhitched.is_empty() {
        String::new()
    } else {
        format!(
            "; {}",
            zbkm_unhitched_listed(unhitched), //
        )
    }
}

/// What a walk broken off by a fatal leaves said: the courses that fell before
/// it and the courses it never reached, spoken ahead of the fatal's own line.
///
/// THE FLOOR OF RUN-THROUGH. A walk ends early only on a fatal — a course that
/// could not be composed, listed or spawned — and a fatal that named only its
/// own cause would leave the reds already met, and the courses behind it,
/// unsaid.
fn zbkm_broken(door: &str, red: &[&str], muzzled: &[&str], unreached: &[&str]) {
    if !red.is_empty() {
        bkco_error_now!(
            "{} course(s) went red before the {} broke off: {}",
            red.len(),
            door,
            red.join(", "), //
        );
    }

    if !muzzled.is_empty() {
        bkco_error_now!(
            "{} course(s) refused by the muzzle before the {} broke off: {}",
            muzzled.len(),
            door,
            muzzled.join(", "), //
        );
    }

    if !unreached.is_empty() {
        bkco_error_now!(
            "the {} broke off, and {} course(s) did not run: {}",
            door,
            unreached.len(),
            unreached.join(", "), //
        );
    }
}

/// The names of the courses given, for a line that lists them.
fn zbkm_named<'a>(courses: &[zbkm_Course<'a>]) -> Vec<&'a str> {
    courses.iter().map(|course| course.zbkm_name()).collect()
}

/// A SUITE THAT RAN NOTHING PROVED NOTHING, and is red (operator, 260905).
///
/// THE READING IS TAKEN ONLY ON A RUN THAT LANDED, and the narrowing is what
/// makes it honest rather than merely strict. A suite that failed to compile also
/// counts no case, and answering that with "this suite declared no test" would
/// be the door diagnosing a build failure as a declaration — a true count
/// carrying a false sentence. A runner that failed has already said what went
/// wrong; what nothing else says is that a run came back GREEN having proven
/// nothing.
///
/// IT IS WHY THE RULE IS THE KENNEL'S AND NOT A RUNNER'S. Nextest refuses an
/// empty selection itself and the composer now asks it to; cargo's own harness
/// reports a clean zero and carries no flag to say otherwise, so for that runner
/// this reading is the only thing standing between an empty suite and a green
/// tree.
fn zbkm_hollow(collar: &str, run: &bkcl_leash::bkcl_Run, tally: &bkcv_voice::bkcv_Tally) {
    if zbkm_hollowed(collar, run, tally) {
        zbkm_barred();
    }
}

/// The hollow reading without its exit, for a walk that presses on past a red
/// course: the course is named red here, and the caller counts it among the
/// walk's reds rather than ending the walk on the single course's verdict.
fn zbkm_hollowed(collar: &str, run: &bkcl_leash::bkcl_Run, tally: &bkcv_voice::bkcv_Tally) -> bool {
    if !run.bkcl_landed() || tally.ran > 0 {
        return false;
    }

    zbkm_hollow_say(collar, run);
    true
}

/// The hollow gate as a PYTHON course presents it.
///
/// THE CONDITION DIFFERS AND THE VERDICT MUST NOT. The gate above asks whether a
/// course LANDED and counted nothing, which is the only shape a cargo harness can
/// present — it reports an empty selection as a clean zero. pytest never presents
/// it: an empty collection is already nonzero there, so the gate above could
/// never fire and a python course asked to run nothing would hand its caller
/// pytest's number instead of the kennel's. This arm reads that one exit
/// (`bkcx_python::BKCX_PYTEST_EMPTY`, where the membrane stands) and bars as the
/// gate above does, so a caller reading an exit gets one verdict whatever ran
/// underneath.
///
/// THE COUNT IS ASKED TOO, AND IT IS NOT REDUNDANT. A run that collected nothing
/// counted nothing, so the two agree — but they are two readings of different
/// things, and requiring both means a future pytest that reused this exit for
/// something else could not quietly turn a course that DID run cases into a
/// hollow refusal.
fn zbkm_python_hollow(collar: &str, run: &bkcl_leash::bkcl_Run, tally: &bkcv_voice::bkcv_Tally) {
    if zbkm_python_hollowed(collar, run, tally) {
        zbkm_barred();
    }
}

/// The python hollow reading without its exit, for a walk that presses on: the
/// course is named here, and the caller counts it among the walk's reds.
fn zbkm_python_hollowed(
    collar: &str,
    run: &bkcl_leash::bkcl_Run,
    tally: &bkcv_voice::bkcv_Tally,
) -> bool {
    if run.code != Some(bkcx_python::BKCX_PYTEST_EMPTY) || tally.ran > 0 {
        return false;
    }

    zbkm_hollow_say(collar, run);
    true
}

/// The hollow sentence, said once whether the door exits on it or presses on.
fn zbkm_hollow_say(collar: &str, run: &bkcl_leash::bkcl_Run) {
    bkco_error_now!(
        "the collar '{}' ran and counted no case at all: {} — a suite that ran nothing proved          nothing, and a green here would report the tree as tested on the strength of an empty          run",
        collar,
        run.spelling, //
    );
}

/// What a walk's red line says of how its course ended: the shape a runner that
/// did not finish ended in, or the counts of one that finished red.
///
/// THE COUNTS PRINT ONLY OVER A FINISHED RUN, on the voice's own rule
/// (`bkcv_voice::bkcv_unfinished`), so this line and the verdict above it say
/// one thing.
fn zbkm_red_said(run: &bkcl_leash::bkcl_Run, tally: &bkcv_voice::bkcv_Tally) -> String {
    match bkcv_voice::bkcv_unfinished(run.code, run.cut.as_deref(), tally) {
        Some(unfinished) => unfinished,
        None => format!(
            "{} case(s) ran, {} failed",
            tally.ran,
            tally.failed.len(), //
        ),
    }
}

/// End a single course's door on how its run ended.
///
/// A COURSE IS A READING OF THE TREE, so its runner's exit is read as one and
/// never handed on: a clean run is sound, and a run that went red bars, the
/// voice having already said how it ended. The runner's own code — nextest's,
/// cargo's 101 for a red test or a failed compile, pytest's — stays in that
/// line, where a code of 100 or above cannot read as a gate the band minted.
///
/// A CUT OVER A CLEAN EXIT ENDS THE DOOR BROKE, since passing the child's zero
/// on would report a run nobody heard whole as green, and a red verdict would
/// send a reader looking for a failing case: what failed is the hearing, which
/// the voice's red line names, and its repair is at the station.
fn zbkm_exit_as_ran(run: &bkcl_leash::bkcl_Run) -> ! {
    match run.code {
        Some(0) if run.cut.is_some() => bkco_output::bkco_broke(),
        Some(0) => std::process::exit(0),
        Some(_) => zbkm_barred(),
        None => bkco_fatal_now!(
            "a signal took the suite: {}",
            run.spelling, //
        ),
    }
}

/// Bank what a green course measured, and do nothing at all for one that was
/// not.
///
/// GREEN ONLY, AND THE READING IS TAKEN HERE RATHER THAN ASKED OF THE WRITER. A
/// red course and every refusal journal nothing, so the one place that knows
/// whether a course landed is the one place that decides whether a record is
/// owed — the writer below is handed a course that ran green or is not called.
///
/// IT STANDS AFTER THE HOLLOW READING, which is what keeps the two rules from
/// disagreeing. A run that landed and counted no case is red, and reddening it
/// after its record had already been banked would leave the studbook holding a
/// measurement of a course the door then refused.
///
/// A NARROWED COURSE CARRIES NO TIME, and the two doors reach that fact
/// differently: mush is handed the narrowing its patterns cut, and derby passes
/// `None` because a derby takes no pattern at all. Where nothing narrowed the
/// course the suite held exactly what it drove, which is why the held count
/// falls back to the tally rather than costing a second listing.
fn zbkm_record(
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &str,
    run: &bkcl_leash::bkcl_Run,
    tally: &bkcv_voice::bkcv_Tally,
    narrowing: Option<&bkci_pipeline::bkci_Narrowing>,
    taken: std::time::Duration,
) {
    if !zbkm_banked(dispatch, collar, run, tally, narrowing, taken) {
        bkco_output::bkco_broke();
    }
}

/// The record's banking without its exit, for a walk that presses on: false
/// says the course ran green and its record did not land, already named red
/// here; true says the record landed or none was owed.
fn zbkm_banked(
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    collar: &str,
    run: &bkcl_leash::bkcl_Run,
    tally: &bkcv_voice::bkcv_Tally,
    narrowing: Option<&bkci_pipeline::bkci_Narrowing>,
    taken: std::time::Duration,
) -> bool {
    if !run.bkcl_landed() {
        return true;
    }

    let course = bkcc_record::bkcc_Course {
        collar: collar.to_string(),
        narrowing: narrowing.map(|cut| cut.chosen.clone()),
        driven: tally.ran,
        passed: tally.passed,
        held: narrowing.map_or(tally.ran, |cut| cut.held.len()),
        wall: match narrowing {
            Some(_) => None,
            None => Some(taken),
        },
        position: bkcs_stamp::bkcs_position().seat.to_string(),
    };

    match bkcc_record::bkcc_write(&course, dispatch) {
        Ok(landed) => {
            for seat in landed {
                bkco_trace_now!(
                    "the course record for '{}' stands at {}",
                    collar,
                    seat.display(), //
                );
            }
            true
        }
        Err(err) => {
            bkco_error_now!(
                "the course '{}' ran green and its record was not banked: {}",
                collar,
                err, //
            );
            false
        }
    }
}

/// *delouse* — the day's sweep, taken ahead of a build door's act.
///
/// A BUILD DOOR THAT WRITES INTO A SWEPT YARD TAKES THE SWEEP FIRST, or the
/// day's first derby sweeps what it built. The forge's build doors build through
/// the launch seam, which never sweeps, and an app collar resident inside its
/// own build directory then meets the derby's preflight as the swept shape after
/// every build its caller ran. Calling this ahead of the build moves the day's
/// sweep to the front of the chain, and the derby after it finds the day already
/// taken.
///
/// IT IS THE SAME STEP DERBY AND HEEL RUN, and nothing else: recorded once a day,
/// a no-op the second time, and refusing as it refuses there. Which yard is stale is
/// the kennel's question, so the bash door calling this decides nothing about
/// staleness and holds no sweep of its own.
///
/// IT TAKES NOTHING, on scoop's ground: the sweep is a statement about the whole
/// yard, and a door naming one collar would suggest the day could be taken for
/// part of it.
fn zbkm_delouse(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if !rest.is_empty() {
        bkco_fatal_now!(
            "'{}' takes nothing and was given {} argument(s) — the day's sweep takes the whole \
             yard or none of it",
            ZBKM_DOOR_DELOUSE,
            rest.len(), //
        );
    }

    zbkm_deloused(repository, dispatch);

    std::process::exit(0);
}

/// *scoop* — the whole yard's clean, on the operator's word alone.
///
/// IT TAKES NOTHING, on the muzzle's precedent and for the muzzle's reason: what
/// it does is a statement about a tree rather than about a crate. A per-collar
/// clean is what a crate's own build directory being removed already is, and the
/// operator asking for one has cargo.
///
/// NO STAMP, NO CADENCE AND NO MEMORY. The delouse is the scheduled sweep and
/// banks its own record; this door is the operator saying now, and a record of
/// having said it would be a fact about a yard that the next build makes false.
///
/// EVERY COLLAR IS SIGHTED BEFORE ANY DIRECTORY IS REMOVED, which is the same
/// law the muzzle keeps one step earlier: the validation refusal above stops an
/// unsound collar, and this stops a sound collar whose build directory is not
/// one this door may delete. A sweep that removed as it walked would have
/// emptied half a yard before meeting the refusal that says it should not have
/// started.
fn zbkm_scoop(repository: &Path, dispatch: &bkcr_resolve::bkcr_Dispatch, rest: &[OsString]) -> ! {
    if !rest.is_empty() {
        bkco_fatal_now!(
            "'{}' takes nothing and was given {} argument(s) — it sweeps every collar the walk \
             finds, and a single crate's build directory is what cargo's own clean removes",
            ZBKM_DOOR_SCOOP,
            rest.len(), //
        );
    }

    // Scoop deletes a collar's build directory and the loosebox is where a
    // derived product that is no cargo target stands, so the door is told the
    // one it may reach into. It refuses rather than falling back: a scoop that
    // treated an absent name as "no loosebox today" would report a clean yard
    // over products it never looked at.
    let loosebox = match bkcc_record::bkcc_loosebox(dispatch) {
        Ok(loosebox) => loosebox,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let collars = zbkm_collars(repository, dispatch, "swept");

    let mut sighted: Vec<(String, bkcy_sweep::bkcy_Sighting)> = Vec::new();
    let mut beyond = 0usize;

    for collar in &collars {
        match bkcy_sweep::bkcy_sight(repository, dispatch, Some(&loosebox), collar) {
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
            Ok(bkcy_sweep::bkcy_Sighting::Beyond(directory)) => {
                beyond += 1;
                bkco_error_now!(
                    "the collar '{}' builds into {}, which stands outside {} — nothing outside \
                     the work tree is removed, the toolchain store and the crate registry least \
                     of all",
                    collar.name,
                    directory.display(),
                    repository.display(), //
                );
            }
            Ok(within) => sighted.push((collar.name.clone(), within)),
        }
    }

    // THE PYTHON ARM JOINS THIS WALK RATHER THAN STANDING BESIDE IT, and the
    // joining is what makes it safe. The refusal is about the SET — a python
    // environment sighted beyond must stop the rust removals too — and the
    // dedup below is about the set as well: two collars over one project name
    // one environment between them, exactly as an app and its suite name one
    // build directory.
    //
    // THE ENVIRONMENT IS THE SEAT'S AND THE SEAT IS COMPOSED HERE, because the
    // composition reads the station's roots out of process-wide state and the
    // sighting must not: the sweep is reached by the delouse for the same act,
    // and a mechanism that read its own roots would answer differently at the
    // two doors.
    let python = zbkm_python_collars(repository, dispatch, "swept");

    for collar in &python {
        let seat = match bkcx_python::bkcx_seat(dispatch, collar) {
            Ok(seat) => seat,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

        match bkcy_sweep::bkcy_sight_python(repository, &loosebox, &seat) {
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
            Ok(bkcy_sweep::bkcy_Sighting::Beyond(directory)) => {
                beyond += 1;
                bkco_error_now!(
                    "the python collar '{}' environs at {}, which stands under neither {} nor the \
                     checkout's loosebox — nothing outside them is removed, the managed \
                     interpreter store and uv's caches least of all",
                    collar.name,
                    directory.display(),
                    repository.display(), //
                );
            }
            Ok(within) => sighted.push((collar.name.clone(), within)),
        }
    }

    let walked = collars.len() + python.len();

    if beyond > 0 {
        bkco_error_now!(
            "{} of {} collar(s) name a derived directory this door may not remove, so nothing was \
             swept",
            beyond,
            walked, //
        );
        zbkm_barred();
    }

    let mut swept: Vec<PathBuf> = Vec::new();
    let mut removed = 0usize;

    for (name, sighting) in &sighted {
        // ONE REMOVAL PER DIRECTORY, REPORTED PER COLLAR, on the muzzle's own
        // reading: two collars over one crate — an app and its suite — name one
        // build directory between them, and the second would report an absent
        // directory as a yard that was already clean rather than as one this
        // drive had just emptied.
        let directory = match sighting {
            bkcy_sweep::bkcy_Sighting::Within(directory)
            | bkcy_sweep::bkcy_Sighting::Loosebox(directory) => directory.clone(),
            bkcy_sweep::bkcy_Sighting::Beyond(_) => continue,
        };

        if swept.contains(&directory) {
            bkco_info_now!(
                "{} builds into {}, which this drive has already taken",
                name,
                directory.display(), //
            );
            continue;
        }

        match bkcy_sweep::bkcy_remove(name, sighting) {
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
            Ok(taken) => {
                if taken.stood {
                    removed += 1;
                    bkco_info_now!(
                        "{} — removed {}",
                        taken.collar,
                        taken.directory.display(), //
                    );
                } else {
                    bkco_info_now!(
                        "{} — nothing stood at {}",
                        taken.collar,
                        taken.directory.display(), //
                    );
                }
                swept.push(directory);
            }
        }
    }

    bkco_info_now!(
        "the yard is clean: {} derived director(ies) removed across {} collar(s), and the kennel \
         binary went with them — the next door rebuilds through the whistle",
        removed,
        walked, //
    );

    std::process::exit(0);
}

/// Every python collar the walk finds, resolved and validated, or a refusal
/// naming each unsound one.
///
/// THE RUST ARM'S LAW OVER THE OTHER FAMILY — never operate on a collar that
/// does not conform — held in a reading of its own rather than in a shared body.
/// The two families resolve through different readers and yield different collar
/// types, so a single generic walk would be a type parameter bought to share
/// nine lines and would put the two rosters' findings in one sentence that could
/// name neither family.
fn zbkm_python_collars(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    acted: &str,
) -> Vec<bkcx_python::bkcx_Collar> {
    let (collars, unsound) = zbkm_python_read(repository, dispatch);

    zbkm_unsound(unsound, collars.len(), acted);

    collars
}

/// Every python collar the walk finds, resolved, each unsound one's findings
/// said where it is met — and the count of them, the refusal being the caller's.
fn zbkm_python_read(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
) -> (Vec<bkcx_python::bkcx_Collar>, usize) {
    let instances = match bkcx_python::bkcx_walk(repository, dispatch.bkcr_weaned()) {
        Ok(instances) => instances,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let mut collars: Vec<bkcx_python::bkcx_Collar> = Vec::new();
    let mut unsound = 0usize;

    for instance in &instances {
        let name = match instance.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => bkco_fatal_now!(
                "the walk found a python collar at {}, which names no instance directory",
                instance.display(), //
            ),
        };

        let resolved = match bkcx_python::bkcx_resolve(repository, dispatch, &name) {
            Ok(resolved) => resolved,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

        if !resolved.findings.is_empty() {
            unsound += 1;
            bkco_error_now!(
                "the python collar '{}' at {} carries {} finding(s):",
                name,
                resolved.collar.instance.display(),
                resolved.findings.len(), //
            );
            for finding in &resolved.findings {
                bkco_error_now!(
                    "  {}",
                    finding, //
                );
            }
        }

        collars.push(resolved.collar);
    }

    (collars, unsound)
}

/// Every collar the walk finds, resolved and validated, or a refusal naming each
/// unsound one.
fn zbkm_collars(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    acted: &str,
) -> Vec<bkcr_resolve::bkcr_Collar> {
    let (collars, unsound) = zbkm_rust_read(repository, dispatch);

    zbkm_unsound(unsound, collars.len(), acted);

    collars
}

/// Every rust collar the walk finds, resolved, each unsound one's findings said
/// where it is met — and the count of them, the refusal being the caller's.
fn zbkm_rust_read(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
) -> (Vec<bkcr_resolve::bkcr_Collar>, usize) {
    let instances = match bkcr_resolve::bkcr_walk(repository, dispatch.bkcr_weaned()) {
        Ok(instances) => instances,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    let mut collars: Vec<bkcr_resolve::bkcr_Collar> = Vec::new();
    let mut unsound = 0usize;

    for instance in &instances {
        let name = match instance.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => bkco_fatal_now!(
                "the walk found a collar at {}, which names no instance directory",
                instance.display(), //
            ),
        };

        let resolved = match bkcr_resolve::bkcr_resolve(repository, dispatch, &name) {
            Ok(resolved) => resolved,
            Err(err) => bkco_fatal_now!(
                "{}",
                err, //
            ),
        };

        if !resolved.findings.is_empty() {
            unsound += 1;
            bkco_error_now!(
                "the collar '{}' at {} carries {} finding(s):",
                name,
                resolved.collar.instance.display(),
                resolved.findings.len(), //
            );
            for finding in &resolved.findings {
                bkco_error_now!(
                    "  {}",
                    finding, //
                );
            }
        }

        collars.push(resolved.collar);
    }

    (collars, unsound)
}

/// Both families' collars, read whole before either refuses — the set a walk
/// runs over.
///
/// EVERY FINDING OF EITHER FAMILY IS SAID BEFORE THE ONE REFUSAL, which is the
/// derby's own law about its set: it refuses whole when any collar in the set is
/// invalid, naming each, and a walk that refused on the first family's findings
/// would hide the second's behind a repair and a fresh drive.
fn zbkm_yard(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    acted: &str,
) -> (
    Vec<bkcr_resolve::bkcr_Collar>,
    Vec<bkcx_python::bkcx_Collar>,
) {
    let (rust, rust_unsound) = zbkm_rust_read(repository, dispatch);
    let (python, python_unsound) = zbkm_python_read(repository, dispatch);

    zbkm_unsound(
        rust_unsound + python_unsound,
        rust.len() + python.len(),
        acted,
    );

    (rust, python)
}

/// The walk's one refusal over unsound collars, taken where any stood.
///
/// IT BARS AT EVERY DOOR THAT WALKS, which is what a collar that does not
/// conform is: a reading of the tree's own declarations, ruled against. Which
/// act the refusal stopped — a lint, a walk of every course, a sweep — is the
/// sentence's `acted`, so the line says what the exit no longer distinguishes.
fn zbkm_unsound(unsound: usize, walked: usize, acted: &str) {
    if unsound == 0 {
        return;
    }

    bkco_error_now!(
        "{} of {} collar(s) do not conform, so nothing was {} — never operate on an invalid collar",
        unsound,
        walked,
        acted, //
    );
    zbkm_barred();
}

/// The leash register's dispatch: the verb and what it owns, handed to the leash
/// over the manifest the arguments name, and the child's exit answered as a
/// verdict.
///
/// THE CHILD'S EXIT IS CLASSIFIED, NEVER HANDED ON (`zbkm_relayed`). Every site
/// this replaces tested cargo's exit for failure and died on it, so a clean run
/// stays sound and every failure stays a failure; what changes is that cargo's
/// 101 and nextest's 100 no longer leave this door as codes the band reads as
/// gates it minted.
///
/// NOTHING THAT REACHES HERE IS A DOOR. Every door word is taken by its own arm
/// above, the blanks included, so this function no longer holds a roster to
/// check one against — a door being built is refused where it is dispatched,
/// which is also where its own sentence will one day answer.
///
/// ONLY THE BUILD VERB TAKES THE VOICE, and every other verb keeps the child's
/// stdout whole on this process's own. A build is a compiler working, which the
/// estate's build doors reach through here and which sounds like a test under
/// the build road (`bkch_heel::bkch_voiced_build`). Any other verb answers its
/// CALLER: the engine build door reads a dependency tree off this stream to
/// prove the shipped binary links no census crate, and a stream the voice had
/// consumed would match nothing — which that guard reads as passing.
fn zbkm_register(
    repository: &Path,
    dispatch: &bkcr_resolve::bkcr_Dispatch,
    arguments: &[OsString],
) -> ! {
    let verb = arguments[0].to_string_lossy().into_owned();

    let (manifest, rest) = zbkm_manifest(&arguments[1..]);

    let attempt = if verb == bkch_heel::BKCH_VERB_BUILD {
        bkch_heel::bkch_voiced_build(repository, dispatch, &manifest, &rest, &[])
            .map(|(run, _)| run)
    } else {
        bkcl_leash::bkcl_drive(repository, dispatch, &manifest, &verb, rest, &[])
    };

    let run = match attempt {
        Ok(run) => run,
        Err(err) => bkco_fatal_now!(
            "{}",
            err, //
        ),
    };

    // WHAT WAS SPELLED IS SAID AFTER IT RAN, and the ordering is the leash's
    // rather than a choice made here: the leash composes the invocation inside
    // itself, which is what keeps the pin, the lock and the manifest out of a
    // caller's reach, and the spelling reaches us only as part of what came back.
    // Said either way, because a reader who needs to know which channel answered
    // needs it most on the run that failed.
    bkco_info_now!(
        "{}",
        run.spelling, //
    );

    match run.code {
        Some(code) => zbkm_relayed(code, &run.spelling),
        None => bkco_fatal_now!(
            "a signal took the invocation: {}",
            run.spelling, //
        ),
    }
}

/// Take the manifest out of the arguments, and answer it beside everything else
/// the caller spelled.
///
/// THE FLAG IS LIFTED RATHER THAN PASSED THROUGH. The leash spells the manifest
/// itself, so an argument list still carrying it would reach cargo with the flag
/// twice and be refused for the duplication — a caller writing the kennel's name
/// where it wrote cargo would meet an error about its own correct call.
///
/// THE LIFT STOPS AT THE SEPARATOR, because past it the arguments are not
/// cargo's at all: they belong to whatever cargo launches, and a flag spelled
/// there is that program's own. Lifting one would take an argument out of a
/// tenant's hands and read it as the leash's — and the tenant most likely to
/// spell this particular flag is the kennel itself, whose register takes it.
fn zbkm_manifest(rest: &[OsString]) -> (PathBuf, Vec<OsString>) {
    let joined = format!(
        "{}=",
        bkcl_leash::BKCL_MANIFEST_FLAG, //
    );

    let mut manifest: Option<PathBuf> = None;
    let mut kept: Vec<OsString> = Vec::new();
    let mut awaiting = false;
    let mut severed = false;

    for argument in rest {
        if severed {
            kept.push(argument.clone());
            continue;
        }

        if awaiting {
            manifest = Some(PathBuf::from(argument));
            awaiting = false;
            continue;
        }

        let spelled = argument.to_string_lossy();

        if spelled == bkcl_leash::BKCL_SEPARATOR {
            severed = true;
            kept.push(argument.clone());
            continue;
        }

        if spelled == bkcl_leash::BKCL_MANIFEST_FLAG {
            awaiting = true;
            continue;
        }

        if let Some(value) = spelled.strip_prefix(&joined) {
            manifest = Some(PathBuf::from(value));
            continue;
        }

        kept.push(argument.clone());
    }

    if awaiting {
        bkco_fatal_now!(
            "{} was given no path",
            bkcl_leash::BKCL_MANIFEST_FLAG, //
        );
    }

    match manifest {
        Some(path) => (path, kept),
        None => bkco_fatal_now!(
            "the leash register is told its manifest: spell {} <path>. Cargo would otherwise \
             discover one by walking up from the working directory, and a discovered manifest is \
             the same inheritance the stated pin exists to close — a call that meant one crate \
             building another",
            bkcl_leash::BKCL_MANIFEST_FLAG, //
        ),
    }
}

// eof
