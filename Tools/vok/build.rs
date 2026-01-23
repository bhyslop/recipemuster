// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

// build.rs - Kit auto-detection for VOK
//
// Detects which kits have veiled/ Rust code and enables corresponding features.
// This allows the vvr binary to include kit-specific functionality when available.

use std::path::Path;

fn main() {
    // Auto-detect kit veiled/ directories with Cargo.toml
    // Each detected kit enables a feature flag

    let kits = [
        ("jjk", "../jjk/vov_veiled/Cargo.toml"),
        // Add more kits here as they gain Rust code
    ];

    for (kit_name, cargo_path) in kits {
        if Path::new(cargo_path).exists() {
            // Enable the feature at compile time via rustc cfg
            println!("cargo:rustc-cfg=feature=\"{}\"", kit_name);
            println!("cargo:warning=Detected {} Rust code, enabling feature", kit_name);
        }
    }

    // Re-run if any kit's Cargo.toml appears or disappears
    for (_, cargo_path) in kits {
        println!("cargo:rerun-if-changed={}", cargo_path);
    }
}
