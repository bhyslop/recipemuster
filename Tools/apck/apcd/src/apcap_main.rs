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
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// RCG output discipline: all emission via apcrl_*! — no direct println!/eprintln!

fn main() {
    apcd::apcrl_info_now!("starting Ann's PHI Clipbuddy");
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("tauri application error: {}", e));
}
