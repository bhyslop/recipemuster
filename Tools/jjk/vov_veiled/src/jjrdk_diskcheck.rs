// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Disk space guard — blocks JJK operations when disk is critically full.
//!
//! Checks all mounted filesystems with capacity > 10 GB. If any is >= 85% full,
//! returns a hard error with actionable diagnostics. No override mechanism.
//!
//! APFS deduplication: multiple APFS volumes sharing a container report identical
//! (total, available) pairs. We deduplicate by that tuple, showing one diagnostic
//! line per unique pair with a representative mount point.

use std::collections::BTreeMap;

const MIN_DISK_BYTES: u64 = 10 * 1024 * 1024 * 1024; // 10 GB
const FULL_THRESHOLD_PCT: f64 = 85.0;

struct DiskViolation {
    mount_point: String,
    used_pct:    f64,
    available:   u64,
    total:       u64,
}

fn zjjrdk_format_bytes(bytes: u64) -> String {
    const TB: u64 = 1024 * 1024 * 1024 * 1024;
    const GB: u64 = 1024 * 1024 * 1024;
    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else {
        format!("{:.0} GB", bytes as f64 / GB as f64)
    }
}

/// Check disk space. Returns `Ok(())` if all disks are below threshold,
/// or `Err(message)` with full diagnostic if any qualifying disk is critical.
pub fn jjrdk_check_disk_space() -> Result<(), String> {
    let disks = sysinfo::Disks::new_with_refreshed_list();

    // Deduplicate by (total, available) — APFS volumes sharing a container
    // report identical values. BTreeMap gives deterministic output order.
    let mut seen: BTreeMap<(u64, u64), DiskViolation> = BTreeMap::new();

    for disk in disks.list() {
        let total = disk.total_space();
        if total < MIN_DISK_BYTES {
            continue;
        }
        let available = disk.available_space();
        let used_pct = (total - available) as f64 / total as f64 * 100.0;
        if used_pct < FULL_THRESHOLD_PCT {
            continue;
        }
        let key = (total, available);
        seen.entry(key).or_insert_with(|| DiskViolation {
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            used_pct,
            available,
            total,
        });
    }

    if seen.is_empty() {
        return Ok(());
    }

    let mut msg = String::from("DISK SPACE CRITICAL — Job Jockey refusing to proceed.\n\n");
    for v in seen.values() {
        msg.push_str(&format!(
            "  {}: {:.1}% full ({} free of {})\n",
            v.mount_point,
            v.used_pct,
            zjjrdk_format_bytes(v.available),
            zjjrdk_format_bytes(v.total),
        ));
    }
    msg.push_str("\nFree space before retrying. Quick wins:\n");
    msg.push_str("  docker container prune       # remove stopped containers\n");
    msg.push_str("  docker image prune            # remove dangling images\n");
    msg.push_str("  docker system prune -a        # remove ALL unused images (will need re-pull)\n");

    Err(msg)
}
