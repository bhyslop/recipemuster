// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCES — the practice seat: a station chore beside the four ceremony
// commands, never a ceremony act (RBSHC "The election"). It stands up the seat
// a practice round runs from, beneath a practice home the operator names: a
// clone of the maintainer tree at its checked-out branch, two levels below the
// home, so the regulus and the walk clone land inside that home without any
// change to where the ceremony puts things; the practice base remote beside
// its origin, push side neutered; a station file naming the station's own
// tool store; and the kennel built.
//
// The palaestra alone: the seat's only remote that reaches GitHub is the
// read-only practice base, so an agora election refuses. It contacts no
// remote and creates no repository at GitHub — the console acts and every push
// stay the operator's. It never seposes or deletes: an existing seat refuses.

use std::path::{
    Component,
    Path,
    PathBuf, //
};
use std::process::ExitCode;

use crate::rbvccd_docimasy::{
    zrbvccd_find_tt,
    RBVCCD_TT_SUBDIR, //
};
use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvcer_run;
use crate::rbvcew_world;
use crate::rbvgeb_breviary;

/// The seat directory beneath the practice home, and the clone beneath it —
/// the clone standing two levels below the home.
pub const RBVCES_SEAT_SUBDIR: &str = "seat";
pub const RBVCES_CLONE_SUBDIR: &str = "rb";

/// The kennel's whistle colophon, which the seat's own tabtarget carries.
const RBVCES_COL_WHISTLE: &str = "bkw-w.";

/// The two dispatch values the seat's CLI function hands the binary, which
/// inherits neither: the maintainer station's file, and its tool store as the
/// dispatch resolved it.
const RBVCES_ENV_STATION_FILE: &str = "BURD_STATION_FILE";
const RBVCES_ENV_TACKROOM: &str = "BURD_TACKROOM";

/// The bash child that sources the maintainer station and prints the two
/// values the seat's station copies.
const RBVCES_STATION_READ: &str =
    r#"source "$1" && printf '%s\n%s\n' "${BURS_USER}" "${BURS_TINCTURE}""#;

/// What the seat's station file carries over from the maintainer station.
pub struct rbvces_Station {
    pub user: String,
    pub tincture: String,
    /// The station's tool store, absolute and standing.
    pub tackroom: PathBuf,
}

/// The seat as stood up.
pub struct rbvces_Seat {
    pub clone_dir: PathBuf,
    pub station_file: PathBuf,
    pub secrets_dir: PathBuf,
}

/// Elect the practice home from the arguments that follow the verb: the world,
/// which must be the palaestra, then the home, absolute. Pure — the
/// self-proofs reach it. A missing world, an unknown word, the agora, a
/// missing home, a relative home and a trailing extra argument each refuse.
pub fn rbvces_elect(command: &str, args: &[String]) -> Result<PathBuf, String> {
    let conductor = rbvgeb_breviary::RBVGEB_HIEROPHANT_BASE;
    let palaestra = rbvgeb_breviary::RBVGEB_PALAESTRA_BASE;
    let (word, home) = match args {
        [] => {
            return Err(format!(
                "{} {}: no world named — this door conducts in the {} alone",
                conductor,
                command,
                palaestra, //
            ));
        }
        [word] => (word.as_str(), None),
        [word, home] => (word.as_str(), Some(home.as_str())),
        [_, _, extra, ..] => {
            return Err(format!(
                "{} {}: unexpected extra argument '{}' after the practice home",
                conductor,
                command,
                extra, //
            ));
        }
    };
    match rbvcew_world::rbvcew_from_word(word) {
        Some(rbvcew_world::rbvcew_World::Palaestra) => {}
        Some(rbvcew_world::rbvcew_World::Agora) => {
            return Err(format!(
                "{} {}: the {} stands no practice seat — this door conducts in the {} alone",
                conductor,
                command,
                word,
                palaestra, //
            ));
        }
        None => {
            return Err(format!(
                "{} {}: unknown world '{}' — this door conducts in the {} alone",
                conductor,
                command,
                word,
                palaestra, //
            ));
        }
    }
    let home = home.ok_or_else(|| {
        format!(
            "{} {}: no practice home named — name the directory, absolute, the seat stands beneath",
            conductor,
            command, //
        )
    })?;
    let home = PathBuf::from(home);
    if !home.is_absolute() {
        return Err(format!(
            "{} {}: the practice home '{}' is not absolute",
            conductor,
            command,
            home.display(), //
        ));
    }
    Ok(home)
}

/// Read the maintainer station's user and tincture through a bash child that
/// sources its file, beside the tool store the dispatch resolved, which must
/// stand.
pub fn rbvces_station_read(
    station_file: &Path,
    tackroom: &Path,
    cwd: &Path,
) -> Result<rbvces_Station, String> {
    let file = rbvcep_repo::rbvcep_as_str(station_file);
    let got = rbvcer_run::rbvcer_capture("bash", &["-c", RBVCES_STATION_READ, "bash", &file], cwd);
    if got.code != 0 {
        return Err(format!(
            "cannot read the maintainer station {} (bash exited {}): {}",
            file,
            got.code,
            got.stderr.trim(), //
        ));
    }
    let mut lines = got.stdout.lines();
    let user = lines.next().unwrap_or("").trim().to_string();
    let tincture = lines.next().unwrap_or("").trim().to_string();
    if user.is_empty() || tincture.is_empty() {
        return Err(format!(
            "the maintainer station {} names no user or no tincture",
            file, //
        ));
    }
    let tackroom = std::fs::canonicalize(tackroom).map_err(|e| {
        format!(
            "the station's tool store {} does not stand: {}",
            tackroom.display(),
            e, //
        )
    })?;
    Ok(rbvces_Station {
        user,
        tincture,
        tackroom,
    })
}

/// Stand up the seat beneath `home`, short of the kennel: the clone of `top`
/// at its checked-out branch, origin left as the cloned path; the bundle's
/// base remote added with its push side neutered to the bundle's sentinel; the
/// station file beside the clone where the clone's configuration regime looks
/// for it, its stores named relative to the clone; and the empty secrets
/// directory beside it. Refuses an existing seat before touching anything, and
/// refuses rather than dies on every deficit, so the self-proofs reach it.
pub fn rbvces_stand(
    top: &Path,
    home: &Path,
    bundle: &rbvcew_world::rbvcew_Bundle,
    station: &rbvces_Station,
) -> Result<rbvces_Seat, String> {
    if !home.is_absolute() {
        return Err(format!(
            "the practice home {} is not absolute",
            home.display(), //
        ));
    }
    let seat_dir = home.join(RBVCES_SEAT_SUBDIR);
    if std::fs::symlink_metadata(&seat_dir).is_ok() {
        return Err(format!(
            "a seat already stands at {} — this door never seposes or deletes; name a fresh practice home, or dispose of that seat by hand",
            seat_dir.display(), //
        ));
    }

    let head = rbvcer_run::rbvcer_capture("git", &["rev-parse", "--abbrev-ref", "HEAD"], top);
    let branch = head.stdout.trim().to_string();
    if head.code != 0 || branch.is_empty() || branch == "HEAD" {
        return Err(format!(
            "cannot read the maintainer tree's checked-out branch at {}: {}",
            top.display(),
            head.stderr.trim(), //
        ));
    }

    zrbvces_mkdir_all(home)?;
    std::fs::create_dir(&seat_dir).map_err(|e| {
        format!(
            "cannot create the seat {}: {}",
            seat_dir.display(),
            e, //
        )
    })?;

    let clone_dir = seat_dir.join(RBVCES_CLONE_SUBDIR);
    let top_str = rbvcep_repo::rbvcep_as_str(top);
    let clone_str = rbvcep_repo::rbvcep_as_str(&clone_dir);
    zrbvces_git(
        &["clone", "--branch", &branch, &top_str, &clone_str],
        home,
        "clone the maintainer tree",
    )?;
    zrbvces_git(
        &["-C", &clone_str, "remote", "add", bundle.base_remote, bundle.base_url],
        home,
        "add the practice base remote",
    )?;
    zrbvces_git(
        &[
            "-C",
            &clone_str,
            "remote",
            "set-url",
            "--push",
            bundle.base_remote,
            bundle.base_push_disabled,
        ],
        home,
        "neuter the practice base remote's push side",
    )?;

    let station_dir = seat_dir.join(rbvcep_repo::RBVCEP_STATION_SUBDIR);
    let secrets_dir = station_dir.join(rbvcep_repo::RBVCEP_SECRETS_SUBDIR);
    zrbvces_mkdir_all(&secrets_dir)?;

    // The station regime names its stores relative to the project root and
    // refuses an absolute one: the logs are the seat's own, beside the clone,
    // and the tool store is the station's, reached from the clone.
    let clone_canonical = std::fs::canonicalize(&clone_dir).map_err(|e| {
        format!(
            "cannot resolve the seat clone {}: {}",
            clone_dir.display(),
            e, //
        )
    })?;
    let tackroom = zrbvces_relative(&clone_canonical, &station.tackroom);
    let burs = format!(
        "BURS_USER={}\nBURS_TINCTURE={}\nBURS_LOG_DIR=../{}\nBURS_TACKROOM={}\n",
        station.user,
        station.tincture,
        rbvcep_repo::RBVCEP_LOGS_SUBDIR,
        tackroom.display(), //
    );
    let station_file = station_dir.join(rbvcep_repo::RBVCEP_STATION_FILE);
    std::fs::write(&station_file, burs).map_err(|e| {
        format!(
            "cannot write the seat's station {}: {}",
            station_file.display(),
            e, //
        )
    })?;

    Ok(rbvces_Seat {
        clone_dir,
        station_file,
        secrets_dir,
    })
}

/// Stand up a practice seat beneath `home` and build its kennel. Fatal on any
/// deficit; ExitCode::SUCCESS only when the seat stands with its kennel built.
pub fn rbvces_seat_conduct(home: PathBuf) -> ExitCode {
    let world = rbvcew_world::rbvcew_World::Palaestra;
    rbvcel_log::rbvcel_elect(world);
    let bundle = rbvcew_world::rbvcew_bundle(world);
    rbvcel_log::rbvcel_section("Hierophant practice seat — a station chore beside the ceremony");
    rbvcel_log::rbvcel_line("Clone, practice base remote, station, kennel — no remote contacted.");

    let top = rbvcep_repo::rbvcep_toplevel();
    rbvcel_log::rbvcel_line(&format!(
        "Maintainer tree: {}",
        top.display(), //
    ));
    rbvcel_log::rbvcel_line(&format!(
        "Practice home:   {}",
        home.display(), //
    ));

    let station_file = zrbvces_env(RBVCES_ENV_STATION_FILE);
    let tackroom = zrbvces_env(RBVCES_ENV_TACKROOM);
    let station = rbvces_station_read(Path::new(&station_file), Path::new(&tackroom), &top)
        .unwrap_or_else(|e| {
            crate::rbvcel_fatal!(
                "{}",
                e, //
            )
        });

    rbvcel_log::rbvcel_section("Stand up the seat");
    let seat = rbvces_stand(&top, &home, bundle, &station).unwrap_or_else(|e| {
        crate::rbvcel_fatal!(
            "{}",
            e, //
        )
    });
    rbvcel_log::rbvcel_line(&format!(
        "seat clone: {} (origin {}; {} read-only)",
        seat.clone_dir.display(),
        top.display(),
        bundle.base_remote, //
    ));
    rbvcel_log::rbvcel_line(&format!(
        "station:    {}",
        seat.station_file.display(), //
    ));
    rbvcel_log::rbvcel_line(&format!(
        "secrets:    {}",
        seat.secrets_dir.display(), //
    ));

    rbvcel_log::rbvcel_section("Build the seat's kennel");
    let whistle = zrbvccd_find_tt(
        &seat.clone_dir.join(RBVCCD_TT_SUBDIR),
        RBVCES_COL_WHISTLE,
        None,
    );
    let code = rbvcer_run::rbvcer_stream(&whistle, &[], &seat.clone_dir, &[]);
    if code != 0 {
        crate::rbvcel_fatal!(
            "the seat's whistle failed (exit {}) — the seat stands at {}; repair, then run its whistle by hand",
            code,
            seat.clone_dir.display(), //
        );
    }

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_success(&format!(
        "Practice seat stands at {} with its kennel built.",
        seat.clone_dir.display(), //
    ));
    rbvcel_log::rbvcel_line(
        "Open each round as the procedure's \"Opening a round\" directs, then run the practice doors from the seat.",
    );
    ExitCode::SUCCESS
}

/// git <args> in `cwd`, captured; a non-zero exit refuses naming the act.
fn zrbvces_git(args: &[&str], cwd: &Path, act: &str) -> Result<(), String> {
    let got = rbvcer_run::rbvcer_capture("git", args, cwd);
    if got.code != 0 {
        return Err(format!(
            "failed to {} (git exited {}): {}",
            act,
            got.code,
            got.stderr.trim(), //
        ));
    }
    Ok(())
}

fn zrbvces_mkdir_all(dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| {
        format!(
            "cannot create {}: {}",
            dir.display(),
            e, //
        )
    })
}

/// A dispatch value the CLI function hands in, or fatal naming the door.
fn zrbvces_env(name: &str) -> String {
    match std::env::var(name) {
        Ok(v) if !v.is_empty() => v,
        _ => crate::rbvcel_fatal!(
            "{} is not set — launch the practice seat through its withheld tabtarget, which hands it in",
            name, //
        ),
    }
}

/// The path from `from` to `to`, both absolute and resolved: up from `from`
/// to their shared ancestor, then down to `to`. Pure.
pub(crate) fn zrbvces_relative(from: &Path, to: &Path) -> PathBuf {
    let from: Vec<Component> = from.components().collect();
    let to: Vec<Component> = to.components().collect();
    let shared = from.iter().zip(to.iter()).take_while(|(a, b)| a == b).count();
    let mut rel = PathBuf::new();
    for _ in shared..from.len() {
        rel.push("..");
    }
    for part in &to[shared..] {
        rel.push(part.as_os_str());
    }
    if rel.as_os_str().is_empty() {
        rel.push(".");
    }
    rel
}
