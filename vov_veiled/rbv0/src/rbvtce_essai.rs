// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTCE — practice essai's stand-in gate (RBSHE step 1, the palaestra arm):
// two scratch bare repositories on the file protocol stand as real public main
// and as the stand-in, the gate's read is taken over them, and its verdict
// passes exactly main and the regulus branch at public main's commit and
// refuses every other shape with the procedure's recreate and seed steps. The
// steps name the practice bundle's repositories whatever the reads were taken
// over, so the expected lines are the procedure's own, spelled here as the
// independent oracle. The scratch stands under the crate's own ignored build
// directory; every git call runs with a scratch global, no system config and
// the file protocol alone, set per command and never on the process.

use std::path::{
    Path,
    PathBuf, //
};
use std::process::Command;

use crate::rbvcce_essai::{
    zrbvcce_standin_reads,
    zrbvcce_standin_verdict, //
};
use crate::rbvcew_world::{
    rbvcew_palaestra_seed_source,
    RBVCEW_PALAESTRA, //
};
use crate::rbvcex_expede::RBVCEX_REGULUS_BRANCH;

/// The procedure's console sentence names the stand-in by this owner/name.
const ZRBVTCE_STANDIN_SLUG: &str = "bhyslop/recipebottle-grandstand-practice";

/// The six seed lines, in order, exactly as the procedure renders them.
const ZRBVTCE_SEED_LINES: [&str; 6] = [
    "export FRESH_CLONE=«an absolute path for a new directory»",
    "git clone git@github.com:scaleinv/recipebottle.git \"${FRESH_CLONE}\"",
    "cd \"${FRESH_CLONE}\"",
    "git remote remove origin",
    "git push git@github.com:bhyslop/recipebottle-grandstand-practice.git main:main",
    "git push git@github.com:bhyslop/recipebottle-grandstand-practice.git main:POSTULANT_LOCAL",
];

/// The closing re-read the steps end on.
const ZRBVTCE_REREAD: &str =
    "git ls-remote git@github.com:bhyslop/recipebottle-grandstand-practice.git";

struct zrbvtce_Scratch {
    root: PathBuf,
    global: PathBuf,
    public: PathBuf,
    standin: PathBuf,
    seed: PathBuf,
    first: String,
    second: String,
}

fn zrbvtce_git(dir: &Path, args: &[&str], global: &Path) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", global)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_ALLOW_PROTOCOL", "file")
        .output()
        .expect("test harness git invocation must spawn");
    assert!(
        out.status.success(),
        "git {:?} in {} failed: {}",
        args,
        dir.display(),
        String::from_utf8_lossy(&out.stderr), //
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn zrbvtce_str(p: &Path) -> String {
    p.to_str().expect("scratch path is UTF-8").to_string()
}

/// A scratch root holding a seed working tree with two commits, a bare
/// standing as real public main at the first, and an empty bare stand-in.
fn zrbvtce_scratch(tag: &str) -> zrbvtce_Scratch {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after the epoch")
        .as_nanos();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!(
            "rbvtce_essai_{}_{}_{}",
            tag,
            std::process::id(),
            nanos, //
        ));
    let seed = root.join("seed");
    std::fs::create_dir_all(&seed).expect("scratch seed must be creatable");
    let global = root.join("global.gitconfig");
    std::fs::write(
        &global,
        "[user]\n\tname = rbvtce\n\temail = rbvtce@example.invalid\n[init]\n\tdefaultBranch = main\n",
    )
    .expect("scratch global must be writable");
    let public = root.join("public.git");
    let standin = root.join("standin.git");
    zrbvtce_git(
        &root,
        &["init", "-q", "--bare", &zrbvtce_str(&public)],
        &global,
    );
    zrbvtce_git(
        &root,
        &["init", "-q", "--bare", &zrbvtce_str(&standin)],
        &global,
    );
    zrbvtce_git(&seed, &["init", "-q", "-b", "main"], &global);
    std::fs::write(seed.join("README.md"), "first\n").expect("seed file must be writable");
    zrbvtce_git(&seed, &["add", "README.md"], &global);
    zrbvtce_git(&seed, &["commit", "-q", "-m", "first"], &global);
    let first = zrbvtce_git(&seed, &["rev-parse", "HEAD"], &global);
    std::fs::write(seed.join("README.md"), "second\n").expect("seed file must be writable");
    zrbvtce_git(&seed, &["commit", "-q", "-am", "second"], &global);
    let second = zrbvtce_git(&seed, &["rev-parse", "HEAD"], &global);
    zrbvtce_git(
        &seed,
        &[
            "push",
            "-q",
            &zrbvtce_str(&public),
            &format!("{}:refs/heads/main", first),
        ],
        &global,
    );
    zrbvtce_Scratch {
        root,
        global,
        public,
        standin,
        seed,
        first,
        second,
    }
}

/// Push `commit` onto the stand-in under `refname`.
fn zrbvtce_plant(s: &zrbvtce_Scratch, commit: &str, refname: &str) {
    zrbvtce_git(
        &s.seed,
        &[
            "push",
            "-q",
            &zrbvtce_str(&s.standin),
            &format!("{}:{}", commit, refname),
        ],
        &s.global,
    );
}

/// The gate's read over the scratch pair, judged with the practice bundle's
/// repositories naming the text.
fn zrbvtce_judge(s: &zrbvtce_Scratch, standin: &Path) -> Result<String, String> {
    let (standin_read, seed_read) =
        zrbvtce_reads(&zrbvtce_str(standin), &zrbvtce_str(&s.public), &s.root);
    zrbvtce_verdict(&standin_read, &seed_read)
}

fn zrbvtce_reads(
    standin: &str,
    seed: &str,
    cwd: &Path,
) -> (
    Result<Vec<(String, String)>, String>,
    Result<Vec<(String, String)>, String>,
) {
    zrbvcce_standin_reads(standin, seed, cwd)
}

fn zrbvtce_verdict(
    standin: &Result<Vec<(String, String)>, String>,
    seed: &Result<Vec<(String, String)>, String>,
) -> Result<String, String> {
    zrbvcce_standin_verdict(
        standin,
        seed,
        RBVCEW_PALAESTRA.base_url,
        rbvcew_palaestra_seed_source(),
    )
}

/// A refusal carries the console sentence naming the stand-in, the six seed
/// lines in order, and the closing re-read after them.
fn zrbvtce_assert_steps(case: &str, refusal: &str) {
    assert!(
        refusal.contains(&format!(
            "At the GitHub console, by hand, delete the stand-in {} and create it again, private and empty, with no README, ignore file or license.",
            ZRBVTCE_STANDIN_SLUG, //
        )),
        "{}: the refusal lacks the console sentence:\n{}",
        case,
        refusal, //
    );
    let lines: Vec<&str> = refusal.lines().map(|l| l.trim()).collect();
    let mut at = 0;
    for expected in ZRBVTCE_SEED_LINES.iter().chain([ZRBVTCE_REREAD].iter()) {
        let found = lines[at..].iter().position(|l| l == expected);
        assert!(
            found.is_some(),
            "{}: the refusal lacks '{}' in order:\n{}",
            case,
            expected,
            refusal, //
        );
        at += found.unwrap() + 1;
    }
}

/// Exactly main and the regulus branch at public main's commit passes, and
/// the verdict is that commit.
#[test]
fn rbvtce_standin_matching_public_main_passes() {
    let s = zrbvtce_scratch("pass");
    zrbvtce_plant(&s, &s.first, "refs/heads/main");
    zrbvtce_plant(
        &s,
        &s.first,
        &format!(
            "refs/heads/{}",
            RBVCEX_REGULUS_BRANCH, //
        ),
    );
    assert_eq!(
        zrbvtce_judge(&s, &s.standin),
        Ok(s.first.clone()),
        "a matching stand-in must pass", //
    );
    std::fs::remove_dir_all(&s.root).expect("scratch root must be removable");
}

/// An empty stand-in, a missing branch, a branch at another commit, a third
/// ref and an unreachable stand-in each refuse with the steps.
#[test]
fn rbvtce_standin_drift_refuses_with_the_steps() {
    let regulus_ref = format!(
        "refs/heads/{}",
        RBVCEX_REGULUS_BRANCH, //
    );

    let empty = zrbvtce_scratch("empty");
    zrbvtce_assert_steps(
        "empty",
        &zrbvtce_judge(&empty, &empty.standin).expect_err("an empty stand-in must refuse"),
    );
    std::fs::remove_dir_all(&empty.root).expect("scratch root must be removable");

    let missing = zrbvtce_scratch("missing");
    zrbvtce_plant(&missing, &missing.first, "refs/heads/main");
    zrbvtce_assert_steps(
        "missing branch",
        &zrbvtce_judge(&missing, &missing.standin).expect_err("a missing branch must refuse"),
    );
    std::fs::remove_dir_all(&missing.root).expect("scratch root must be removable");

    let moved = zrbvtce_scratch("moved");
    zrbvtce_plant(&moved, &moved.first, "refs/heads/main");
    zrbvtce_plant(&moved, &moved.second, &regulus_ref);
    zrbvtce_assert_steps(
        "branch at another commit",
        &zrbvtce_judge(&moved, &moved.standin).expect_err("a branch at another commit must refuse"),
    );
    std::fs::remove_dir_all(&moved.root).expect("scratch root must be removable");

    let tagged = zrbvtce_scratch("tagged");
    zrbvtce_plant(&tagged, &tagged.first, "refs/heads/main");
    zrbvtce_plant(&tagged, &tagged.first, &regulus_ref);
    zrbvtce_plant(&tagged, &tagged.first, "refs/tags/v1");
    zrbvtce_assert_steps(
        "third ref",
        &zrbvtce_judge(&tagged, &tagged.standin).expect_err("a third ref must refuse"),
    );
    std::fs::remove_dir_all(&tagged.root).expect("scratch root must be removable");

    let gone = zrbvtce_scratch("gone");
    let absent = gone.root.join("absent.git");
    let refusal = zrbvtce_judge(&gone, &absent).expect_err("an unreachable stand-in must refuse");
    zrbvtce_assert_steps("unreachable", &refusal);
    assert!(
        refusal.contains("cannot read the stand-in"),
        "the unreachable refusal names no failed read:\n{}",
        refusal, //
    );
    std::fs::remove_dir_all(&gone.root).expect("scratch root must be removable");
}

/// Real public main unreadable, or carrying no main, refuses naming the read
/// rather than demanding a stand-in reset no one could judge.
#[test]
fn rbvtce_public_main_unreadable_or_absent_refuses() {
    let s = zrbvtce_scratch("public");
    zrbvtce_plant(&s, &s.first, "refs/heads/main");
    let regulus_ref = format!(
        "refs/heads/{}",
        RBVCEX_REGULUS_BRANCH, //
    );
    zrbvtce_plant(&s, &s.first, &regulus_ref);

    let absent = s.root.join("absent.git");
    let (standin_read, seed_read) =
        zrbvtce_reads(&zrbvtce_str(&s.standin), &zrbvtce_str(&absent), &s.root);
    let unreadable = zrbvtce_verdict(&standin_read, &seed_read)
        .expect_err("an unreadable public main must refuse");
    assert!(
        unreadable.contains("cannot read real public main"),
        "the refusal does not name the failed read:\n{}",
        unreadable, //
    );

    let (standin_read, _) =
        zrbvtce_reads(&zrbvtce_str(&s.standin), &zrbvtce_str(&s.public), &s.root);
    let no_main: Result<Vec<(String, String)>, String> =
        Ok(vec![(s.first.clone(), "refs/heads/other".to_string())]);
    let absent_main = zrbvtce_verdict(&standin_read, &no_main)
        .expect_err("a public repository without main must refuse");
    assert!(
        absent_main.contains("real public main is absent"),
        "the refusal does not name the absent main:\n{}",
        absent_main, //
    );
    std::fs::remove_dir_all(&s.root).expect("scratch root must be removable");
}
