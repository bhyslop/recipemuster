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

//! The leash: the one chokepoint every cargo invocation the kennel issues
//! passes through.
//!
//! Three disciplines, applied here and nowhere else, each closing a finding the
//! rust-build memo's survey left standing:
//!
//! THE PIN IS STATED, NEVER INHERITED. A toolchain file is discovered from the
//! working directory, and the engine-invocation census found a build door
//! running from a repository root where the pin standing at a subdirectory was
//! out of scope — rustup answered "(default)" there, and the channel that ran
//! was the pinned one only because the store's default happened to be it. That
//! is the memo's silently-inert-bump finding standing in the tree. So the
//! channel is read from the nearest pin at or above the manifest's own
//! directory, and spelled at the invocation as `+channel`, where nothing about
//! the working directory can change the answer.
//!
//! THE LOCK IS ENFORCED. `--locked` rides every invocation but one, and the
//! exception is a door rather than a flag: `bkcl_gangline` below is the single
//! face that lifts it, for the single act of re-deriving a lock on purpose. The
//! enforcement being total is exactly what makes that act necessary — a manifest
//! that gained a dependency can otherwise reach no lock that answers it. The
//! census found the flag
//! absent from every standing invocation in the estate, its refusal available
//! and merely unasked-for: without it, a manifest edit committed without its
//! lockfile builds anyway, silently resolving whatever the registry offers that
//! day, and the position a stamp records stops describing what was built.
//!
//! THE HOMES ARE STATED ON EVERY TOOLCHAIN COMMAND, NEVER INHERITED. The tackroom
//! is read from the dispatch's file and the two homes are derived beneath it in
//! the layout `Tools/bkk/bk0/bkcb_tackroom.sh` composes, so this process carries
//! no `CARGO_HOME` or `RUSTUP_HOME` of its own and each cargo, plugin, uv and
//! rustup it spawns is told the store it stands on. The fence still proves the
//! redirect: both homes must resolve inside the declared tackroom. Neither the
//! bash fence nor this one provisions, because provisioning is a converge and a
//! routine invocation may not download: the pinned toolchain is laid by the rebed
//! door, the kennel's readiness converge, and every refusal below that finds it
//! missing or gutted names that door.

use crate::bkcf_guard;
use crate::bkco_output;
use std::ffi::OsStr;
use std::path::{
    Path,
    PathBuf, //
};
use std::process::{
    Command,
    Stdio, //
};

/// The station's shared tool store, as the dispatch's file names it.
pub const BKCL_TACKROOM_VAR: &str = "BURD_TACKROOM";

/// The two homes the fence redirects, stated on every toolchain command the
/// leash composes.
pub const BKCL_CARGO_HOME_VAR: &str = "CARGO_HOME";
pub const BKCL_RUSTUP_HOME_VAR: &str = "RUSTUP_HOME";

/// Where each home stands beneath the tackroom.
///
/// SPELLED ONCE PER REGISTER, and the bash register's spelling stands in
/// `Tools/bkk/bk0/bkcb_tackroom.sh` (`BKCB_CARGO_HOME`, `BKCB_RUSTUP_HOME`), which
/// composes the same layout for the whistle before this binary exists. A hurdle
/// holds the two spellings equal, because a home derived differently at either
/// seat is a second store the other never fills.
pub const BKCL_CARGO_DIR: &str = "cargo";
pub const BKCL_RUSTUP_DIR: &str = "rustup";

/// The file a channel is stated in. A repository states one at its root, so
/// every crate it holds is answered for; a kit that pins its own stands one
/// beside its manifest and overrides the root for itself alone.
pub const BKCL_PIN_FILE: &str = "rust-toolchain.toml";

/// The flag that makes an out-of-date lockfile a refusal rather than a silent
/// re-resolution.
///
/// The spelling is the tool's own, declared as a xenonym spelling line under
/// that authority's carrier, which is where the ruling lives now; VOr_9ww
/// honours the carrier and stands the value down.
pub const BKCL_LOCKED_FLAG: &str = "--locked";

/// The flag that names the crate an invocation is about, so cargo discovers none
/// by walking up from wherever the process happens to stand.
///
/// It spells a foreign name for the same reason the lock flag does: cargo owns
/// it. Published rather than kept interior because the register's dispatch must
/// recognize the flag in a caller's arguments to lift it back out, and two
/// spellings of one foreign word are two things to keep in step.
pub const BKCL_MANIFEST_FLAG: &str = "--manifest-path";

/// THE VERBS A KIBBLE SERVES, and the kibble that serves each.
///
/// A CARGO SUBCOMMAND THE KENNEL DID NOT BUILD IS SPAWNED FROM ITS OWN
/// RESIDENCE AND NEVER FOUND. Cargo resolves a plugin by looking in
/// `CARGO_HOME/bin` and then along the station's path, and either road answers
/// with whatever happens to be installed — which is exactly the trust a kibble
/// exists to remove. So a verb in this table is not handed to cargo at all: the
/// composition below spawns the kibble's own binary at an absolute path, and the
/// plugin takes its subcommand as its first argument the same way cargo hands it
/// one.
///
/// A VERSION CONSTANT ONCE STOOD HERE and is retired into the declaration. The
/// kennel's claim about a station used to be a number in this file, checked by
/// spawning the program and reading what it said about itself; what stands now
/// is a POINTER, and the version it resolves to is read from the kibble every
/// time. So a bump is one edit in one file, nothing in this crate can name a
/// version the kibble no longer declares, and the check is a path test rather
/// than a program's self-report.
///
/// NOTHING HERE DOWNLOADS, which is the posture the channel check below keeps
/// and for the same reason. An absent residence refuses and names *heel*.
const ZBKCL_KIBBLED: &[(&str, &str)] = &[(
    crate::bkcm_mush::BKCM_VERB_NEXTEST,
    crate::bkcq_kibble::BKCQ_NEXTEST,
)];

/// The environment variable the kennel's own build script demands, carrying the
/// position the build is taken at (`Tools/bkk/bk0/build.rs`).
///
/// STATED BY THE COMPOSITION AND BY NO CALLER. The build script dies without it,
/// so before this was homed here every door that could build a kennel-linking
/// crate had to compose the reading itself — the election read, the position
/// walked, the variable exported — and a door that did not know it was owed
/// simply could not build such a crate at all. The engine was that door, and the
/// course record's roster stood spelled twice because of it.
pub const BKCL_SEAT_VAR: &str = "BKCL_SEAT_POSITION";

/// Where the kennel's own election stands, repo-relative — the one home of a
/// path five bash doors and one rust door each used to compose.
///
/// ITS PRESENCE IS THE QUESTION ASKED. The position is a fact about the KENNEL
/// rather than about the tree under cargo, and the two are not always one tree:
/// a lure carrying one small crate holds no kennel, so there is no position to
/// read and no build script that would want one. An absent election is an answer
/// and never a failure; a present one that cannot be read still refuses, which
/// is what keeps the absence from swallowing a real fault.
pub const BKCL_KENNEL_ELECTION: &str = "Tools/bkk/bk0/bkce_roots.txt";

/// Rustup's own environment selector for a channel, which is what states the pin
/// to a program that is NOT cargo.
///
/// TWO SPELLINGS OF ONE PIN, AND ONE HOME. Cargo takes a channel as a leading
/// `+channel` argument and a plugin spawned directly takes none — it is not
/// cargo and would read the token as its own. Rustup honors this variable at
/// every shim it installs, so the channel reaches the compiler either way. Both
/// spellings are composed below from the ONE channel this module reads out of
/// the nearest pin file, so there is no second reading to drift: what differs is
/// how the answer is said, never what was asked.
const ZBKCL_TOOLCHAIN_VAR: &str = "RUSTUP_TOOLCHAIN";

pub const BKCL_GIT_PROGRAM: &str = "git";

/// GIT TAKES A FLOOR, not an exact pin: it is the station's own utility, held
/// by the station's own packaging, and stations differ. The floor is THE LOWEST
/// VERSION THE KENNEL'S OWN INVOCATIONS NEED and is not a round number chosen
/// to look like one — a floor invented above what the code asks for refuses
/// stations for no reason the kennel could name.
///
/// What sets it: `git -C <directory>`, which arrived in 1.8.5 and is the newest
/// thing any invocation of this crate asks for. Everything else the kennel
/// spells is older — `status --porcelain`, `rev-parse`, `log --first-parent`
/// and the `%H`/`%ct` format specifiers. The floor MOVES when an invocation
/// starts asking for something newer, and moving it is part of that edit.
pub const BKCL_GIT_FLOOR: &str = "1.8.5";

/// The version floor rustup must clear, read from `rustup --version`.
///
/// SET AT THIS LANDING TO THE VERSION THIS STATION REPORTS, and lowered only
/// deliberately. Rustup is the toolchain's trust root rather than ground: it
/// fetches and verifies the compiler the whole discipline stands on, so what
/// verifies the toolchain is itself a fact the kennel judges rather than takes
/// on faith.
pub const BKCL_RUSTUP_FLOOR: &str = "1.29.1";

/// The separator past which arguments stop belonging to cargo and start
/// belonging to whatever cargo launched. The one piece of grammar the leash may
/// read without knowing a verb, which is why the discipline's flags stand ahead
/// of it rather than after.
pub const BKCL_SEPARATOR: &str = "--";

/// A cargo invocation composed under the discipline, and what came of it.
#[derive(Debug, Clone)]
pub struct bkcl_Run {
    /// The whole command line as spelled, for the record and for a diagnostic
    /// that means to show what was actually asked.
    pub spelling: String,
    /// The child's exit code, or `None` where a signal took it.
    pub code: Option<i32>,
    /// What broke an attended stream's read before the child closed it, or
    /// `None` where every stream was read to its end or none was read at all.
    ///
    /// A CUT IS THE KENNEL'S OWN FACT, never the child's. The child may have
    /// gone on to exit cleanly, but what it said past the cut reached neither
    /// the counts nor the record, so a run carrying one is a run nobody heard
    /// whole.
    pub cut: Option<String>,
}

impl bkcl_Run {
    /// Whether the invocation succeeded: a clean exit over streams heard whole.
    pub fn bkcl_landed(&self) -> bool {
        self.code == Some(0) && self.cut.is_none()
    }
}

/// An invocation whose streams the caller TOOK rather than inherited, and what
/// the child said on each of them.
#[derive(Debug, Clone)]
pub struct bkcl_Recall {
    /// The invocation as spelled, and the exit it took — the same record a
    /// driven run answers with, carried rather than restated so the two faces
    /// cannot come to describe a run differently.
    pub run: bkcl_Run,
    /// The child's stdout, VERBATIM AND UNDECODED. Cargo's machine-readable
    /// forms are JSON, and a lossy decode here would corrupt an answer before
    /// its reader ever saw it — the one failure a caller could not detect,
    /// because what arrives is still well-formed text.
    pub said: Vec<u8>,
    /// The child's stderr as text, which is where cargo puts its own account of
    /// a refusal. Lossy on purpose: this is read by a person or quoted into a
    /// finding, and is never parsed.
    pub grievance: String,
}

/// Read the channel stated by the pin standing AT one directory, and nowhere
/// else. Interior, because which directory to ask is the whole question and no
/// caller outside this module should be answering it for itself.
///
/// The parse is deliberately narrow — the first `channel = "..."` line wins —
/// and the narrowness is the point: a TOML crate in the kennel's closure would
/// be a permitted-dependency question asked for two lines of text, and the pin
/// file's shape is fixed by rustup rather than by us.
fn zbkcl_pin_at(home: &Path) -> Result<String, String> {
    let path = home.join(BKCL_PIN_FILE);

    let text = std::fs::read_to_string(&path).map_err(|err| {
        format!(
            "no pin at {}: {} — every crate the kennel builds is answered for by a stated \
             channel, and a build that inherited one would be pinned only by luck",
            path.display(),
            err, //
        )
    })?;

    for line in text.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("channel") else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix('=') else {
            continue;
        };
        let value = rest.trim().trim_matches('"').trim();
        if !value.is_empty() {
            return Ok(value.to_string());
        }
    }

    Err(format!(
        "the pin at {} names no channel",
        path.display(), //
    ))
}

/// Read the channel that governs one crate: the nearest pin at or above the
/// crate's own directory, searched no further than the repository that holds it.
///
/// ANCHORED AT THE CRATE, BOUNDED BY THE REPOSITORY, and both halves are
/// load-bearing.
///
/// Anchored at the crate, because that is what keeps a kit's own pin the one
/// that answers for it. A channel read from the process's working directory
/// would find whatever stands at the root and ignore the pin beside the manifest
/// entirely — which is the rust-build memo's silently-inert-bump finding exactly,
/// and the property the founding proved by bumping the kennel's pin and watching
/// the compiler follow it.
///
/// Bounded by the repository, because a walk that ran off the top would leave
/// the tree under test and read a pin belonging to whatever happens to stand
/// above it — a build pinned by the shape of somebody's home directory. The
/// bound also keeps a lure honest: a lure composes a repository of its own, so a
/// hurdle that removes its pin gets a refusal rather than a neighbor's answer.
///
/// A repository holding one crate states its channel once at its root and this
/// walk finds it in a step, which is why the reading is the same one whether the
/// kennel is driven over a lure or over an estate of many kits.
pub fn bkcl_pin(repository: &Path, crate_dir: &Path) -> Result<String, String> {
    let bound = zbkcl_resolved(repository).ok_or_else(|| {
        format!(
            "the repository at {} does not resolve",
            repository.display(), //
        )
    })?;

    let mut standing = zbkcl_resolved(crate_dir).ok_or_else(|| {
        format!(
            "the crate at {} does not resolve",
            crate_dir.display(), //
        )
    })?;

    // THE BOUND IS PROVEN BEFORE THE WALK, never merely reached during it. A
    // crate standing outside the repository never equals the bound, so a loop
    // that only watched for equality would walk clean past it to the filesystem
    // root and answer with a stranger's pin — the silently-wrong-channel this
    // whole module exists to close, let back in through the one path nobody
    // would drive on purpose. Refused here, where being outside the tree is the
    // finding rather than a step on the way to one.
    if !standing.starts_with(&bound) {
        return Err(format!(
            "the crate at {} stands outside the repository at {}, so no pin of that repository's \
             governs it and a walk upward would answer with whatever pin happens to stand above \
             the tree. Name the repository the crate belongs to",
            standing.display(),
            bound.display(), //
        ));
    }

    loop {
        if standing.join(BKCL_PIN_FILE).is_file() {
            return zbkcl_pin_at(&standing);
        }

        if standing == bound {
            break;
        }

        match standing.parent() {
            Some(above) => standing = above.to_path_buf(),
            None => break,
        }
    }

    Err(format!(
        "no pin governs {}: no {} stands there, nor anywhere above it within {}. Every crate the \
         kennel builds is answered for by a stated channel — a repository states one at its root \
         and a kit that pins its own overrides it — because a build that inherited its channel \
         would be pinned only by whatever the station happened to default to",
        crate_dir.display(),
        BKCL_PIN_FILE,
        bound.display(), //
    ))
}
/// The rust host triple the pinned toolchain declares for this station.
///
/// THE TOKEN IS RUSTC'S AND IS NEVER COMPARED, which is the whole shape of this
/// reading rather than a caution about it. A prebuilt kibble joins this token
/// into the archive name its vendor's bordereau gives, and joins it as opaque
/// text: the kennel holds no platform table, tests no operating system, and
/// reads no architecture out of the string. What it knows is that rustc said
/// this, and that the vendor either published an archive under that spelling or
/// did not.
///
/// THROUGH THE PIN LIKE EVERY OTHER TOOLCHAIN QUESTION. The channel is read
/// from the pin governing the given directory, and the compiler asked is that
/// channel's own in the tackroom — so the triple answered is the one the
/// toolchain a build would use declares, and never whatever rustc the station's
/// path happens to offer.
///
/// THE FENCE STANDS FIRST, because a rustc reached outside the tackroom would be
/// reading a home this kennel does not answer for.
///
/// THE ANSWER IS THE VERIFY'S OWN. Proving the channel already asks the
/// toolchain's compiler for its verbose version, and the host triple is a line
/// of that answer, so the triple is taken from the proof rather than asked for a
/// second time.
pub fn bkcl_host(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    crate_dir: &Path,
) -> Result<String, String> {
    let tackroom = bkcl_fenced(dispatch)?;

    let channel = bkcl_pin(repository, crate_dir)?;
    zbkcl_channel_answers(dispatch, &tackroom, &channel)
}

/// The compiler asked for its own making, the flag that asks, and the labels
/// that answer. Spelled once so the reading and the refusals that name it cannot
/// drift apart.
const ZBKCL_RUSTC: &str = "rustc";
const ZBKCL_VERBOSE_VERSION: &str = "-vV";
const ZBKCL_HOST_LABEL: &str = "host: ";
const ZBKCL_RELEASE_LABEL: &str = "release: ";

/// Where rustup lays a toolchain beneath its home, and where the compiler stands
/// inside one: rustup's own layout, `<home>/toolchains/<channel>-<host>/bin`.
const ZBKCL_TOOLCHAINS_DIR: &str = "toolchains";
const ZBKCL_TOOLCHAIN_BIN: &str = "bin";

/// Where a toolchain keeps the compiler's driver library, and the stem the
/// library's name opens on: `lib/librustc_driver-<hash>.<ext>`, or
/// `bin/rustc_driver-<hash>.dll` on Windows.
const ZBKCL_TOOLCHAIN_LIB: &str = "lib";
const ZBKCL_DRIVER_STEM: &str = "rustc_driver-";

/// What every refusal of the channel's proof hands its reader, ahead of the door
/// it names.
///
/// SPELLED ONCE PER REGISTER, and the bash register's spelling stands in
/// `Tools/bkk/bk0/bkcb_tackroom.sh`, where the whistle refuses before this
/// binary exists. A hurdle holds the two spellings equal, because a reader
/// meeting the refusal at either seat is being told the same thing.
pub const BKCL_CONVERGE_SENTENCE: &str = "A routine invocation verifies and refuses; it never \
     downloads unasked. Every toolchain the kennel requires is laid by its readiness converge,";

/// Prove the tackroom's toolchain for the channel an invocation is about to
/// state ANSWERS, and refuse naming the door that re-lays it where it does not.
///
/// THE PROOF IS THE COMPILER'S OWN ANSWER, NEVER RUSTUP'S LEDGER. A listing says
/// a channel was installed; it went on saying so through a storage purge that
/// had removed the compiler's own libraries, and the first thing to run the
/// compiler was a build script, which panicked. So the toolchain's rustc is
/// asked at its absolute path for its verbose version, and the proof is that
/// `release:` names the channel and `host:` names something.
///
/// NEVER THROUGH A PROXY. Told to run a channel it does not hold, rustup fetches
/// one — quietly, and on a slow link for long enough that the build reads as
/// hung — which is a download the standing ruling forbids of a routine
/// invocation. A compiler reached at its own path fetches nothing, and an absent
/// channel refuses here before anything is spawned at all.
///
/// THE PURE HALF, AND IT IS PURE SO IT CAN BE HURDLED. The home is taken as a
/// value, so a hurdle poses one beside it without writing the process
/// environment its neighbours read — the seam `bkco_tabtarget_in` and
/// `bkcq_feedroom_at` are split at, for the same reason. Two toolchain
/// directories answering one channel is an ambiguity this refuses rather than
/// resolves, on `bkco_tabtarget_in`'s none-on-any-doubt ground.
///
/// Answers the host triple the compiler declared, which `bkcl_host` hands on.
pub fn bkcl_channel_answers(
    rustup_home: &Path,
    channel: &str,
    tabtargets: Option<&Path>,
) -> Result<String, String> {
    let door = bkco_output::bkco_call(tabtargets, bkco_output::BKCO_VERB_REBED, None);
    let refuse = |condition: String| {
        format!(
            "{}. {} {}",
            condition,
            BKCL_CONVERGE_SENTENCE,
            door, //
        )
    };

    // A CHANNEL THE ANSWER CANNOT EQUAL IS REFUSED, NOT GUESSED AT. The proof
    // compares the pin against the release the compiler names, and a named
    // channel or a two-part version resolves to a release only rustup knows,
    // which is the ledger this proof exists to stop trusting.
    let numeric = channel.split('.').count() == 3
        && channel
            .split('.')
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()));
    if !numeric {
        return Err(refuse(format!(
            "the pin names {}, which is no release a compiler answers with: the kennel proves a \
             channel by the release its compiler names, so the pin is a three-part version",
            channel, //
        )));
    }

    let toolchains = rustup_home.join(ZBKCL_TOOLCHAINS_DIR);
    let prefix = format!(
        "{}-",
        channel, //
    );

    let mut laid: Vec<PathBuf> = std::fs::read_dir(&toolchains)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.file_name().to_string_lossy().starts_with(&prefix))
                .map(|entry| entry.path())
                .collect()
        })
        .unwrap_or_default();

    let toolchain = match laid.len() {
        0 => {
            return Err(refuse(format!(
                "the tackroom holds no toolchain for {} under {}",
                channel,
                toolchains.display(), //
            )));
        }
        1 => laid.remove(0),
        _ => {
            laid.sort();
            return Err(refuse(format!(
                "the tackroom holds {} toolchains answering {} under {} — {} — and the kennel \
                 will not pick one on the operator's behalf",
                laid.len(),
                channel,
                toolchains.display(),
                laid.iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", "), //
            )));
        }
    };

    // THE DRIVER LIBRARY IS READ BEFORE THE COMPILER IS SPAWNED — a membrane at
    // the Palisade, held alike in the bash register at
    // `bkcb_channel_answers_predicate`. The signature: on macOS a storage purge
    // removed `lib/librustc_driver-<hash>.dylib` and left `bin/rustc`, and a
    // compiler spawned without its driver aborts at load, which the station
    // answers with a crash dialog a person must dismiss. So a toolchain holding
    // no driver library is refused as not answering and nothing is spawned.
    // Windows lays the driver beside the compiler in `bin`, so either seat
    // admits it. It retires when the station no longer raises a dialog for a
    // compiler aborting at load; the spawn below then reaches the same verdict.
    if !zbkcl_driver_stands(&toolchain) {
        return Err(refuse(format!(
            "the toolchain for {} stands at {} without its compiler's driver library ({}/lib{}*), so \
             its compiler would abort at load and is not spawned",
            channel,
            toolchain.display(),
            ZBKCL_TOOLCHAIN_LIB,
            ZBKCL_DRIVER_STEM, //
        )));
    }

    let rustc = toolchain.join(ZBKCL_TOOLCHAIN_BIN).join(ZBKCL_RUSTC);

    let asked = Command::new(&rustc)
        .arg(ZBKCL_VERBOSE_VERSION)
        .stdin(Stdio::null())
        .output()
        .map_err(|err| {
            refuse(format!(
                "the toolchain for {} stands at {} but its compiler did not answer: {}",
                channel,
                toolchain.display(),
                err, //
            ))
        })?;

    if !asked.status.success() {
        return Err(refuse(format!(
            "'{} {}' refused: {}",
            rustc.display(),
            ZBKCL_VERBOSE_VERSION,
            String::from_utf8_lossy(&asked.stderr).trim(), //
        )));
    }

    let said = String::from_utf8_lossy(&asked.stdout);

    // THE LABELS ARE RUSTC'S OWN AND ARE MATCHED RATHER THAN COUNTED TO. The
    // verbose version answer is a block of `key: value` lines whose order rustc
    // has moved between releases, so a reading that took the nth line would be
    // pinned to a layout nobody promised.
    let labelled = |label: &str| {
        said.lines()
            .find_map(|line| line.strip_prefix(label))
            .map(|rest| rest.trim().to_string())
            .filter(|value| !value.is_empty())
    };

    let release = labelled(ZBKCL_RELEASE_LABEL);
    if release.as_deref() != Some(channel) {
        return Err(refuse(format!(
            "'{} {}' answered release {} where the pin names {}. What it said:\n{}",
            rustc.display(),
            ZBKCL_VERBOSE_VERSION,
            release.as_deref().unwrap_or("<none>"),
            channel,
            said.trim(), //
        )));
    }

    labelled(ZBKCL_HOST_LABEL).ok_or_else(|| {
        refuse(format!(
            "'{} {}' answered no '{}' line. What it said:\n{}",
            rustc.display(),
            ZBKCL_VERBOSE_VERSION,
            ZBKCL_HOST_LABEL.trim(),
            said.trim(), //
        ))
    })
}

/// Whether a laid toolchain holds its compiler's driver library: a file
/// wearing the driver's stem under `lib`, or under `bin` where Windows lays it.
fn zbkcl_driver_stands(toolchain: &Path) -> bool {
    [
        (
            ZBKCL_TOOLCHAIN_LIB,
            format!(
                "lib{}",
                ZBKCL_DRIVER_STEM, //
            ),
        ),
        (ZBKCL_TOOLCHAIN_BIN, ZBKCL_DRIVER_STEM.to_string()),
    ]
    .iter()
    .any(|(seat, stem)| {
        std::fs::read_dir(toolchain.join(seat))
            .map(|entries| {
                entries.flatten().any(|entry| {
                    entry.path().is_file()
                        && entry
                            .file_name()
                            .to_string_lossy()
                            .starts_with(stem.as_str())
                })
            })
            .unwrap_or(false)
    })
}

/// The proof above, over the rustup home beneath the fenced tackroom.
///
/// The fence stands proven before this runs, so the home read here is the
/// tackroom's, and the proof answers for the store the build will actually reach
/// rather than the station user's own.
///
/// The whistle holds this same proof for the kennel's own pin and cannot share
/// this one: it runs before the binary carrying it exists.
fn zbkcl_channel_answers(
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    tackroom: &Path,
    channel: &str,
) -> Result<String, String> {
    bkcl_channel_answers(
        &bkcl_rustup_home(tackroom),
        channel,
        dispatch.bkcr_tabtargets(),
    )
}

/// The cargo home beneath a tackroom.
pub fn bkcl_cargo_home(tackroom: &Path) -> PathBuf {
    tackroom.join(BKCL_CARGO_DIR)
}

/// The rustup home beneath a tackroom.
pub fn bkcl_rustup_home(tackroom: &Path) -> PathBuf {
    tackroom.join(BKCL_RUSTUP_DIR)
}

/// State both homes beneath the tackroom on one toolchain command.
///
/// ON THE COMMAND AND NEVER ON THIS PROCESS. A home this process held would reach
/// every child it spawns, a course among them, and a course is handed the
/// dispatch's pointer and nothing else of the kit; a home stated here reaches the
/// one program that asked for it.
fn zbkcl_homed(command: &mut Command, tackroom: &Path) {
    command.env(BKCL_CARGO_HOME_VAR, bkcl_cargo_home(tackroom));
    command.env(BKCL_RUSTUP_HOME_VAR, bkcl_rustup_home(tackroom));
}

/// Prove the tackroom the dispatch names is a store the homes stand confined
/// to, and answer the store.
///
/// The tackroom must be declared in the dispatch's file, and both homes beneath
/// it must resolve inside it. The check is on the RESOLVED paths, so a home that
/// is itself a symlink out of the store is caught: the lexical spelling would
/// read as confined while the bytes landed in the station user's own rustup.
pub fn bkcl_fenced(dispatch: &crate::bkcr_resolve::bkcr_Dispatch) -> Result<PathBuf, String> {
    let tackroom = dispatch.bkcr_said(BKCL_TACKROOM_VAR).ok_or_else(|| {
        format!(
            "{} carries no {}. The kennel homes the pinned toolchain and the cargo registry in \
             the station's tackroom and will not fall back to a per-clone copy; the station \
             declares it as BURS_TACKROOM, and the fence that guards it is \
             Tools/bkk/bk0/bkcb_tackroom.sh, reached through the whistle",
            dispatch.bkcr_whence(),
            BKCL_TACKROOM_VAR, //
        )
    })?;

    let root = zbkcl_resolved(Path::new(tackroom)).ok_or_else(|| {
        format!(
            "{} names {}, which does not resolve — the station declares a store that is not \
             there, and neither the fence nor the kennel creates one",
            BKCL_TACKROOM_VAR,
            tackroom, //
        )
    })?;

    for (var, declared) in [
        (BKCL_CARGO_HOME_VAR, bkcl_cargo_home(Path::new(tackroom))),
        (BKCL_RUSTUP_HOME_VAR, bkcl_rustup_home(Path::new(tackroom))),
    ] {
        let home = zbkcl_resolved(&declared).ok_or_else(|| {
            format!(
                "{} would name {}, which does not resolve",
                var,
                declared.display(), //
            )
        })?;

        if !home.starts_with(&root) {
            return Err(format!(
                "tackroom posture violated: {} resolved to {}, which stands outside the declared \
                 tackroom {}",
                var,
                home.display(),
                root.display(), //
            ));
        }
    }

    // THE TRUST ROOT IS PROVEN HERE, beside the redirect it verifies. Every
    // composition already stands on this fence, so checking rustup's floor here
    // is what makes every door meet it once, cached at `bkcl_rustup_held`
    // rather than re-asked per invocation.
    bkcl_rustup_held(&root)?;

    Ok(root)
}

/// Compose and run a cargo invocation under the whole discipline, HANDING the
/// child this process's streams.
///
/// The caller states the verb and whatever that verb owns; the pin, the lock
/// flag and the manifest are the leash's and are not the caller's to spell. That
/// is what makes this a chokepoint rather than a helper — a caller cannot reach
/// cargo with the discipline half-applied, because the parts it could get wrong
/// are not parameters.
///
/// THE CHILD INHERITS THIS PROCESS'S STREAMS, and that is now this face's one
/// reason to stand. Every cargo act the kennel's voice renders — a build, a
/// suite, a lock's re-derivation — takes the attended face below and speaks as
/// the door's own; what is left here is the leash register's verbs other than
/// a build, whose stdout is the CALLER'S data rather than a console. The engine
/// build door reads a dependency tree off that stream to prove the shipped
/// binary links no census crate, and a stream the voice had consumed would
/// match nothing, which that guard reads as passing. A caller that wants the
/// child's answer as data inside this process takes the recall below; the
/// discipline the faces apply is one composition and is stated at it.
///
/// `stated` names environment the child needs and the caller alone knows —
/// applied at the composition, where the fence already sets the tackroom's own.
/// It reaches the child and never the flags: a caller states environment here
/// and arguments in `rest`, and neither road passes the discipline.
pub fn bkcl_drive<I, S>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    manifest: &Path,
    verb: &str,
    rest: I,
    stated: &[(&str, &OsStr)],
) -> Result<bkcl_Run, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let (mut command, spelling) =
        zbkcl_composed(repository, dispatch, manifest, verb, rest, stated, true)?;

    let status = command.status().map_err(|err| {
        format!(
            "could not run cargo: {} ({})",
            err,
            spelling, //
        )
    })?;

    Ok(bkcl_Run {
        spelling,
        code: status.code(),
        cut: None,
    })
}

/// Compose and run a cargo invocation under the whole discipline, TAKING the
/// child's streams instead of handing them on.
///
/// THE DISCIPLINE IS THE DRIVEN ONE, UNCHANGED. The pin, the lock, the manifest
/// and the fence are composed in the one place both faces reach, so a caller
/// cannot get a half-applied leash by asking for the answer instead of the
/// exit. What parts the two is the streams and nothing else.
///
/// WHY A SECOND FACE RATHER THAN A FLAG. The postures are not variants of one
/// act: a driven run hands cargo's stream on to whoever spawned this process,
/// to read as its own; a recalled run exists so a program here reads cargo's own
/// answer, and letting that stream reach a terminal would put a census's data on
/// the operator's screen and nowhere a caller could read it. A boolean between them would make the caller's most
/// consequential choice its least visible one.
///
/// THE EXIT IS THE CHILD'S HERE TOO, and a caller that means to refuse on it
/// must test it: this face returns `Ok` for a cargo that exited nonzero, the
/// refusal being about reaching cargo at all. The failure this shape invites is
/// reading `said` from a run that failed, so a caller reads the code first.
///
/// `stated` carries the caller's environment for the child, exactly as it does
/// at the driven face.
pub fn bkcl_recall<I, S>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    manifest: &Path,
    verb: &str,
    rest: I,
    stated: &[(&str, &OsStr)],
) -> Result<bkcl_Recall, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let (mut command, spelling) =
        zbkcl_composed(repository, dispatch, manifest, verb, rest, stated, true)?;

    let out = command.output().map_err(|err| {
        format!(
            "could not run cargo: {} ({})",
            err,
            spelling, //
        )
    })?;

    Ok(bkcl_Recall {
        run: bkcl_Run {
            spelling,
            code: out.status.code(),
            cut: None,
        },
        said: out.stdout,
        grievance: String::from_utf8_lossy(&out.stderr).into_owned(),
    })
}

/// What the attended face hands its caller, one at a time, while the child runs.
///
/// A LULL IS AN EVENT, and that is the whole reason this is an enum rather than
/// a line callback. A voice that renders a liveness mark on a cadence needs to
/// be told that time passed with nothing said — which is exactly the moment no
/// line arrives, so a per-line callback can never deliver it. Putting the lull
/// in the stream makes the cadence the leash's mechanism and what to DO about it
/// the caller's, which is the same split the two older faces already draw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkcl_Heard {
    /// One line of the child's output, newline stripped, from either stream.
    Said(String),
    /// The cadence elapsed with nothing said.
    Lull,
}

/// Compose and run a cargo invocation under the whole discipline, ATTENDING the
/// child's streams line by line as they arrive.
///
/// THE THIRD FACE, AND THE DISCIPLINE IS STILL THE DRIVEN ONE. Everything about
/// the invocation is composed in the one interior place the other two reach, so
/// this face cannot get a half-applied leash either. What parts it from them is
/// WHEN the caller hears the child: the driven face never hears it at all and
/// the recalled face hears it once, whole, after the exit.
///
/// WHY A THIRD FACE RATHER THAN A FLAG ON THE SECOND. A recalled run buffers to
/// completion, so a caller wanting to say something WHILE a suite runs cannot be
/// served by it at any setting — the answer arrives when there is no longer
/// anything to be live about. That is not a variant of the recall; it is the
/// posture the recall structurally excludes, which is what has earned each face
/// its own name here.
///
/// BOTH STREAMS ARE MERGED, DELIBERATELY. The two admitted runners do not agree
/// about which stream carries a verdict — nextest's summary is on stderr, the
/// standard harness's on stdout — so a caller that had to pick a stream would be
/// picking a runner. Merging is also what the record wants: what goes into the
/// log family is the child's WHOLE stream, and a family assembled from two
/// separately-ordered halves would record an interleaving that never happened.
///
/// THE EXIT IS THE CHILD'S, as at both other faces, and this face likewise
/// returns `Ok` for a cargo that exited nonzero.
pub fn bkcl_attend<I, S, F>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    manifest: &Path,
    verb: &str,
    rest: I,
    stated: &[(&str, &OsStr)],
    cadence: std::time::Duration,
    heed: F,
) -> Result<bkcl_Run, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnMut(bkcl_Heard),
{
    let (mut command, spelling) =
        zbkcl_composed(repository, dispatch, manifest, verb, rest, stated, true)?;
    zbkcl_coursed(&mut command, dispatch);

    zbkcl_attended(command, spelling, ZBKCL_CARGO, cadence, heed)
}

/// The kit's families, as the prefixes every name it speaks across an exec
/// wears.
const ZBKCL_KIT_FAMILIES: &[&str] = &[
    "BUC_",
    "BURC_",
    "BURD_",
    "BURE_",
    "BURS_",
    "BURV_",
    "BURX_",
    "BUZ_", //
];

/// Hand an attended child — every course a suite door launches, and the heel's
/// own builds — the dispatch's pointer and nothing else of the kit.
///
/// STRIPPED AT THE COURSE LAUNCH, on the two attended faces, and nowhere else:
/// the app arm hands its tenant this process's streams and environment as the
/// tenant's own, and rebed's heel and strike spawns are doors of their own. A
/// course reads its dispatch's values from the file the pointer names, so a value
/// of the kit this process carried that survived into it would be a second
/// channel for a fact the file already states — the run-root overrides and the
/// zipper's folio among them, each of which once made a hurdle's verdict a
/// function of the door that drove it. The environment regime goes too: a course
/// is a test, and its verdict must not move with the operator's dial.
fn zbkcl_coursed(command: &mut Command, dispatch: &crate::bkcr_resolve::bkcr_Dispatch) {
    for (name, _) in std::env::vars_os() {
        let spelled = name.to_string_lossy();
        if ZBKCL_KIT_FAMILIES
            .iter()
            .any(|family| spelled.starts_with(family))
        {
            command.env_remove(&name);
        }
    }

    command.env(
        crate::bkcr_resolve::BKCR_DISPATCH_FILE_VAR,
        dispatch.bkcr_whence(),
    );
}

/// The program each composition spawns, as a refusal names it.
const ZBKCL_CARGO: &str = "cargo";
const ZBKCL_UV: &str = "uv";

/// Spawn a composed command with both streams taken, and hand each line to
/// `heed` as it arrives and each silence of a whole cadence as a lull.
///
/// ONE ATTENDED LOOP FOR EVERY ATTENDED FACE. What parts the faces is the
/// composition — cargo's discipline with the lock riding or lifted, or uv's —
/// and what they do with the child once composed is the same act, so it is
/// spelled once.
fn zbkcl_attended<F>(
    mut command: Command,
    spelling: String,
    program: &str,
    cadence: std::time::Duration,
    mut heed: F,
) -> Result<bkcl_Run, String>
where
    F: FnMut(bkcl_Heard),
{
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|err| {
        format!(
            "could not run {}: {} ({})",
            program,
            err,
            spelling, //
        )
    })?;

    // A READER PER STREAM, because a single thread reading them in turn
    // DEADLOCKS: a child filling the pipe we are not reading blocks on the
    // write, and we block on the read of the other. Both runners fill both
    // pipes, so this is the ordinary case rather than a pathological one.
    let (post, arrivals) = std::sync::mpsc::channel::<String>();

    let mut porters = Vec::new();
    for stream in [
        child.stdout.take().map(zbkcl_Stream::Out),
        child.stderr.take().map(zbkcl_Stream::Err),
    ]
    .into_iter()
    .flatten()
    {
        let post = post.clone();
        porters.push(std::thread::spawn(move || zbkcl_porter(stream, post)));
    }
    // THE LAST SENDER MUST GO, or the drain below never sees the channel close
    // and waits on a stream nobody is filling.
    drop(post);

    // THE DRAIN IS A TIMED RECEIVE rather than a blocking one, which is what
    // turns silence into an event. A run whose compiler works for a minute
    // without a word is the case the liveness mark exists for, and a blocking
    // receive would render nothing for exactly as long as there was nothing to
    // render.
    loop {
        match arrivals.recv_timeout(cadence) {
            Ok(line) => heed(bkcl_Heard::Said(line)),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => heed(bkcl_Heard::Lull),
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    let cut = zbkcl_joined(porters);

    let status = child.wait().map_err(|err| {
        format!(
            "could not wait on {}: {} ({})",
            program,
            err,
            spelling, //
        )
    })?;

    Ok(bkcl_Run {
        spelling,
        code: status.code(),
        cut,
    })
}

/// The two pipes a porter may carry. Named rather than boxed so the read loop
/// below is one function over both.
enum zbkcl_Stream {
    Out(std::process::ChildStdout),
    Err(std::process::ChildStderr),
}

/// Carry one stream's lines onto the channel until it closes.
///
/// LOSSY DECODING IS CORRECT HERE and is the opposite of the recall's rule. What
/// this face carries is destined for a console and a log — read by a person,
/// matched by a recognizer — where a replacement character is a blemish; the
/// recall carries cargo's JSON, where one is a corruption its reader cannot
/// detect. Same crate, two faces, two right answers, so the reason is written at
/// both.
///
/// A FINAL LINE WITHOUT A TRAILING NEWLINE IS STILL CARRIED, which the log
/// family's own dicta require of every member: the substrate's curators guard
/// that case explicitly, and a reader here that dropped it would make the
/// kennel's record differ from a logged dispatch's in exactly one line — the
/// last, which is the one a reader looks at first.
fn zbkcl_porter(stream: zbkcl_Stream, post: std::sync::mpsc::Sender<String>) -> Option<String> {
    match stream {
        zbkcl_Stream::Out(out) => zbkcl_carried(std::io::BufReader::new(out), post),
        zbkcl_Stream::Err(err) => zbkcl_carried(std::io::BufReader::new(err), post),
    }
}

/// The porter's read loop over any reader, answering what cut the stream short
/// or `None` where it was read to its end.
///
/// OVER A READER RATHER THAN A PIPE so the cut can be planted: a pipe that fails
/// mid-read cannot be composed on demand, and a reader that fails after its
/// first lines can.
pub(crate) fn zbkcl_carried<R: std::io::BufRead>(
    mut reader: R,
    post: std::sync::mpsc::Sender<String>,
) -> Option<String> {
    let mut carried = Vec::new();
    loop {
        carried.clear();
        match reader.read_until(b'\n', &mut carried) {
            Ok(0) => break,
            Ok(_) => {
                while matches!(carried.last(), Some(b'\n') | Some(b'\r')) {
                    carried.pop();
                }
                if post
                    .send(String::from_utf8_lossy(&carried).into_owned())
                    .is_err()
                {
                    break;
                }
            }
            // A READ THAT FAILS IS A CUT AND IS ANSWERED AS ONE. Ending here as
            // if the stream had closed would hand the voice half a run with
            // nothing to say it was half.
            Err(err) => return Some(err.to_string()),
        }
    }

    None
}

/// Join the porters, answering the first cut any of them met.
///
/// A PORTER THAT PANICKED CUT ITS STREAM as surely as one whose read failed:
/// whatever the child said after it went unheard either way.
fn zbkcl_joined(porters: Vec<std::thread::JoinHandle<Option<String>>>) -> Option<String> {
    let mut cut = None;
    for porter in porters {
        let met = porter
            .join()
            .unwrap_or_else(|_| Some("the reader of one stream panicked".to_string()));
        if cut.is_none() {
            cut = met;
        }
    }
    cut
}

/// Compose and run a cargo invocation with THE LOCK FLAG LIFTED, attending the
/// child's streams line by line as the attended face does.
///
/// THIS IS THE ONE PLACE IN THE KENNEL WHERE `--locked` DOES NOT RIDE, and it
/// exists because the enforcement above is otherwise total: every invocation
/// refuses a lock that would change, so a manifest that gained a dependency has
/// no road to a lock that answers it. Two closes hand-wrote lock entries against
/// that wall before this face stood.
///
/// IT IS NAMED FOR ITS ONE CALLER RATHER THAN FOR WHAT IT LIFTS, and the naming
/// is the guard. A face called `unlocked` invites any caller who finds the lock
/// inconvenient; a face called `gangline` says in its own name that it belongs
/// to one door, so a second call site reads as the trespass it would be. The
/// door pace proves that by grep, and a reader who wonders whether the lift has
/// spread greps this identifier and reads the answer in two lines.
///
/// IT IS THE ATTENDED POSTURE, as every cargo act the kennel's voice renders
/// is. Re-deriving a lock reaches the registry's index, which on a slow link is
/// long enough to read as hung, and the cadence marks that wait; what cargo
/// says of the packages it locked, added and updated is what an operator reads
/// to see what moved, and the voice that heeds it keeps it on the console.
///
/// The whole rest of the discipline is unchanged and is not the caller's to
/// spell: the pin is still read from the manifest's own nearest pin file, the
/// fence is still stood on, and the manifest is still stated rather than
/// discovered. What lifts is one flag, for one act, at one door.
pub fn bkcl_gangline<I, S, F>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    manifest: &Path,
    verb: &str,
    rest: I,
    stated: &[(&str, &OsStr)],
    cadence: std::time::Duration,
    heed: F,
) -> Result<bkcl_Run, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnMut(bkcl_Heard),
{
    let (command, spelling) =
        zbkcl_composed(repository, dispatch, manifest, verb, rest, stated, false)?;

    zbkcl_attended(command, spelling, ZBKCL_CARGO, cadence, heed)
}

/// Compose and run a UV invocation from the kibble's own residence, handing the
/// child this process's streams.
///
/// A SECOND COMPOSITION RATHER THAN A BRANCH IN THE FIRST. The cargo composition
/// below is cargo-shaped to its roots — it reads a rust pin, states a channel,
/// spells the manifest flag and rides the lock flag — and uv answers to none of
/// it: uv's pin is a python version, its project is stated by its own flag, and
/// a rust channel means nothing to it. Threading a boolean into that composition
/// would make every one of those lines conditional and leave one function
/// describing two disciplines. What the two genuinely share is the fence and the
/// closed stdin, and both are spelled here rather than reached for.
///
/// THE WORKING DIRECTORY IS THE CALLER'S AND IS STATED RATHER THAN INHERITED,
/// WHICH THE CARGO COMPOSITION HAS NO NEED OF. Cargo is told its manifest by a
/// flag and answers the same wherever it stands; pytest is not, and TWO of its
/// behaviours turn on where it was invoked — its rootdir, and how it RENDERS a
/// node id. Its listing spells ids relative to the ROOTDIR while its per-case
/// lines spell them relative to the INVOCATION DIRECTORY, so a run spawned from
/// anywhere but the project reports cases under names the listing never said,
/// and every name the selection pipeline matched would fail to answer the name
/// the voice read back. Stating the directory is what makes the two spellings
/// one.
///
/// THE ENVIRONMENT IS THE WHOLE OF THE KENNEL'S CONTROL, AND THE CALLER COMPOSES
/// IT. What a project environment path or an interpreter store IS belongs to the
/// python family's own module; a leash that knew would hold a second copy of it,
/// free to disagree with the first. What the leash owns is that uv is reached at
/// its residence and at no other path on the station.
///
/// NO CONFIGURATION FLAG IS SPELLED HERE, AND THE OMISSION IS A RULING RATHER
/// THAN AN OVERSIGHT. uv's no-config flag drops the project's own uv table AND
/// the index declarations inside it, and drops them without refusing anything —
/// a project pinning a private index resolves from the public one under a green
/// exit. The kennel's control is the stated environment, which beats both the
/// project's own table and any `uv.toml` discovered above it. A reader reaching
/// for that flag here is answered at the site.
pub fn bkcl_uv<I, S>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    at: &Path,
    rest: I,
    stated: &[(&str, &OsStr)],
) -> Result<bkcl_Run, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let (mut command, spelling) = zbkcl_uv_composed(repository, dispatch, at, rest, stated)?;

    let status = command.status().map_err(|err| {
        format!(
            "could not run uv: {} ({})",
            err,
            spelling, //
        )
    })?;

    Ok(bkcl_Run {
        spelling,
        code: status.code(),
        cut: None,
    })
}

/// Compose and run a UV invocation, TAKING the child's streams instead of
/// handing them on — the uv plane's answer to the cargo recall beside it.
///
/// THE LISTING STEP IS THE STANDING CONSUMER. The selection pipeline asks a
/// runner to name its own hurdles and READS THE ANSWER, which is a program
/// reading data rather than an operator watching work; letting that stream reach
/// a terminal would put the listing on the screen and nowhere a caller could
/// parse it. The parting is the same one the cargo faces draw and is stated at
/// both.
///
/// THE EXIT IS THE CHILD'S, so a caller that means to refuse on a listing that
/// did not land must test it: this face returns `Ok` for a uv that exited
/// nonzero, the refusal being about reaching uv at all.
pub fn bkcl_uv_recall<I, S>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    at: &Path,
    rest: I,
    stated: &[(&str, &OsStr)],
) -> Result<bkcl_Recall, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let (mut command, spelling) = zbkcl_uv_composed(repository, dispatch, at, rest, stated)?;

    let out = command.output().map_err(|err| {
        format!(
            "could not run uv: {} ({})",
            err,
            spelling, //
        )
    })?;

    Ok(bkcl_Recall {
        run: bkcl_Run {
            spelling,
            code: out.status.code(),
            cut: None,
        },
        said: out.stdout,
        grievance: String::from_utf8_lossy(&out.stderr).into_owned(),
    })
}

/// Compose and run a UV invocation, ATTENDING the child's streams line by line
/// as they arrive — the uv plane's answer to the cargo attend beside it.
///
/// WHY THE CARGO ATTEND COULD NOT SERVE. That face composes through the cargo
/// chokepoint, which reads a rust pin, states a channel and spells the manifest
/// flag; uv answers to none of it. What the two share is the posture — a voice
/// rendering a tenant's stream as it runs — and the porter that carries it, so
/// the porter is the cargo face's own and is reached rather than copied.
///
/// BOTH STREAMS ARE MERGED, on the cargo attend's own ground: pytest writes its
/// cases to stdout and uv writes its own account to stderr, so a caller that had
/// to pick a stream would be picking which half of the run it heard.
pub fn bkcl_uv_attend<I, S, F>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    at: &Path,
    rest: I,
    stated: &[(&str, &OsStr)],
    cadence: std::time::Duration,
    heed: F,
) -> Result<bkcl_Run, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnMut(bkcl_Heard),
{
    let (mut command, spelling) = zbkcl_uv_composed(repository, dispatch, at, rest, stated)?;
    zbkcl_coursed(&mut command, dispatch);

    zbkcl_attended(command, spelling, ZBKCL_UV, cadence, heed)
}

/// The composition every uv face stands on: the fence, the residence, the
/// caller's directory, the caller's environment, and a command not yet run.
///
/// Interior on the cargo composition's own ground — a caller holding a composed
/// `Command` could add to it past the point where the residence was placed,
/// which is exactly the reach the chokepoint exists to deny.
fn zbkcl_uv_composed<I, S>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    at: &Path,
    rest: I,
    stated: &[(&str, &OsStr)],
) -> Result<(Command, String), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let tackroom = bkcl_fenced(dispatch)?;

    let residence = zbkcl_placed(repository, dispatch, crate::bkcx_python::BKCX_UV)?;

    let mut command = Command::new(&residence);
    command.current_dir(at);
    zbkcl_homed(&mut command, &tackroom);
    let mut spelling = residence.display().to_string();

    for arg in rest {
        let arg = arg.as_ref();
        spelling.push(' ');
        spelling.push_str(&arg.to_string_lossy());
        command.arg(arg);
    }

    // THE STATED ENVIRONMENT IS APPLIED WHOLE AND LAST, so nothing this function
    // spells can shadow what the caller stated. The kennel's roster is the
    // caller's; the residence is this function's; and the two never contend for
    // one name.
    for (name, value) in stated {
        command.env(name, value);
    }

    // NO STDIN, on the cargo composition's own ground: a uv that stopped to read
    // from a terminal would hang a door with nothing on screen to say why.
    command.stdin(Stdio::null());

    Ok((command, spelling))
}

/// The composition every face stands on: the whole discipline applied, and a
/// command not yet run.
///
/// Interior, and it must stay interior. A caller holding a composed `Command`
/// could add to it past the point where the flags were placed, which is exactly
/// the reach the chokepoint exists to deny — the parts a caller could get wrong
/// are not parameters, and they must not become reachable by another road.
///
/// THE DISCIPLINE'S OWN FLAGS COME LAST, after everything the caller spelled,
/// because the leash does not know a verb's interior grammar and must not
/// pretend to. A cargo PLUGIN carries a subcommand of its own — `cargo nextest`
/// takes `run` and `list` — and refuses a flag standing between the two: driven
/// as `cargo nextest --manifest-path <path> --locked list`, cargo-nextest answers
/// `unexpected argument '--locked' found`. Spelling the flags ahead of the
/// caller's arguments is therefore a guess about where one verb's options end,
/// and it is wrong for every plugin. Spelling them after is a guess about
/// nothing: an option is accepted there by cargo's own subcommands and by every
/// plugin alike.
///
/// The one thing that placement must respect is the `--` SEPARATOR, which is not
/// a guess about any verb but the universal mark for "everything past here
/// belongs to what the verb launched". The flags go ahead of it where the caller
/// spelled one, so a test filter never swallows the lock.
fn zbkcl_composed<I, S>(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    manifest: &Path,
    verb: &str,
    rest: I,
    stated: &[(&str, &OsStr)],
    locked: bool,
) -> Result<(Command, String), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let tackroom = bkcl_fenced(dispatch)?;

    // The crate is where its manifest stands, and the pin is read from there
    // upward. The repository is the walk's bound and nothing else here: which
    // channel answers is a question about the crate, never about where the
    // process happens to be standing.
    let crate_dir = match manifest.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir.to_path_buf(),
        _ => PathBuf::from("."),
    };

    let channel = bkcl_pin(repository, &crate_dir)?;
    zbkcl_channel_answers(dispatch, &tackroom, &channel)?;

    let spelled: Vec<std::ffi::OsString> = rest
        .into_iter()
        .map(|arg| arg.as_ref().to_os_string())
        .collect();

    let severed = spelled
        .iter()
        .position(|arg| arg == BKCL_SEPARATOR)
        .unwrap_or(spelled.len());

    // WHICH PROGRAM ANSWERS THIS VERB, decided from the kibble table and never
    // from what the station holds. A kibbled verb is spawned from its own
    // residence with the channel stated in the environment; every other verb is
    // cargo's own, spawned as cargo with the channel stated as its leading
    // argument. The two roads differ in HOW the one channel is said and in
    // nothing else.
    let (mut command, mut spelling) = match bkcl_kibbled(repository, dispatch, verb)? {
        Some(residence) => {
            let mut command = Command::new(&residence);
            command.arg(verb);
            command.env(ZBKCL_TOOLCHAIN_VAR, &channel);
            let spelling = format!(
                "{} {}",
                residence.display(),
                verb, //
            );
            (command, spelling)
        }
        None => {
            let pinned = format!(
                "+{}",
                channel, //
            );
            let mut command = Command::new("cargo");
            command.arg(&pinned).arg(verb);
            let spelling = format!(
                "cargo {} {}",
                pinned,
                verb, //
            );
            (command, spelling)
        }
    };

    let mut say = |command: &mut Command, arg: &OsStr| {
        spelling.push(' ');
        spelling.push_str(&arg.to_string_lossy());
        command.arg(arg);
    };

    for arg in &spelled[..severed] {
        say(&mut command, arg);
    }

    say(&mut command, OsStr::new(BKCL_MANIFEST_FLAG));
    say(&mut command, manifest.as_os_str());

    // THE ONE CONDITIONAL IN THE COMPOSITION, and it is a parameter of this
    // interior function alone — never of a public face, which is the whole
    // reason the lift cannot leak. A caller chooses between named entry points,
    // each of which states in its own name and prose which side of this branch
    // it stands on; no caller passes a boolean, so no caller can pass the wrong
    // one by threading a variable it did not read.
    if locked {
        say(&mut command, OsStr::new(BKCL_LOCKED_FLAG));
    }

    for arg in &spelled[severed..] {
        say(&mut command, arg);
    }

    drop(say);

    zbkcl_homed(&mut command, &tackroom);

    // THE KENNEL'S OWN SEAT POSITION, STATED FOR EVERY INVOCATION OVER A TREE
    // THAT HOLDS THE KENNEL. The kennel's build script refuses to compile
    // without it, so any crate LINKING the kennel demands it too — and a caller
    // that had to know this would be composing the reading itself, which is
    // exactly the per-door authoring this chokepoint exists to end.
    //
    // STATED FOR EVERY CRATE, NEVER FOR THE KENNEL'S COLLAR ALONE. One crate
    // reads it, and a composition that decided which crate that was would carry
    // a second copy of a fact the build script already owns, while a variable
    // nothing reads costs the child nothing.
    //
    // AN ABSENT ELECTION REMOVES IT RATHER THAN LEAVING IT INHERITED, and that
    // arm is the load-bearing one. Every door that spawns this composition was
    // itself launched by a door that may hold the variable — the kennelman's own
    // suite is exactly that — so a tree holding no kennel would otherwise be
    // handed a position measured over a repository it has never heard of, and
    // stamp an artifact with it.
    match zbkcl_seat(repository)? {
        Some(position) => {
            command.env(BKCL_SEAT_VAR, &position);
        }
        None => {
            command.env_remove(BKCL_SEAT_VAR);
        }
    }

    // THE CALLER'S STATED ENVIRONMENT, applied here because the child's
    // environment is already this composition's to hold — the fence's homes are
    // stated a few lines above, and a caller that reached the child by
    // another road would be outside the chokepoint entirely. What a caller may
    // state is environment and never arguments: the flags are the discipline's,
    // and nothing here lets a caller past them.
    //
    // The muzzle is the standing consumer. The linter reads its lint list from a
    // directory named by an environment variable and by no flag at all, and the
    // muzzle STATES that directory rather than letting the linter search for
    // one, so that one lint list governs crates sharing no root.
    for (name, value) in stated {
        command.env(name, value);
    }

    // NO STDIN, AT EITHER FACE. A cargo that stopped to read from a terminal
    // would hang a door with nothing on screen to say why, and a recalled run
    // has no terminal to stop at.
    command.stdin(Stdio::null());

    Ok((command, spelling))
}

/// The kennel's own seat position where this repository carries the kennel, and
/// `None` where it does not.
///
/// THE WALK IS OVER THE ELECTION, on the precedent every door compiling these
/// sources already sets. A binary's own stamp was weighed and declined: it is
/// the position that process was STRUCK at, which is the seat's only where the
/// caller proved the binary current, and a composition that read it would answer
/// differently than every other door for the same crate.
fn zbkcl_seat(repository: &Path) -> Result<Option<String>, String> {
    let seat = repository.join(BKCL_KENNEL_ELECTION);

    if !seat.is_file() {
        return Ok(None);
    }

    let election = bkcf_guard::bkcf_election(&seat)?;
    bkcf_guard::bkcf_seat_position(repository, &election).map(Some)
}

/// Whether the version a station answered stands AT OR ABOVE a floor, read as
/// dotted numbers.
///
/// PUBLISHED SO IT CAN BE HURDLED DIRECTLY. Every station this kennel runs on
/// holds a git far above the floor, so the below-the-floor arm is unreachable
/// through any door here — and an arm nothing exercises is an arm nothing is
/// keeping honest.
///
/// TOLERANT OF A STATION'S OWN DECORATION, deliberately. Git is packaged by
/// whoever packaged the station, and the version it reports carries their marks:
/// `2.43.0.windows.1`, `2.39.5 (Apple Git-154)`. A comparison that refused
/// those would be refusing the packaging rather than the version, so a segment
/// is read for the number it starts with and a segment carrying no number ends
/// the reading.
///
/// A floor with fewer segments than the answer is satisfied by the segments it
/// states — 2.43.0 clears a floor of 1.8.5 at the first segment, and 1.8.5
/// clears a floor of 1.8 at the second.
pub fn bkcl_at_or_above(found: &str, floor: &str) -> bool {
    let read = |segment: &str| -> Option<u64> {
        let digits: String = segment.chars().take_while(char::is_ascii_digit).collect();
        digits.parse::<u64>().ok()
    };

    let mut answered = found.split('.');

    for wanted in floor.split('.') {
        let Some(wanted) = read(wanted) else {
            return true;
        };

        let held = answered.next().and_then(read).unwrap_or(0);

        if held > wanted {
            return true;
        }

        if held < wanted {
            return false;
        }
    }

    true
}

/// The version `git --version` reports.
///
/// CITED FROM A CAPTURED LINE, as above. On this station, 260905:
///
/// ```text
/// git version 2.43.0
/// ```
///
/// The third word, which is where a station's own decoration rides when it
/// rides at all — the comparison above is what tolerates it.
fn zbkcl_git_said(said: &str) -> Option<String> {
    said.lines()
        .next()?
        .split_whitespace()
        .nth(2)
        .map(str::to_string)
}

/// Ask git what version it is, and judge it against the floor.
fn zbkcl_git_read() -> Result<(), String> {
    let asked = Command::new(BKCL_GIT_PROGRAM)
        .arg("--version")
        .stdin(Stdio::null())
        .output();

    let out =
        match asked {
            Ok(out) if out.status.success() => out,
            Ok(out) => {
                return Err(format!(
                    "'{}' is on this station and would not say what version it is: {}. Every door \
                 reads the repository it stands in through it, and the kennel expects {} or \
                 above. Converge it through the station's own packaging",
                    BKCL_GIT_PROGRAM,
                    String::from_utf8_lossy(&out.stderr).trim(),
                    BKCL_GIT_FLOOR, //
                ));
            }
            Err(err) => {
                return Err(format!(
                "no '{}' answered on this station: {}. Every door reads the repository it stands \
                 in through it, and the kennel provisions nothing it spawns; it expects {} or \
                 above. Converge it through the station's own packaging",
                BKCL_GIT_PROGRAM, err, BKCL_GIT_FLOOR, //
            ));
            }
        };

    let said = String::from_utf8_lossy(&out.stdout);

    let Some(found) = zbkcl_git_said(&said) else {
        return Err(format!(
            "'{}' answered something this kennel cannot read a version out of: {}. It expects {} \
             or above and will not guess what it is holding. Converge it through the station's \
             own packaging",
            BKCL_GIT_PROGRAM,
            said.trim(),
            BKCL_GIT_FLOOR, //
        ));
    };

    if bkcl_at_or_above(&found, BKCL_GIT_FLOOR) {
        return Ok(());
    }

    Err(format!(
        "'{}' on this station answers {}, and the kennel expects {} or above — the floor is what \
         its own invocations need, and below it a door would fail on a flag rather than on the \
         tree it was reading. Converge it through the station's own packaging",
        BKCL_GIT_PROGRAM,
        found,
        BKCL_GIT_FLOOR, //
    ))
}

/// THE VERDICT IS TAKEN ONCE PER PROCESS, at git's first check.
///
/// A door invocation spawns git four or five times, and asking it its version
/// before every spawn would price a fact that cannot change inside one run at
/// the rate of the spawns. Cached here rather than at each caller so the
/// once-ness is a property of the check instead of something every calling site
/// has to remember.
///
/// A HURDLE THAT PLANTS A MISMATCH THEREFORE SPAWNS THE DOOR, which is what the
/// launch seam's hurdles do: the plant rides the child's own environment, and a
/// child process holds no verdict from before it existed.
///
/// NO SUCH CACHE STANDS FOR A KIBBLE, and the difference is what the two checks
/// ARE. Git's is a spawn — a process started and its answer read — and is worth
/// taking once. A kibble's is a path test against a residence the process
/// already knows how to compose, which costs a `stat` and cannot be worth
/// remembering. Caching it would also make a converge invisible to the run that
/// performed it, which is exactly the case a hurdle drives.
static ZBKCL_GIT_VERDICT: std::sync::OnceLock<Result<(), String>> = std::sync::OnceLock::new();

/// Read the leading word and the dotted version off rustup's own report —
/// `rustup 1.29.1 (d95a37b6a 2026-08-13)`.
///
/// THE LABEL IS THE LEADING WORD, the way `bkcl_host` reads rustc's `host: `
/// line: a report that does not open with `rustup` is not one this reading
/// will guess a version out of.
fn zbkcl_rustup_said(said: &str) -> Option<String> {
    let mut words = said.lines().next()?.split_whitespace();
    if words.next()? != "rustup" {
        return None;
    }
    let version = words.next()?;
    if version.is_empty() {
        return None;
    }
    Some(version.to_string())
}

/// Judge a rustup version report against a floor.
///
/// PUBLISHED SO A HURDLE CAN DRIVE IT DIRECTLY over posed report strings:
/// posing a rustup on the PATH is not a thing the lure affords, so the
/// comparison is proven here rather than through a spawn.
pub fn bkcl_rustup_judged(said: &str, floor: &str) -> Result<(), String> {
    let Some(found) = zbkcl_rustup_said(said) else {
        return Err(format!(
            "rustup answered something this kennel cannot read a version out of: {}. It is the \
             toolchain's trust root and the kennel expects {} or above; it will not guess what \
             it is holding",
            said.trim(),
            floor, //
        ));
    };

    if bkcl_at_or_above(&found, floor) {
        return Ok(());
    }

    Err(format!(
        "rustup on this station answers {}, and the kennel expects {} or above — it is the \
         toolchain's trust root and a version below the floor is not one the kennel trusts to \
         verify what it fetches. Converge it through the station's own packaging",
        found,
        floor, //
    ))
}

/// Ask rustup what version it is, and judge it against the floor, the
/// tackroom's homes stated on the asking.
fn zbkcl_rustup_read(tackroom: &Path) -> Result<(), String> {
    let mut command = Command::new("rustup");
    command.arg("--version").stdin(Stdio::null());
    zbkcl_homed(&mut command, tackroom);
    let asked = command.output();

    let out = match asked {
        Ok(out) if out.status.success() => out,
        Ok(out) => {
            return Err(format!(
                "'rustup' is on this station and would not say what version it is: {}. It fetches \
                 and verifies the compiler this kennel's whole discipline stands on, and the \
                 kennel expects {} or above",
                String::from_utf8_lossy(&out.stderr).trim(),
                BKCL_RUSTUP_FLOOR, //
            ));
        }
        Err(err) => {
            return Err(format!(
                "no 'rustup' answered on this station: {}. It is the toolchain's trust root and \
                 the kennel provisions nothing it spawns; it expects {} or above",
                err,
                BKCL_RUSTUP_FLOOR, //
            ));
        }
    };

    let said = String::from_utf8_lossy(&out.stdout);
    bkcl_rustup_judged(&said, BKCL_RUSTUP_FLOOR)
}

/// THE VERDICT IS TAKEN ONCE PER PROCESS, on `ZBKCL_GIT_VERDICT`'s own
/// precedent: the fence runs at every invocation, and asking rustup its
/// version before each one would price a fact that cannot change inside one
/// run at the rate of the spawns.
static ZBKCL_RUSTUP_VERDICT: std::sync::OnceLock<Result<(), String>> = std::sync::OnceLock::new();

/// Prove the station holds a rustup the kennel trusts as the toolchain's trust
/// root, at or above the floor.
///
/// ASKED FROM THE FENCE, so every door meets it once: `bkcl_fenced` is the
/// one place every composition already stands on to prove the tackroom
/// redirect, and rustup is what verified what the redirect points the
/// toolchain at.
pub fn bkcl_rustup_held(tackroom: &Path) -> Result<(), String> {
    ZBKCL_RUSTUP_VERDICT
        .get_or_init(|| zbkcl_rustup_read(tackroom))
        .clone()
}

/// The residence a kibbled verb is spawned from, or `None` where the verb is
/// cargo's own.
///
/// THE PATH TEST IS THE WHOLE CHECK, and it happens here rather than at a door
/// so that no road to a kibbled verb can skip it. A composition that reached a
/// declared-but-unplaced residence would spawn a file that is not there and
/// hand its caller an operating system's error where a sentence naming *heel*
/// belongs.
///
/// A VERB OUTSIDE THE TABLE ANSWERS `None` WITHOUT TOUCHING THE FILESYSTEM,
/// which is what keeps every ordinary cargo invocation free of this reading.
pub fn bkcl_kibbled(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    verb: &str,
) -> Result<Option<PathBuf>, String> {
    let Some((_, named)) = ZBKCL_KIBBLED.iter().find(|(served, _)| *served == verb) else {
        return Ok(None);
    };

    zbkcl_placed(repository, dispatch, named).map(Some)
}

/// The residence a NAMED kibble stands at, proven placed.
///
/// EXTRACTED SO TWO ROADS CANNOT COME TO DISAGREE. The verb table above reaches
/// a kibble by the cargo subcommand it serves; the uv arm reaches one by name,
/// having no cargo verb to be found under. Both owe the identical three
/// readings — the declaration conforms, the residence composes, the file stands
/// — and both owe the identical refusals, which are the kibble's own sentences
/// rather than either caller's. A second copy of this would be a second place
/// for the placement check to be forgotten.
fn zbkcl_placed(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    named: &str,
) -> Result<PathBuf, String> {
    let resolved = crate::bkcq_kibble::bkcq_resolve(repository, dispatch.bkcr_weaned(), named)?;

    if !resolved.findings.is_empty() {
        let mut said = format!(
            "the kibble '{}' carries {} finding(s), and no door spawns a program declared by a \
             kibble that does not conform:",
            named,
            resolved.findings.len(), //
        );
        for finding in &resolved.findings {
            said.push_str("\n  ");
            said.push_str(finding);
        }
        return Err(said);
    }

    let feedroom = crate::bkcq_kibble::bkcq_feedroom(dispatch)?;
    let residence = resolved.kibble.bkcq_residence(&feedroom);

    if !residence.is_file() {
        return Err(resolved
            .kibble
            .bkcq_absent(&feedroom, dispatch.bkcr_tabtargets()));
    }

    Ok(residence)
}

/// Prove the station holds a git the kennel's own invocations can be spelled to.
///
/// Asked at the DOOR LAW, which is where every door's first git spawn stands:
/// the law reads the repository through git before anything else happens, so a
/// station without one meets this sentence rather than a failure to read a tree.
pub fn bkcl_git_held() -> Result<(), String> {
    ZBKCL_GIT_VERDICT.get_or_init(zbkcl_git_read).clone()
}

/// Resolve a path to its real location, following symlinks.
fn zbkcl_resolved(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

// eof
