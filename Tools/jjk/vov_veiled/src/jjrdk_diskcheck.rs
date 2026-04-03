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

struct DiskEntry {
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

/// Check disk space. Returns `Ok(survey)` with disk survey if all disks are
/// below threshold, or `Err(message)` with diagnostic if any qualifying disk
/// is critical. The survey is always computed for observability.
pub fn jjrdk_check_disk_space() -> Result<String, String> {
    let disks = sysinfo::Disks::new_with_refreshed_list();

    // Deduplicate by (total, available) — APFS volumes sharing a container
    // report identical values. BTreeMap gives deterministic output order.
    let mut seen: BTreeMap<(u64, u64), DiskEntry> = BTreeMap::new();

    for disk in disks.list() {
        let total = disk.total_space();
        let available = disk.available_space();
        if total < MIN_DISK_BYTES { continue; }
        let used_pct = if total > 0 { (total - available) as f64 / total as f64 * 100.0 } else { 0.0 };
        seen.entry((total, available)).or_insert_with(|| DiskEntry {
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            used_pct,
            available,
            total,
        });
    }

    // One-line survey: threshold + percent per unique disk
    let disk_entries: Vec<String> = seen.values()
        .map(|v| format!("{}: {:.0}%", zjjrdk_format_bytes(v.total), v.used_pct))
        .collect();
    let survey = format!("Disk survey (threshold {:.0}%): {}",
        FULL_THRESHOLD_PCT,
        if disk_entries.is_empty() { "no qualifying disks".to_string() } else { disk_entries.join(", ") });

    let violations: Vec<&DiskEntry> = seen.values()
        .filter(|v| v.used_pct >= FULL_THRESHOLD_PCT)
        .collect();

    if violations.is_empty() {
        return Ok(survey);
    }

    let mut msg = String::from("DISK SPACE CRITICAL — Job Jockey refusing to proceed.\n\n");
    for v in &violations {
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
    msg.push('\n');
    msg.push_str(&survey);

    Err(msg)
}
