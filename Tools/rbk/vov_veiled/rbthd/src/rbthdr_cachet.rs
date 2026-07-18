// Copyright 2026 Scale Invariant, Inc.
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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — cachet: the durable verdict the docimasy grants and the ostend
// requires (RBS0 rbth_cachet; RBSHD "Grant the cachet"; RBSHO "Require the
// cachet"). A small crate module, not a worker: stored beside the candidate
// (a sibling of the clone, in the candidate's parent directory — the same
// location retire-aside already sweeps, so retiring a candidate aside
// retires its cachet with it for free), keyed to the candidate's tree hash,
// attesting the gauntlet ran green against exactly these bytes. A re-cut
// candidate carries none; a rehearsal never grants one.

use std::path::Path;

use crate::rbthdr_log;
use crate::rbthdr_repo;
use crate::rbthdr_run;

/// The cachet's fixed basename, a sibling of the candidate clone. Interior —
/// hearting, minted at this pace's mount under the crate's rbthdr_ pattern.
const RBTHDR_CACHET_FILE: &str = "cachet.env";

const RBTHDR_CACHET_KEY_TREE: &str = "RBTHDR_CACHET_TREE";
const RBTHDR_CACHET_KEY_TIP: &str = "RBTHDR_CACHET_TIP";
const RBTHDR_CACHET_KEY_MAINTAINER_HEAD: &str = "RBTHDR_CACHET_MAINTAINER_HEAD";
const RBTHDR_CACHET_KEY_STAMP: &str = "RBTHDR_CACHET_STAMP";

/// The granted fact: the candidate's tree hash and tip, the maintainer HEAD
/// that cut it, and the stamp (RBSHD "Grant the cachet").
pub struct rbthdr_Cachet {
    pub tree: String,
    pub tip: String,
    pub maintainer_head: String,
    pub stamp: String,
}

/// Render a cachet to its on-disk key=value form. Pure — tested without git
/// or the filesystem.
pub fn render(cachet: &rbthdr_Cachet) -> String {
    format!(
        "{}={}\n{}={}\n{}={}\n{}={}\n",
        RBTHDR_CACHET_KEY_TREE, cachet.tree,
        RBTHDR_CACHET_KEY_TIP, cachet.tip,
        RBTHDR_CACHET_KEY_MAINTAINER_HEAD, cachet.maintainer_head,
        RBTHDR_CACHET_KEY_STAMP, cachet.stamp,
    )
}

/// Parse a cachet's on-disk key=value form back to its fields. Pure — tested
/// with synthetic content, including malformed input. Every field is
/// required and non-empty: a partially-written cachet must refuse, never be
/// read as a verdict for whichever fields happen to be present.
pub fn parse(content: &str) -> Result<rbthdr_Cachet, String> {
    let mut tree = None;
    let mut tip = None;
    let mut maintainer_head = None;
    let mut stamp = None;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("cachet line is not key=value: '{}'", line))?;
        match key {
            RBTHDR_CACHET_KEY_TREE => tree = Some(value.to_string()),
            RBTHDR_CACHET_KEY_TIP => tip = Some(value.to_string()),
            RBTHDR_CACHET_KEY_MAINTAINER_HEAD => maintainer_head = Some(value.to_string()),
            RBTHDR_CACHET_KEY_STAMP => stamp = Some(value.to_string()),
            other => return Err(format!("cachet carries an unknown key: '{}'", other)),
        }
    }
    let tree = tree.filter(|v| !v.is_empty()).ok_or_else(|| format!("cachet missing {}", RBTHDR_CACHET_KEY_TREE))?;
    let tip = tip.filter(|v| !v.is_empty()).ok_or_else(|| format!("cachet missing {}", RBTHDR_CACHET_KEY_TIP))?;
    let maintainer_head = maintainer_head
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("cachet missing {}", RBTHDR_CACHET_KEY_MAINTAINER_HEAD))?;
    let stamp = stamp.filter(|v| !v.is_empty()).ok_or_else(|| format!("cachet missing {}", RBTHDR_CACHET_KEY_STAMP))?;
    Ok(rbthdr_Cachet { tree, tip, maintainer_head, stamp })
}

/// The tree-hash-mismatch refusal, pure: compare a recorded cachet's tree
/// hash against the standing candidate's current one. `pub(crate)` so the
/// self-proofs (rbthdt_cachet) can call the real refusal logic directly,
/// without git or the filesystem.
pub(crate) fn zrbthdr_check(cachet: &rbthdr_Cachet, standing_tree: &str) -> Result<(), String> {
    if cachet.tree != standing_tree {
        return Err(format!(
            "cachet tree {} does not match the standing candidate's tree {} — a re-cut candidate carries no cachet",
            cachet.tree, standing_tree
        ));
    }
    Ok(())
}

fn zrbthdr_path(candidate_parent: &Path) -> std::path::PathBuf {
    candidate_parent.join(RBTHDR_CACHET_FILE)
}

/// Grant a cachet beside the candidate: capture the candidate's tree hash and
/// tip, the maintainer HEAD, and the stamp, and store it (RBSHD "Grant the
/// cachet"). Fatal on any read or write failure.
pub fn grant(candidate_parent: &Path, candidate_clone: &Path, top: &Path) {
    let cachet = rbthdr_Cachet {
        tree: rbthdr_repo::tree_hash(candidate_clone, top),
        tip: rbthdr_repo::commit_sha(candidate_clone, top),
        maintainer_head: rbthdr_repo::commit_sha(top, top),
        stamp: rbthdr_run::timestamp(top),
    };
    let path = zrbthdr_path(candidate_parent);
    std::fs::write(&path, render(&cachet))
        .unwrap_or_else(|e| crate::rbthdr_fatal!("failed to write the cachet {}: {}", path.display(), e));
    rbthdr_log::line(&format!("cachet granted: {} (tree {})", path.display(), cachet.tree));
}

/// Require a standing cachet beside the candidate, its tree hash equal to the
/// standing candidate's (RBSHO "Require the cachet"). Fatal on absence or
/// mismatch, naming the remedy: conduct the docimasy.
pub fn require(candidate_parent: &Path, candidate_clone: &Path, top: &Path) -> rbthdr_Cachet {
    let path = zrbthdr_path(candidate_parent);
    let content = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        crate::rbthdr_fatal!(
            "no cachet standing beside the candidate ({}) — conduct the docimasy first (RBSHO)",
            path.display()
        )
    });
    let cachet = parse(&content).unwrap_or_else(|e| crate::rbthdr_fatal!("cachet {} is malformed: {}", path.display(), e));
    let standing_tree = rbthdr_repo::tree_hash(candidate_clone, top);
    if let Err(e) = zrbthdr_check(&cachet, &standing_tree) {
        crate::rbthdr_fatal!("{} — conduct the docimasy again (RBSHO)", e);
    }
    rbthdr_log::line(&format!("cachet required and verified: {} (tree {})", path.display(), cachet.tree));
    cachet
}

/// Require a standing cachet, TOLERATING absence with a loud warning — the
/// rehearse reading (RBSHO rehearse note: "tolerates an absent cachet with a
/// loud warning and stops before the disclosure line"). A cachet that IS
/// present but mismatched is still fatal — only absence is tolerated.
pub fn require_rehearse(candidate_parent: &Path, candidate_clone: &Path, top: &Path) -> Option<rbthdr_Cachet> {
    let path = zrbthdr_path(candidate_parent);
    if !path.is_file() {
        rbthdr_log::warn(&format!(
            "no cachet standing beside the candidate ({}) — rehearsal tolerates this and stops before the disclosure line",
            path.display()
        ));
        return None;
    }
    Some(require(candidate_parent, candidate_clone, top))
}
