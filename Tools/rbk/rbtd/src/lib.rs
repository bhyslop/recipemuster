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

#![allow(non_camel_case_types)]
#![allow(private_interfaces)]
#![deny(unused_variables)]

pub mod rbtdrc_crucible;
pub mod rbtdrd_dummy;
pub mod rbtdre_engine;
pub mod rbtdrf_fast;
pub mod rbtdri_invocation;
pub mod rbtdrm_manifest;

#[cfg(test)]
mod rbtdte_engine;
#[cfg(test)]
mod rbtdti_invocation;
#[cfg(test)]
mod rbtdtm_manifest;
