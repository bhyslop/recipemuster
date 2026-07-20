// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Blotter engine — the journal ceremony bracket every blotter write passes
//! through (JJSVJ-journal.adoc, `jjdb_journal`), engine-known bootstrap config
//! for a blotter instance (JJSVB-blotter.adoc, bootstrap config), the founding
//! ceremony that stands an instance up from nothing (`jjdb_found`, JJSAS
//! Founding-and-cutover), and the lock-free staleness-tolerant read path
//! (`jjdk_lockless_reads`).
//!
//! Generic over any `jjrfr_FarrierCore + jjrfr_FarrierLock` kind — the same
//! machinery serves the studbook today and the mews fleet store later (blotter
//! sheaf: "two repos, two locks: instances share machinery, never state").
//!
//! Scope note: the journal ceremony's "work half" (JJSVJ step 1 — lodging and
//! consigning issued work on the *work repo*, present only for a notch or a
//! wrap) runs on a different root than the blotter and is therefore a caller
//! concern, not this module's — a caller that has issued work produces its own
//! counterfoil first and folds it into what it writes via the `mutate` closure
//! below. `jjdb_journal` covers the blotter-side bracket: lock, advance, mutate,
//! proffer, release.

use crate::jjrf_favor::jjrf_emblazon_ordinal;
use crate::jjrfr_farrier::{
    jjrfr_ConsignLease,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_LockGuard,
    jjrfr_Rejection,
};
use std::path::{
    Path,
    PathBuf,
};

// ---- Bootstrap config ----

/// The studbook's fixed dirname within the infield peer ring (`jjqs_studbook`,
/// cosmology sheaf `jjdw_yard`).
pub const JJDB_STUDBOOK_DIRNAME: &str = "jjqs_studbook";

/// The studbook's real founding remote (JJSVS Founding-and-cutover: "Remote: a
/// GitHub private repo", "Separate at birth" — its own bare repo, distinct
/// from the JJ kit repo `jjqa_app`). Founded 260713 (₢BrAAU).
const ZJJDB_STUDBOOK_REMOTE: &str = "git@github.com:bhyslop/jjqs_studbook.git";

/// The studbook's one line of work. A blotter is linear and never branches,
/// so this names it once.
const ZJJDB_STUDBOOK_TRUNK: &str = "trunk";

/// The studbook's revision-ordinal sigil (`jjdb_catchword`, blotter sheaf
/// "Revision ordinals"): glyph ₶ (U+20B6 livre tournois).
pub const JJDB_CATCHWORD_SIGIL: char = '\u{20B6}';

/// The studbook's revision-ordinal founding value (`jjdb_catchword`): 200000 —
/// a width horizon of ~800K revisions before a future leading-digit x width pair.
pub const JJDB_CATCHWORD_FOUNDING: u64 = 200000;

/// Engine-known bootstrap coordinates for one blotter instance (`jjdb_blotter`
/// bootstrap config, blotter sheaf): where it lives locally, its remote, its
/// one line of work, and its revision-ordinal mark (`jjdb_catchword`/
/// `jjdb_varvel` — sigil plus founding value, one pair per instance). Never
/// pedigree-resolved — a store cannot look itself up in itself; the pedigree
/// resolves consumer repos, the engine carries its own stores' coordinates.
/// Plain data — tests construct their own pointed at a scratch repo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjdb_BlotterConfig {
    pub local_root: PathBuf,
    pub remote_url: String,
    pub trunk: String,
    pub ordinal_sigil: char,
    pub ordinal_founding: u64,
}

/// The studbook's engine-known bootstrap config, given the station's infield
/// root. `infield_root` is captured once by the caller (the no-cwd rule this
/// engine shares with the farrier trait: the door captures cwd, this function
/// never reads the environment or the working directory itself).
pub fn jjdb_studbook_config(infield_root: &Path) -> jjdb_BlotterConfig {
    jjdb_BlotterConfig {
        local_root: infield_root.join(JJDB_STUDBOOK_DIRNAME),
        remote_url: ZJJDB_STUDBOOK_REMOTE.to_string(),
        trunk: ZJJDB_STUDBOOK_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    }
}

// ---- Revision ordinals ----

/// Bake a revision ordinal into a commit message as its leading token — the
/// one place the mark is composed with message text (`jjdb_catchword`/
/// `jjdb_varvel`, blotter sheaf "Revision ordinals"). Bare decimal is
/// charset-valid, so the sigil rides along via `jjrf_emblazon_ordinal`: an
/// ordinal never circulates glyphless in operator-facing output, and a commit
/// message is exactly that.
fn zjjrvb_bake_ordinal(sigil: char, ordinal: u64, message: &str) -> String {
    format!("{}: {}", jjrf_emblazon_ordinal(sigil, ordinal), message)
}

// ---- Founding ceremony ----

/// Found a blotter instance from nothing (JJSVS Founding-and-cutover): local
/// `git init` on the config's trunk branch, one seed commit, wire the
/// pre-existing empty bare `origin` in, and push. Plain git, hardwired for
/// the MVP's one farrier kind — founding sits outside the farrier trait by
/// design (JJSVF "Deliberately absent": init and clone are never trait ops,
/// the operator's hands or an engine ceremony like this one).
/// The remote itself is not created here: an already-empty bare repo is an
/// operator prerequisite.
///
/// Unlocked by design: founding runs once, single-actor, before any other
/// writer can exist to race it — the journal's lock discipline begins with
/// the instance's first `jjdb_journal` write, not before the instance
/// exists. Panics on any git failure: this is an attended, one-shot
/// ceremony, not a composed primitive with a rejection taxonomy to honor.
///
/// Returns the new HEAD SHA — the position now also live on the remote.
pub fn jjdb_found<M>(config: &jjdb_BlotterConfig, seed: M) -> String
where
    M: FnOnce(&Path) -> (Vec<PathBuf>, String),
{
    let root = config.local_root.as_path();
    std::fs::create_dir_all(root).unwrap_or_else(|e| panic!("jjdb_found: could not create {}: {}", root.display(), e));

    zjjrvb_found_git(root, &["init", "-q", "-b", &config.trunk]);

    let (files, message) = seed(root);
    let file_strs: Vec<String> = files.iter().map(|p| p.to_string_lossy().into_owned()).collect();

    let mut add_args: Vec<&str> = vec!["add", "--"];
    add_args.extend(file_strs.iter().map(String::as_str));
    zjjrvb_found_git(root, &add_args);

    // The genesis commit takes the store's founding value: no zeroth special
    // case (blotter sheaf "Revision ordinals").
    let baked_message = zjjrvb_bake_ordinal(config.ordinal_sigil, config.ordinal_founding, &message);
    let mut commit_args: Vec<&str> = vec!["commit", "-q", "-m", &baked_message, "--"];
    commit_args.extend(file_strs.iter().map(String::as_str));
    zjjrvb_found_git(root, &commit_args);

    zjjrvb_found_git(root, &["remote", "add", "origin", &config.remote_url]);
    zjjrvb_found_git(root, &["push", "-q", "-u", "origin", &config.trunk]);

    zjjrvb_found_git(root, &["rev-parse", "HEAD"]).trim().to_string()
}

/// Run `git -C root <args>`, panicking loud on failure — founding has no
/// rejection taxonomy of its own (it precedes the instance the taxonomy
/// governs).
fn zjjrvb_found_git(root: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("jjdb_found: git spawn failed for -C {} {:?}: {}", root.display(), args, e));
    if !out.status.success() {
        panic!("jjdb_found: git -C {} {:?} failed: {}", root.display(), args, String::from_utf8_lossy(&out.stderr));
    }
    String::from_utf8(out.stdout).expect("git stdout must be UTF-8")
}

// ---- Read path ----

/// The lock-free, staleness-tolerant read path (`jjdk_lockless_reads`, blotter
/// sheaf): reads a blotter's local clone directly — no lock, no fetch, no
/// blocking on the network. Consumers reach a blotter's content through this,
/// never raw JSON off an unmanaged path.
pub fn jjdb_read(config: &jjdb_BlotterConfig, rel_path: &Path) -> std::io::Result<Vec<u8>> {
    std::fs::read(config.local_root.join(rel_path))
}

// ---- Journal ceremony ----

/// The journal ceremony (`jjdb_journal`, journal sheaf): the blotter-side
/// bracket every write passes through, ordered durable-first. `guidon` is the
/// caller-composed lock-holder mark (officium, station, acquire time,
/// operation); `mutate` receives the blotter's local root — post-lock,
/// post-advance, so the working tree it reads and writes IS the remote tip's
/// state and a stale pre-lock read can never be carried across the lock — and
/// returns the explicit file list plus commit message `jjrfr_proffer` will use
/// (additive discipline — no stage-all, no amend).
///
/// Sequence: glean (opportunistic) -> stake (via the RAII guard) -> sight
/// (confirm the held guidon is ours) -> advance (fast-forward-only, to the
/// gleaned remote tip) -> mutate -> proffer (compose against the tip without
/// moving the local branch, push atomic-under-lease, adopt locally only on
/// acceptance — `JJr_b52`) -> release (best-effort pluck via the guard's drop).
///
/// Returns the accepted position's SHA on success — live on the remote and
/// adopted locally. A rejection at any lock-held step leaves `mutate` never
/// called; a rejection at advance or proffer still releases the lock on the
/// way out (the guard drops on every exit path), and a refused proffer leaves
/// the local branch and its record untouched — no residue exists for any later
/// ceremony to scrub.
pub fn jjdb_journal<F, M>(
    farrier: &F,
    config: &jjdb_BlotterConfig,
    guidon: &str,
    mutate: M,
) -> Result<String, jjrfr_Rejection>
where
    F: jjrfr_FarrierCore + jjrfr_FarrierLock,
    M: FnOnce(&Path) -> (Vec<PathBuf>, String),
{
    let root = config.local_root.as_path();

    // Take the lock: glean is opportunistic (never blocks on the network, and
    // its outcome does not gate the ceremony); stake is the guard's own
    // compare-and-swap; sight confirms the held guidon is ours before we trust
    // it enough to advance and write under it.
    let _ = farrier.jjrfr_glean(root);
    let guard = jjrfr_LockGuard::jjrfr_acquire(farrier, root, guidon)?;
    let sighted = farrier.jjrfr_sight(root)?;
    if sighted.as_deref() != Some(guard.jjrfr_guidon()) {
        panic!(
            "jjdb_journal: sight after stake did not confirm our own guidon at {} (expected {:?}, saw {:?})",
            root.display(),
            guard.jjrfr_guidon(),
            sighted
        );
    }

    // Advance: fast-forward-only to the gleaned remote tip (JJr_b52) — under
    // compose-then-push the local branch holds nothing of its own, so there is
    // never anything to retrench; a diverged clone rejects for the attended
    // session instead.
    farrier.jjrfr_advance(root)?;

    // Mutate: the caller writes its content against the tip's own state; we
    // allocate the next revision ordinal under the lock we already hold —
    // derived from the linear history's length at the tip we just advanced to,
    // no side table (blotter sheaf "Revision ordinals") — and bake it in.
    let (files, message) = mutate(root);
    let ordinal = config.ordinal_founding + zjjrvb_commit_count(root);
    let baked_message = zjjrvb_bake_ordinal(config.ordinal_sigil, ordinal, &message);

    // Proffer: compose the commit against the tip without moving the local
    // branch, push atomic-under-lease against our own lock ref — if the lock
    // was broken under us, the whole push fails (journal sheaf, step 5) — and
    // adopt locally only on acceptance (JJr_b52). No corruption, no service
    // but git: a rejection lands nothing on the remote and leaves the local
    // branch and its record untouched.
    let new_head = farrier.jjrfr_proffer(
        root,
        &config.trunk,
        &files,
        &baked_message,
        &jjrfr_ConsignLease(guard.jjrfr_guidon().to_string()),
    )?;

    // Release: best-effort pluck via the guard's drop, right here at the
    // ceremony's natural end.
    drop(guard);
    Ok(new_head)
}

/// The linear history's length at the tree's current position — a blotter
/// never branches, so `git rev-list --count HEAD` is unambiguous and is what
/// makes the next ordinal derivable with no side table (blotter sheaf
/// "Revision ordinals"). Reaches straight to git, same as the founding
/// ceremony above (`zjjrvb_found_git`): the farrier trait carries no op for
/// this, and this engine, like founding, is not the vocabulary Palisade's
/// concern — that boundary governs the trait's own op names, not this module.
/// Panics on git failure: an environment fact, not a farrier rejection.
fn zjjrvb_commit_count(root: &Path) -> u64 {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-list", "--count", "HEAD"])
        .output()
        .unwrap_or_else(|e| panic!("zjjrvb_commit_count: git spawn failed at {}: {}", root.display(), e));
    if !out.status.success() {
        panic!(
            "zjjrvb_commit_count: git rev-list --count HEAD failed at {}: {}",
            root.display(),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    String::from_utf8(out.stdout)
        .expect("git stdout must be UTF-8")
        .trim()
        .parse()
        .unwrap_or_else(|e| panic!("zjjrvb_commit_count: could not parse count at {}: {}", root.display(), e))
}

// ---- Gallops-over-studbook surface (enablement seam) ----

/// Enablement seam: the gallops-over-studbook surface below is complete and
/// tested, but not yet live. Every `jjx_*` command still calls jjri_io's
/// jjdr_load/jjdr_save/jjri_persist/jjri_consign directly and unconditionally —
/// nothing outside this module and its tests reads this constant. Flipping it
/// is the conversion heat's act (JJSVS Founding-and-cutover), not this pace's.
pub const JJDB_GALLOPS_OVER_STUDBOOK_ENABLED: bool = false;

/// The gallops file's fixed relative path within the studbook — its first
/// tenant (`jjdb_studbook` Scope at birth).
pub const JJDB_GALLOPS_REL_PATH: &str = "gallops.json";

// ---- Pinned-snapshot ref-read (the read path, object-database only) ----

/// Run `git -C root <args>` for a read, returning raw stdout bytes on success
/// and git's stderr as the error on any non-zero exit. Reaches straight to git,
/// same as the founding ceremony (`zjjrvb_found_git`): the read path is a plain
/// object-database query the farrier trait carries no op for, and this engine —
/// like founding — is not the vocabulary Palisade's concern. A spawn failure is
/// an environment precondition violation, not a classified outcome, so it panics.
fn zjjdb_read_git(root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("jjdb ref-read: git spawn failed for -C {} {:?}: {}", root.display(), args, e));
    if !out.status.success() {
        return Err(format!(
            "git -C {} {:?} failed: {}",
            root.display(),
            args,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(out.stdout)
}

/// Pin the studbook's read snapshot: resolve the SHA that the fetched
/// remote-tracking ref `refs/remotes/origin/<trunk>` points at. Pure-local —
/// reads the ref store, never the network: the dispatch door's glean is what
/// advances the ref, and every read taken behind a single pin sees one coherent
/// commit. This strengthens `jjdk_lockless_reads` — the read touches only the
/// object database, never the studbook working tree (writer-only scratch under
/// the journal lock).
pub fn jjdb_pin(config: &jjdb_BlotterConfig) -> Result<String, String> {
    let refname = format!("refs/remotes/{}/{}", "origin", config.trunk);
    let out = zjjdb_read_git(&config.local_root, &["rev-parse", "--verify", &refname])?;
    let sha = String::from_utf8(out)
        .map_err(|e| format!("git rev-parse of {} returned non-UTF-8: {}", refname, e))?
        .trim()
        .to_string();
    if sha.is_empty() {
        return Err(format!("studbook clone at {} has no {} — glean it before pinning", config.local_root.display(), refname));
    }
    Ok(sha)
}

/// Read one tenant blob from a pinned snapshot: `git -C local_root show
/// <pin>:<rel>`. Object-database only — never the working tree, so a read is
/// blind to any uncommitted studbook state and to the writer-only scratch the
/// journal ceremony mutates under lock. `rel_path` is a studbook-relative posix
/// path (a wire constant like `JJDB_GALLOPS_REL_PATH`, never a station path).
pub fn jjdb_read_pinned(config: &jjdb_BlotterConfig, pin: &str, rel_path: &str) -> Result<Vec<u8>, String> {
    zjjdb_read_git(&config.local_root, &["show", &format!("{}:{}", pin, rel_path)])
}

/// Persist a Gallops through the studbook's journal ceremony, mutate-as-transform
/// (`JJr_b52`): `transform` receives the gallops as the locked, advanced remote
/// tip holds it — `None` only when the tip carries no gallops tenant yet — and
/// returns the record to write, so a stale pre-lock read can never be carried
/// across the lock. Reuses `jjdr_save`'s atomic write plus load-back validation
/// unchanged (the old path's own machinery, frozen) for the file itself, then
/// commits it through `jjdb_journal` instead of `vvc::machine_commit` — the
/// studbook is a JJ-owned blotter, not a consumer repo, so the vvc commit-lock
/// apparatus does not apply here.
pub fn jjdb_gallops_journal_save<F, M>(
    farrier: &F,
    config: &jjdb_BlotterConfig,
    guidon: &str,
    transform: M,
    message: String,
) -> Result<String, jjrfr_Rejection>
where
    F: jjrfr_FarrierCore + jjrfr_FarrierLock,
    M: FnOnce(Option<crate::jjri_io::jjdr_ValidatedGallops>) -> crate::jjrt_types::jjrg_Gallops,
{
    jjdb_journal(farrier, config, guidon, |root| {
        // Post-lock, post-advance: the pin IS the tip the local branch now
        // stands on, so this read is the store's truth at this moment. A
        // corrupt tenant panics loud — halt-and-surface, never paper over.
        let current = match zjjdb_tip_gallops(config) {
            Ok(current) => current,
            Err(e) => panic!("jjdb_gallops_journal_save: could not read the locked tip's gallops: {}", e),
        };
        let gallops = transform(current);
        let path = root.join(JJDB_GALLOPS_REL_PATH);
        crate::jjri_io::jjdr_save(&gallops, &path)
            .unwrap_or_else(|e| panic!("jjdb_gallops_journal_save: jjdr_save failed at {}: {}", path.display(), e));
        (vec![PathBuf::from(JJDB_GALLOPS_REL_PATH)], message)
    })
}

/// The pinned tip's gallops tenant, or `None` when the tip has no such tenant
/// yet (a pre-seed store) — distinguished structurally via `ls-tree`, never by
/// sniffing an error message.
fn zjjdb_tip_gallops(config: &jjdb_BlotterConfig) -> Result<Option<crate::jjri_io::jjdr_ValidatedGallops>, String> {
    let pin = jjdb_pin(config)?;
    let listing = zjjdb_read_git(&config.local_root, &["ls-tree", &pin, "--", JJDB_GALLOPS_REL_PATH])?;
    if listing.is_empty() {
        return Ok(None);
    }
    let bytes = jjdb_read_pinned(config, &pin, JJDB_GALLOPS_REL_PATH)?;
    crate::jjri_io::jjdr_hark(&bytes).map(Some)
}

/// Load a Gallops from the studbook via ref-read: pin the fetched snapshot, then
/// read `gallops.json` from that one commit's object database — never the
/// studbook working tree. Reuses `jjdr_hark` (the read-only, never-re-saved
/// sibling of `jjdr_load`: same deserialize, reprieve write-forward, and
/// semantic validation, round-trip check stood down since the pinned bytes are
/// never saved back) — a read mutates nothing, so the working-tree freshen the
/// old path implied is gone entirely.
pub fn jjdb_gallops_journal_load(
    config: &jjdb_BlotterConfig,
) -> Result<crate::jjri_io::jjdr_ValidatedGallops, String> {
    let pin = jjdb_pin(config)?;
    let bytes = jjdb_read_pinned(config, &pin, JJDB_GALLOPS_REL_PATH)?;
    crate::jjri_io::jjdr_hark(&bytes)
}
