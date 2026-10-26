// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCET — cachet: the durable verdict the docimasy grants and the ostend
// requires (RBS0 rbshc_cachet; RBSHD "Grant the cachet"; RBSHO "Require the
// cachet"). A small crate module, not a worker: stored beside the regulus
// (a sibling of the clone, in the regulus's parent directory — the same
// location seposition already sweeps, so seposing a regulus seposes
// its cachet with it for free), keyed to the regulus's tree hash and marked
// with the world it was granted in (RBSHC "The election"), attesting the
// regulus was proved against exactly these bytes — in the agora, by a green
// gauntlet. A re-cut regulus carries none, and a cachet granted in one world
// admits no showing in the other: the tree hash cannot part the worlds (a
// practice regulus cut from the same HEAD atop a stand-in seeded at public
// main carries the real regulus's tree), so the recorded world is the byte
// that refuses.

use std::path::Path;

use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvcer_run;
use crate::rbvcew_world;
use crate::rbvgeb_breviary;

/// The cachet's fixed basename, a sibling of the regulus clone. Interior —
/// hearting, minted at this pace's mount under the crate's rbvce_ pattern.
const RBVCET_CACHET_FILE: &str = "cachet.env";

const RBVCET_CACHET_KEY_TREE: &str = "RBTHDR_CACHET_TREE";
const RBVCET_CACHET_KEY_TIP: &str = "RBTHDR_CACHET_TIP";
const RBVCET_CACHET_KEY_MAINTAINER_HEAD: &str = "RBTHDR_CACHET_MAINTAINER_HEAD";
const RBVCET_CACHET_KEY_STAMP: &str = "RBTHDR_CACHET_STAMP";
/// The fifth custody carrier (RBSHC "Custody carriers"), conserved from the
/// first practice cachet granted: its value is the world's word.
const RBVCET_CACHET_KEY_WORLD: &str = "RBTHDR_CACHET_WORLD";

/// The granted fact: the regulus's tree hash and tip, the maintainer HEAD
/// that cut it, the stamp, and the world it was granted in (RBSHD "Grant the
/// cachet").
pub struct rbvcet_Cachet {
    pub tree: String,
    pub tip: String,
    pub maintainer_head: String,
    pub stamp: String,
    pub world: rbvcew_world::rbvcew_World,
}

/// Render a cachet to its on-disk key=value form. Pure — tested without git
/// or the filesystem.
pub fn rbvcet_render(cachet: &rbvcet_Cachet) -> String {
    format!(
        "{}={}\n{}={}\n{}={}\n{}={}\n{}={}\n",
        RBVCET_CACHET_KEY_TREE, cachet.tree,
        RBVCET_CACHET_KEY_TIP, cachet.tip,
        RBVCET_CACHET_KEY_MAINTAINER_HEAD, cachet.maintainer_head,
        RBVCET_CACHET_KEY_STAMP, cachet.stamp,
        RBVCET_CACHET_KEY_WORLD, rbvcew_world::rbvcew_word(cachet.world),
    )
}

/// Parse a cachet's on-disk key=value form back to its fields. Pure — tested
/// with synthetic content, including malformed input. Every field is
/// required and non-empty: a partially-written cachet must refuse, never be
/// read as a verdict for whichever fields happen to be present.
pub fn rbvcet_parse(content: &str) -> Result<rbvcet_Cachet, String> {
    let mut tree = None;
    let mut tip = None;
    let mut maintainer_head = None;
    let mut stamp = None;
    let mut world = None;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("cachet line is not key=value: '{}'", line))?;
        match key {
            RBVCET_CACHET_KEY_TREE => tree = Some(value.to_string()),
            RBVCET_CACHET_KEY_TIP => tip = Some(value.to_string()),
            RBVCET_CACHET_KEY_MAINTAINER_HEAD => maintainer_head = Some(value.to_string()),
            RBVCET_CACHET_KEY_STAMP => stamp = Some(value.to_string()),
            RBVCET_CACHET_KEY_WORLD => world = Some(value.to_string()),
            other => return Err(format!("cachet carries an unknown key: '{}'", other)),
        }
    }
    let tree = tree.filter(|v| !v.is_empty()).ok_or_else(|| format!("cachet missing {}", RBVCET_CACHET_KEY_TREE))?;
    let tip = tip.filter(|v| !v.is_empty()).ok_or_else(|| format!("cachet missing {}", RBVCET_CACHET_KEY_TIP))?;
    let maintainer_head = maintainer_head
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("cachet missing {}", RBVCET_CACHET_KEY_MAINTAINER_HEAD))?;
    let stamp = stamp.filter(|v| !v.is_empty()).ok_or_else(|| format!("cachet missing {}", RBVCET_CACHET_KEY_STAMP))?;
    let world = world
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("{} missing {}", rbvgeb_breviary::RBVGEB_CACHET_BASE, RBVCET_CACHET_KEY_WORLD))?;
    let world = rbvcew_world::rbvcew_from_word(&world)
        .ok_or_else(|| format!("{} names an unknown world: '{}'", rbvgeb_breviary::RBVGEB_CACHET_BASE, world))?;
    Ok(rbvcet_Cachet { tree, tip, maintainer_head, stamp, world })
}

/// The refusals, pure: a cachet recorded in a world other than the elected
/// one, in either direction, and a recorded tree hash other than the standing
/// regulus's current one. `pub(crate)` so the self-proofs (rbvtet_cachet) can
/// call the real refusal logic directly, without git or the filesystem.
pub(crate) fn zrbvcet_check(
    cachet: &rbvcet_Cachet,
    standing_tree: &str,
    elected: rbvcew_world::rbvcew_World,
) -> Result<(), String> {
    if cachet.world != elected {
        return Err(format!(
            "{} recorded in the {} admits no showing in the {}",
            rbvgeb_breviary::RBVGEB_CACHET_BASE_LETTRINE,
            rbvcew_world::rbvcew_word(cachet.world),
            rbvcew_world::rbvcew_word(elected)
        ));
    }
    if cachet.tree != standing_tree {
        return Err(format!(
            "cachet tree {} does not match the standing regulus's tree {} — a re-cut regulus carries no cachet",
            cachet.tree, standing_tree
        ));
    }
    Ok(())
}

fn zrbvcet_path(regulus_parent: &Path) -> std::path::PathBuf {
    regulus_parent.join(RBVCET_CACHET_FILE)
}

/// Grant a cachet beside the regulus: capture the regulus's tree hash and
/// tip, the maintainer HEAD, the stamp, and the elected world, and store it
/// (RBSHD "Grant the cachet"). Fatal on any read or write failure.
pub fn rbvcet_grant(regulus_parent: &Path, regulus_clone: &Path, top: &Path, world: rbvcew_world::rbvcew_World) {
    let cachet = rbvcet_Cachet {
        tree: rbvcep_repo::rbvcep_tree_hash(regulus_clone, top),
        tip: rbvcep_repo::rbvcep_commit_sha(regulus_clone, top),
        maintainer_head: rbvcep_repo::rbvcep_commit_sha(top, top),
        stamp: rbvcer_run::rbvcer_timestamp(top),
        world,
    };
    let path = zrbvcet_path(regulus_parent);
    std::fs::write(&path, rbvcet_render(&cachet))
        .unwrap_or_else(|e| crate::rbvcel_fatal!("failed to write the cachet {}: {}", path.display(), e));
    rbvcel_log::rbvcel_line(&format!("cachet granted: {} (tree {})", path.display(), cachet.tree));
}

/// Require a standing cachet beside the regulus, granted in the elected world
/// and its tree hash equal to the standing regulus's (RBSHO "Require the
/// cachet"). Fatal on absence, a world apart, or mismatch, naming the remedy:
/// conduct the docimasy.
pub fn rbvcet_require(
    regulus_parent: &Path,
    regulus_clone: &Path,
    top: &Path,
    world: rbvcew_world::rbvcew_World,
) -> rbvcet_Cachet {
    let path = zrbvcet_path(regulus_parent);
    let content = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        crate::rbvcel_fatal!(
            "no cachet standing beside the regulus ({}) — conduct the docimasy first (RBSHO)",
            path.display()
        )
    });
    let cachet = rbvcet_parse(&content).unwrap_or_else(|e| crate::rbvcel_fatal!("cachet {} is malformed: {}", path.display(), e));
    let standing_tree = rbvcep_repo::rbvcep_tree_hash(regulus_clone, top);
    if let Err(e) = zrbvcet_check(&cachet, &standing_tree, world) {
        crate::rbvcel_fatal!("{} — conduct the docimasy again (RBSHO)", e);
    }
    rbvcel_log::rbvcel_line(&format!("cachet required and verified: {} (tree {})", path.display(), cachet.tree));
    cachet
}
