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

//! The program writers' own hurdles.
//!
//! THEY STAND UNDER THE DISPATCH'S TEMP ROOT. Absent `BURD_TEMP_DIR` the hurdle
//! REFUSES and reaches for no ambient temp directory, so these are always driven
//! through the substrate.
//!
//! A written program is RUN, not merely inspected: the bit is what the writers
//! promise, and the exec is what the bit is for.

use super::bkprs_scrivener::{bkprs_copy, bkprs_program_copy, bkprs_program_write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The dispatch's temp root. ABSENT IS A REFUSAL, never a fallback.
const ZBKPTS_TEMP_ROOT_VAR: &str = "BURD_TEMP_DIR";

/// A program that says one word and exits.
const ZBKPTS_SAYS: &str = "#!/bin/sh\necho composed\n";

/// A composed directory under the temp root, removed when the hurdle drops it.
struct zbkpts_Yard {
    root: PathBuf,
}

impl zbkpts_Yard {
    /// Compose an empty yard named for the hurdle that asks for it.
    fn zbkpts_compose(name: &str) -> zbkpts_Yard {
        let declared = std::env::var(ZBKPTS_TEMP_ROOT_VAR).unwrap_or_else(|_| {
            panic!(
                "{} is unset. The writers' hurdles stand under the dispatch's temp root and reach \
                 for no ambient temp directory; drive the suite through its own door",
                ZBKPTS_TEMP_ROOT_VAR
            )
        });

        let root = Path::new(&declared).join(format!("bkpts-{}", name));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root)
            .unwrap_or_else(|err| panic!("could not compose a yard at {}: {}", root.display(), err));

        zbkpts_Yard { root }
    }

    /// A path inside the yard.
    fn zbkpts_at(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }
}

impl Drop for zbkpts_Yard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// Whether a file stands armed.
fn zbkpts_armed(path: &Path) -> bool {
    std::fs::metadata(path)
        .unwrap_or_else(|err| panic!("could not read {}: {}", path.display(), err))
        .permissions()
        .mode()
        & 0o111
        != 0
}

/// Run a program and answer what it printed, refusing the hurdle if it fails.
fn zbkpts_run(program: &Path) -> String {
    let out = Command::new(program)
        .output()
        .unwrap_or_else(|err| panic!("could not run {}: {}", program.display(), err));
    assert!(
        out.status.success(),
        "{} exited {}",
        program.display(),
        out.status
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A written program stands armed, carries its body, and runs — in a directory
/// the writer made.
#[test]
fn zbkpts_a_written_program_runs() {
    let yard = zbkpts_Yard::zbkpts_compose("written-runs");
    let program = yard.zbkpts_at("deep/er/says.sh");

    bkprs_program_write(&program, ZBKPTS_SAYS).expect("the writer writes");

    assert!(zbkpts_armed(&program));
    assert_eq!(std::fs::read_to_string(&program).unwrap(), ZBKPTS_SAYS);
    assert_eq!(zbkpts_run(&program), "composed\n");
}

/// A program written over one already standing replaces it whole.
#[test]
fn zbkpts_a_program_rewritten_is_replaced() {
    let yard = zbkpts_Yard::zbkpts_compose("rewritten");
    let program = yard.zbkpts_at("says.sh");

    bkprs_program_write(&program, "#!/bin/sh\necho first and longer\n").expect("the first write");
    bkprs_program_write(&program, ZBKPTS_SAYS).expect("the second write");

    assert_eq!(zbkpts_run(&program), "composed\n");
}

/// A copied program is armed whatever its source's mode.
#[test]
fn zbkpts_a_copied_program_is_armed() {
    let yard = zbkpts_Yard::zbkpts_compose("program-copy");
    let source = yard.zbkpts_at("source.sh");
    std::fs::write(&source, ZBKPTS_SAYS).unwrap();
    assert!(!zbkpts_armed(&source), "the source must stand unarmed for this to prove anything");
    let copy = yard.zbkpts_at("bin/says.sh");

    bkprs_program_copy(&source, &copy).expect("the copier copies");

    assert_eq!(zbkpts_run(&copy), "composed\n");
}

/// A plain copy carries its source's arming in both directions, and an unarmed
/// copy over an armed file leaves no bit behind.
#[test]
fn zbkpts_a_copy_carries_whether_it_is_a_program() {
    let yard = zbkpts_Yard::zbkpts_compose("carry");
    let armed = yard.zbkpts_at("armed.sh");
    bkprs_program_write(&armed, ZBKPTS_SAYS).unwrap();
    let plain = yard.zbkpts_at("plain.txt");
    std::fs::write(&plain, "plain\n").unwrap();

    let armed_copy = yard.zbkpts_at("out/armed.sh");
    bkprs_copy(&armed, &armed_copy).expect("an armed source copies");
    assert_eq!(zbkpts_run(&armed_copy), "composed\n");

    let plain_copy = yard.zbkpts_at("out/plain.txt");
    bkprs_copy(&plain, &plain_copy).expect("a plain source copies");
    assert!(!zbkpts_armed(&plain_copy));
    assert_eq!(std::fs::read_to_string(&plain_copy).unwrap(), "plain\n");

    bkprs_copy(&plain, &armed_copy).expect("a plain source copies over a program");
    assert!(
        !zbkpts_armed(&armed_copy),
        "a plain copy over a program must not keep the program's bit"
    );
}

/// A source that does not stand refuses, naming it.
#[test]
fn zbkpts_an_absent_source_refuses_naming_it() {
    let yard = zbkpts_Yard::zbkpts_compose("absent");
    let absent = yard.zbkpts_at("nowhere.sh");
    let named = absent.display().to_string();

    let carried = bkprs_copy(&absent, &yard.zbkpts_at("to.sh")).expect_err("nothing to copy");
    assert!(carried.contains(&named), "the refusal reads: {}", carried);

    let program = bkprs_program_copy(&absent, &yard.zbkpts_at("to.sh")).expect_err("nothing to copy");
    assert!(program.contains("to.sh"), "the refusal reads: {}", program);
}

/// A copy onto its own source refuses and leaves the source standing.
#[test]
fn zbkpts_a_copy_onto_its_source_refuses() {
    let yard = zbkpts_Yard::zbkpts_compose("onto-itself");
    let program = yard.zbkpts_at("says.sh");
    bkprs_program_write(&program, ZBKPTS_SAYS).unwrap();

    bkprs_copy(&program, &program).expect_err("a carry onto its source refuses");
    bkprs_program_copy(&program, &yard.zbkpts_at("./says.sh")).expect_err("a copy onto its source refuses");

    assert_eq!(zbkpts_run(&program), "composed\n");
}

/// Threads writing programs and running them at once are never refused.
///
/// THIS IS THE RACE ITSELF, driven: every thread spawns while its siblings
/// write, which is the shape that meets `ETXTBSY` when a thread of the process
/// holds a program open for writing.
#[test]
fn zbkpts_programs_written_beside_spawning_threads_all_run() {
    let yard = zbkpts_Yard::zbkpts_compose("beside-spawns");
    let root = yard.root.clone();

    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let root = root.clone();
            std::thread::spawn(move || {
                for round in 0..25 {
                    let program = root.join(format!("w{}-r{}.sh", worker, round));
                    bkprs_program_write(&program, ZBKPTS_SAYS).expect("the writer writes");
                    assert_eq!(zbkpts_run(&program), "composed\n");
                }
            })
        })
        .collect();

    for worker in workers {
        worker.join().expect("every worker runs every program it wrote");
    }
}

// eof
