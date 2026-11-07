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

//! Bale: pack every aged entry of the two swept folders into one archive, read
//! it back, and only then remove what it holds.
//!
//! THE LAST DAY IS ALWAYS GIVEN, never defaulted, and none later than two days
//! before today is taken, so a bale never reaches what a run may still be
//! writing.
//!
//! A UNIT IS ONE TOP-LEVEL ENTRY of either swept folder, judged by age alone and
//! never by name: a file or a link by its own modification time, a folder by the
//! newest regular file anywhere beneath it, or by its own time where it holds
//! none. A unit goes when its day is on or before the last day given. The
//! dispatch output folder is stepped over wherever it stands, and so is any
//! entry holding it.
//!
//! ONE ARCHIVE PER RUN, named for the machine's tag, the day of the oldest file
//! it holds and the last day given. Its members stand under the two folders'
//! own names. An archive already standing is never overwritten, nor is a
//! temporary of its name, and a run finding nothing old enough writes nothing.
//!
//! THE READ-BACK IS THE WHOLE SAFETY CHECK. The archive is written under a
//! temporary name in the archive folder and flushed to disk; the check reads
//! every member back and compares its kind, its name, its bytes or its link
//! target and its modification time with the original. Only a passing check is
//! followed by the rename — a hard link that refuses where the final name
//! stands, so nothing is ever overwritten — and only then are the originals
//! removed: exactly the members the check passed, deepest first, a link removed
//! as itself and a folder only where it stands empty. A failing check removes
//! nothing and leaves the temporary standing for inspection.
//!
//! THE CHECK IS A PARAMETER, so a hurdle can hand a failing one and watch
//! nothing go; the binary always hands `bkhlrb_read_back`.
//!
//! THE ARCHIVE STREAMS. Members are written with GNU headers in complete header
//! mode, so modification times survive and a name past a hundred characters
//! rides GNU's long-name extension; links are archived as links and never
//! followed. Neither the archive nor any member is held whole in memory.

use crate::bkhlrd_day::{
    bkhlrd_Day,
    bkhlrd_seconds, //
};
use crate::bkhlrg_ground::{
    bkhlrg_Ground,
    bkhlrg_within,
    BKHLRG_LOG_KEY,
    BKHLRG_TEMP_KEY, //
};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{
    File,
    Metadata,
    OpenOptions, //
};
use std::io::{
    BufReader,
    BufWriter,
    Read, //
};
use std::path::{
    Path,
    PathBuf, //
};
use tar::{
    Archive,
    Builder,
    EntryType,
    HeaderMode, //
};

/// The ending every archive name wears.
pub const BKHLRB_SUFFIX: &str = ".tar.gz";

/// How many days before today the latest admissible last day stands.
pub const BKHLRB_DAYS_BACK: i64 = 2;

/// What precedes a temporary archive's name: a hidden name, which the listing
/// never offers.
const ZBKHLRB_TEMPORARY_MARK: &str = ".";

/// The size of each slice the read-back compares.
const ZBKHLRB_SLICE: usize = 64 * 1024;

/// What one member is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum bkhlrb_Kind {
    /// A regular file.
    File,
    /// A symbolic link, archived as itself.
    Link,
    /// A folder.
    Folder,
}

/// One member: its name inside the archive and the original it was taken from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkhlrb_Member {
    /// The member's name inside the archive.
    pub name: PathBuf,
    /// The original on disk.
    pub origin: PathBuf,
    /// What the original is.
    pub kind: bkhlrb_Kind,
}

/// The read-back check: handed the written archive and every member in the
/// order written, it passes or says why not.
pub type bkhlrb_Check<'a> = dyn Fn(&Path, &[bkhlrb_Member]) -> Result<(), String> + 'a;

/// What a bale that wrote an archive did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkhlrb_Baled {
    /// The archive, at its final name.
    pub archive: PathBuf,
    /// The day of the oldest file it holds.
    pub first: bkhlrd_Day,
    /// The last day given.
    pub last: bkhlrd_Day,
    /// How many top-level entries it took.
    pub units: usize,
    /// How many members it holds.
    pub members: usize,
    /// Originals the check passed that could not be removed, each with why.
    pub standing: Vec<(PathBuf, String)>,
}

/// What a bale did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkhlrb_Outcome {
    /// Nothing was old enough, and nothing was written.
    bkhlrb_Nothing(bkhlrd_Day),
    /// An archive was written, checked and renamed, and its originals removed.
    bkhlrb_Wrote(bkhlrb_Baled),
}

/// One top-level entry and everything beneath it.
struct zbkhlrb_Unit {
    members: Vec<bkhlrb_Member>,
    newest: i64,
    oldest: i64,
}

/// The archive name for a machine tag and two days.
pub fn bkhlrb_name(tincture: &str, first: bkhlrd_Day, last: bkhlrd_Day) -> String {
    format!(
        "{}-{}-{}{}",
        tincture,
        first.bkhlrd_digits(),
        last.bkhlrd_digits(),
        BKHLRB_SUFFIX, //
    )
}

/// The latest last day a bale run on `today` admits.
pub fn bkhlrb_latest(today: bkhlrd_Day) -> bkhlrd_Day {
    today.bkhlrd_back(BKHLRB_DAYS_BACK)
}

/// Bale every unit dated on or before `last` into one archive in the archive
/// folder, checking it with `check` before anything is removed; a `last` later
/// than two days before `today` is refused.
pub fn bkhlrb_bale(
    ground: &bkhlrg_Ground,
    last: bkhlrd_Day,
    today: bkhlrd_Day,
    check: &bkhlrb_Check<'_>,
) -> Result<bkhlrb_Outcome, String> {
    let hayloft = ground.bkhlrg_hayloft()?;
    let latest = bkhlrb_latest(today);

    if last > latest {
        return Err(format!(
            "the last day {} is later than {}: a bale takes nothing newer than {} days before \
             today, which is {} in UTC",
            last.bkhlrd_digits(),
            latest.bkhlrd_digits(),
            BKHLRB_DAYS_BACK,
            today.bkhlrd_digits(), //
        ));
    }

    let log_name = zbkhlrb_folder_name(BKHLRG_LOG_KEY, &ground.log)?;
    let temp_name = zbkhlrb_folder_name(BKHLRG_TEMP_KEY, &ground.temp)?;
    if log_name == temp_name {
        return Err(format!(
            "{} and {} end in the same name '{}', so their members would collide in one archive",
            BKHLRG_LOG_KEY,
            BKHLRG_TEMP_KEY,
            log_name, //
        ));
    }

    let mut taken: Vec<zbkhlrb_Unit> = Vec::new();
    for (folder, name) in [(&ground.log, &log_name), (&ground.temp, &temp_name)] {
        for unit in zbkhlrb_units(folder, name, &ground.output)? {
            if bkhlrd_Day::bkhlrd_from_time(zbkhlrb_moment(unit.newest)) <= last {
                taken.push(unit);
            }
        }
    }

    if taken.is_empty() {
        return Ok(bkhlrb_Outcome::bkhlrb_Nothing(last));
    }

    let oldest = taken
        .iter()
        .map(|unit| unit.oldest)
        .min()
        .unwrap_or_default();
    let first = bkhlrd_Day::bkhlrd_from_time(zbkhlrb_moment(oldest));

    let name = bkhlrb_name(&ground.tincture, first, last);
    let archive = hayloft.folder.join(&name);
    let temporary = hayloft.folder.join(format!(
        "{}{}",
        ZBKHLRB_TEMPORARY_MARK,
        name, //
    ));

    for standing in [&archive, &temporary] {
        if std::fs::symlink_metadata(standing).is_ok() {
            return Err(format!(
                "{} already stands, and an archive is never overwritten: nothing was written or \
                 removed",
                standing.display(), //
            ));
        }
    }

    let members: Vec<bkhlrb_Member> = taken
        .iter()
        .flat_map(|unit| unit.members.iter().cloned())
        .collect();

    std::fs::create_dir_all(&hayloft.folder).map_err(|err| {
        format!(
            "could not create the archive folder {}: {}",
            hayloft.folder.display(),
            err, //
        )
    })?;

    zbkhlrb_write(&temporary, &members).map_err(|why| {
        format!(
            "{} - nothing was removed, and the partial archive stands at {}",
            why,
            temporary.display(), //
        )
    })?;

    check(&temporary, &members).map_err(|why| {
        format!(
            "the written archive did not read back equal to its originals: {} - nothing was \
             removed, and the archive stands for inspection at {}",
            why,
            temporary.display(), //
        )
    })?;

    std::fs::hard_link(&temporary, &archive).map_err(|err| {
        format!(
            "could not give the checked archive its name {}: {} - nothing was removed, and the \
             archive stands at {}",
            archive.display(),
            err,
            temporary.display(), //
        )
    })?;
    std::fs::remove_file(&temporary).map_err(|err| {
        format!(
            "the archive stands at {}, but its temporary {} could not be removed: {} - nothing \
             else was removed",
            archive.display(),
            temporary.display(),
            err, //
        )
    })?;
    File::open(&hayloft.folder)
        .and_then(|folder| folder.sync_all())
        .map_err(|err| {
            format!(
                "the archive stands at {}, but its folder could not be flushed to disk: {} - \
                 nothing else was removed",
                archive.display(),
                err, //
            )
        })?;

    let standing = zbkhlrb_remove(&members);

    Ok(bkhlrb_Outcome::bkhlrb_Wrote(bkhlrb_Baled {
        archive,
        first,
        last,
        units: taken.len(),
        members: members.len(),
        standing,
    }))
}

/// The read-back check the binary hands: every member, in the order written,
/// compared with its original by kind, name, modification time and bytes or
/// link target, and nothing in the archive beyond them.
pub fn bkhlrb_read_back(archive: &Path, members: &[bkhlrb_Member]) -> Result<(), String> {
    let opened = File::open(archive).map_err(|err| {
        format!(
            "could not open {}: {}",
            archive.display(),
            err, //
        )
    })?;
    let mut reader = Archive::new(GzDecoder::new(BufReader::new(opened)));
    let entries = reader.entries().map_err(|err| err.to_string())?;
    let mut expected = members.iter();

    for entry in entries {
        let mut entry = entry.map_err(|err| err.to_string())?;
        let member = expected.next().ok_or_else(|| {
            "the archive holds more members than were written".to_string() //
        })?;

        let name = entry.path().map_err(|err| err.to_string())?.into_owned();
        if name != member.name {
            return Err(format!(
                "member {} stands where {} was written",
                name.display(),
                member.name.display(), //
            ));
        }

        let original = std::fs::symlink_metadata(&member.origin).map_err(|err| {
            format!(
                "{} no longer reads: {}",
                member.origin.display(),
                err, //
            )
        })?;
        let entry_type = entry.header().entry_type();

        match member.kind {
            bkhlrb_Kind::Folder => {
                if entry_type != EntryType::Directory || !original.is_dir() {
                    return Err(format!(
                        "{} is not a folder on both sides",
                        member.name.display(), //
                    ));
                }
            }
            bkhlrb_Kind::Link => {
                let target = entry.link_name().map_err(|err| err.to_string())?;
                let held = std::fs::read_link(&member.origin).map_err(|err| err.to_string())?;
                if entry_type != EntryType::Symlink || target.as_deref() != Some(held.as_path()) {
                    return Err(format!(
                        "{} is not the same link on both sides",
                        member.name.display(), //
                    ));
                }
            }
            bkhlrb_Kind::File => {
                if entry_type != EntryType::Regular || !original.is_file() {
                    return Err(format!(
                        "{} is not a regular file on both sides",
                        member.name.display(), //
                    ));
                }
                let mtime = entry.header().mtime().map_err(|err| err.to_string())?;
                if mtime as i64 != zbkhlrb_mtime(&original)? {
                    return Err(format!(
                        "{} carries another modification time",
                        member.name.display(), //
                    ));
                }
                let mut origin = File::open(&member.origin).map_err(|err| err.to_string())?;
                zbkhlrb_same_bytes(&mut entry, &mut origin).map_err(|why| {
                    format!(
                        "{}: {}",
                        member.name.display(),
                        why, //
                    )
                })?;
            }
        }
    }

    if let Some(missing) = expected.next() {
        return Err(format!(
            "the archive ends before member {}",
            missing.name.display(), //
        ));
    }

    Ok(())
}

/// The last component of a swept folder, under which its members stand.
fn zbkhlrb_folder_name(key: &str, folder: &Path) -> Result<String, String> {
    if !folder.is_dir() {
        return Err(format!(
            "{} names {}, where no folder stands",
            key,
            folder.display(), //
        ));
    }
    folder
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .ok_or_else(|| {
            format!(
                "{} resolves to {}, which ends in no readable name",
                key,
                folder.display(), //
            )
        })
}

/// Every unit of one swept folder, in name order, the output folder and any
/// entry holding it stepped over.
fn zbkhlrb_units(folder: &Path, name: &str, output: &Path) -> Result<Vec<zbkhlrb_Unit>, String> {
    let mut units = Vec::new();
    for origin in zbkhlrb_sorted(folder)? {
        if bkhlrg_within(output, &origin) {
            continue;
        }
        let file_name = origin.file_name().unwrap_or_default();
        let mut unit = zbkhlrb_Unit {
            members: Vec::new(),
            newest: i64::MIN,
            oldest: i64::MAX,
        };
        zbkhlrb_walk(&origin, &Path::new(name).join(file_name), &mut unit)?;
        if unit.newest == i64::MIN {
            let own = zbkhlrb_mtime(&zbkhlrb_lstat(&origin)?)?;
            unit.newest = own;
            unit.oldest = own;
        }
        units.push(unit);
    }
    Ok(units)
}

/// Take one entry and, for a folder, everything beneath it in name order, the
/// folder ahead of its contents.
fn zbkhlrb_walk(origin: &Path, name: &Path, unit: &mut zbkhlrb_Unit) -> Result<(), String> {
    let stat = zbkhlrb_lstat(origin)?;
    let kind = if stat.file_type().is_symlink() {
        bkhlrb_Kind::Link
    } else if stat.is_dir() {
        bkhlrb_Kind::Folder
    } else if stat.is_file() {
        bkhlrb_Kind::File
    } else {
        return Err(format!(
            "{} is neither a file, a folder nor a link, and is not archived: nothing was written \
             or removed",
            origin.display(), //
        ));
    };

    unit.members.push(bkhlrb_Member {
        name: name.to_path_buf(),
        origin: origin.to_path_buf(),
        kind,
    });

    match kind {
        bkhlrb_Kind::File => {
            let mtime = zbkhlrb_mtime(&stat)?;
            unit.newest = unit.newest.max(mtime);
            unit.oldest = unit.oldest.min(mtime);
        }
        bkhlrb_Kind::Folder => {
            for child in zbkhlrb_sorted(origin)? {
                let child_name = name.join(child.file_name().unwrap_or_default());
                zbkhlrb_walk(&child, &child_name, unit)?;
            }
        }
        bkhlrb_Kind::Link => {}
    }

    Ok(())
}

/// A folder's entries, sorted by name.
fn zbkhlrb_sorted(folder: &Path) -> Result<Vec<PathBuf>, String> {
    let mut held: Vec<PathBuf> = std::fs::read_dir(folder)
        .map_err(|err| {
            format!(
                "could not read {}: {}",
                folder.display(),
                err, //
            )
        })?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|err| {
            format!(
                "could not read {}: {}",
                folder.display(),
                err, //
            )
        })?;
    held.sort();
    Ok(held)
}

/// An entry's own metadata, never its link's target's.
fn zbkhlrb_lstat(origin: &Path) -> Result<Metadata, String> {
    std::fs::symlink_metadata(origin).map_err(|err| {
        format!(
            "could not read {}: {}",
            origin.display(),
            err, //
        )
    })
}

/// An entry's modification time in whole seconds from the epoch.
fn zbkhlrb_mtime(stat: &Metadata) -> Result<i64, String> {
    stat.modified()
        .map(bkhlrd_seconds)
        .map_err(|err| err.to_string())
}

/// The moment so many seconds from the epoch.
fn zbkhlrb_moment(seconds: i64) -> std::time::SystemTime {
    let span = std::time::Duration::from_secs(seconds.unsigned_abs());
    if seconds >= 0 {
        std::time::UNIX_EPOCH + span
    } else {
        std::time::UNIX_EPOCH - span
    }
}

/// Write every member into a new archive at `temporary`, flushed to disk.
fn zbkhlrb_write(temporary: &Path, members: &[bkhlrb_Member]) -> Result<(), String> {
    let created = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temporary)
        .map_err(|err| {
            format!(
                "could not create {}: {}",
                temporary.display(),
                err, //
            )
        })?;

    let mut builder = Builder::new(GzEncoder::new(
        BufWriter::new(created),
        Compression::default(),
    ));
    builder.mode(HeaderMode::Complete);
    builder.follow_symlinks(false);

    for member in members {
        let appended = match member.kind {
            bkhlrb_Kind::Folder => builder.append_dir(&member.name, &member.origin),
            bkhlrb_Kind::File | bkhlrb_Kind::Link => {
                builder.append_path_with_name(&member.origin, &member.name)
            }
        };
        appended.map_err(|err| {
            format!(
                "could not archive {}: {}",
                member.origin.display(),
                err, //
            )
        })?;
    }

    let flushed = builder
        .into_inner()
        .and_then(|encoder| encoder.finish())
        .and_then(|buffered| buffered.into_inner().map_err(|err| err.into_error()))
        .and_then(|file| file.sync_all());

    flushed.map_err(|err| {
        format!(
            "could not finish {}: {}",
            temporary.display(),
            err, //
        )
    })
}

/// Read two streams slice by slice, refusing at the first difference.
fn zbkhlrb_same_bytes(archived: &mut dyn Read, original: &mut dyn Read) -> Result<(), String> {
    let mut held = vec![0u8; ZBKHLRB_SLICE];
    let mut kept = vec![0u8; ZBKHLRB_SLICE];
    let mut offset: u64 = 0;
    loop {
        let from_archive = zbkhlrb_fill(archived, &mut held).map_err(|err| err.to_string())?;
        let from_original = zbkhlrb_fill(original, &mut kept).map_err(|err| err.to_string())?;
        if from_archive != from_original || held[..from_archive] != kept[..from_original] {
            return Err(format!(
                "the bytes differ at or after offset {}",
                offset, //
            ));
        }
        if from_archive == 0 {
            return Ok(());
        }
        offset += from_archive as u64;
    }
}

/// Read until the slice is full or the stream ends, answering how much was read.
fn zbkhlrb_fill(stream: &mut dyn Read, slice: &mut [u8]) -> std::io::Result<usize> {
    let mut filled = 0;
    while filled < slice.len() {
        match stream.read(&mut slice[filled..]) {
            Ok(0) => break,
            Ok(read) => filled += read,
            Err(err) if err.kind() == std::io::ErrorKind::Interrupted => {}
            Err(err) => return Err(err),
        }
    }
    Ok(filled)
}

/// Remove exactly the checked members, deepest first: a file or a link as
/// itself, a folder only where it stands empty. What cannot go is answered with
/// why, and nothing else is touched.
fn zbkhlrb_remove(members: &[bkhlrb_Member]) -> Vec<(PathBuf, String)> {
    let mut standing = Vec::new();
    for member in members.iter().rev() {
        let removed = match member.kind {
            bkhlrb_Kind::File | bkhlrb_Kind::Link => std::fs::remove_file(&member.origin),
            bkhlrb_Kind::Folder => std::fs::remove_dir(&member.origin),
        };
        if let Err(err) = removed {
            standing.push((member.origin.clone(), err.to_string()));
        }
    }
    standing
}

// eof
