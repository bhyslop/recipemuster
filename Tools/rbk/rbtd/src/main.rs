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
// RBTD Theurge — crucible test orchestrator for Recipe Bottle

use std::process::ExitCode;

/// Colophon strings theurge depends on.
/// Each must appear in the manifest passed from the bash zipper at launch.
const RBTD_REQUIRED_COLOPHONS: &[&str] = &[
    "rbw-cC", // crucible charge
    "rbw-cQ", // crucible quench
];

fn rbtd_verify_colophon_manifest(manifest: &str) -> Result<(), String> {
    for colophon in RBTD_REQUIRED_COLOPHONS {
        let found = manifest.split_whitespace().any(|token| token == *colophon);
        if !found {
            return Err(format!(
                "rbtd: colophon '{}' not found in zipper manifest",
                colophon
            ));
        }
    }
    Ok(())
}

fn rbtd_main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let manifest = args
        .get(1)
        .ok_or("rbtd: no colophon manifest argument — theurge must be launched via tabtarget")?;

    rbtd_verify_colophon_manifest(manifest)?;

    // Theurge operational logic goes here in later paces
    eprintln!("rbtd: colophon manifest verified, theurge not yet implemented");
    Err("rbtd: no test tier specified".to_string())
}

fn main() -> ExitCode {
    match rbtd_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("{}", msg);
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_accepts_valid_manifest() {
        let manifest = "rbw-PL rbw-gPI rbw-cC rbw-cQ rbw-Qf";
        assert!(rbtd_verify_colophon_manifest(manifest).is_ok());
    }

    #[test]
    fn verify_rejects_missing_colophon() {
        let manifest = "rbw-PL rbw-gPI rbw-cC rbw-Qf";
        let err = rbtd_verify_colophon_manifest(manifest).unwrap_err();
        assert!(err.contains("rbw-cQ"));
    }

    #[test]
    fn verify_rejects_empty_manifest() {
        let err = rbtd_verify_colophon_manifest("").unwrap_err();
        assert!(err.contains("rbw-cC"));
    }

    #[test]
    fn verify_no_partial_match() {
        let manifest = "rbw-cCC rbw-cQQ";
        let err = rbtd_verify_colophon_manifest(manifest).unwrap_err();
        assert!(err.contains("rbw-cC"));
    }
}
