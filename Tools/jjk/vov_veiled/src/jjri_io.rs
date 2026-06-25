// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops I/O operations with validation
//!
//! Provides validated load/save operations for Gallops JSON with round-trip guarantees.

use std::fs;
use std::path::Path;
use crate::jjrt_types::jjrg_Gallops;
use crate::jjrv_validate::jjrg_validate;

/// Validated Gallops wrapper
///
/// This newtype ensures that Gallops instances are validated on load.
/// Only jjdr_load can create instances, guaranteeing all ValidatedGallops
/// have passed both deserialization and semantic validation.
pub struct jjdr_ValidatedGallops(jjrg_Gallops);

impl jjdr_ValidatedGallops {
    /// Access the inner Gallops (immutable)
    pub fn inner(&self) -> &jjrg_Gallops {
        &self.0
    }

    /// Access the inner Gallops (mutable)
    pub fn inner_mut(&mut self) -> &mut jjrg_Gallops {
        &mut self.0
    }

    /// Consume the wrapper and return the inner Gallops
    pub fn into_inner(self) -> jjrg_Gallops {
        self.0
    }

    /// Test-only constructor (bypasses validation)
    #[cfg(test)]
    pub fn test_wrap(g: jjrg_Gallops) -> Self {
        Self(g)
    }
}

/// Find first byte position where two byte slices differ
fn zjjdr_find_first_diff(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .position(|(x, y)| x != y)
        .unwrap_or_else(|| a.len().min(b.len()))
}

/// Encode a firemark string into a case-safe paddock filename path.
///
/// Each character is prefixed with 'u' (uppercase) or 'l' (lowercase),
/// ensuring the resulting filename is unambiguous on case-insensitive
/// filesystems (macOS HFS+/APFS).
///
/// Examples:
///   "AG" → ".claude/jjm/jjp_uAuG.md"
///   "Ag" → ".claude/jjm/jjp_uAlg.md"
///   "ag" → ".claude/jjm/jjp_lalg.md"
pub fn jjri_paddock_path(firemark: &str) -> String {
    let encoded: String = firemark.chars().map(|c| {
        if c.is_uppercase() {
            format!("u{}", c)
        } else {
            format!("l{}", c)
        }
    }).collect();
    format!(".claude/jjm/jjp_{}.md", encoded)
}

/// Reprieve mechanism rivet — opaque cited token (MCM `mcm_rivet`); the proposition and
/// rationale live in JJS0 `jjdz_reprieve`.
///
/// A rivet ID carries no meaning, unlike a quoin's readable name — the opaque tail leaks no
/// semantics into the rust that ships without the veiled spec. Single source of the token string
/// (String Boundary Discipline): one `grep JJr_a7c` returns the spec quoin (the permanent
/// registry/probe/nag operating manual) and every code and spec site the mechanism governs. The
/// jjx_open nag emits it beside the legible label below, so a console reader can grep straight to
/// the spec — the census surface a jailer rivet rides on its phase announcement (JDG JDo_101).
pub const JJDZ_RIVET_REPRIEVE: &str = "JJr_a7c";

/// Operator-facing label for the reprieve mechanism — the readable form the jjx_open nag prints
/// beside the opaque rivet token. The rivet stays meaningless; the human reads this.
pub const JJDZ_LABEL_REPRIEVE: &str = "reprieve";

/// Verdict words for the open-time nag, one per `jjdz_Status` live value.
const ZJJDZ_PENDING: &str = "pending";
const ZJJDZ_DORMANT: &str = "dormant";

/// Inline gloss per verdict — what the word means and when the episode is removable, so the nag
/// line is actionable without opening code or spec. One per `jjdz_Status` live value.
const ZJJDZ_GLOSS_PENDING: &str = "old shape still present here, tolerance load-bearing";
const ZJJDZ_GLOSS_DORMANT: &str = "store canonical here, episode removable once dormant on every clone";

/// Per-episode reprieve status for an on-disk Gallops (output of jjdz_probe).
pub struct jjdz_Status {
    /// Human label for the episode (e.g. "V3→V4").
    pub label: &'static str,
    /// true = pending: this episode's old shape is present, so its tolerance is
    /// load-bearing on this install. false = dormant: the on-disk Gallops is already
    /// canonical for this episode, so the tolerance is a removal candidate here.
    pub live: bool,
}

impl jjdz_Status {
    /// The nag verdict word for this status — pending when load-bearing, dormant otherwise.
    pub fn jjdz_verdict(&self) -> &'static str {
        if self.live { ZJJDZ_PENDING } else { ZJJDZ_DORMANT }
    }

    /// The inline gloss for this status — the verdict's meaning and removal rule, so the nag line
    /// reads as a standing reminder rather than an opaque token.
    pub fn jjdz_gloss(&self) -> &'static str {
        if self.live { ZJJDZ_GLOSS_PENDING } else { ZJJDZ_GLOSS_DORMANT }
    }
}

/// One registered reprieve episode: a tolerated old on-disk shape with a live-test.
/// The demolition condition and lifecycle are spec data (JJS0 `jjdz_reprieve`), not here.
struct zjjdz_Episode {
    label: &'static str,
    is_live: fn(&jjrg_Gallops, &[u8]) -> bool,
}

/// V3→V4 episode live-test — rivet JJr_a7c.
///
/// True when any pre-V4 residue is present:
///   - heat_order absent (added in V4; BTreeMap sort order differs from original furlough order)
///   - stale next_pensum_seed field (removed in v3.7; serde drops it on the next save)
fn zjjdz_episode_v3_to_v4_live(gallops: &jjrg_Gallops, original_bytes: &[u8]) -> bool {
    const PENSUM_SEED_KEY: &[u8] = b"\"next_pensum_seed\"";
    let stale_pensum_seed = !gallops.heats.is_empty()
        && original_bytes.windows(PENSUM_SEED_KEY.len()).any(|w| w == PENSUM_SEED_KEY);
    gallops.heat_order.is_empty() || stale_pensum_seed
}

/// schema_version-drop episode live-test — rivet JJr_a7c.
///
/// True when the on-disk bytes still carry the now-removed `jjgrn_schema_version` key. No
/// write-forward body is needed: the field is gone from the type, so the next save omits the
/// key on its own; the episode exists only to flag the file as a known old shape, standing the
/// round-trip gate down so that first save can land (JJS0 `jjdz_reprieve`).
fn zjjdz_episode_schema_version_drop_live(_gallops: &jjrg_Gallops, original_bytes: &[u8]) -> bool {
    const SCHEMA_VERSION_KEY: &[u8] = b"\"jjgrn_schema_version\"";
    original_bytes.windows(SCHEMA_VERSION_KEY.len()).any(|w| w == SCHEMA_VERSION_KEY)
}

/// tack-text→lines episode live-test — rivet JJr_a7c.
///
/// True when the on-disk bytes still carry a string-valued `jjgtn_text` (the legacy docket
/// shape). The pretty serializer emits `"jjgtn_text": "` only for a string value; an array
/// value reads `"jjgtn_text": [`. The custom field deserializer has already normalized both
/// shapes to the line array in the parsed struct (so the struct alone cannot tell them apart),
/// which is why detection sniffs the raw bytes. When live, jjdr_load stands the round-trip gate
/// down — a string→array reserialization would otherwise mismatch — and the write-forward
/// collapses any legacy multi-tack history to its newest element.
fn zjjdz_episode_tack_text_to_lines_live(_gallops: &jjrg_Gallops, original_bytes: &[u8]) -> bool {
    const TACK_TEXT_STRING_KEY: &[u8] = b"\"jjgtn_text\": \"";
    original_bytes.windows(TACK_TEXT_STRING_KEY.len()).any(|w| w == TACK_TEXT_STRING_KEY)
}

/// bridle-retirement episode live-test — rivet JJr_a7c.
///
/// True when the on-disk bytes still carry the retired bridle/warrant surface: the
/// `jjgte_bridled` pace-state token, or the removed `jjgtn_direction` warrant key. The
/// `jjrg_PaceState` deserializer already demotes a legacy bridled value to Rough via serde
/// alias, and serde drops the unknown `jjgtn_direction` key on parse — so the parsed struct
/// alone cannot tell an old file apart, which is why detection sniffs the raw bytes. When
/// live, jjdr_load stands the round-trip gate down (a bridled→rough reserialization, and the
/// dropped direction key, would otherwise mismatch); no write-forward body is needed, since
/// the next jjdr_save emits `jjgte_rough` and omits `jjgtn_direction` on its own. The V3 state
/// alias `primed` needs no detection here: a `primed` store predates V4's `heat_order`, so the
/// V3→V4 episode already flags it (and a bridled V3 pace also carries `jjgtn_direction`).
fn zjjdz_episode_bridle_retirement_live(_gallops: &jjrg_Gallops, original_bytes: &[u8]) -> bool {
    const BRIDLED_STATE_TOKEN: &[u8] = b"\"jjgte_bridled\"";
    const DIRECTION_KEY: &[u8] = b"\"jjgtn_direction\"";
    original_bytes.windows(BRIDLED_STATE_TOKEN.len()).any(|w| w == BRIDLED_STATE_TOKEN)
        || original_bytes.windows(DIRECTION_KEY.len()).any(|w| w == DIRECTION_KEY)
}

/// The reprieve registry — every tolerated old on-disk schema, one entry per episode.
/// Permanent infrastructure: episodes are appended as schema changes land and removed once
/// dormant on every operated clone (the per-episode lifecycle in JJS0 `jjdz_reprieve`).
const ZJJDZ_REGISTRY: &[zjjdz_Episode] = &[
    zjjdz_Episode { label: "V3→V4", is_live: zjjdz_episode_v3_to_v4_live },
    zjjdz_Episode { label: "schema_version drop", is_live: zjjdz_episode_schema_version_drop_live },
    zjjdz_Episode { label: "tack text→lines", is_live: zjjdz_episode_tack_text_to_lines_live },
    zjjdz_Episode { label: "bridle retirement", is_live: zjjdz_episode_bridle_retirement_live },
];

/// Read-only reprieve probe — the single source of "what counts as old-format".
///
/// Pure: reads the parsed Gallops and its on-disk bytes, mutates nothing. Returns one status
/// per registered episode. jjdr_load consults it to decide migration mode (the loader keeps no
/// second copy of detection); the jjx_open nag consults it to report, per install, which
/// episodes are still load-bearing versus dormant.
pub fn jjdz_probe(gallops: &jjrg_Gallops, original_bytes: &[u8]) -> Vec<jjdz_Status> {
    ZJJDZ_REGISTRY
        .iter()
        .map(|ep| jjdz_Status {
            label: ep.label,
            live: (ep.is_live)(gallops, original_bytes),
        })
        .collect()
}

/// Migration write-forward — rivet JJr_a7c. The canonical-form struct transform the reprieve
/// mechanism applies once a legacy shape is detected: populate heat_order from the heat keys when
/// absent (V3→V4; BTreeMap guarantees sorted order), and collapse any legacy multi-tack history to
/// the single newest tack (tacks[0]) — tack evolution now lives in git per JJS0 Git-as-Journal.
/// serde already drops the removed schema_version key and any stale next_pensum_seed on its own, so
/// those episodes need no body here.
///
/// The single source of the forward transform: jjdr_load applies it in migration mode, and
/// validate-normalize (JJSCVL) applies it as its canonicalizer — neither keeps a second copy.
/// Idempotent: a store already canonical for these is untouched, so an unconditional application is
/// safe.
pub fn jjdz_write_forward(gallops: &mut jjrg_Gallops) {
    if gallops.heat_order.is_empty() {
        gallops.heat_order = gallops.heats.keys().cloned().collect();
    }
    for heat in gallops.heats.values_mut() {
        for pace in heat.paces.values_mut() {
            if pace.tacks.len() > 1 {
                pace.tacks.truncate(1);
            }
        }
    }
}

// ============================================================================
// Retention — chat-history capture policy (read + classify)
//
// An instance of the open-time monitum (JJS0 `jjdz_monitum`): a read-only, best-effort,
// never-gating self-report at officium open. Sibling to the reprieve nag, deliberately kept
// independent — no shared monitum abstraction until a third instance earns it. The field is read
// here and surfaced by the open monitum; the value is consumed by the capture mechanism and set by
// its operator-facing setter, both of which land separately.
// ============================================================================

/// The ISO date format the retention field is validated against — operator-typeable and
/// unambiguous. A non-empty value that does not parse against this is malformed.
pub const JJRI_RETENTION_DATE_FORMAT: &str = "%Y-%m-%d";

/// Classified retention policy read from a Gallops — the three states the open monitum reports.
pub enum jjri_RetentionState {
    /// Field absent or empty — retention off, capture is a no-op. The shareable default.
    Off,
    /// A valid `YYYY-MM-DD` — retention on since this date (the trimmed value).
    On(String),
    /// Non-empty but unparseable — capture disabled; the monitum reports it loud at open. The raw
    /// value is carried so the operator sees their own typo.
    Malformed(String),
}

/// Read and classify the retention policy from a Gallops. Pure, total, never fails — validation is
/// a read-time classification, never a parse gate, so a malformed date can never make the store
/// illegitimate (it lands as `Malformed`, not a load error). Single source of the three-state
/// determination, shared by the open monitum and the capture gate.
pub fn jjri_retention_state(gallops: &jjrg_Gallops) -> jjri_RetentionState {
    let raw = match &gallops.retention_since {
        Some(s) => s.trim(),
        None => return jjri_RetentionState::Off,
    };
    if raw.is_empty() {
        return jjri_RetentionState::Off;
    }
    match chrono::NaiveDate::parse_from_str(raw, JJRI_RETENTION_DATE_FORMAT) {
        Ok(_) => jjri_RetentionState::On(raw.to_string()),
        Err(_) => jjri_RetentionState::Malformed(raw.to_string()),
    }
}

/// Load and validate Gallops from a file
///
/// Performs these steps in order:
/// 1. Deserialize JSON to Gallops struct
/// 2. Round-trip validation: reserialize and compare bytes (validates stored format)
/// 3. Semantic validation via jjrg_validate
///
/// Returns ValidatedGallops wrapper that guarantees the data has passed all checks.
pub fn jjdr_load(path: &Path) -> Result<jjdr_ValidatedGallops, String> {
    // Read original bytes
    let original_bytes = fs::read(path)
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;
    zjjdr_from_bytes(&original_bytes, true)
}

/// Hark (retrospective) load — read-only sibling of jjdr_load. JJS0 `jjdr_hark`.
///
/// Loads a Gallops from in-memory bytes lifted from a prior git revision (jjri_show_blob), for
/// read-only retrospective display (jjx_show hark mode). Shares jjdr_load's deserialize, reprieve
/// probe + write-forward, and semantic validation; skips the round-trip canonical check
/// unconditionally (historical bytes are never re-saved) and never writes. Rivet JJr_a7c.
pub fn jjdr_hark(bytes: &[u8]) -> Result<jjdr_ValidatedGallops, String> {
    zjjdr_from_bytes(bytes, false)
}

/// Read one file's bytes as of a prior git revision: `git show <rev>:<path>`.
///
/// The input side of jjdr_hark — lifts a historical blob (Gallops or paddock) without touching the
/// working tree, via the vvc git helper (stdin-nulled). An unresolvable revision or absent path
/// surfaces git's stderr verbatim. JJS0 `jjdr_hark`.
pub fn jjri_show_blob(rev: &str, path: &str) -> Result<Vec<u8>, String> {
    let spec = format!("{}:{}", rev, path);
    let output = vvc::vvce_git_command(&["show", &spec])
        .output()
        .map_err(|e| format!("git show {}: failed to run: {}", spec, e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git show {}: {}", spec, stderr.trim()));
    }
    Ok(output.stdout)
}

/// Shared load tail for jjdr_load (disk) and jjdr_hark (git blob).
///
/// `check_roundtrip` gates the canonical-form round-trip (steps 4-5); hark passes false because the
/// historical bytes are never re-saved. The reprieve probe + in-memory write-forward and the
/// semantic validation run identically on both paths, so a hark's Gallops is as validated as a disk
/// load's and the unbypassable-validation invariant holds.
fn zjjdr_from_bytes(original_bytes: &[u8], check_roundtrip: bool) -> Result<jjdr_ValidatedGallops, String> {
    let mut gallops: jjrg_Gallops = serde_json::from_slice(original_bytes)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Reprieve probe is the single source of old-format detection (rivet JJr_a7c).
    // Any live episode means the on-disk shape is not yet canonical; tolerate the
    // round-trip mismatch so the next save rewrites the clean format back to disk.
    let reprieve = jjdz_probe(&gallops, original_bytes);
    let is_migration_mode = reprieve.iter().any(|s| s.live);

    if check_roundtrip && !is_migration_mode {
        let reserialized = serde_json::to_string_pretty(&gallops)
            .map_err(|e| format!("Failed to reserialize JSON: {}", e))?;

        if reserialized.as_bytes() != original_bytes {
            let diff_pos = zjjdr_find_first_diff(original_bytes, reserialized.as_bytes());
            return Err(format!("Round-trip validation failed at byte {}", diff_pos));
        }
    }

    // Reprieve write-forward — rivet JJr_a7c. In migration mode, run the shared canonical-form
    // transform (the single source jjdz_write_forward, also driven by validate-normalize) so the
    // next jjdr_save lands clean. The loader keeps no second copy of the transform; serde already
    // drops the removed schema_version key and any stale next_pensum_seed on its own. Hark never
    // saves, so for a hark this transform is display-only.
    if is_migration_mode {
        jjdz_write_forward(&mut gallops);
    }

    // Semantic validation
    jjrg_validate(&gallops)
        .map_err(|errors| format!("Validation failed: {}", errors.join("; ")))?;

    Ok(jjdr_ValidatedGallops(gallops))
}

/// Save Gallops to a file with validation
///
/// Performs atomic write with validation:
/// 1. Serialize to JSON
/// 2. Write to temp file
/// 3. Validate temp file via jjdr_load
/// 4. Rename temp to target (atomic)
///
/// If validation fails, temp file is deleted and error returned.
pub fn jjdr_save(gallops: &jjrg_Gallops, path: &Path) -> Result<(), String> {
    // Serialize to pretty JSON
    let json = serde_json::to_string_pretty(gallops)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    // Create temp file in same directory for atomic rename
    let parent = path.parent().ok_or_else(|| "Invalid path: no parent directory".to_string())?;
    let temp_path = parent.join(format!(".tmp.{}.json", std::process::id()));

    // Write to temp file
    fs::write(&temp_path, &json)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    // Validate what we wrote by loading it back
    match jjdr_load(&temp_path) {
        Ok(_) => {
            // Validation passed, rename to target
            fs::rename(&temp_path, path)
                .map_err(|e| format!("Failed to rename temp file to target: {}", e))?;
            Ok(())
        }
        Err(e) => {
            // Validation failed, clean up temp file
            let _ = fs::remove_file(&temp_path);
            Err(format!("Save validation failed: {}", e))
        }
    }
}

/// Save Gallops and commit with paddock in a single operation
///
/// This is the standard routine for JJK operations that modify gallops.
/// It saves the gallops file, then commits both the gallops and the paddock file
/// for the given heat using machine_commit.
///
/// Returns the commit hash on success.
pub fn jjri_persist(
    lock: &vvc::vvcc_CommitLock,
    gallops: &jjrg_Gallops,
    file: &Path,
    firemark: &crate::jjrf_favor::jjrf_Firemark,
    message: String,
    size_limit: u64,
    output: &mut vvc::vvco_Output,
) -> Result<String, String> {
    // Save gallops first
    jjdr_save(gallops, file)?;

    // Construct paths for commit
    let gallops_path = file.to_string_lossy().to_string();
    let paddock_path = jjri_paddock_path(firemark.jjrf_as_str());

    // Commit using machine_commit with explicit file list
    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![
            gallops_path,
            paddock_path,
        ],
        message,
        size_limit,
        warn_limit: 30000,
    };

    vvc::machine_commit(lock, &commit_args, output)
}

/// Commit the gallops alone — the gallops-wide persist path.
///
/// validate-normalize (JJSCVL) is the consumer. Unlike jjri_persist, which co-commits a heat's
/// paddock under a firemark identity, this commits only the gallops file and carries no heat/pace
/// affiliation, because validate is gallops-wide. The caller holds the commit lock (compile-time
/// proof via `_lock`). Saves the canonical gallops, then commits it under `size_limit`.
///
/// Returns `Ok(Some(hash))` when a commit landed, `Ok(None)` when the saved canonical form already
/// matched the committed store and no merge was pending (nothing to commit — the working tree was
/// re-canonicalized but HEAD was already clean), and `Err` on failure. On commit failure the
/// working-tree gallops is reverted to HEAD so a blocked commit (e.g. over budget) leaves the store
/// byte-for-byte as it was found — validate's "file untouched" contract for a non-clean exit.
///
/// Merge finalization rides for free: when a merge is in progress (MERGE_HEAD present) the
/// underlying `git commit` finalizes it with two parents — validate is the post-merge gallops
/// cleanup step.
pub fn jjri_consign(
    lock: &vvc::vvcc_CommitLock,
    gallops: &jjrg_Gallops,
    file: &Path,
    message: String,
    size_limit: u64,
    output: &mut vvc::vvco_Output,
) -> Result<Option<String>, String> {
    // Save the canonical gallops first (atomic write + load-back validation).
    jjdr_save(gallops, file)?;

    let path_str = file.to_string_lossy().to_string();

    // Commit when the gallops differs from HEAD, or when a merge is mid-flight (the commit
    // finalizes it). When neither holds, the saved form already matches the committed store, so
    // there is nothing to commit and no merge to seal — report Ok(None).
    let dirty = vvc::vvce_git_command(&["status", "--porcelain", "--", path_str.as_str()])
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(true);
    let merging = vvc::vvce_git_command(&["rev-parse", "--verify", "--quiet", "MERGE_HEAD"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !dirty && !merging {
        return Ok(None);
    }

    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![path_str.clone()],
        message,
        size_limit,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    match vvc::machine_commit(lock, &commit_args, output) {
        Ok(hash) => Ok(Some(hash)),
        Err(e) => {
            // Revert the working-tree gallops to HEAD and unstage it, so a blocked commit leaves
            // the store untouched. (Tool-internal git restoring its own uncommitted write — the
            // same gesture jjx_open's convergence uses after an over-budget commit.)
            let _ = vvc::vvce_git_command(&["checkout", "HEAD", "--", path_str.as_str()]).output();
            let _ = vvc::vvce_git_command(&["reset", "--quiet", "--", path_str.as_str()]).output();
            Err(e)
        }
    }
}
