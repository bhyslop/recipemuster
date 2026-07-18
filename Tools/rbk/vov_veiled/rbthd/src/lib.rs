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
// RBTHD Hierophant — the veiled delivery-ceremony conductor. Library root.
//
// The hierophant conducts the delivery ceremony as the theurge conducts the
// tests (RBSHC). It owns three commands split on the one load-bearing seam,
// reversible vs irreversible: essai (the repair lap, built here), and — later
// paces — ostend (the guided reveal) and harbinger (the stranger rig). This
// crate is withheld from delivery by directory construction; it may therefore
// cite the closed spec (RBSHE/RBSHC, rbth_ quoins) directly, which shipped
// source may never do.

#![allow(non_camel_case_types)]
#![deny(warnings)]

pub mod rbthdr_log;
pub mod rbthdr_run;
pub mod rbthdr_repo;
pub mod rbthdr_rig;
pub mod rbthdr_essai;
