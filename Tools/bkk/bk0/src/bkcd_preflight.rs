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
// A PYTHON COURSE'S ENVIRONMENT IS ONE MORE SUCH PRECONDITION. It stands outside
// the tree under the seat's loosebox and the converge alone writes it, so a walk
// that met it absent or behind its lock inside a course would report a refusal
// about the station as a red course — or, on the launch's own road, have the
// course's `uv run` quietly sync it.
//
// A BINARY'S CURRENCY IS ASKED THROUGH THE ELECTION AND NEVER BY A READ OF THIS
// MODULE'S OWN. The election already holds the whole rule — a writer pinned
// with no position asked, a reader borrowed while the trees match, the own
// residence consulted under dispatch — and a second comparison here would have
// re-derived it and got the pin wrong: the engine's binary answers no seat
// reading at all, and a read that asked it one would call it outrun forever.
//
// A STRUCK EXERGUE FILE IS ASKED A DIFFERENT QUESTION, against a different far
// side. It carries the LANDED reading — the newest trunk landing over its
// roots, written by `Tools/buk/bue_exergue.sh` — so what stales it is the trunk
// moving beneath those roots, and its comparison runs against the trunk
// counterpart and never `HEAD`. Asking the landed reading the seat's question
// here would refuse every billet that had committed under the roots, with no
// strike able to clear it.
// ============================================================================

use std::path::{Path, PathBuf};

use crate::bkce_election;
use crate::bkce_election::bkce_Geography;
use crate::bkco_output::{bkco_call, BKCO_VERB_HEEL};
use crate::bkcq_kibble;
use crate::bkcr_resolve::bkcr_Collar;
use crate::bkcx_python;
use crate::bkcx_python::{bkcx_Collar, bkcx_Seat, BKCX_UV};

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
    weaned: &Path,
    feedroom: &bkcq_kibble::bkcq_Feedroom,
    geography: &bkce_Geography,
    collars: &[bkcr_Collar],
    python: &[(&bkcx_Collar, &bkcx_Seat)],
) -> Result<bkcd_Ready, String> {
    let mut absent: Vec<bkcd_Absence> = Vec::new();
    let mut unhitched: Vec<(String, bkcx_python::bkcx_Unhitched)> = Vec::new();
    let mut uv_absent = false;
    let mut uv_declared = false;

    for instance in bkcq_kibble::bkcq_walk(repository, weaned)? {
        let name = match instance.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => continue,
        };

        uv_declared |= name == BKCX_UV;

        let resolved = bkcq_kibble::bkcq_resolve(repository, weaned, &name)?;

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
                    resolved.kibble.bkcq_field("BKRK_VERSION")
                ),
                at: residence,
                stands: None,
                supply: bkco_call(
                    BKCO_VERB_HEEL,
                    Some(&name),
                ),
            });
        }
    }

    for collar in collars {
        if !collar.bkcr_app() {
            continue;
        }

        zbkcd_residence(repository, geography, collar, &mut absent)?;
    }

    zbkcd_exergues(repository, collars, &mut absent)?;

    zbkcd_environments(repository, weaned, python, uv_absent, uv_declared, &mut absent, &mut unhitched)?;

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
    weaned: &Path,
    python: &[(&bkcx_Collar, &bkcx_Seat)],
    uv_absent: bool,
    uv_declared: bool,
    absent: &mut Vec<bkcd_Absence>,
    unhitched: &mut Vec<(String, bkcx_python::bkcx_Unhitched)>,
) -> Result<(), String> {
    let mut seen: Vec<(&Path, Option<bkcx_python::bkcx_Unhitched>)> = Vec::new();

    for (collar, seat) in python {
        if let Some((_, declared)) = seen.iter().find(|(at, _)| *at == seat.environment.as_path()) {
            if let Some(declared) = declared {
                unhitched.push((collar.name.clone(), declared.clone()));
            }
            continue;
        }

        let declared = if uv_absent || !uv_declared {
            None
        } else {
            bkcx_python::bkcx_unhitched_at(repository, weaned, collar, seat)?
        };

        seen.push((seat.environment.as_path(), declared.clone()));

        if let Some(declared) = declared {
            unhitched.push((collar.name.clone(), declared));
            continue;
        }

        let unanswered = if uv_absent {
            bkcx_python::bkcx_marque_at(repository, collar, seat)?
        } else {
            bkcx_python::bkcx_unanswered_at(repository, weaned, collar, seat)?
        };

        let Some(unanswered) = unanswered else {
            continue;
        };

        absent.push(bkcd_Absence {
            subject: format!("the environment of the python collar '{}'", collar.name),
            at: seat.environment.clone(),
            stands: unanswered.bkcx_plaint(collar, seat),
            supply: unanswered.bkcx_supply(collar),
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
fn zbkcd_supply(collar: &bkcr_Collar) -> String {
    let strike = collar.bkcr_strike();

    if strike.is_empty() {
        bkco_call(BKCO_VERB_HEEL, Some(&collar.name))
    } else {
        strike.join(", ")
    }
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
    absent: &mut Vec<bkcd_Absence>,
) -> Result<(), String> {
    let own = bkce_election::bkce_residence(repository, collar);
    let blessed = match geography.bkce_delivered() {
        Some(delivered) => bkce_election::bkce_residence(delivered, collar),
        None => own.clone(),
    };
    let subject = format!("the app collar '{}'", collar.name);

    if !own.is_file() && !blessed.is_file() {
        absent.push(bkcd_Absence {
            subject,
            at: own,
            stands: None,
            supply: zbkcd_supply(collar),
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
                supply: zbkcd_supply(collar),
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
                    own.display()
                ));
            }

            let at = if own.is_file() { own } else { blessed };

            absent.push(bkcd_Absence {
                subject,
                at,
                stands: Some(why),
                supply: zbkcd_supply(collar),
            });
            Ok(())
        }
    }
}

/// Every exergue the collars declare: absent, or standing and outrun by the
/// trunk, or standing and current.
///
/// ONE READING PER FILE, HOWEVER MANY COLLARS DECLARE IT. The matricula's two
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
                declared, collar.name
            );
            let supply = zbkcd_supply(collar);

            if !at.is_file() {
                absent.push(bkcd_Absence {
                    subject,
                    at,
                    stands: None,
                    supply,
                });
                continue;
            }

            let struck = match zbkcd_struck(&at) {
                Ok(struck) => struck,
                Err(why) => {
                    absent.push(bkcd_Absence {
                        subject,
                        at,
                        stands: Some(why),
                        supply,
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
                        struck.sha, far, concord.plaint
                    )),
                    supply,
                });
            }
        }
    }

    Ok(())
}

/// What a struck exergue file says about its own striking: the sha, and the
/// roots it was measured over.
struct zbkcd_Struck {
    sha: String,
    /// The roots, whitespace-joined into the form `bkce_concord` splits.
    roots: String,
}

/// Read the sha and the roots off a struck exergue file.
///
/// THE SHAPE IS THE STRIKER'S AND THE CONSTANT'S NAME IS THE CALLER'S.
/// `Tools/buk/bue_exergue.sh` writes `pub const <NAME>: &str = "<sha>";` and
/// `pub const <NAME>_ROOTS: &[&str] = &[ ... ];` with one quoted root per line,
/// exclusion pathspecs included, and the striker's caller states `<NAME>`. So
/// the sha is the first string constant shaped like a position, and the roots
/// are the quoted elements of the `_ROOTS` block; the ancestry block is stepped
/// over, being the record-holding reader's instrument and not this one's.
///
/// CONSUMED AND NEVER RE-DERIVED. The file states what it was struck from;
/// walking the trunk a second time here would be a second striker, and the two
/// would part the day the walk changed. What the kennel asks is whether what the
/// file states is still the tree, and that question consumes the sha and the
/// roots as they stand.
///
/// A FILE THAT DOES NOT SAY REFUSES, fail-closed like a binary that states no
/// position: an exergue whose shape has moved from under this reader cannot be
/// presumed current, and the plaint says what was not found.
fn zbkcd_struck(at: &Path) -> Result<zbkcd_Struck, String> {
    let text = std::fs::read_to_string(at)
        .map_err(|err| format!("could not read the exergue at {}: {}", at.display(), err))?;

    let mut sha: Option<String> = None;
    let mut roots: Vec<String> = Vec::new();
    let mut within_roots = false;

    for line in text.lines() {
        let line = line.trim();

        if within_roots {
            if line.starts_with("];") {
                within_roots = false;
            } else if let Some(root) = zbkcd_quoted(line) {
                roots.push(root);
            }
            continue;
        }

        let Some(declared) = line.strip_prefix("pub const ") else {
            continue;
        };

        if declared.contains("_ROOTS:") && declared.ends_with("&[") {
            within_roots = true;
            continue;
        }

        if sha.is_none() && declared.contains(": &str = ") {
            if let Some(value) = zbkcd_quoted(declared) {
                if bkce_election::bkce_position_shaped(&value) {
                    sha = Some(value);
                }
            }
        }
    }

    let Some(sha) = sha else {
        return Err(format!(
            "the exergue at {} carries no position constant in the striker's shape — `pub const \
             <NAME>: &str = \"<sha>\";` — so what it was struck from cannot be read, and it may not \
             be presumed current",
            at.display()
        ));
    };

    if roots.is_empty() {
        return Err(format!(
            "the exergue at {} carries no roots constant in the striker's shape — `pub const \
             <NAME>_ROOTS: &[&str] = &[ ... ];` — so the set its sha was measured over cannot be \
             read, and it may not be presumed current",
            at.display()
        ));
    }

    Ok(zbkcd_Struck {
        sha,
        roots: roots.join(" "),
    })
}

/// The text between the first pair of double quotes on a line, where the line
/// carries one.
fn zbkcd_quoted(line: &str) -> Option<String> {
    let open = line.find('"')?;
    let rest = &line[open + 1..];
    let close = rest.find('"')?;
    Some(rest[..close].to_string())
}

/// The report a door hands its reader for a tree that cannot be green as it
/// stands.
///
/// IT NAMES EVERY ABSENCE AND THEN THE RE-RUN, in that order. The re-run is the
/// door the reader was already running, resolved through the same lookup as
/// every supplying act above, so a reader who has worked the list has the whole
/// sequence in front of them and never a remembered filename.
///
/// THE TWO SHAPES RENDER AS TWO SENTENCES. "Nothing stands" is what the delouse
/// leaves behind and any strike cures it; "stands, and may not be presumed
/// current" carries the plaint saying which roots moved, and is what a pull
/// leaves behind. A reader who has met the first on a morning that opened on a
/// derby must not read the second as more of the same.
pub fn bkcd_unready(absent: &[bkcd_Absence], rerun: &str) -> String {
    let mut said = format!(
        "{} precondition(s) this tree declares are not met, so no course is run — a walk over a \
         tree that cannot be green would spend a compile proving it. The kennel verifies and \
         never provisions, so each is supplied by the act beside it:",
        absent.len()
    );

    for absence in absent {
        let stood = match &absence.stands {
            None => format!("nothing stands at {}", absence.at.display()),
            Some(why) => format!(
                "stands at {}, and may not be presumed current: {}",
                absence.at.display(),
                why
            ),
        };

        said.push_str(&format!(
            "\n  {}\n    {}\n    supplied by: {}",
            absence.subject, stood, absence.supply
        ));
    }

    said.push_str(&format!("\n  then, once every one of them stands: {}", rerun));

    said
}

// eof
