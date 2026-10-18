// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — expede: the cut. Build the delivery regulus by ADDITION into a
// clone of the public repository (RBSHC "The cut, and the single matcher";
// RBSHE step 3). Absorbed into the hierophant by the 260718 re-ruling —
// essai runs this in-process, judging every path through the one matcher
// (rbvcem_perambulation), with the regulus's own sterilize as the sole
// subprocess.
//
// EXPEDE IS A PURE LOCAL CONSTRUCTOR. It clones the real public base, builds
// the regulus atop it, and PUSHES NOTHING. It severs the clone's origin, so
// the finished regulus holds zero remotes: the reveal to the public is
// human-hands-only not by discipline but by structural incapacity — the cut
// wires no remote that could reach the public target, so no bug in it can.
//
// THE REGULUS IS BUILT BY ADDITION, and everything else here follows from
// it. The clone is of the PUBLIC repository, so the object graph the push
// walks began with no private object in it and never receives one. Nothing is
// stripped, because nothing withheld is ever put in. The 2026-07-13 regulus
// was built the other way — the whole repository, then removals — and its TIP
// was spotless while its history went to the remote at 292 MiB. Construction
// is the prevention; the delta sweep below is not the guard, it is the proof
// that the guard held.

use std::path::{Path, PathBuf};

use crate::rbvccd_docimasy::RBVCCD_TT_SUBDIR;
use crate::rbvcel_log;
use crate::rbvcem_perambulation;
use crate::rbvcep_repo;
use crate::rbvcer_run;

// ── The cut's constants ─────────────────────────────────────

/// The base remote. The cut clones this — the REAL public repository — and
/// cuts the regulus one commit atop its live main. It is the ONLY endpoint
/// the cut knows: it is read (cloned) and never pushed to, and the clone's
/// origin is severed after cloning so the name cannot be pushed to even by
/// accident. Its name is loud on purpose (_UPSTREAM, all caps): it can never
/// be confused with origin, main, or a hand-typed default.
pub const RBVCEX_BASE_REMOTE: &str = "ENGROSSMENT_UPSTREAM";

/// The base is READ-ONLY, and that is asserted, not merely intended. A
/// push-capable remote to the public target in the maintainer repo is the one
/// catastrophe the ceremony forbids, so the base remote's PUSH url must be
/// neutered to this sentinel, and the cut refuses until it is. The clone
/// still reads (fetch) the real URL; only the push side is dead.
pub const RBVCEX_BASE_PUSH_DISABLED: &str = "DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY";

/// The base's expected fetch URL, hardcoded and asserted before anything is
/// cloned. The base-inventory sweep below is NON-FATAL by design — it
/// tolerates already-disclosed withheld history because the base is the
/// genuinely public repository. That tolerance is safe ONLY if the base truly
/// IS that repository: a fetch URL fat-fingered at the private maintainer
/// repo would clone the whole private history, the inventory would wave it
/// through as "already disclosed", the delta sweep would see only the clean
/// one commit, and the regulus would green-light — then a push would upload
/// the entire private ancestry (the 292 MiB catastrophe, back through the one
/// unlocked door). The endpoint is a load-bearing fact, not a runtime input.
pub const RBVCEX_BASE_URL: &str = "git@github.com:scaleinv/recipebottle.git";

/// The ephemeral private narthex (RBS0 rbshc_narthex; RELEASE.md "The
/// narthex"): created empty and private by the operator's own hand before
/// a cut, reached only by explicit URL — never a configured remote. Fixed and
/// known, unlike the repository's ephemeral CONTENTS: only its existence is
/// per-cycle. The single home for docimasy and ostend alike — both gate on
/// the same repository, and a duplicated copy is exactly the drift a
/// privacy/freshness gate must never carry (RBSHD "Gate the narthex",
/// RBSHO "Re-assert the ground").
pub const RBVCEX_NARTHEX_URL: &str = "git@github.com:scaleinv/recipebottle-staging.git";

/// The anonymous-read form of the same repository, for the 404 privacy gate.
/// A private GitHub repository 404s to an unauthenticated request; a public
/// or misnamed one does not.
pub const RBVCEX_NARTHEX_HTTPS: &str = "https://github.com/scaleinv/recipebottle-staging";

/// The expected HTTP status of an anonymous read of a private narthex.
const RBVCEX_NARTHEX_PRIVATE_STATUS: &str = "404";

/// The regulus's local branch — and, reused by operator ruling (260715),
/// the public staging branch name too. Deliberately NOT "main": a branch
/// named main would let a default-shaped push land on public main. It is the
/// only branch the finished clone carries; the preview and staging pushes
/// both spell POSTULANT_LOCAL:POSTULANT_LOCAL, and only the far-side
/// promotion spells POSTULANT_LOCAL:main — by hand, exactly once.
pub const RBVCEX_REGULUS_BRANCH: &str = "POSTULANT_LOCAL";

/// The subject of the single commit the regulus carries. One commit, so one
/// subject: it names the act, not the contents.
const RBVCEX_REGULUS_SUBJECT: &str = "Recipe Bottle release candidate";

/// The sterilize script, repo-relative. The cut runs THE REGULUS'S copy of
/// this path, never the maintainer's — the perambulation is what guarantees
/// the regulus has it (RBSHC: the one boundary deliberately not absorbed).
const RBVCEX_STERILIZE_PATH: &str = "Tools/rbk/rblm_sterilize.sh";

/// The consumer CLAUDE.md template, repo-relative. The cut transposes THIS
/// file's committed bytes onto the regulus's root CLAUDE.md between
/// materialization and the commit: the regulus must carry the consumer's
/// context, never the maintainer's veiled-path-laden one.
const RBVCEX_CONSUMER_CLAUDE_PATH: &str = "vov_veiled/CLAUDE.consumer.md";

/// The regulus's root CLAUDE.md — the transposition's target, and a path
/// the perambulation ships so the sweep expects it in the regulus graph.
const RBVCEX_REGULUS_CLAUDE_PATH: &str = "CLAUDE.md";

/// The scratch directory beneath the target dir — the archive tar and the
/// sterilize subprocess's temp root land here, beside the clone and never
/// inside it, so `git add --all` in the clone can never stage them.
const RBVCEX_SCRATCH_SUBDIR: &str = "scratch";

/// The BUK module subtree, relative to a repo root — handed to the sterilize
/// subprocess as its BURD_BUK_DIR (utility modules only; every
/// identity-bearing module the script sources resolves from its own tree).
const RBVCEX_BUK_SUBDIR: &str = "Tools/buk";

// ── The cut ─────────────────────────────────────────────────

/// Cut the delivery regulus by addition into {target_dir}/regulus.
/// Fatal on any deficit; returns the regulus clone path only when every
/// refusal is satisfied: one commit atop the public base on POSTULANT_LOCAL,
/// no withheld path in the delta, zero remotes.
pub fn rbvcex_cut(top: &Path, target_dir: &Path) -> PathBuf {
    if !target_dir.is_absolute() {
        crate::rbvcel_fatal!("target directory must be an absolute path: {}", target_dir.display());
    }
    if target_dir.exists() {
        crate::rbvcel_fatal!("target directory already exists: {}", target_dir.display());
    }

    // Clean-tree gate. Every shipped byte is taken from the COMMITTED record,
    // so an uncommitted edit would be silently absent from the regulus —
    // the tree the operator is looking at would not be the tree that shipped.
    let status = rbvcer_run::rbvcer_capture("git", &["status", "--porcelain"], top);
    if status.code != 0 {
        crate::rbvcel_fatal!("git status failed:\n{}", status.stderr.trim());
    }
    if !status.stdout.trim().is_empty() {
        crate::rbvcel_fatal!(
            "working tree not clean — commit before the cut; the regulus is cut from committed bytes:\n{}",
            status.stdout.trim()
        );
    }

    // The perambulation must be structurally sound and TOTAL before anything
    // is cut from it. An unjudged path is not a warning to be carried
    // forward: it means the project has not ruled on a file it tracks, and a
    // regulus cut in that state would silently ship it or silently drop it.
    // Red until judged, and the cut is where that bites.
    if let Err(e) = rbvcem_perambulation::rbvcem_validate(rbvcem_perambulation::RBVCEM_ROWS) {
        crate::rbvcel_fatal!("{}", e);
    }
    let tracked = zrbvcex_tracked(top);
    let unjudged = rbvcem_perambulation::rbvcem_unjudged(&tracked);
    if !unjudged.is_empty() {
        for path in &unjudged {
            rbvcel_log::rbvcel_line(&format!("unjudged: {}", path));
        }
        crate::rbvcel_fatal!(
            "{} tracked path(s) the perambulation has not judged — rule them ship or withhold in the perambulation table",
            unjudged.len()
        );
    }
    let dead = rbvcem_perambulation::rbvcem_dead_rows(&tracked);
    if !dead.is_empty() {
        for (prefix, disposition) in &dead {
            rbvcel_log::rbvcel_line(&format!("dead row: {}|{}", prefix, disposition));
        }
        crate::rbvcel_fatal!(
            "{} perambulation row(s) judge no tracked path — stale or shadowed",
            dead.len()
        );
    }

    // The base. A regulus is one commit atop the real PUBLIC main, so the
    // base remote is not optional scenery — it is the thing being added to.
    // The cut only ever CLONES it; it is severed from the clone below.
    let fetch = rbvcer_run::rbvcer_capture("git", &["remote", "get-url", RBVCEX_BASE_REMOTE], top);
    if fetch.code != 0 {
        crate::rbvcel_fatal!(
            "{} is not configured — the regulus is built by addition atop the real public repository, so a remote pointing at it is required (git remote add {} {}):\n{}",
            RBVCEX_BASE_REMOTE, RBVCEX_BASE_REMOTE, RBVCEX_BASE_URL, fetch.stderr.trim()
        );
    }
    let fetch_url = fetch.stdout.trim().to_string();
    if fetch_url.is_empty() {
        crate::rbvcel_fatal!("{} URL is empty", RBVCEX_BASE_REMOTE);
    }

    // The base's IDENTITY, asserted before anything is cloned — the safety
    // condition of the non-fatal base inventory (see RBVCEX_BASE_URL).
    if fetch_url != RBVCEX_BASE_URL {
        crate::rbvcel_fatal!(
            "{} points at {}, not the expected public base {} — refusing to cut atop the wrong repository (the base-inventory sweep is non-fatal ONLY because the base is the genuinely public repo)",
            RBVCEX_BASE_REMOTE, fetch_url, RBVCEX_BASE_URL
        );
    }

    // The base is read-only, asserted. A live push url to the public target
    // is refused before a single object is cloned.
    let push = rbvcer_run::rbvcer_capture("git", &["remote", "get-url", "--push", RBVCEX_BASE_REMOTE], top);
    if push.code != 0 {
        crate::rbvcel_fatal!("cannot read {} push url:\n{}", RBVCEX_BASE_REMOTE, push.stderr.trim());
    }
    if push.stdout.trim() != RBVCEX_BASE_PUSH_DISABLED {
        crate::rbvcel_fatal!(
            "{} has a live push url ({}) — the base must be read-only. Neuter it: git remote set-url --push {} {}",
            RBVCEX_BASE_REMOTE, push.stdout.trim(), RBVCEX_BASE_REMOTE, RBVCEX_BASE_PUSH_DISABLED
        );
    }

    let head = rbvcer_run::rbvcer_capture("git", &["rev-parse", "HEAD"], top);
    if head.code != 0 {
        crate::rbvcel_fatal!("git rev-parse HEAD failed:\n{}", head.stderr.trim());
    }
    let head = head.stdout.trim().to_string();

    // The consumer CLAUDE.md template must exist in the committed record
    // before the cut begins — the transposition below reads its bytes, and a
    // missing template would surface only after the whole regulus was built.
    let template_ref = format!("{}:{}", head, RBVCEX_CONSUMER_CLAUDE_PATH);
    let template_probe = rbvcer_run::rbvcer_capture("git", &["cat-file", "-e", &template_ref], top);
    if template_probe.code != 0 {
        crate::rbvcel_fatal!(
            "the consumer CLAUDE.md template is absent from {}: {}\n{}",
            head, RBVCEX_CONSUMER_CLAUDE_PATH, template_probe.stderr.trim()
        );
    }

    let shipped = rbvcem_perambulation::rbvcem_shipped(&tracked);
    if shipped.is_empty() {
        crate::rbvcel_fatal!("the perambulation ships nothing — refusing to cut an empty regulus");
    }

    let clone_dir = target_dir.join(rbvcep_repo::RBVCEP_REGULUS_SUBDIR);
    let clone = rbvcep_repo::rbvcep_as_str(&clone_dir);

    rbvcel_log::rbvcel_section("The cut — build the regulus by addition (RBSHE step 3)");
    rbvcel_log::rbvcel_line(&format!("Source commit:        {}", head));
    rbvcel_log::rbvcel_line(&format!("Public base remote:   {}", RBVCEX_BASE_REMOTE));
    rbvcel_log::rbvcel_line(&format!("Public base URL:      {}", fetch_url));
    rbvcel_log::rbvcel_line(&format!("Regulus clone:      {}", clone_dir.display()));
    rbvcel_log::rbvcel_line(&format!("Regulus branch:     {}", RBVCEX_REGULUS_BRANCH));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("The regulus is built by ADDITION in a clone of the real PUBLIC");
    rbvcel_log::rbvcel_line("repository. No private object enters the object graph, because none");
    rbvcel_log::rbvcel_line("is ever put there. Nothing is stripped.");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line(&format!("Every shipped path ({}) is materialized from the committed bytes,", shipped.len()));
    rbvcel_log::rbvcel_line("judged by the perambulation's one matcher, then lustrated and");
    rbvcel_log::rbvcel_line("regenerated by the clone's own copy of rblm_sterilize.sh. The root");
    rbvcel_log::rbvcel_line("CLAUDE.md is transposed to the consumer template and byte-asserted.");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("The clone receives NO station and NO secrets directory, and it is");
    rbvcel_log::rbvcel_line("SEVERED from its origin: the finished regulus holds zero remotes,");
    rbvcel_log::rbvcel_line("so the cut cannot push it anywhere. The reveal is a human step.");
    rbvcel_log::rbvcel_blank();

    rbvcel_log::rbvcel_step("Cloning the public base");
    if let Err(e) = std::fs::create_dir(target_dir) {
        crate::rbvcel_fatal!("failed to create target directory {}: {}", target_dir.display(), e);
    }
    let scratch_dir = target_dir.join(RBVCEX_SCRATCH_SUBDIR);
    if let Err(e) = std::fs::create_dir(&scratch_dir) {
        crate::rbvcel_fatal!("failed to create scratch directory {}: {}", scratch_dir.display(), e);
    }
    let code = rbvcer_run::rbvcer_stream("git", &["clone", &fetch_url, &clone], top, &[]);
    if code != 0 {
        crate::rbvcel_fatal!("failed to clone the public base (git exited {})", code);
    }

    // The base is surveyed BEFORE anything is added to it, over its whole
    // object graph. Under a real public base this is an INVENTORY, not a
    // gate: the public history may already carry withheld paths a prior era
    // disclosed, and already-disclosed history can be known but not
    // un-disclosed. Surfaced for the operator to acknowledge — loud, never
    // fatal, never silent. What this cut is answerable for is the DELTA it
    // adds, swept fatally below.
    rbvcel_log::rbvcel_step("Surveying the public base (inventory)");
    let base_graph = zrbvcex_graph_paths(&clone_dir, &["--all"], "base");
    let base_inventory = rbvcem_perambulation::rbvcem_sweep(&base_graph);
    if base_inventory.is_empty() {
        rbvcel_log::rbvcel_line("Base object graph clean: no withheld path in the public history");
    } else {
        rbvcel_log::rbvcel_blank();
        rbvcel_log::rbvcel_warn(&format!(
            "BASE INVENTORY — {} withheld path(s) already in the public history.",
            base_inventory.len()
        ));
        rbvcel_log::rbvcel_line("Already disclosed; cannot be un-disclosed by this cut. This is not");
        rbvcel_log::rbvcel_line("a leak of THIS cut — acknowledge and proceed.");
        for path in &base_inventory {
            rbvcel_log::rbvcel_line(&format!("  {}", path));
        }
        rbvcel_log::rbvcel_blank();
    }

    // The public repository may carry no commit at all — an empty base is the
    // legitimate state before the first regulus is ever published. Then the
    // regulus is a ROOT commit, and "one commit atop the base" means one
    // commit, full stop. The base SHA is captured now, as a value, so the
    // delta range and the commit count below survive the branch surgery.
    let base_sha = {
        let got = rbvcer_run::rbvcer_capture("git", &["-C", &clone, "rev-parse", "HEAD"], top);
        if got.code == 0 {
            let sha = got.stdout.trim().to_string();
            rbvcel_log::rbvcel_line(&format!("Public base commit:   {}", sha));
            Some(sha)
        } else {
            rbvcel_log::rbvcel_line("Public base commit:   (none — the base is empty; this regulus is a root commit)");
            None
        }
    };

    // Sever the clone from its origin. From here the clone holds no remote,
    // so nothing it does can reach the public repository. Materialization is
    // from the maintainer repo's own object store (git archive of HEAD),
    // never from the clone's remote, so the sever costs the build nothing.
    // Zero remotes is asserted again at the end, as the finished regulus's
    // standing property.
    rbvcel_log::rbvcel_step("Severing the clone from its origin");
    zrbvcex_git_capture(&clone, &["remote", "remove", "origin"], top, "sever the clone's origin");

    // Open the regulus's own branch, and drop every other. The clone must
    // carry exactly ONE branch, named POSTULANT_LOCAL — so even a forbidden
    // fan-out push (--all) could name nothing but POSTULANT_LOCAL, never
    // main. On an empty base the unborn branch is simply renamed.
    rbvcel_log::rbvcel_step(&format!("Opening the regulus branch {}", RBVCEX_REGULUS_BRANCH));
    zrbvcex_git_capture(&clone, &["checkout", "-b", RBVCEX_REGULUS_BRANCH], top, "open the regulus branch");
    let heads = rbvcer_run::rbvcer_capture(
        "git",
        &["-C", &clone, "for-each-ref", "--format=%(refname:short)", "refs/heads/"],
        top,
    );
    if heads.code != 0 {
        crate::rbvcel_fatal!("failed to list the clone's branches:\n{}", heads.stderr.trim());
    }
    for head_ref in heads.stdout.lines() {
        let head_ref = head_ref.trim();
        if head_ref.is_empty() || head_ref == RBVCEX_REGULUS_BRANCH {
            continue;
        }
        zrbvcex_git_capture(&clone, &["branch", "-D", head_ref], top, "drop a base branch");
    }

    // Removed rather than overwritten. A path the previous release shipped
    // and this one withholds must LEAVE the tree; materializing on top of the
    // base would let it persist forever, unnoticed, because nothing would
    // ever name it again.
    rbvcel_log::rbvcel_step("Clearing the base tree");
    let base_files = rbvcer_run::rbvcer_capture("git", &["-C", &clone, "ls-files"], top);
    if base_files.code != 0 {
        crate::rbvcel_fatal!("git ls-files failed in the clone:\n{}", base_files.stderr.trim());
    }
    for file in base_files.stdout.lines() {
        let file = file.trim();
        if file.is_empty() {
            continue;
        }
        let path = clone_dir.join(file);
        match std::fs::remove_file(&path) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => crate::rbvcel_fatal!("failed to clear {}: {}", path.display(), e),
        }
    }

    // git archive, not a per-path copy: it reads the committed bytes of the
    // named commit and preserves the mode bits. A shipped tree of several
    // hundred scripts that arrived without their executable bit would be a
    // regulus that cannot run. The pathspec is handed as arguments — git
    // archive takes no pathspec file, and a shell relay would split any path
    // that carried a space.
    rbvcel_log::rbvcel_step(&format!("Materializing the shipped paths from {}", head));
    let archive_path = scratch_dir.join("shipped.tar");
    let archive = rbvcep_repo::rbvcep_as_str(&archive_path);
    let mut archive_args: Vec<&str> = vec!["archive", "--format=tar", "-o", &archive, &head, "--"];
    for path in &shipped {
        archive_args.push(path.as_str());
    }
    let got = rbvcer_run::rbvcer_capture("git", &archive_args, top);
    if got.code != 0 {
        crate::rbvcel_fatal!("failed to archive the shipped paths from {}:\n{}", head, got.stderr.trim());
    }
    let got = rbvcer_run::rbvcer_capture("tar", &["-x", "-f", &archive, "-C", &clone], top);
    if got.code != 0 {
        crate::rbvcel_fatal!("failed to materialize the shipped paths into the clone:\n{}", got.stderr.trim());
    }

    // The clone runs ITS OWN copy — a process cannot regenerate from
    // lustrated values unless its modules are the lustrated modules (see
    // rblm_sterilize.sh). The one subprocess of the cut, by design. Its
    // dispatch env is provided explicitly: the hierophant's stream() scrubs
    // the inherited BURD_ state, and the script needs exactly three values —
    // a temp root (the cut's scratch), the BUK utility modules, and the
    // RELATIVE tabtarget dir it regenerates the context from.
    rbvcel_log::rbvcel_step("Lustrating and regenerating in the clone");
    let sterilize = clone_dir.join(RBVCEX_STERILIZE_PATH);
    if !sterilize.is_file() {
        crate::rbvcel_fatal!(
            "the materialized regulus carries no sterilize script — the perambulation must ship {}",
            RBVCEX_STERILIZE_PATH
        );
    }
    let sterilize = rbvcep_repo::rbvcep_as_str(&sterilize);
    let scratch = rbvcep_repo::rbvcep_as_str(&scratch_dir);
    let buk_dir = rbvcep_repo::rbvcep_as_str(&top.join(RBVCEX_BUK_SUBDIR));
    let code = rbvcer_run::rbvcer_stream(
        "bash",
        &[&sterilize],
        top,
        &[
            ("BURD_TEMP_DIR", &scratch),
            ("BURD_BUK_DIR", &buk_dir),
            ("BURD_TABTARGET_DIR", RBVCCD_TT_SUBDIR),
        ],
    );
    if code != 0 {
        crate::rbvcel_fatal!("sterilization failed in the clone (exit {})", code);
    }

    // Transpose the consumer context onto the regulus's CLAUDE.md. The
    // perambulation ships CLAUDE.md, so the materialization above wrote the
    // MAINTAINER's copy into the working tree — veiled paths on its face.
    // That copy is overwritten now, before the commit, so the maintainer's
    // CLAUDE.md never enters the regulus's object graph: the committed blob
    // is the consumer template's.
    rbvcel_log::rbvcel_step(&format!("Transposing the consumer context onto {}", RBVCEX_REGULUS_CLAUDE_PATH));
    let template = rbvcer_run::rbvcer_capture_bytes("git", &["show", &template_ref], top);
    if template.code != 0 {
        crate::rbvcel_fatal!("failed to read the consumer CLAUDE.md template:\n{}", template.stderr.trim());
    }
    let claude_target = clone_dir.join(RBVCEX_REGULUS_CLAUDE_PATH);
    if let Err(e) = std::fs::write(&claude_target, &template.stdout) {
        crate::rbvcel_fatal!("failed to transpose the consumer CLAUDE.md template: {}", e);
    }

    // Byte-assert the result equals the committed template. A transposition
    // that silently wrote nothing, or wrote a truncated stream, would ship
    // the wrong context under a green battery. The template is re-read from
    // the object store and the target re-read from disk, so the assert
    // compares what was committed against what will be.
    let expect = rbvcer_run::rbvcer_capture_bytes("git", &["show", &template_ref], top);
    if expect.code != 0 {
        crate::rbvcel_fatal!("failed to re-read the consumer CLAUDE.md template for assertion:\n{}", expect.stderr.trim());
    }
    let written = std::fs::read(&claude_target)
        .unwrap_or_else(|e| crate::rbvcel_fatal!("failed to re-read {}: {}", claude_target.display(), e));
    if expect.stdout.is_empty() {
        crate::rbvcel_fatal!("the consumer CLAUDE.md template read back empty");
    }
    if written != expect.stdout {
        crate::rbvcel_fatal!(
            "transposition byte-mismatch: {} does not equal {}",
            RBVCEX_REGULUS_CLAUDE_PATH, RBVCEX_CONSUMER_CLAUDE_PATH
        );
    }

    rbvcel_log::rbvcel_step("Committing the regulus");
    zrbvcex_git_capture(&clone, &["add", "--all"], top, "stage the regulus");
    zrbvcex_git_capture(&clone, &["commit", "-m", RBVCEX_REGULUS_SUBJECT], top, "commit the regulus");

    // One commit. Not a convention — the property that makes the regulus
    // mergeable by construction and provable by inspection.
    let range = match &base_sha {
        Some(base) => format!("{}..HEAD", base),
        None => "HEAD".to_string(),
    };
    let count = rbvcer_run::rbvcer_capture("git", &["-C", &clone, "rev-list", "--count", &range], top);
    if count.code != 0 {
        crate::rbvcel_fatal!("git rev-list failed in the clone:\n{}", count.stderr.trim());
    }
    if count.stdout.trim() != "1" {
        crate::rbvcel_fatal!(
            "the regulus is {} commits atop the public base, not 1 — refusing to call it a regulus",
            count.stdout.trim()
        );
    }

    // Sweep the DELTA, fatally. The object graph the regulus adds atop the
    // base must carry no withheld path. This is the assertion the base
    // inventory is not: the base's already-disclosed history is tolerated,
    // but THIS cut adds nothing withheld, and a leak here is fatal.
    rbvcel_log::rbvcel_step("Sweeping the regulus delta");
    let delta_graph = zrbvcex_graph_paths(&clone_dir, &[range.as_str()], "regulus");
    let leaks = rbvcem_perambulation::rbvcem_sweep(&delta_graph);
    if !leaks.is_empty() {
        for path in &leaks {
            rbvcel_log::rbvcel_line(&format!("leak: {}", path));
        }
        crate::rbvcel_fatal!("the regulus delta adds {} withheld path(s)", leaks.len());
    }
    rbvcel_log::rbvcel_line("Regulus delta clean: this cut adds no withheld path");

    // Zero remotes, asserted as the finished regulus's standing property.
    // The clone was severed above; this proves the sever held and nothing
    // re-wired a remote. With no remote, no command in the clone can reach
    // the public target: the reveal is human-hands-only by structural
    // incapacity, not by a rule anyone remembered.
    rbvcel_log::rbvcel_step("Asserting the regulus holds zero remotes");
    let remotes = rbvcer_run::rbvcer_capture("git", &["-C", &clone, "remote"], top);
    if remotes.code != 0 {
        crate::rbvcel_fatal!("failed to read the clone's remotes:\n{}", remotes.stderr.trim());
    }
    if !remotes.stdout.trim().is_empty() {
        crate::rbvcel_fatal!(
            "the regulus clone carries a remote — the cut must leave zero:\n{}",
            remotes.stdout.trim()
        );
    }

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line(&format!("Regulus:  {}", clone_dir.display()));
    rbvcel_log::rbvcel_line(&format!("Branch:     {}", RBVCEX_REGULUS_BRANCH));
    rbvcel_log::rbvcel_line(&format!(
        "Base:       {}",
        base_sha.as_deref().unwrap_or("(root commit — the base was empty)")
    ));
    rbvcel_log::rbvcel_line("Commits:    1");
    rbvcel_log::rbvcel_line("Remotes:    0 (severed — the cut can reach nothing)");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_success("Regulus expedited — one commit atop the public base, zero remotes, no withheld path in the delta");

    clone_dir
}

// ── Helpers ─────────────────────────────────────────────────

/// The live tracked set, from git, never from a list. The perambulation is
/// judged against what the repository actually carries at this commit; any
/// other source of truth is a second copy waiting to drift.
fn zrbvcex_tracked(top: &Path) -> Vec<String> {
    let got = rbvcer_run::rbvcer_capture("git", &["ls-files"], top);
    if got.code != 0 {
        crate::rbvcel_fatal!("git ls-files failed:\n{}", got.stderr.trim());
    }
    let tracked: Vec<String> = got
        .stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    if tracked.is_empty() {
        crate::rbvcel_fatal!("no tracked paths — the perambulation has nothing to judge");
    }
    tracked
}

/// Walk an object-graph range in the clone and return every PATH reachable in
/// it. rev-list --objects emits "SHA path" for anything with a path and a
/// bare SHA for commits; the bare lines are dropped. Fed --all it reads the
/// whole graph (the base inventory); fed {base}..HEAD it reads only what the
/// regulus added — precisely the reading that would have caught the 292 MiB
/// regulus, whose face was clean and whose history was not.
fn zrbvcex_graph_paths(clone_dir: &Path, range: &[&str], label: &str) -> Vec<String> {
    let clone = rbvcep_repo::rbvcep_as_str(clone_dir);
    let mut args: Vec<&str> = vec!["-C", &clone, "rev-list", "--objects"];
    args.extend_from_slice(range);
    let got = rbvcer_run::rbvcer_capture("git", &args, clone_dir);
    if got.code != 0 {
        crate::rbvcel_fatal!(
            "failed to walk the object graph of {} ({}):\n{}",
            clone_dir.display(), label, got.stderr.trim()
        );
    }
    got.stdout
        .lines()
        .filter_map(|line| line.split_once(' ').map(|(_, path)| path))
        .filter(|path| !path.is_empty())
        .map(|path| path.to_string())
        .collect()
}

/// git -C <clone> <args>, captured, fatal on non-zero with the act named.
fn zrbvcex_git_capture(clone: &str, args: &[&str], top: &Path, act: &str) {
    let mut full = vec!["-C", clone];
    full.extend_from_slice(args);
    let got = rbvcer_run::rbvcer_capture("git", &full, top);
    if got.code != 0 {
        crate::rbvcel_fatal!("failed to {} (git exited {}):\n{}", act, got.code, got.stderr.trim());
    }
}

/// Assert an anonymous read of the narthex 404s — the single privacy gate
/// shared by docimasy's own narthex gate (RBSHD step 1a) and ostend's
/// re-assertion of the ground (RBSHO step 2): a private GitHub repository
/// 404s to an unauthenticated request, so anything else means the narthex
/// is public or misnamed. Fatal otherwise.
pub fn rbvcex_assert_narthex_private(top: &Path) {
    let status = rbvcer_run::rbvcer_capture(
        "curl",
        &["-s", "-o", "/dev/null", "-w", "%{http_code}", RBVCEX_NARTHEX_HTTPS],
        top,
    );
    if status.code != 0 {
        crate::rbvcel_fatal!("anonymous read of the narthex failed to execute (curl exited {})", status.code);
    }
    if status.stdout.trim() != RBVCEX_NARTHEX_PRIVATE_STATUS {
        crate::rbvcel_fatal!(
            "anonymous read of the narthex ({}) returned HTTP {}, not {} — the narthex is public or misnamed",
            RBVCEX_NARTHEX_HTTPS, status.stdout.trim(), RBVCEX_NARTHEX_PRIVATE_STATUS
        );
    }
    rbvcel_log::rbvcel_line("narthex reads anonymous-404: private (or absent), never public");
}

// ── The freshness matcher ───────────────────────────────────

/// Assert that a scratch re-cut of the maintainer tree's shipped bytes, right
/// now, is tree-equal to the standing regulus — the ONE freshness matcher
/// shared by the docimasy and the ostend (RBSHD step 2, RBSHO step 2, RBSHC
/// "The cut, and the single matcher": laps and the reveal are decoupled in
/// time, so freshness is asserted, never presumed).
///
/// Reuses `cut` itself as the matcher — a second, independent freshness check
/// would be exactly the second matcher the single-matcher rule forbids. The
/// scratch cut lands at a FIXED sibling location distinct from the standing
/// regulus's own (RBVCEP_FRESHNESS_DIRNAME), so this can never collide with,
/// or dispose of, the regulus it is comparing against; it is disposed by
/// the same seposition rename discipline as every other sibling artifact,
/// win or lose.
///
/// Fatal on drift, naming the standing remedy: the cycle returns to essai,
/// never forward.
pub fn rbvcex_assert_fresh(top: &Path, parent: &Path, regulus_clone: &Path) {
    let freshness_parent = parent.join(rbvcep_repo::RBVCEP_FRESHNESS_DIRNAME);
    rbvcep_repo::rbvcep_guard_disposable(&freshness_parent, rbvcep_repo::RBVCEP_FRESHNESS_DIRNAME, top);
    if !rbvcep_repo::rbvcep_sepose(&freshness_parent, top) {
        rbvcel_log::rbvcel_line("no prior freshness scratch to sepose");
    }

    rbvcel_log::rbvcel_step("Scratch re-cutting the maintainer tree's shipped bytes to assert freshness");
    let scratch_clone = rbvcex_cut(top, &freshness_parent);

    let standing_tree = rbvcep_repo::rbvcep_tree_hash(regulus_clone, top);
    let scratch_tree = rbvcep_repo::rbvcep_tree_hash(&scratch_clone, top);

    rbvcel_log::rbvcel_step("Disposing the freshness scratch");
    rbvcep_repo::rbvcep_sepose(&freshness_parent, top);

    if standing_tree != scratch_tree {
        crate::rbvcel_fatal!(
            "regulus freshness drift: a re-cut of the maintainer tree's shipped bytes now (tree {}) does not match the standing regulus (tree {}) — the cycle returns to essai, never forward",
            scratch_tree, standing_tree
        );
    }
    rbvcel_log::rbvcel_line(&format!("regulus fresh: re-cut tree {} matches the standing regulus", standing_tree));
}
