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

// ============================================================================
// The preflight — every precondition a walk needs, read before the walk runs.
//
// A DERBY IS A STATEMENT ABOUT A TREE, and a tree that cannot be green as it
// stands should say so before it spends a compile proving it. Three conditions
// have reddened one on this estate, and none reached its reader as anything a
// reader could act on: a kibble the station never placed at the residence the
// declaration now composes, met as a spawn failure inside whichever hurdle
// happened to reach it first; an app's residence swept out from under the
// hurdles that exec it, met as a run of failing cases about nothing; and an
// app's residence STANDING AND OUTRUN — a pull that moved the collar's elected
// roots beneath a binary that still stood, which an existence test waved
// through and the hurdles then met as that binary's own staleness fence, exec
// by exec.
//
// THIS MODULE READS AND NEVER REFUSES, as the door law's reading does (`bkcf_guard`)
// and for the same reason: the refusal belongs to the DOOR. What stands here is the
// roster of absences and the report composed over it.
//
// IT VERIFIES AND NEVER PROVISIONS. Nothing here fetches a kibble or builds a
// tenant's crate to satisfy a precondition: a door reports what is absent and
// names the act that supplies it, and the act is the operator's.
//
// THE ROSTER IS ALSO THE CONVERGE'S READING, which is why each absence says
// whether a heel or the collar's own strike doors are the whole of its supplying
// act. Rebed performs exactly those and nothing else, so it converges what a
// derby would otherwise name without holding a list of its own, and the derby's
// refusal names it as the one act; an absence neither supplies — a lock that
// must be re-derived and committed first — stays the operator's, named as it
// always was.
//
// A PYTHON COURSE'S ENVIRONMENT IS ONE MORE SUCH PRECONDITION. It stands outside
// the tree under the seat's loosebox and the converge alone writes it, so a walk
// that met it absent or behind its lock inside a course would report a refusal
// about the station as a red course — or, on the launch's own road, have the
// course's `uv run` quietly sync it.
//
// A BINARY'S CURRENCY IS ASKED THROUGH THE ELECTION AND NEVER BY A READ OF THIS
// MODULE'S OWN. The election already holds the whole rule — a writer pinned
// with no position asked, a current own residence first under dispatch, a reader
// borrowed while the trees match and the borrow's fence reading admits it — and
// a second comparison here would have
// re-derived it and got the pin wrong: the engine's binary answers no seat
// reading at all, and a read that asked it one would call it outrun forever.
//
// A STRUCK EXERGUE FILE IS ASKED A DIFFERENT QUESTION, against a different far
// side, and the election module holds its reader (`bkce_struck`), its fence
// reading putting the same question to a borrow candidate's. It carries the LANDED reading — the newest trunk landing over its
// roots, written by `Tools/buk/bue_exergue.sh` — so what stales it is the trunk
// moving beneath those roots, and its comparison runs against the trunk
// counterpart and never `HEAD`. Asking the landed reading the seat's question
// here would refuse every billet that had committed under the roots, with no
// strike able to clear it.
// ============================================================================

use std::path::{
    Path,
    PathBuf, //
};

use crate::bkce_election;
use crate::bkce_election::bkce_Geography;
use crate::bkco_output::{
    bkco_call,
    BKCO_VERB_HEEL,
    BKCO_VERB_REBED, //
};
use crate::bkcq_kibble;
use crate::bkcr_resolve::bkcr_Collar;
use crate::bkcx_python;
use crate::bkcx_python::{
    bkcx_Collar,
    bkcx_Seat,
    BKCX_UV, //
};

/// One precondition the tree does not meet, and the act that would supply it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcd_Absence {
    /// What is missing, named as the declaration names it.
    pub subject: String,
    /// Where the thing was looked for.
    pub at: PathBuf,
    /// Where something DOES stand at `at` and may not be presumed current, why
    /// — the verdict's own plaint. `None` where nothing stands there at all.
    ///
    /// TWO SHAPES, AND A READER MUST BE ABLE TO TELL THEM APART. The swept
    /// shape is what the delouse produces on a day whose first kennel act is a
    /// derby — a build door takes the day's sweep ahead of its build, so a chain
    /// of builds then a derby never meets it — and any strike cures it; the outrun shape is what a pull produces, and the
    /// plaint says which roots moved. Both name the same supplying act, so only
    /// this field parts them, and the report renders them as two sentences
    /// rather than one.
    pub stands: Option<String>,
    /// The act that supplies it, already composed as a reader would type it.
    pub supply: String,
    /// The name a heel converges to supply it, where that heel is the WHOLE of
    /// the supplying act: a kibble, an app collar declaring no strike door, a
    /// python environment whose lock still answers its manifest. `None` where
    /// the act is a door the kennel does not hold, or a write that must be
    /// committed before any converge — the report names those, and nothing
    /// performs them on the operator's behalf.
    pub heel: Option<String>,
    /// The tree's own build doors that supply it, in the order its collar
    /// declares them, where running them is the WHOLE of the supplying act: an
    /// app collar's residence or a declared exergue, its collar naming a strike.
    /// Empty where a heel supplies it or nothing the converge runs does.
    pub strike: Vec<String>,
}

/// What the preflight reads of a tree: every precondition it does not meet, and
/// every python course its project declares this station out of.
///
/// TWO LISTS BECAUSE TWO FACTS. An absence is supplied by the act beside it and
/// refuses the walk until it is; an unhitched course is supplied by nothing,
/// refuses nothing, and is named by the walk beside the courses it ran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcd_Ready {
    /// Every precondition not met.
    pub absent: Vec<bkcd_Absence>,
    /// Every python suite collar unhitched here, beside its project's
    /// declaration, in walk order.
    pub unhitched: Vec<(String, bkcx_python::bkcx_Unhitched)>,
}

/// Every precondition the walk's own declarations state and this tree does not
/// meet, gathered whole, and every python course unhitched on this station.
///
/// THE KIBBLE ROOT AND THE GEOGRAPHY ARE PARAMETERS AND NOT READS, which is what
/// makes this hurdlable: a hurdle poses a root it owns and a seat by naming a
/// geography, rather than by writing the process every other hurdle in the same
/// runner is standing in, or by standing a worktree the reading would have to go
/// and derive. The kibble root is split at exactly this seam
/// (`bkcq_feedroom_at`), and a hurdle that wrote the
/// environment instead reddened an election hurdle three files away that
/// passed alone and failed in company. The geography is taken by the derby arm
/// behind the kennel's bare self-report, where the election module's laziness
/// law puts it, and threaded here as a value.
///
/// EVERY DECLARED KIBBLE, EVERY APP RESIDENCE AND EVERY DECLARED EXERGUE, never
/// a narrowing to what the courses appear to need. Which kibble a course
/// reaches is decided inside a hurdle at run time and the kennel cannot read
/// it; a preflight that guessed would pass the tree that then failed, which is
/// the whole condition it exists to catch.
///
/// EVERY ABSENCE IS GATHERED BEFORE ANYTHING IS REPORTED, which is the collar
/// resolver's discipline held here: a first-absence refusal hands its reader one
/// repair at a time and a fresh drive between each, and a cold seat owes several.
///
/// EVERY PYTHON COURSE'S ENVIRONMENT, read by the launch door's own verify. The
/// seats are composed by the door and handed in, on the kibble root's ground: a
/// seat's roots are read out of the process, which a hurdle could only pose by
/// writing.
pub fn bkcd_preflight_at(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    feedroom: &bkcq_kibble::bkcq_Feedroom,
    geography: &bkce_Geography,
    collars: &[bkcr_Collar],
    python: &[(&bkcx_Collar, &bkcx_Seat)],
) -> Result<bkcd_Ready, String> {
    let mut absent: Vec<bkcd_Absence> = Vec::new();
    let mut unhitched: Vec<(String, bkcx_python::bkcx_Unhitched)> = Vec::new();
    let mut uv_absent = false;
    let mut uv_declared = false;

    for instance in bkcq_kibble::bkcq_walk(repository, dispatch.bkcr_weaned())? {
        let name = match instance.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => continue,
        };

        uv_declared |= name == BKCX_UV;

        let resolved = bkcq_kibble::bkcq_resolve(repository, dispatch.bkcr_weaned(), &name)?;

        // AN UNSOUND KIBBLE IS NOT AN ABSENCE, and is stepped over here rather
        // than reported as one. Its findings are the collar walk's to report and
        // the residence a malformed declaration composes says nothing about the
        // station — a vacant version composes a path no converge would ever
        // place at, so reporting it would send a reader to heel for a repair
        // that belongs in the declaration.
        if !resolved.findings.is_empty() {
            continue;
        }

        let residence = resolved.kibble.bkcq_residence(feedroom);

        if !residence.is_file() {
            uv_absent |= name == BKCX_UV;

            absent.push(bkcd_Absence {
                subject: format!(
                    "the kibble '{}' — {} {}",
                    name,
                    resolved.kibble.bkcq_field("BKRK_PROGRAM"),
                    resolved.kibble.bkcq_field("BKRK_VERSION"), //
                ),
                at: residence,
                stands: None,
                supply: bkco_call(dispatch.bkcr_tabtargets(), BKCO_VERB_HEEL, Some(&name)),
                heel: Some(name.clone()),
                strike: Vec::new(),
            });
        }
    }

    for collar in collars {
        if !collar.bkcr_app() {
            continue;
        }

        zbkcd_residence(
            repository,
            geography,
            collar,
            dispatch.bkcr_tabtargets(),
            &mut absent,
        )?;
    }

    zbkcd_exergues(repository, collars, dispatch.bkcr_tabtargets(), &mut absent)?;

    zbkcd_environments(
        repository,
        dispatch,
        python,
        uv_absent,
        uv_declared,
        &mut absent,
        &mut unhitched,
    )?;

    Ok(bkcd_Ready { absent, unhitched })
}

/// Every python course's environment: absent, or standing and not answering what
/// its project declares, or standing and answering.
///
/// THE LAUNCH DOOR'S OWN READING, and its verdict rendered as a precondition
/// rather than as a refusal. A walk that read the environment its own way would
/// be free to pass one the launch door then refused, which is the two-readings
/// failure the verify exists to avoid.
///
/// ONE READING PER ENVIRONMENT, HOWEVER MANY COURSES STAND ON IT, on the
/// exergues' own ground: two suite collars over one project share one
/// environment, and one converge over either supplies both, so the first in walk
/// order names the act.
///
/// THE DECLARATION IS READ AHEAD OF THE ENVIRONMENT, by the launch door's own
/// reading, and an environment its project declares this station out of is no
/// absence: no converge could stand it, so naming one would send a reader to a
/// door that refuses. Every suite collar standing on it is unhitched, the
/// reading taken once.
///
/// AN UNPLACED UV IS ALREADY IN THE REPORT, and every reading past the
/// environment's own record reaches uv at its residence — the declaration's
/// among them. So the record alone is read then — an environment standing
/// nowhere is still told — and the declaration, the lock and the environment
/// are read on the drive after uv is placed, never refused ahead of it as
/// though the absence were theirs. A tree declaring no uv at all owns no
/// declaration reading either, the environment's record standing alone.
fn zbkcd_environments(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    python: &[(&bkcx_Collar, &bkcx_Seat)],
    uv_absent: bool,
    uv_declared: bool,
    absent: &mut Vec<bkcd_Absence>,
    unhitched: &mut Vec<(String, bkcx_python::bkcx_Unhitched)>,
) -> Result<(), String> {
    let mut seen: Vec<(&Path, Option<bkcx_python::bkcx_Unhitched>)> = Vec::new();

    for (collar, seat) in python {
        if let Some((_, declared)) = seen
            .iter()
            .find(|(at, _)| *at == seat.environment.as_path())
        {
            if let Some(declared) = declared {
                unhitched.push((collar.name.clone(), declared.clone()));
            }
            continue;
        }

        let declared = if uv_absent || !uv_declared {
            None
        } else {
            bkcx_python::bkcx_unhitched_at(repository, dispatch, collar, seat)?
        };

        seen.push((seat.environment.as_path(), declared.clone()));

        if let Some(declared) = declared {
            unhitched.push((collar.name.clone(), declared));
            continue;
        }

        let unanswered = if uv_absent {
            bkcx_python::bkcx_marque_at(repository, collar, seat)?
        } else {
            bkcx_python::bkcx_unanswered_at(repository, dispatch, collar, seat)?
        };

        let Some(unanswered) = unanswered else {
            continue;
        };

        absent.push(bkcd_Absence {
            subject: format!(
                "the environment of the python collar '{}'",
                collar.name, //
            ),
            at: seat.environment.clone(),
            stands: unanswered.bkcx_plaint(collar, seat),
            supply: unanswered.bkcx_supply(collar, dispatch.bkcr_tabtargets()),
            heel: unanswered.bkcx_heel(collar),
            strike: Vec::new(),
        });
    }

    Ok(())
}

/// The act that supplies an app collar's residence or its exergues: the
/// collar's own doors, read as declared.
///
/// THE SUPPLYING DOOR IS THE COLLAR'S OWN. Which build door a given crate keeps
/// is a fact the kennel does not hold, and the collar states it: `BKRR_STRIKE`
/// names the doors that strike this collar, in the order the tree fixes, and
/// every one of them is named. A collar declaring none is handed the kennel's
/// own converge, which is the one act this kit can promise.
fn zbkcd_supply(collar: &bkcr_Collar, tabtargets: Option<&Path>) -> String {
    let strike = collar.bkcr_strike();

    if strike.is_empty() {
        bkco_call(tabtargets, BKCO_VERB_HEEL, Some(&collar.name))
    } else {
        strike.join(", ")
    }
}

/// The heel that supplies an app collar's residence, where heel is the act the
/// collar is handed — the same reading as `zbkcd_supply` above, as a name.
///
/// A COLLAR DECLARING A STRIKE IS HANDED ITS STRIKE AND NEVER A HEEL, even
/// though heel would build its crate: the strike is the tree's own build door,
/// and a heel standing in for it would skip whatever else that door does — an
/// exergue struck, a seat laid — and seat a residence the tree never asked for.
fn zbkcd_heel(collar: &bkcr_Collar) -> Option<String> {
    if collar.bkcr_strike().is_empty() {
        Some(collar.name.clone())
    } else {
        None
    }
}

/// The strike doors that supply an app collar's residence or its exergues, as
/// the collar declares them — the reading `zbkcd_supply` renders, as names a
/// converge runs.
fn zbkcd_strike(collar: &bkcr_Collar) -> Vec<String> {
    collar
        .bkcr_strike()
        .into_iter()
        .map(str::to_string)
        .collect()
}

/// An app collar's residence: absent, standing and outrun, or standing and
/// current — the election's verdict, framed as a precondition.
///
/// NOTHING STANDING IS AN ABSENCE BEFORE ANY ELECTION IS HELD, and the order is
/// the election's own contract rather than an economy. A writer is pinned
/// without a position being asked, so the election cannot say whether the
/// blessed residence exists; a reader over an empty residence would spend a
/// spawn to learn what a path test says; and a collar whose roots no commit
/// has touched is refused by the seat reading, which an empty residence should
/// never reach. So the path test stands first, over both residences a
/// dispatched seat has, and the election is held only where one of them stands.
///
/// WHICH RESIDENCE IS NAMED IS WHICH ONE THE STRIKE WOULD WRITE. The own
/// residence is where a seat's converge lands, so an absence names it; an
/// outrun report names the residence that stands, the own one first, so the
/// path in the report is a file a reader can go and look at.
fn zbkcd_residence(
    repository: &Path,
    geography: &bkce_Geography,
    collar: &bkcr_Collar,
    tabtargets: Option<&Path>,
    absent: &mut Vec<bkcd_Absence>,
) -> Result<(), String> {
    let own = bkce_election::bkce_residence(repository, collar);
    let blessed = match geography.bkce_delivered() {
        Some(delivered) => bkce_election::bkce_residence(delivered, collar),
        None => own.clone(),
    };
    let subject = format!(
        "the app collar '{}'",
        collar.name, //
    );

    if !own.is_file() && !blessed.is_file() {
        absent.push(bkcd_Absence {
            subject,
            at: own,
            stands: None,
            supply: zbkcd_supply(collar, tabtargets),
            heel: zbkcd_heel(collar),
            strike: zbkcd_strike(collar),
        });
        return Ok(());
    }

    let verdict = bkce_election::bkce_elect(repository, geography, collar)?;

    match verdict.answers.clone() {
        // A BINARY ANSWERS AND STANDS: the borrow, the current own build, or a
        // pin over a blessed residence that is there.
        Some(answers) if answers.is_file() => Ok(()),
        // A PIN OVER A BLESSED RESIDENCE THAT IS NOT THERE. The election never
        // looked, by its own rule, so the path test is taken here for it.
        Some(answers) => {
            absent.push(bkcd_Absence {
                subject,
                at: answers,
                stands: None,
                supply: zbkcd_supply(collar, tabtargets),
                heel: zbkcd_heel(collar),
                strike: zbkcd_strike(collar),
            });
            Ok(())
        }
        // SOMETHING STANDS AND MAY NOT ANSWER — the outrun shape, in the
        // election's own words rather than a second account of one binary.
        None => {
            let mut why = verdict.bkce_plaint();

            // UNDER DISPATCH THE OWN RESIDENCE MAY SIMPLY NOT BE THERE — the
            // day's sweep takes it, and a fresh billet never had one — and the
            // election's fail-closed collapse reads that as a binary that would
            // not say. The preflight says so in one more clause, because the own
            // residence is the one the supplying act writes, and a reader sent
            // to look at a file that is not there has been told the wrong thing.
            if !own.is_file() {
                why.push_str(&format!(
                    "; nothing stands at the seat's own residence {}, which is where the \
                     supplying act writes",
                    own.display(), //
                ));
            }

            let at = if own.is_file() { own } else { blessed };

            absent.push(bkcd_Absence {
                subject,
                at,
                stands: Some(why),
                supply: zbkcd_supply(collar, tabtargets),
                heel: zbkcd_heel(collar),
                strike: zbkcd_strike(collar),
            });
            Ok(())
        }
    }
}

/// Every exergue the collars declare: absent, or standing and outrun by the
/// trunk, or standing and current.
///
/// ONE READING PER FILE, HOWEVER MANY COLLARS DECLARE IT. Two
/// collars declare the same struck module and three declare the `equerry's`, so
/// a preflight reporting per declaration would hand its reader one repair three
/// times over. The first declaring collar in walk order names the supplying
/// doors; every declarer of one file names the same striker by construction,
/// the collar resolver refusing an exergue with no door beside it.
///
/// THE TRUNK IS RESOLVED ONCE, AND ONLY WHERE A FILE STANDS. A tree declaring
/// no exergue, or one whose declared files are all absent, never asks git for a
/// remote — which is what lets every lure that never stood a remote keep its
/// derby, and keeps the swept shape's repair a strike rather than a fetch.
fn zbkcd_exergues(
    repository: &Path,
    collars: &[bkcr_Collar],
    tabtargets: Option<&Path>,
    absent: &mut Vec<bkcd_Absence>,
) -> Result<(), String> {
    let mut seen: Vec<&str> = Vec::new();
    let mut trunk: Option<Result<String, String>> = None;

    for collar in collars {
        for declared in collar.bkcr_exergue() {
            if seen.contains(&declared) {
                continue;
            }
            seen.push(declared);

            let at = repository.join(declared);
            let subject = format!(
                "the exergue '{}' declared by the collar '{}'",
                declared,
                collar.name, //
            );
            let supply = zbkcd_supply(collar, tabtargets);
            let strike = zbkcd_strike(collar);

            if !at.is_file() {
                absent.push(bkcd_Absence {
                    subject,
                    at,
                    stands: None,
                    supply,
                    heel: None,
                    strike: strike.clone(),
                });
                continue;
            }

            let struck = match bkce_election::bkce_struck(&at) {
                Ok(struck) => struck,
                Err(why) => {
                    absent.push(bkcd_Absence {
                        subject,
                        at,
                        stands: Some(why),
                        supply,
                        heel: None,
                        strike: strike.clone(),
                    });
                    continue;
                }
            };

            let far = match trunk.get_or_insert_with(|| bkce_election::bkce_trunk(repository)) {
                Ok(far) => far.clone(),
                Err(why) => {
                    absent.push(bkcd_Absence {
                        subject,
                        at,
                        stands: Some(why.clone()),
                        supply,
                        heel: None,
                        strike: strike.clone(),
                    });
                    continue;
                }
            };

            let concord =
                bkce_election::bkce_concord(repository, Some(&struck.sha), &far, &struck.roots);

            if !concord.identical {
                absent.push(bkcd_Absence {
                    subject,
                    at,
                    stands: Some(format!(
                        "struck at {} over the roots it carries, and {} has since moved the tree \
                         beneath them — {}",
                        struck.sha,
                        far,
                        concord.plaint, //
                    )),
                    supply,
                    heel: None,
                    strike: strike.clone(),
                });
            }
        }
    }

    Ok(())
}

/// The report a door hands its reader for a tree that cannot be green as it
/// stands.
///
/// IT NAMES EVERY ABSENCE, THEN THE ONE CONVERGE, THEN THE RE-RUN. Each absence
/// keeps the truth of what supplies it — the heel or the strike doors rebed
/// performs for it, or the act no converge performs — and the closing line
/// names rebed as the one act to take before the door the reader was already
/// running, both resolved through the same lookup, so a reader is handed one
/// converge and never a list of acts to type.
///
/// THE TWO SHAPES RENDER AS TWO SENTENCES. "Nothing stands" is what the delouse
/// leaves behind and any strike cures it; "stands, and may not be presumed
/// current" carries the plaint saying which roots moved, and is what a pull
/// leaves behind. A reader who has met the first on a morning that opened on a
/// derby must not read the second as more of the same.
pub fn bkcd_unready(absent: &[bkcd_Absence], rerun: &str, tabtargets: Option<&Path>) -> String {
    let lead = format!(
        "{} precondition(s) this tree declares are not met, so no course is run — a walk over a \
         tree that cannot be green would spend a compile proving it. The walk verifies and \
         never provisions; each is supplied by the act beside it:",
        absent.len(), //
    );

    zbkcd_listed(
        lead,
        absent,
        &format!(
            "then the one converge, performing every act above that it can: {}\n  and after \
             it, the walk again: {}",
            bkco_call(tabtargets, BKCO_VERB_REBED, None),
            rerun, //
        ),
    )
}

/// The report rebed hands its reader for what still stands unmet once every
/// heel and strike it could perform has run.
///
/// THE SAME LIST UNDER ANOTHER LEAD, and the lead is the whole difference: the
/// derby's says the walk never provisions, which is false of a converge that
/// just did. What remains is what rebed does not supply, so the lead says that,
/// the list beside it names each act exactly as the derby would, and the
/// re-run is rebed itself.
pub fn bkcd_unsupplied(absent: &[bkcd_Absence], rerun: &str) -> String {
    let lead = format!(
        "{} precondition(s) this tree declares stand unmet after every heel and strike the \
         kennel could perform — each is supplied by the act beside it:",
        absent.len(), //
    );

    zbkcd_listed(
        lead,
        absent,
        &format!(
            "then, once every one of them stands: {}",
            rerun, //
        ),
    )
}

/// A lead, then every absence with its shape and its act, then the closing.
///
/// AN ACT REBED PERFORMS IS SAID AS REBED'S, never as a door to type: rebed
/// first, then a heel by its verb and collar or a strike by the doors the
/// collar names. Only an act no
/// converge performs is handed over whole, as its supply composes it.
fn zbkcd_listed(lead: String, absent: &[bkcd_Absence], closing: &str) -> String {
    let mut said = lead;

    for absence in absent {
        let stood = match &absence.stands {
            None => format!(
                "nothing stands at {}",
                absence.at.display(), //
            ),
            Some(why) => format!(
                "stands at {}, and may not be presumed current: {}",
                absence.at.display(),
                why, //
            ),
        };

        let supplied = match &absence.heel {
            Some(name) => format!(
                "{}, which performs {} {}",
                BKCO_VERB_REBED,
                BKCO_VERB_HEEL,
                name, //
            ),
            None if !absence.strike.is_empty() => {
                format!(
                    "{}, which runs {}",
                    BKCO_VERB_REBED,
                    absence.strike.join(", "), //
                )
            }
            None => absence.supply.clone(),
        };

        said.push_str(&format!(
            "\n  {}\n    {}\n    supplied by: {}",
            absence.subject,
            stood,
            supplied, //
        ));
    }

    said.push_str(&format!(
        "\n  {}",
        closing, //
    ));

    said
}

// eof
