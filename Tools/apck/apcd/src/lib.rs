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

pub mod apcrl_log;
pub mod apcre_engine;
pub mod apcrp_parse;
pub mod apcrm_match;
pub mod apcrd_dictionaries;
pub mod apcru_update;

#[cfg(test)] mod apcte_engine;
#[cfg(test)] mod apctp_parse;
#[cfg(test)] mod apctm_match;
#[cfg(test)] mod apctd_dictionaries;
