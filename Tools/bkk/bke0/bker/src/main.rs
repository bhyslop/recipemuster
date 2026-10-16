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

//! `bker` — the kennel's rust hello exemplar: a binary that greets, naming
//! itself, and the suite beside it.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

mod bkera_hello;

#[cfg(test)]
mod bkert_hello;

fn main() -> std::process::ExitCode {
    bkera_hello::bkera_main()
}

// eof
