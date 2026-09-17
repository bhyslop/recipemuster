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

//! Bash Kennel Master — the library face.
//!
//! The kennel is a library and a binary, and every door-level chokepoint is this
//! library driven from that binary. The split is load-bearing rather than
//! structural tidiness: the two hold DIFFERENT POSTURES, and only a split can
//! express that. The door refuses an uncommitted repository so every record maps
//! to a position; the library refuses nothing, because a consumer holds its own
//! posture — one such consumer reads cargo metadata in-process on exactly the
//! dirty tree its own census is driven against, before every commit.
//!
//! The regime reader stands beside this crate as `bkl` and is linked rather
//! than reimplemented: every collar this crate reads is read through it, and the
//! catena law is honored in exactly one place in rust.
//!
//! Two consumers stand already. A census tool's manifest corroboration read is
//! one, and it reaches cargo here — the stopgap membrane that stood ahead of
//! this crate named this library as its removal condition and is struck. The
//! collar validate door's closure read is the other, and it arrives with the
//! roster.
//!
//! THE LIBRARY HAS TWO FACES AND ONE DISCIPLINE: `bkcl_drive` hands the child
//! this process's streams, `bkcl_recall` takes them, and both compose the
//! invocation in the one interior place, so neither can reach cargo with the
//! discipline half-applied. A consumer that wants cargo's answer as data takes
//! the recall; one that wants an operator to watch a compiler work takes the
//! drive.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

// SIX OF THESE MODULES ARE BLANK, one per letter the rollout has pre-allocated
// for a mechanism still to be built. They stand here rather than arriving with
// the paces that fill them because those paces run in parallel billets: a
// module line each of them added beside its neighbours' is a merge conflict at
// every refit, and a line each of them EDITS is not. Every blank carries a doc
// line saying what will fill it and nothing else, and its word is a guess its
// own pace may change.
pub mod bkca_whereabouts;
pub mod bkcc_record;
pub mod bkce_election;
pub mod bkcf_guard;
pub mod bkcg_gangline;
pub mod bkch_heel;
pub mod bkci_pipeline;
pub mod bkcj_json;
pub mod bkcl_leash;
pub mod bkcm_mush;
pub mod bkcn_delouse;
pub mod bkco_output;
pub mod bkcq_kibble;
pub mod bkcr_resolve;
pub mod bkcs_stamp;
pub mod bkct_tattoo;
pub mod bkcv_voice;
pub mod bkcx_python;
pub mod bkcy_sweep;
pub mod bkcz_muzzle;

// THE GENERATED BREVIARY, rendered whole and never hand-edited - the file's own
// head says so, and names no tool, because this crate ships and the render door
// does not: the set forms of the governed words this crate prints, held in
// usufruct from the substrate, which has no Rust target of its own. It seats at
// the `bk` grain rather than under `bkc` because it is no module of the
// kennel's own discipline - a hand has no business in it - on the vok kit's
// precedent for its generated file.
pub mod bkg_breviary;

// A TWIN APIECE FOR THE BLANKS ABOVE, and for the same reason: the hurdle file
// a pace will fill is opened here so no two paces author a line beside each
// other.
#[cfg(test)]
mod bkta_whereabouts;
#[cfg(test)]
mod bktb_substrate;
#[cfg(test)]
mod bktc_record;
#[cfg(test)]
mod bkte_election;
#[cfg(test)]
mod bktf_guard;
#[cfg(test)]
mod bktg_gangline;
#[cfg(test)]
mod bkth_heel;
#[cfg(test)]
mod bkti_pipeline;
#[cfg(test)]
mod bktj_json;
#[cfg(test)]
mod bktl_leash;
#[cfg(test)]
mod bktm_mush;
#[cfg(test)]
mod bktn_delouse;
#[cfg(test)]
mod bktq_kibble;
#[cfg(test)]
mod bktr_resolve;
#[cfg(test)]
mod bkts_stamp;
#[cfg(test)]
mod bktt_tattoo;
#[cfg(test)]
mod bktv_voice;
#[cfg(test)]
mod bktw_whistle;
#[cfg(test)]
mod bktx_python;
#[cfg(test)]
mod bkty_sweep;
#[cfg(test)]
mod bktz_muzzle;

// THE COMPOSING SURFACE, AND THE ONE MODULE HERE THAT A CONSUMER MAY REACH. It
// is the only home for reaching the temp root, composing a repository or
// spawning the kennel (BKSLR-Lure.adoc "The Lure"), and the substrate's own
// suite composes seats through it, so the surface has to cross a crate boundary
// that `cfg(test)` alone cannot: a dependent's build never sets it.
//
// THE FEATURE IS WHAT KEEPS IT OUT OF THE BINARY. No binary build of this crate
// names it, and a consumer that does takes this crate as a dev-dependency, which
// cargo compiles into a test target and never into a binary — so the exergue
// election's carve-out of src/bkt* (Tools/bkk/bk0/bkce_roots.txt) states what it
// has always stated: nothing under it can change the built artifact.
#[cfg(any(test, feature = "lure"))]
pub mod bktu_lure;

// eof
