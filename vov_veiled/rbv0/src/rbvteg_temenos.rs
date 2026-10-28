// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTEG — the temenos's push-refusal self-proof (RBSHH "Guard the clone"): a
// temenos stood up from a local bare repository refuses a plain push at its
// hook and a `--no-verify` push to either world's harbinger address at its
// redirect, under a global config that relocates hooks and maps both
// addresses onto the bare — so without the clone's own guards every push
// below would land. Nothing reaches a network: every git call runs with a
// scratch global, no system config and the file protocol alone, set per
// command and never on the process, the harness running cases as threads.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::rbvceg_temenos::{rbvceg_stand_up, RBVCEG_TEMENOS_HOOK_REFUSAL, RBVCEG_TEMENOS_REFUSED_DESTINATION};
use crate::rbvcew_world::{RBVCEW_AGORA, RBVCEW_PALAESTRA};

/// The walk branch the temenos cuts and every push below names.
const ZRBVTEG_WALK_BRANCH: &str = "coldwalk";

/// git -C <dir> <args> under the scratch global, no system config, and the
/// file protocol alone.
fn zrbvteg_git(dir: &Path, args: &[&str], global: &Path) -> Output {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", global)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_ALLOW_PROTOCOL", "file")
        .output()
        .expect("test harness git invocation must spawn")
}

/// The same, asserting success, returning stdout.
fn zrbvteg_git_ok(dir: &Path, args: &[&str], global: &Path) -> String {
    let out = zrbvteg_git(dir, args, global);
    assert!(
        out.status.success(),
        "git {:?} in {} failed: {}",
        args,
        dir.display(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn zrbvteg_stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// A fresh scratch root under the system temp directory, unique to this run.
fn zrbvteg_scratch_root() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after the epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("rbvteg_temenos_{}_{}", std::process::id(), nanos));
    std::fs::create_dir_all(&root).expect("scratch root must be creatable");
    root
}

/// A TEMENOS PUSHES NOTHING, EVEN PAST A RELOCATING GLOBAL. The scratch
/// global relocates hooks to an empty directory and maps both worlds'
/// harbinger addresses onto the bare; the plain push still dies at the
/// clone's own hook, each `--no-verify` push dies at the refused destination,
/// and the bare's refs stand unchanged. The control lifts the hooks pin and
/// the plain push then lands, so the refusal is the pin's and not a hooks
/// path the station never reads.
#[test]
fn rbvteg_temenos_refuses_plain_and_no_verify_pushes_past_a_relocating_global() {
    let root = zrbvteg_scratch_root();
    let bare = root.join("public.git");
    let seed = root.join("seed");
    let bogus_hooks = root.join("bogus_hooks");
    let parent = root.join("parent");
    let top = root.join("top");
    for dir in [&seed, &bogus_hooks, &parent, &top] {
        std::fs::create_dir_all(dir).expect("scratch directory must be creatable");
    }
    let bare_str = bare.to_str().expect("scratch path is UTF-8").to_string();

    let global = root.join("global.gitconfig");
    std::fs::write(
        &global,
        format!(
            "[user]\n\tname = rbvteg\n\temail = rbvteg@example.invalid\n[core]\n\thooksPath = {}\n[url \"{}\"]\n\tinsteadOf = {}\n\tinsteadOf = {}\n",
            bogus_hooks.display(),
            bare_str,
            RBVCEW_AGORA.harbinger_url,
            RBVCEW_PALAESTRA.harbinger_url
        ),
    )
    .expect("scratch global must be writable");

    // The bare, seeded with a README on main, its HEAD pointed at main.
    zrbvteg_git_ok(&root, &["init", "-q", "--bare", &bare_str], &global);
    zrbvteg_git_ok(&seed, &["init", "-q"], &global);
    std::fs::write(seed.join("README.md"), "rbvteg seed\n").expect("seed README must be writable");
    zrbvteg_git_ok(&seed, &["add", "README.md"], &global);
    zrbvteg_git_ok(&seed, &["commit", "-q", "-m", "rbvteg seed"], &global);
    zrbvteg_git_ok(&seed, &["push", "-q", &bare_str, "HEAD:refs/heads/main"], &global);
    zrbvteg_git_ok(&bare, &["symbolic-ref", "HEAD", "refs/heads/main"], &global);

    let temenos = rbvceg_stand_up(&parent, &top, &bare_str, &top, &RBVCEW_AGORA);
    let clone = temenos.clone_dir.clone();

    let refs_before = zrbvteg_git_ok(&bare, &["for-each-ref"], &global);
    assert!(
        zrbvteg_git_ok(&clone, &["remote"], &global).trim().is_empty(),
        "the stood-up clone names a remote"
    );

    // A re-added remote: the plain push dies at the clone's own hook.
    zrbvteg_git_ok(&clone, &["remote", "add", "probe", &bare_str], &global);
    let plain = zrbvteg_git(&clone, &["push", "probe", ZRBVTEG_WALK_BRANCH], &global);
    assert!(!plain.status.success(), "a plain push to a re-added remote landed");
    assert!(
        zrbvteg_stderr(&plain).contains(RBVCEG_TEMENOS_HOOK_REFUSAL),
        "the plain push did not refuse with the hook's text: {}",
        zrbvteg_stderr(&plain)
    );

    // An explicit-URL push past the hook: each world's address dies at the
    // refused destination, never at a transport the scratch forbids.
    for url in [RBVCEW_AGORA.harbinger_url, RBVCEW_PALAESTRA.harbinger_url] {
        let skipped = zrbvteg_git(&clone, &["push", "--no-verify", url, ZRBVTEG_WALK_BRANCH], &global);
        let err = zrbvteg_stderr(&skipped);
        assert!(!skipped.status.success(), "a --no-verify push to {} landed", url);
        assert!(
            err.contains(RBVCEG_TEMENOS_REFUSED_DESTINATION),
            "the --no-verify push to {} did not name the refused destination: {}",
            url,
            err
        );
        assert!(!err.contains("transport"), "the push to {} met a transport, not the redirect: {}", url, err);
    }

    let refs_after = zrbvteg_git_ok(&bare, &["for-each-ref"], &global);
    assert_eq!(refs_before, refs_after, "the bare's refs moved under refused pushes");

    // Control: lift the pin, and the relocating global lets the plain push land.
    zrbvteg_git_ok(&clone, &["config", "--local", "--unset", "core.hooksPath"], &global);
    let landed = zrbvteg_git(&clone, &["push", "probe", ZRBVTEG_WALK_BRANCH], &global);
    assert!(
        landed.status.success(),
        "with the pin lifted the plain push still refused: {}",
        zrbvteg_stderr(&landed)
    );
    let walk_ref = format!("refs/heads/{}", ZRBVTEG_WALK_BRANCH);
    assert!(
        !zrbvteg_git_ok(&bare, &["for-each-ref", &walk_ref], &global).trim().is_empty(),
        "the control push left no walk branch in the bare"
    );

    std::fs::remove_dir_all(&root).expect("scratch root must be removable");
}
