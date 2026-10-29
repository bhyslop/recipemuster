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

//! VVK Parcel Release Utilities
//!
//! Provides asset collection and the record for VVK parcel creation.
//!
//! - `bper_collect`: Enumerate kit files, copy to staging (git-tracked), and
//!   write the forge's behest beside where the record will stand
//! - `bper_require_tracked_kits`: Refuse a declared kit set any member of which
//!   stands at the forge as no tracked folder, naming every such kit at once
//! - `bper_off_trunk`: Whether the forge's main checkout stands off its trunk,
//!   the one trunk reading the release and the forge's survey both ask
//! - `bper_issue_tamga`: Allocate the next release number, write the record
//! - `bper_target_name`: Name the target a parcel is cut for, from its `burc.env`
//! - `bper_git_tracked_files`: List git-tracked files under a root — the
//!   generalized candidate-gate primitive shared consumers (a name census
//!   among them) compose with their own allowlist.
//! - `bper_git_untracked_files`: The tracked walk's complement — files that
//!   stand on disk, are not in the index, and are not ignored. What a
//!   census built on the tracked walk could not reach.
//!
//! Veiled content is excluded by location, not by a predicate. The walk roots
//! at `Tools/{kit}` and the veiling precinct stands at the repository root, so
//! nothing veiled is in the collected set to begin with — there is no veiled
//! path to recognize, and no gate to keep in step with the convention.
//!
//! Collection gates on the git-tracked set, not on a raw filesystem walk.
//! A walk collects whatever stands on the forge's disk, so untracked build
//! output rides the parcel and vvmc_commit does not determine the bytes the
//! record names. It is also what lets freight be staged rather than collected:
//! a release copies the census binary and the word table under the
//! carrying kit after this walk runs, and neither stands tracked in the forge,
//! so no walk of the tracked set can reach the tool's source.

use std::collections::BTreeMap;
use std::fs;
use std::path::{
    Path,
    PathBuf, //
};

use bkp::bkprs_scrivener::bkprs_copy;

use crate::bpee_emplace::{
    bpee_Behest,
    bpee_Signets, //
};

// =============================================================================
// Constants
// =============================================================================

/// Starting release number for new registries
const BPER_TAMGA_NUMBER_START: u32 = 1000;

/// Record JSON field tags
const BPER_TAMGA_NUMBER: &str = "vvmt_tamga";
const BPER_TAMGA_DATE: &str = "vvmd_date";
const BPER_TAMGA_COMMIT: &str = "vvmc_commit";
const BPER_TAMGA_FORGE: &str = "vvmf_forge";
const BPER_TAMGA_KITS: &str = "vvmk_kits";
const BPER_TAMGA_VULGATE: &str = "vvmv_vulgate";
const BPER_TAMGA_TARGET: &str = "vvmr_target";

/// Registry JSON field
const BPER_REGISTRY_TAMGAS: &str = "bper_tamgas";

/// A registry row's members: when the release number was taken, and which forge took it.
const BPER_ROW_DATE: &str = "date";
const BPER_ROW_FORGE: &str = "forge";

/// The remote whose default line names a forge's trunk.
const ZBPER_REMOTE: &str = "origin";

/// The bootstraps' names at a parcel's root, whatever path they were copied
/// from.
const ZBPER_INSTALL_BOOTSTRAP: &str = "bpi_install.sh";
const ZBPER_UNINSTALL_BOOTSTRAP: &str = "bpu_uninstall.sh";

// =============================================================================
// Public Types
// =============================================================================

/// One kit a forge may distribute: its identifier, the directory name it is
/// collected from, and the cipher its Claude assets wear.
#[derive(Debug, Clone)]
pub struct bper_Kit {
    pub id: String,
    pub cipher: String,
}

/// The kits a forge may distribute, and the signets their Claude assets wear
/// after the cipher. Handed in by the caller, whose registry is the one home
/// of both: collection refuses any kit the roster does not name.
#[derive(Debug, Clone)]
pub struct bper_Roster {
    pub kits: Vec<bper_Kit>,
    pub signets: bpee_Signets,
}

/// Result of asset collection.
#[derive(Debug)]
pub struct bper_CollectResult {
    /// Number of files copied per kit
    pub kit_counts: BTreeMap<String, u32>,
    /// Total files copied
    pub total_files: u32,
    /// Commands routed to commands/ directory
    pub commands_routed: u32,
}

/// Result of issuing a record.
#[derive(Debug)]
pub struct bper_TamgaResult {
    /// Allocated release number
    pub number: u32,
}

/// Registry entry for a release number.
///
/// A row records that a release number is taken, when, and by which forge. It does not
/// describe the parcel: a parcel's identity is the commit it was built from,
/// which the record carries as `vvmc_commit`. A row whose only purpose is
/// to reserve a number for another estate is an ordinary row naming that
/// estate as its forge, indistinguishable from any other and needing no
/// sentinel to be one.
#[derive(Debug, Clone)]
pub struct bper_RegistryEntry {
    pub date: String,
    /// The forge's own `BURC_ENDONYM`, never defaulted
    pub forge: String,
}

// =============================================================================
// Public Functions
// =============================================================================

/// Collect kit assets to staging directory.
///
/// # Arguments
/// * `tools_dir` - Source Tools/ directory
/// * `staging_dir` - Target staging directory
/// * `install_script_path` - Path to bpi_install.sh to copy to staging root
/// * `uninstall_script_path` - Path to bpu_uninstall.sh to copy to staging root
/// * `managed_kits` - The kits the target declares (its BURC_MANAGED_KITS),
///   each of which must stand tracked under `tools_dir`
/// * `roster` - The kits the forge may distribute, and their assets' signets
/// * `behest` - What the install and the vacate need that only the forge
///   knows, composed by the forge for this target's declared kits and naming
///   nothing a kit it does not declare carries, written at the staging root as
///   the parcel's behest
///
/// Both bootstraps ride the parcel root, and the uninstall one is what makes
/// removal reachable for every kit set: the parcel is the one seat that holds
/// a bootstrap, the install binary and the behest for every target.
///
/// # Returns
/// CollectResult with file counts, or error message
pub fn bper_collect(
    tools_dir: &Path,
    staging_dir: &Path,
    install_script_path: &Path,
    uninstall_script_path: &Path,
    managed_kits: &[String],
    roster: &bper_Roster,
    behest: &bpee_Behest,
) -> Result<bper_CollectResult, String> {
    let mut kit_counts = BTreeMap::new();
    let mut total_files = 0u32;
    let mut commands_routed = 0u32;

    // The kit forge root, resolved from the tools directory rather than
    // assumed to be the process CWD. The launcher does cd to the project
    // root, but a collection that silently depends on that has no way to
    // fail when it stops being true — it just collects from somewhere else.
    let forge_root = bper_forge_root(tools_dir)?;

    // Exactly the declared set or nothing: every declared kit stands tracked
    // before any is looked up or copied, so no partial parcel is begun.
    bper_require_tracked_kits(tools_dir, managed_kits)?;

    // Create staging directories
    let kits_dir = staging_dir.join("kits");
    fs::create_dir_all(&kits_dir).map_err(|e| {
        format!(
            "Failed to create kits dir: {}",
            e, //
        )
    })?;

    // Process only kits in managed_kits list
    for kit_id in managed_kits {
        let kit = roster
            .kits
            .iter()
            .find(|k| k.id == *kit_id)
            .ok_or_else(|| {
                format!(
                    "Kit '{}' not found in registry",
                    kit_id, //
                )
            })?;

        let kit_source = tools_dir.join(kit_id);
        let kit_staging = kits_dir.join(kit_id);

        let (count, cmd_count) =
            zbper_collect_kit(&kit_source, &kit_staging, &kit.cipher, &roster.signets)?;
        kit_counts.insert(kit_id.clone(), count);
        total_files += count;
        commands_routed += cmd_count;

        // Public claude-{kit}-*.md guidance files ride along via the normal kit
        // walk above (they live in the kit root, which is the collected set). No
        // separate template copy: CLAUDE.md content is @-included, never inlined.

        // Collect Claude config assets (commands and hooks) from kit forge
        let (cmds, hks) =
            zbper_collect_claude_assets(&forge_root, staging_dir, &kit.cipher, &roster.signets)?;
        total_files += cmds + hks;
        commands_routed += cmds;
    }

    // Copy both bootstraps to staging root. The uninstall one is staged
    // whatever the kit set, for the same reason the install binary is: the
    // parcel is the one seat that exists for every consumer, and the vacate
    // reads the behest only a parcel carries.
    let install_dest = staging_dir.join(ZBPER_INSTALL_BOOTSTRAP);
    bkprs_copy(install_script_path, &install_dest).map_err(|e| {
        format!(
            "Failed to copy install script: {}",
            e, //
        )
    })?;
    total_files += 1;

    let uninstall_dest = staging_dir.join(ZBPER_UNINSTALL_BOOTSTRAP);
    bkprs_copy(uninstall_script_path, &uninstall_dest).map_err(|e| {
        format!(
            "Failed to copy uninstall script: {}",
            e, //
        )
    })?;
    total_files += 1;

    // THE BEHEST, beside where the record will stand and never inside it: the
    // record says what the parcel IS and travels into every target, while the
    // behest says what the forge bids the install do and stays in the parcel.
    let behest_dest = staging_dir.join(crate::bpee_emplace::BPEE_BEHEST_FILE);
    let behest_json = crate::bpee_emplace::bpee_behest_json(behest)?;
    let behest_body = serde_json::to_string_pretty(&behest_json).map_err(|e| {
        format!(
            "Failed to compose the behest: {}",
            e, //
        )
    })?;
    fs::write(&behest_dest, behest_body + "\n").map_err(|e| {
        format!(
            "Failed to write {}: {}",
            behest_dest.display(),
            e, //
        )
    })?;
    total_files += 1;

    Ok(bper_CollectResult {
        kit_counts,
        total_files,
        commands_routed,
    })
}

/// Issue the record over a staging directory: its release number and metadata.
///
/// # Arguments
/// * `staging_dir` - Staging directory with collected assets
/// * `registry_path` - Path to vofr_registry.json
/// * `commit_sha` - Current git commit SHA
/// * `vulgate_sha` - The corpus trunk commit the word table was read at, where
///   the parcel carries the table; none where its target declares no `vmk`
/// * `forge` - The minting repository's own `BURC_ENDONYM`, recorded in the
///   record and on the registry row alike
/// * `target` - The target the parcel was cut for (`bper_target_name`)
/// * `managed_kits` - List of kit IDs included (the target's BURC_MANAGED_KITS)
///
/// # Returns
/// TamgaResult with the release number, or error message
///
/// Allocation is monotonic and unconditional: every release takes the next
/// release number, writes a registry row, and leaves the caller to publish it. A release number
/// names an allocation rather than a content set, so nothing is looked up
/// before one is handed out. What a parcel was built from is `vvmc_commit`,
/// which git determines exactly and which the record carries downstream.
pub fn bper_issue_tamga(
    staging_dir: &Path,
    registry_path: &Path,
    commit_sha: &str,
    vulgate_sha: Option<&str>,
    forge: &str,
    target: &str,
    managed_kits: &[String],
) -> Result<bper_TamgaResult, String> {
    // No row is written for a forge that names itself nowhere.
    if forge.is_empty() {
        return Err("A release needs the forge that mints it, its own BURC_ENDONYM".to_string());
    }

    // Load or create registry
    let mut registry = zbper_load_registry(registry_path)?;

    let number = zbper_allocate_tamga_number(&registry);

    // Generate timestamp
    let date = zbper_generate_date();

    registry.insert(
        number,
        bper_RegistryEntry {
            date: date.clone(),
            forge: forge.to_string(),
        },
    );
    zbper_save_registry(registry_path, &registry)?;

    // Write record with the target's declared kits as the kit list
    let tamga_path = staging_dir.join(crate::bpee_emplace::BPEE_TAMGA_FILE);
    zbper_write_tamga(
        &tamga_path,
        &bper_tamga_json(
            number,
            &date,
            commit_sha,
            vulgate_sha,
            forge,
            target,
            managed_kits,
        )?,
    )?;

    Ok(bper_TamgaResult { number })
}

/// Name the target a parcel is cut for: the directory its `burc.env`'s
/// `BURC_PROJECT_ROOT` resolves to, as the sheaves identify a target
/// repository.
///
/// Read through the install's own parser, so the release and the install can
/// never resolve one declaration to two different repositories.
pub fn bper_target_name(burc_path: &Path) -> Result<String, String> {
    let burc = crate::bpee_emplace::bpee_parse_burc(burc_path)?;
    burc.project_root
        .file_name()
        .and_then(|n| n.to_str())
        .map(String::from)
        .ok_or_else(|| {
            format!(
                "The target's project root names no directory: {}",
                burc.project_root.display(), //
            )
        })
}

/// Load registry from JSON file.
pub fn bper_load_registry(path: &Path) -> Result<BTreeMap<u32, bper_RegistryEntry>, String> {
    zbper_load_registry(path)
}

/// List every git-tracked file under `repo_root`, relative to it.
///
/// The candidate-gate primitive. It shells to git rather than
/// walking the filesystem and re-deriving tracked status from `.gitignore`:
/// tracked-ness is git's answer to give, and the index is what makes a
/// parcel's bytes follow from the commit that names them.
pub fn bper_git_tracked_files(repo_root: &Path) -> Result<Vec<PathBuf>, String> {
    use std::process::Command;

    let output = Command::new("git")
        .arg("ls-files")
        .current_dir(repo_root)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git ls-files: {}",
                e, //
            )
        })?;

    if !output.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&output.stderr), //
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(PathBuf::from).collect())
}

/// List every file under `repo_root` that stands on disk, is absent from the
/// index, and is not ignored — the tracked walk's complement, relative to the
/// same root.
///
/// The boundary is the INDEX, not HEAD: `git ls-files` reads the index, so a
/// staged-but-uncommitted file is already tracked and does not appear here.
/// A reader that says "not yet committed" says something this function does
/// not mean, and would send an operator to the wrong remedy — the remedy is
/// `git add`, and staging alone discharges it.
///
/// Ignored files are excluded, which is the whole reason this is git's answer
/// rather than a filesystem walk. Build output and scratch stand on disk in
/// exactly the shapes an allowlist admits; a raw walk would name every one of
/// them, and a notice that names noise is read past on its second appearance.
pub fn bper_git_untracked_files(repo_root: &Path) -> Result<Vec<PathBuf>, String> {
    use std::process::Command;

    let output = Command::new("git")
        .arg("ls-files")
        .arg("--others")
        .arg("--exclude-standard")
        .current_dir(repo_root)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git ls-files --others: {}",
                e, //
            )
        })?;

    if !output.status.success() {
        return Err(format!(
            "git ls-files --others failed: {}",
            String::from_utf8_lossy(&output.stderr), //
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(PathBuf::from).collect())
}

/// Refuse a declared kit set any member of which has no tracked file beneath
/// `tools_dir` under its own name, naming every such kit in the one refusal.
///
/// A release carries exactly the kits its target declares or refuses, whether
/// the forge authored a kit or received it. A folder standing on disk with
/// nothing tracked in it collects nothing, so it is refused exactly as an absent
/// one is. The tracked set is the index's, as the collection's own walk is.
pub fn bper_require_tracked_kits(tools_dir: &Path, managed_kits: &[String]) -> Result<(), String> {
    // Run from inside the tools directory, git ls-files reports paths relative
    // to it, so a kit's tracked files are the paths under its own name.
    let tracked = bper_git_tracked_files(tools_dir)?;
    let missing: Vec<String> = managed_kits
        .iter()
        .filter(|kit| !tracked.iter().any(|path| path.starts_with(kit.as_str())))
        .map(|kit| {
            format!(
                "'{}'",
                kit, //
            )
        })
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "The target declares {} kit(s) the forge holds no tracked folder for under {}: {} - a release carries exactly the kits its target declares, so no parcel is cut",
        missing.len(),
        tools_dir.display(),
        missing.join(", "), //
    ))
}

/// Whether the forge's main checkout stands off its trunk: `None` where it
/// stands on the line its origin names as its default, `Some` with the reason
/// and its remedy where it stands detached, records no default line, or stands
/// on another line. The main checkout is the one `forge` belongs to, so a
/// linked worktree asks about the checkout it hangs from.
///
/// The one home of the trunk reading: the release asks it before it cuts, and
/// the forge's survey asks it before the minting half opens. Each judging
/// creed of the release reads whatever branch stands, so this alone asks that
/// it be the trunk.
///
/// It writes nothing. `Err` is a reading git would not run for, and is never a
/// pass.
pub fn bper_off_trunk(forge: &Path) -> Result<Option<String>, String> {
    let main = zbper_git_read(
        forge,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?
    .and_then(|common| Path::new(&common).parent().map(Path::to_path_buf));
    let main = match main {
        None => {
            return Ok(Some(format!(
                "{} stands in no git checkout — drive this from the forge's main checkout",
                forge.display(), //
            )));
        }
        Some(main) => main,
    };
    let line = zbper_git_read(&main, &["symbolic-ref", "--quiet", "--short", "HEAD"])?;
    let origin_head = format!(
        "refs/remotes/{}/HEAD",
        ZBPER_REMOTE, //
    );
    let remote_prefix = format!(
        "{}/",
        ZBPER_REMOTE, //
    );
    let trunk = zbper_git_read(&main, &["symbolic-ref", "--quiet", "--short", &origin_head])?.map(
        |named| {
            named
                .strip_prefix(&remote_prefix)
                .map(str::to_string)
                .unwrap_or(named)
        },
    );
    Ok(match (line, trunk) {
        (None, _) => Some(format!(
            "the forge's main checkout at {} stands detached — check out its trunk",
            main.display(), //
        )),
        (Some(_), None) => Some(format!(
            "the forge's main checkout at {} records no default line for its origin, so its trunk cannot be read — set it with `git remote set-head {} --auto`",
            main.display(),
            ZBPER_REMOTE, //
        )),
        (Some(line), Some(trunk)) if line != trunk => Some(format!(
            "the forge's main checkout at {} stands on '{}', not on its trunk '{}' — check out the trunk",
            main.display(),
            line,
            trunk, //
        )),
        _ => None,
    })
}

/// Resolve the kit forge repository root from any path inside it.
///
/// Collection and the record both need the forge root, and neither may take
/// it from the process CWD: the launcher's cd to the project root is a
/// convention, and a convention that is silently depended on cannot fail
/// loud when it stops holding.
pub fn bper_forge_root(inside: &Path) -> Result<PathBuf, String> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(inside)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git rev-parse --show-toplevel: {}",
                e, //
            )
        })?;

    if !output.status.success() {
        return Err(format!(
            "Not inside a git repository ({}): {}",
            inside.display(),
            String::from_utf8_lossy(&output.stderr).trim(), //
        ));
    }

    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if root.is_empty() {
        return Err("git rev-parse --show-toplevel returned nothing".to_string());
    }

    Ok(PathBuf::from(root))
}

// =============================================================================
// Internal Functions (zbper_*)
// =============================================================================

/// One git read for the trunk reading: `Some` with what git printed, `None`
/// where git answered no or printed nothing, and `Err` only where git would not
/// run.
fn zbper_git_read(root: &Path, args: &[&str]) -> Result<Option<String>, String> {
    use std::process::Command;

    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git {:?} in {}: {}",
                args,
                root.display(),
                e, //
            )
        })?;
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok((output.status.success() && !text.is_empty()).then_some(text))
}

/// Collect a single kit's assets.
fn zbper_collect_kit(
    source: &Path,
    staging: &Path,
    cipher: &str,
    signets: &bpee_Signets,
) -> Result<(u32, u32), String> {
    let mut count = 0u32;
    let mut cmd_count = 0u32;

    // The git-tracked set under the kit, not a filesystem walk. git ls-files
    // run with its CWD inside a work tree scopes itself to that directory and
    // reports CWD-relative paths, so this yields exactly the rel_path shape
    // the loop below already consumed.
    let tracked = bper_git_tracked_files(source)?;

    for rel_path in tracked {
        let rel_path = rel_path.as_path();
        let entry = source.join(rel_path);

        // Determine destination
        let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        let dest = if zbper_is_command_file(file_name, cipher, signets) {
            // Route commands to commands/ subdirectory
            cmd_count += 1;
            let commands_dir = staging.join("commands");
            fs::create_dir_all(&commands_dir).map_err(|e| {
                format!(
                    "Failed to create commands dir: {}",
                    e, //
                )
            })?;
            commands_dir.join(file_name)
        } else {
            // Default: preserve relative path
            staging.join(rel_path)
        };

        // Create parent directories
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create dir {}: {}",
                    parent.display(),
                    e, //
                )
            })?;
        }

        // Copy file
        bkprs_copy(&entry, &dest).map_err(|e| {
            format!(
                "Failed to copy {} to {}: {}",
                entry.display(),
                dest.display(),
                e, //
            )
        })?;

        count += 1;
    }

    Ok((count, cmd_count))
}

/// Collect Claude config assets for a single kit.
/// Scans .claude/commands/ and .claude/hooks/ at the kit forge root for
/// cipher-matched files, gated on the git-tracked set exactly as the kit
/// walk is — an untracked scratch command is not deliverable content.
/// Returns (commands_collected, hooks_collected).
fn zbper_collect_claude_assets(
    kit_forge_root: &Path,
    staging: &Path,
    cipher: &str,
    signets: &bpee_Signets,
) -> Result<(u32, u32), String> {
    let mut commands_count = 0u32;
    let mut hooks_count = 0u32;

    // Collect commands from .claude/commands/
    let commands_source = kit_forge_root.join(".claude").join("commands");
    if commands_source.exists() {
        let commands_dest = staging.join("claude").join("commands");
        fs::create_dir_all(&commands_dest).map_err(|e| {
            format!(
                "Failed to create claude/commands dir: {}",
                e, //
            )
        })?;

        commands_count =
            zbper_copy_matching_files(&commands_source, &commands_dest, cipher, signets, true)?;
    }

    // Collect hooks from .claude/hooks/
    let hooks_source = kit_forge_root.join(".claude").join("hooks");
    if hooks_source.exists() {
        let hooks_dest = staging.join("claude").join("hooks");
        fs::create_dir_all(&hooks_dest).map_err(|e| {
            format!(
                "Failed to create claude/hooks dir: {}",
                e, //
            )
        })?;

        hooks_count =
            zbper_copy_matching_files(&hooks_source, &hooks_dest, cipher, signets, false)?;
    }

    Ok((commands_count, hooks_count))
}

/// Copy git-tracked files matching cipher pattern from source to dest.
/// is_command determines whether to match commands (c-) or hooks (h-).
/// Returns count of files copied.
fn zbper_copy_matching_files(
    source_dir: &Path,
    dest_dir: &Path,
    cipher: &str,
    signets: &bpee_Signets,
    is_command: bool,
) -> Result<u32, String> {
    let mut count = 0u32;

    for rel_path in bper_git_tracked_files(source_dir)? {
        let path = source_dir.join(&rel_path);

        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        let matches = if is_command {
            zbper_is_command_file(file_name, cipher, signets)
        } else {
            zbper_is_hook_file(file_name, cipher, signets)
        };

        if matches {
            let dest_path = dest_dir.join(file_name);
            bkprs_copy(&path, &dest_path).map_err(|e| {
                format!(
                    "Failed to copy {} to {}: {}",
                    path.display(),
                    dest_path.display(),
                    e, //
                )
            })?;
            count += 1;
        }
    }

    Ok(count)
}

/// Check if file is a command file (matches {cipher}c-*.md pattern).
fn zbper_is_command_file(file_name: &str, cipher: &str, signets: &bpee_Signets) -> bool {
    let prefix = format!(
        "{}{}",
        cipher,
        signets.command, //
    );
    file_name.starts_with(&prefix) && file_name.ends_with(".md")
}

/// Check if file is a hook file (matches {cipher}h-*.md pattern).
fn zbper_is_hook_file(file_name: &str, cipher: &str, signets: &bpee_Signets) -> bool {
    let prefix = format!(
        "{}{}",
        cipher,
        signets.hook, //
    );
    file_name.starts_with(&prefix) && file_name.ends_with(".md")
}

/// Load registry from JSON file.
fn zbper_load_registry(path: &Path) -> Result<BTreeMap<u32, bper_RegistryEntry>, String> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }

    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Failed to read registry: {}",
            e, //
        )
    })?;

    let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        format!(
            "Failed to parse registry JSON: {}",
            e, //
        )
    })?;

    let numbers = json
        .get(BPER_REGISTRY_TAMGAS)
        .and_then(|v| v.as_object())
        .ok_or_else(|| {
            format!(
                "Registry missing '{}' object",
                BPER_REGISTRY_TAMGAS, //
            )
        })?;

    let mut registry = BTreeMap::new();

    for (key, value) in numbers {
        let number: u32 = key.parse().map_err(|e| {
            format!(
                "Invalid number key '{}': {}",
                key,
                e, //
            )
        })?;

        let date = value
            .get(BPER_ROW_DATE)
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                format!(
                    "Row {} missing '{}'",
                    number,
                    BPER_ROW_DATE, //
                )
            })?
            .to_string();

        // A row naming no forge is refused, never read as the forge's own:
        // every row names the forge that took it.
        let forge = value
            .get(BPER_ROW_FORGE)
            .and_then(|v| v.as_str())
            .filter(|f| !f.is_empty())
            .ok_or_else(|| {
                format!(
                    "Row {} missing '{}'",
                    number,
                    BPER_ROW_FORGE, //
                )
            })?
            .to_string();

        registry.insert(number, bper_RegistryEntry { date, forge });
    }

    Ok(registry)
}

/// Save registry to JSON file.
fn zbper_save_registry(
    path: &Path,
    registry: &BTreeMap<u32, bper_RegistryEntry>,
) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&bper_registry_json(registry)).map_err(|e| {
        format!(
            "Failed to serialize registry: {}",
            e, //
        )
    })?;

    fs::write(path, json).map_err(|e| {
        format!(
            "Failed to write registry: {}",
            e, //
        )
    })?;

    Ok(())
}

/// Compose the registry's document, writing nothing.
///
/// PUBLIC for the census's syncrisis reading, on the record's own ground:
/// the registry is a conserved record, and its wire is proved against the
/// spelling the corpus declares only by a reading that holds what this
/// function emits.
pub fn bper_registry_json(registry: &BTreeMap<u32, bper_RegistryEntry>) -> serde_json::Value {
    let mut numbers = serde_json::Map::new();

    for (number, entry) in registry {
        let mut entry_obj = serde_json::Map::new();
        entry_obj.insert(
            BPER_ROW_DATE.to_string(),
            serde_json::Value::String(entry.date.clone()),
        );
        entry_obj.insert(
            BPER_ROW_FORGE.to_string(),
            serde_json::Value::String(entry.forge.clone()),
        );
        numbers.insert(number.to_string(), serde_json::Value::Object(entry_obj));
    }

    let mut root = serde_json::Map::new();
    root.insert(
        BPER_REGISTRY_TAMGAS.to_string(),
        serde_json::Value::Object(numbers),
    );
    serde_json::Value::Object(root)
}

/// Allocate next sequential release number.
fn zbper_allocate_tamga_number(registry: &BTreeMap<u32, bper_RegistryEntry>) -> u32 {
    registry
        .keys()
        .max()
        .map(|m| m + 1)
        .unwrap_or(BPER_TAMGA_NUMBER_START)
}

/// Generate date string in YYMMDD-HHMM format.
fn zbper_generate_date() -> String {
    use std::process::Command;

    let output = Command::new("date")
        .args(["+%y%m%d-%H%M"])
        .output()
        .expect("Failed to run date command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Write a composed record to staging.
fn zbper_write_tamga(path: &Path, record: &serde_json::Value) -> Result<(), String> {
    let json = serde_json::to_string_pretty(record).map_err(|e| {
        format!(
            "Failed to serialize record: {}",
            e, //
        )
    })?;

    fs::write(path, json).map_err(|e| {
        format!(
            "Failed to write record: {}",
            e, //
        )
    })?;

    Ok(())
}

/// Compose the record's document, writing nothing.
///
/// PUBLIC so the census's syncrisis reading can drive the very composition
/// the release writes: the record is a conserved record, and its wire is
/// proved against the spellings the corpus declares only by a reading that
/// holds what this function emits rather than a list typed beside it.
pub fn bper_tamga_json(
    number: u32,
    date: &str,
    commit: &str,
    vulgate: Option<&str>,
    forge: &str,
    target: &str,
    kits: &[String],
) -> Result<serde_json::Value, String> {
    let mut root = serde_json::Map::new();

    // The release number is a 4-digit identifier, not a quantity: it serializes as text
    // and the registry keys it as text too.
    // Arithmetic on it belongs to allocation alone.
    root.insert(
        BPER_TAMGA_NUMBER.to_string(),
        serde_json::Value::String(number.to_string()),
    );
    root.insert(
        BPER_TAMGA_DATE.to_string(),
        serde_json::Value::String(date.to_string()),
    );
    root.insert(
        BPER_TAMGA_COMMIT.to_string(),
        serde_json::Value::String(commit.to_string()),
    );

    // Every forge allocates its numbers alone, so a record names the forge that
    // took it, spelled as that forge declares itself and never derived.
    if forge.is_empty() {
        return Err("A record needs the forge that minted it".to_string());
    }
    root.insert(
        BPER_TAMGA_FORGE.to_string(),
        serde_json::Value::String(forge.to_string()),
    );

    // A parcel carrying the word table is cut from a PAIR of positions, the
    // forge's commit above and the corpus trunk's here, and a reader asking what
    // a crossing carried needs both. A parcel whose target declares no vmk
    // carries no table, so it names no second position.
    if let Some(vulgate) = vulgate {
        if vulgate.is_empty() {
            return Err(
                "A record needs the corpus position the word table was read at".to_string(),
            );
        }
        root.insert(
            BPER_TAMGA_VULGATE.to_string(),
            serde_json::Value::String(vulgate.to_string()),
        );
    }

    // Every parcel is cut for one target, so no record stands without naming it.
    if target.is_empty() {
        return Err("A record needs the target the parcel was cut for".to_string());
    }
    root.insert(
        BPER_TAMGA_TARGET.to_string(),
        serde_json::Value::String(target.to_string()),
    );

    let kit_values: Vec<serde_json::Value> = kits
        .iter()
        .map(|k| serde_json::Value::String(k.clone()))
        .collect();
    root.insert(
        BPER_TAMGA_KITS.to_string(),
        serde_json::Value::Array(kit_values),
    );

    Ok(serde_json::Value::Object(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The signets the tests' assets wear, spelled here because the roster's
    /// own home is its caller's registry.
    fn zbper_test_signets() -> bpee_Signets {
        bpee_Signets {
            command: "c-".to_string(),
            hook: "h-".to_string(),
        }
    }

    /// The behest the tests' forge hands its collection, naming no seat.
    fn zbper_test_behest() -> bpee_Behest {
        bpee_Behest {
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbper_test_signets(),
            binary: Some(crate::bpee_emplace::bpee_Binary {
                kit: "vvk".to_string(),
                canonical: "vvx".to_string(),
                suffixed_prefix: "vvx-".to_string(),
            }),
            seat_endonym: None,
        }
    }

    #[test]
    fn bper_is_command_file_test() {
        let signets = zbper_test_signets();
        assert!(zbper_is_command_file("jjc-heat-groom.md", "jj", &signets));
        assert!(zbper_is_command_file("vvc-commit.md", "vv", &signets));
        assert!(!zbper_is_command_file("jju_utility.sh", "jj", &signets));
        assert!(!zbper_is_command_file("README.md", "jj", &signets));
        assert!(!zbper_is_command_file("jjc-heat-groom.txt", "jj", &signets));
    }

    #[test]
    fn bper_git_tracked_files_test() {
        // The crate's own manifest is always git-tracked in this repo.
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let tracked = bper_git_tracked_files(&repo_root).expect("git ls-files failed");
        assert!(tracked
            .iter()
            .any(|p| p == Path::new("Tools/bpk/bpe/Cargo.toml")));
    }

    /// A tree with no `.claude/` directory at all — a fresh kit-split clone
    /// the asset move has never reached — must collect cleanly rather than
    /// refuse: nothing tracked to collect is an honest zero, announced via
    /// `commands_routed`, never a reason to bar the whole collection.
    #[test]
    fn bper_collect_succeeds_without_claude_dir_test() {
        use std::process::Command;

        let temp_dir = std::env::temp_dir().join("bper_test_no_claude_dir");
        let _ = fs::remove_dir_all(&temp_dir);

        let tools_dir = temp_dir.join("tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/buc_command.sh"), b"# stub").unwrap();

        let install_script = temp_dir.join("bpi_install.sh");
        fs::write(&install_script, b"# stub install").unwrap();
        let uninstall_script = temp_dir.join("bpu_uninstall.sh");
        fs::write(&uninstall_script, b"# stub uninstall").unwrap();

        let staging_dir = temp_dir.join("staging");

        let run = |args: &[&str]| {
            let output = Command::new("git")
                .args(args)
                .current_dir(&temp_dir)
                .output()
                .expect("git invocation failed");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr), //
            );
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "test@example.com"]);
        run(&["config", "user.name", "Test"]);
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "initial"]);

        let claude_dir = temp_dir.join(".claude");
        assert!(!claude_dir.is_dir());

        let result = bper_collect(
            &tools_dir,
            &staging_dir,
            &install_script,
            &uninstall_script,
            &["buk".to_string()],
            &bper_Roster {
                kits: vec![bper_Kit {
                    id: "buk".to_string(),
                    cipher: "bu".to_string(),
                }],
                signets: zbper_test_signets(),
            },
            &zbper_test_behest(),
        )
        .expect("collect must succeed with no .claude/ directory at all");

        assert_eq!(result.commands_routed, 0);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// A release carries exactly the target's declared kits or refuses: a kit
    /// with no tracked file under the forge's tools directory is refused,
    /// whether it stands nowhere or stands on disk untracked, and every such
    /// kit is named in the one refusal. The roster names every declared kit, so
    /// the refusal is the tracked check's and never the roster lookup's.
    #[test]
    fn bper_collect_refuses_every_declared_kit_not_tracked_test() {
        use std::process::Command;

        let temp_dir = std::env::temp_dir().join("bper_test_untracked_kits");
        let _ = fs::remove_dir_all(&temp_dir);

        let tools_dir = temp_dir.join("tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/buc_command.sh"), b"# stub").unwrap();

        let install_script = temp_dir.join("bpi_install.sh");
        fs::write(&install_script, b"# stub install").unwrap();
        let uninstall_script = temp_dir.join("bpu_uninstall.sh");
        fs::write(&uninstall_script, b"# stub uninstall").unwrap();

        let run = |args: &[&str]| {
            let output = Command::new("git")
                .args(args)
                .current_dir(&temp_dir)
                .output()
                .expect("git invocation failed");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr), //
            );
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "test@example.com"]);
        run(&["config", "user.name", "Test"]);
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "initial"]);

        // On disk after the commit, so the folder stands and nothing in it is tracked.
        fs::create_dir_all(tools_dir.join("jjk")).unwrap();
        fs::write(tools_dir.join("jjk/jjw_workbench.sh"), b"# stub").unwrap();

        let roster = bper_Roster {
            kits: vec![
                bper_Kit {
                    id: "buk".to_string(),
                    cipher: "bu".to_string(),
                },
                bper_Kit {
                    id: "cmk".to_string(),
                    cipher: "cm".to_string(),
                },
                bper_Kit {
                    id: "jjk".to_string(),
                    cipher: "jj".to_string(),
                },
            ],
            signets: zbper_test_signets(),
        };

        let refused = bper_collect(
            &tools_dir,
            &temp_dir.join("staging_refused"),
            &install_script,
            &uninstall_script,
            &["buk".to_string(), "cmk".to_string(), "jjk".to_string()],
            &roster,
            &zbper_test_behest(),
        )
        .expect_err("a declared kit with no tracked folder must refuse the collection");
        assert!(
            refused.contains("'cmk'"),
            "the absent kit is named: {}",
            refused, //
        );
        assert!(
            refused.contains("'jjk'"),
            "the untracked kit is named: {}",
            refused, //
        );
        assert!(
            !refused.contains("'buk'"),
            "the tracked kit is not named: {}",
            refused, //
        );

        let admitted = bper_collect(
            &tools_dir,
            &temp_dir.join("staging_admitted"),
            &install_script,
            &uninstall_script,
            &["buk".to_string()],
            &roster,
            &zbper_test_behest(),
        )
        .expect("a declared set standing tracked whole is admitted");
        assert_eq!(admitted.kit_counts.get("buk"), Some(&1));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// The trunk reading passes a forge whose main checkout stands on the line
    /// its origin names as its default, and refuses one standing on a stray
    /// line at the same commit, naming the stray line and the trunk.
    #[test]
    fn bper_off_trunk_refuses_a_stray_line_naming_it_and_the_trunk_test() {
        use std::process::Command;

        let temp_dir = std::env::temp_dir().join("bper_test_off_trunk");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(temp_dir.join("README.md"), b"# stub").unwrap();

        let run = |args: &[&str]| {
            let output = Command::new("git")
                .args(args)
                .current_dir(&temp_dir)
                .output()
                .expect("git invocation failed");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr), //
            );
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "test@example.com"]);
        run(&["config", "user.name", "Test"]);
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "initial"]);
        run(&["checkout", "-q", "-B", "trunkline"]);
        run(&["update-ref", "refs/remotes/origin/trunkline", "HEAD"]);
        run(&[
            "symbolic-ref",
            "refs/remotes/origin/HEAD",
            "refs/remotes/origin/trunkline",
        ]);

        assert_eq!(
            bper_off_trunk(&temp_dir).expect("git runs"),
            None,
            "the forge on its trunk passes", //
        );

        run(&["checkout", "-q", "-B", "strayline"]);
        let off = bper_off_trunk(&temp_dir)
            .expect("git runs")
            .expect("the forge on a stray line is off its trunk");
        assert!(
            off.contains("'strayline'") && off.contains("'trunkline'"),
            "the reason names both lines: {}",
            off, //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// The record carries both positions a parcel is cut from, under their
    /// registered field tags, and refuses a record that cannot name the second.
    #[test]
    fn bper_tamga_records_both_positions_test() {
        let temp_dir = std::env::temp_dir().join("bper_test_tamga_positions");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        let path = temp_dir.join(crate::bpee_emplace::BPEE_TAMGA_FILE);

        let record = bper_tamga_json(
            1099,
            "260922-1200",
            "abc123",
            Some("def456"),
            "buon_quorge",
            "seat",
            &["vmk".to_string()],
        )
        .expect("a record naming both positions must compose");
        zbper_write_tamga(&path, &record).expect("and write");
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(json[BPER_TAMGA_COMMIT], "abc123");
        assert_eq!(json[BPER_TAMGA_VULGATE], "def456");
        assert_eq!(json[BPER_TAMGA_KITS][0], "vmk");

        assert!(bper_tamga_json(
            1099,
            "260922-1200",
            "abc123",
            Some(""),
            "buon_quorge",
            "seat",
            &[]
        )
        .is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Every record names the target its parcel was cut for, and refuses one that
    /// cannot; a parcel carrying no word table names no corpus position at all.
    #[test]
    fn bper_tamga_records_its_target_test() {
        let temp_dir = std::env::temp_dir().join("bper_test_tamga_target");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        let path = temp_dir.join(crate::bpee_emplace::BPEE_TAMGA_FILE);

        let kits = ["buk".to_string(), "jjk".to_string(), "vvk".to_string()];
        let record = bper_tamga_json(
            1099,
            "260928-1200",
            "abc123",
            None,
            "buon_quorge",
            "consumer",
            &kits,
        )
        .expect("a record naming its target and no table must compose");
        zbper_write_tamga(&path, &record).expect("and write");
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(json[BPER_TAMGA_TARGET], "consumer");
        assert!(
            json.get(BPER_TAMGA_VULGATE).is_none(),
            "a parcel carrying no table names no corpus position", //
        );
        assert_eq!(json[BPER_TAMGA_KITS].as_array().unwrap().len(), 3);

        assert!(bper_tamga_json(
            1099,
            "260928-1200",
            "abc123",
            None,
            "buon_quorge",
            "",
            &kits
        )
        .is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// The target's name is the directory its declaration's project root
    /// resolves to, read through the install's own parser.
    #[test]
    fn bper_target_name_is_the_resolved_project_root_test() {
        let temp_dir = std::env::temp_dir().join("bper_test_target_name");
        let _ = fs::remove_dir_all(&temp_dir);
        let repo = temp_dir.join("consumer");
        fs::create_dir_all(repo.join(".buk")).unwrap();
        fs::create_dir_all(repo.join("Tools")).unwrap();
        let burc = repo.join(".buk/burc.env");
        fs::write(
            &burc,
            "BURC_TOOLS_DIR=Tools\nBURC_PROJECT_ROOT=..\nBURC_MANAGED_KITS=buk,jjk,vvk\n",
        )
        .unwrap();

        assert_eq!(
            bper_target_name(&burc).expect("a declared target names itself"),
            "consumer"
        );
        assert!(bper_target_name(&temp_dir.join("absent/burc.env")).is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn bper_tamga_number_allocation_test() {
        let empty: BTreeMap<u32, bper_RegistryEntry> = BTreeMap::new();
        assert_eq!(zbper_allocate_tamga_number(&empty), BPER_TAMGA_NUMBER_START);

        let mut registry = BTreeMap::new();
        registry.insert(
            1000,
            bper_RegistryEntry {
                date: "260117-1400".to_string(),
                forge: "buon_quorge".to_string(),
            },
        );
        assert_eq!(zbper_allocate_tamga_number(&registry), 1001);

        // Allocation steps past a gap rather than filling it. A row reserving
        // a number for another estate is an ordinary row, so the reservation
        // needs nothing of its own to be stepped over.
        registry.insert(
            1005,
            bper_RegistryEntry {
                date: "260117-1500".to_string(),
                forge: "buon_zentorge".to_string(),
            },
        );
        assert_eq!(zbper_allocate_tamga_number(&registry), 1006);
    }

    /// A record names the forge that minted it, spelled as handed in, and a record
    /// that cannot name one refuses rather than writing a forge nobody declared.
    #[test]
    fn bper_tamga_names_its_forge_test() {
        let temp_dir = std::env::temp_dir().join("bper_test_tamga_forge");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        let path = temp_dir.join(crate::bpee_emplace::BPEE_TAMGA_FILE);

        let record = bper_tamga_json(
            1099,
            "261022-1200",
            "abc123",
            None,
            "buon_quorge",
            "consumer",
            &["buk".to_string()],
        )
        .expect("a record naming its forge must compose");
        zbper_write_tamga(&path, &record).expect("and write");
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(json[BPER_TAMGA_FORGE], "buon_quorge");
        assert_eq!(json[BPER_TAMGA_NUMBER], "1099");

        assert!(bper_tamga_json(1099, "261022-1200", "abc123", None, "", "consumer", &[]).is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// An allocation writes a registry row naming the forge that took it beside
    /// the record naming the same forge; a release naming no forge writes no
    /// row, and a registry carrying a row that names none refuses to load.
    #[test]
    fn bper_registry_allocation_names_its_forge_test() {
        let temp_dir = std::env::temp_dir().join("bper_test_registry_forge");
        let _ = fs::remove_dir_all(&temp_dir);
        let staging = temp_dir.join("staging");
        fs::create_dir_all(&staging).unwrap();
        let registry_path = temp_dir.join("registry.json");
        let mut seeded = BTreeMap::new();
        seeded.insert(
            1050,
            bper_RegistryEntry {
                date: "260803-0855".to_string(),
                forge: "buon_zentorge".to_string(),
            },
        );
        zbper_save_registry(&registry_path, &seeded).unwrap();

        let kits = ["buk".to_string()];
        assert!(bper_issue_tamga(
            &staging,
            &registry_path,
            "abc123",
            None,
            "",
            "consumer",
            &kits
        )
        .is_err());
        assert_eq!(
            zbper_load_registry(&registry_path).unwrap().len(),
            1,
            "a release naming no forge writes no row", //
        );

        let result = bper_issue_tamga(
            &staging,
            &registry_path,
            "abc123",
            None,
            "buon_quorge",
            "consumer",
            &kits,
        )
        .expect("an allocation naming its forge");
        assert_eq!(result.number, 1051);
        let registry = zbper_load_registry(&registry_path).unwrap();
        assert_eq!(registry[&1051].forge, "buon_quorge");
        assert_eq!(
            registry[&1050].forge, "buon_zentorge",
            "a reservation keeps the estate that claims it", //
        );
        let record: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(staging.join(crate::bpee_emplace::BPEE_TAMGA_FILE)).unwrap(),
        )
        .unwrap();
        assert_eq!(record[BPER_TAMGA_FORGE], "buon_quorge");

        let mut rows = bper_registry_json(&registry);
        rows[BPER_REGISTRY_TAMGAS]["1050"]
            .as_object_mut()
            .unwrap()
            .remove(BPER_ROW_FORGE);
        fs::write(&registry_path, serde_json::to_string_pretty(&rows).unwrap()).unwrap();
        assert!(
            zbper_load_registry(&registry_path).is_err(),
            "a row naming no forge is never defaulted", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: the release writes the behest beside the record and never
    /// inside it. A collection and an issue over one staging directory leave
    /// both at its root; the behest reads back as the forge composed it, and
    /// the record carries none of its members.
    #[test]
    fn bper_release_writes_the_behest_beside_the_record_and_never_inside_it_test() {
        use std::process::Command;

        let temp_dir = std::env::temp_dir().join("bper_test_behest_beside");
        let _ = fs::remove_dir_all(&temp_dir);
        let tools_dir = temp_dir.join("tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/buc_command.sh"), b"# stub").unwrap();
        let install_script = temp_dir.join("bpi_install.sh");
        fs::write(&install_script, b"# stub install").unwrap();
        let uninstall_script = temp_dir.join("bpu_uninstall.sh");
        fs::write(&uninstall_script, b"# stub uninstall").unwrap();
        for args in [
            &["init", "-q"][..],
            &["config", "user.email", "test@example.com"],
            &["config", "user.name", "Test"],
            &["add", "-A"],
            &["commit", "-q", "-m", "initial"],
        ] {
            let output = Command::new("git")
                .args(args)
                .current_dir(&temp_dir)
                .output()
                .expect("git invocation failed");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr), //
            );
        }

        let staging = temp_dir.join("staging");
        let behest = zbper_test_behest();
        let kits = ["buk".to_string()];
        bper_collect(
            &tools_dir,
            &staging,
            &install_script,
            &uninstall_script,
            &kits,
            &bper_Roster {
                kits: vec![bper_Kit {
                    id: "buk".to_string(),
                    cipher: "bu".to_string(),
                }],
                signets: zbper_test_signets(),
            },
            &behest,
        )
        .expect("the collection writes the behest");
        bper_issue_tamga(
            &staging,
            &temp_dir.join("registry.json"),
            "abc123",
            None,
            "buon_quorge",
            "consumer",
            &kits,
        )
        .expect("the issue writes its file");

        let read = |name: &str| -> serde_json::Value {
            serde_json::from_str(&fs::read_to_string(staging.join(name)).unwrap()).unwrap()
        };
        let written = read(crate::bpee_emplace::BPEE_BEHEST_FILE);
        assert_eq!(
            written,
            crate::bpee_emplace::bpee_behest_json(&behest).unwrap(),
            "the behest is the forge's own", //
        );
        let record = read(crate::bpee_emplace::BPEE_TAMGA_FILE);
        for member in written.as_object().unwrap().keys() {
            assert!(
                record.get(member).is_none(),
                "the issued file carries the behest's member '{}': {}",
                member,
                record, //
            );
        }
        assert!(
            staging.join(ZBPER_INSTALL_BOOTSTRAP).is_file()
                && staging.join(ZBPER_UNINSTALL_BOOTSTRAP).is_file()
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
