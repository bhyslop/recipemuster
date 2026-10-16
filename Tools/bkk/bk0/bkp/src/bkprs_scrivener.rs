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

//! The program writers.
//!
//! A PROGRAM IS WRITTEN BY A CHILD, NEVER BY THIS PROCESS. Every spawn forks
//! the spawning process, and a fork carries each handle the process holds into a
//! child that keeps it until its own exec; the kernel refuses to exec a file
//! that any process holds open for writing, with `ETXTBSY`. So in a process
//! where one thread writes a file another will run while any thread spawns, the
//! run can be refused — a test runner holding its cases as threads of one
//! process is the plainest such process, and a server answering calls on a
//! thread pool is another. Close-on-exec closes the handle only at the
//! sibling's exec, so no per-thread isolation reaches the window, a private
//! directory included, and a retry around the spawn only narrows it.
//!
//! Here the window never opens. Each writer hands its write to a short-lived
//! shell that alone opens the file and has exited before the writer returns, so
//! no handle to a program ever exists in this process for a fork to carry. The
//! shell is forked by this process and by nobody else, and it removes the file
//! it is about to write before writing it, so it never writes into a file
//! another process may be running and the mode it leaves is its own.
//!
//! THE RAW CALLS ARE BARRED WHEREVER A MUZZLE REACHES, each bar naming the
//! writer here that replaces it. A file nobody will run needs no writer: an
//! ordinary write is safe, because a file without an executable bit cannot be
//! exec'd, and `bkprs_copy` takes that road itself for a source that is not
//! armed.
//!
//! Every writer makes the directories above its target, and a copy onto its own
//! source refuses.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// The shell a writer hands its write to.
const ZBKPRS_SHELL: &str = "/bin/sh";

/// What the writing child runs: the body arrives on its input, and the file is
/// armed before the child exits.
const ZBKPRS_WRITE: &str = "rm -f -- \"$1\" && cat > \"$1\" && chmod 755 \"$1\"";

/// What the program-copying child runs, arming the copy whatever its source's
/// mode.
const ZBKPRS_PROGRAM_COPY: &str = "rm -f -- \"$2\" && cp -- \"$1\" \"$2\" && chmod 755 \"$2\"";

/// What the carrying child runs: the copy takes its source's mode, as a fresh
/// file under the process's own mask.
const ZBKPRS_CARRY: &str = "rm -f -- \"$2\" && cp -- \"$1\" \"$2\"";

/// Write a program at `path` and arm it.
pub fn bkprs_program_write(path: &Path, body: impl AsRef<[u8]>) -> Result<(), String> {
    zbkprs_child(ZBKPRS_WRITE, &[path], Some(body.as_ref()))
}

/// Copy a program from `from` to `to` and arm the copy.
pub fn bkprs_program_copy(from: &Path, to: &Path) -> Result<(), String> {
    zbkprs_apart(from, to)?;
    zbkprs_child(ZBKPRS_PROGRAM_COPY, &[from, to], None)
}

/// Copy any file from `from` to `to`, carrying whether it is a program.
///
/// The replacement for a plain copy wherever the source may be a program. An
/// ARMED SOURCE is copied by a child, so the copy is armed as its source is and
/// no handle to it stands in this process; an UNARMED SOURCE is read and
/// written here, the target removed first so a copy never keeps the bit of a
/// program it replaces.
pub fn bkprs_copy(from: &Path, to: &Path) -> Result<(), String> {
    let standing = std::fs::metadata(from)
        .map_err(|err| format!("could not read {}: {}", from.display(), err))?;
    zbkprs_apart(from, to)?;
    if zbkprs_armed(&standing) {
        return zbkprs_child(ZBKPRS_CARRY, &[from, to], None);
    }

    let body =
        std::fs::read(from).map_err(|err| format!("could not read {}: {}", from.display(), err))?;
    zbkprs_parent(to)?;
    match std::fs::remove_file(to) {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(format!("could not replace {}: {}", to.display(), err)),
    }
    std::fs::write(to, body).map_err(|err| format!("could not write {}: {}", to.display(), err))
}

/// Refuse a copy onto its own source.
///
/// Every copy removes its target before writing it, so a target that IS the
/// source would be gone before it was read. Compared as resolved paths, so a
/// spelling through a symlink or a `.` still answers the same; a target that
/// does not stand yet cannot be its source.
fn zbkprs_apart(from: &Path, to: &Path) -> Result<(), String> {
    match (std::fs::canonicalize(from), std::fs::canonicalize(to)) {
        (Ok(source), Ok(target)) if source == target => Err(format!(
            "{} is its own source, and a copy onto it would remove what it reads",
            to.display()
        )),
        _ => Ok(()),
    }
}

/// Whether a file carries any executable bit.
#[cfg(unix)]
fn zbkprs_armed(standing: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;

    standing.permissions().mode() & 0o111 != 0
}

/// Whether a file carries any executable bit: none can, off unix.
#[cfg(not(unix))]
fn zbkprs_armed(_standing: &std::fs::Metadata) -> bool {
    false
}

/// Make the directories above `target`.
fn zbkprs_parent(target: &Path) -> Result<(), String> {
    match target.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => std::fs::create_dir_all(parent)
            .map_err(|err| format!("could not make {}: {}", parent.display(), err)),
        _ => Ok(()),
    }
}

/// Run one writer's child to its exit; the last operand is the file it writes.
fn zbkprs_child(script: &str, operands: &[&Path], body: Option<&[u8]>) -> Result<(), String> {
    let target = operands[operands.len() - 1];
    zbkprs_parent(target)?;

    let mut child = Command::new(ZBKPRS_SHELL)
        .arg("-c")
        .arg(script)
        .arg(ZBKPRS_SHELL)
        .args(operands)
        .stdin(if body.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("could not start the writer of {}: {}", target.display(), err))?;

    if let Some(bytes) = body {
        let mut input = child
            .stdin
            .take()
            .ok_or_else(|| format!("the writer of {} took no input", target.display()))?;
        if let Err(err) = input.write_all(bytes) {
            drop(input);
            let _ = child.wait();
            return Err(format!("could not hand {} its body: {}", target.display(), err));
        }
    }

    let out = child
        .wait_with_output()
        .map_err(|err| format!("could not wait on the writer of {}: {}", target.display(), err))?;

    if out.status.success() {
        return Ok(());
    }
    Err(format!(
        "the writer of {} failed: {} — {}",
        target.display(),
        out.status,
        String::from_utf8_lossy(&out.stderr).trim()
    ))
}

// eof
