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

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

//! Fixture loader — writes synthetic Epic clipboard HTML for testing.
//! Usage: cargo run --bin apcal [fixture-file]

// RCG output discipline: all emission via apcrl_*! — no direct println!/eprintln!

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        apcd::apcrl_fatal_now!("usage: apcal <fixture-file-path>");
    }
    let path = &args[1];

    let html = std::fs::read_to_string(path)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("failed to read {}: {}", path, e));

    let mut clipboard = arboard::Clipboard::new()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("failed to open clipboard: {}", e));

    clipboard.set_html(&html, Some(&html))
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("failed to set clipboard HTML: {}", e));

    apcd::apcrl_info_now!("loaded {} onto clipboard as HTML ({} bytes)", path, html.len());
}
