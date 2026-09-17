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

//! Literatim — the estate's regime reader.
//!
//! A regime file is read TWICE: by bash when the file is sourced, and by hand
//! when a reader outside a shell parses it for itself. The substrate states that
//! law once, for every family and every reader; this crate is the second reader,
//! and its whole promise is that the two answer alike.
//!
//! THE LAW IS NOT CITED BY NAME ANYWHERE IN THIS KIT, and the silence is
//! deliberate rather than an omission: the kit is delivered as source and the
//! document that states the law is not, so naming it here would carry a name out
//! to consumers who hold nothing it points at.
//!
//! IT ADMITS THE STATED SUBSET AND REFUSES EVERYTHING ELSE, naming the line. The
//! subset is deliberately narrower than bash accepts, and the narrowness is the
//! design: a foreign reader that guessed at single quotes, unquoted values
//! carrying whitespace, `export` prefixes, command substitution or variable
//! references would become a second and divergent implementation of a shell, and
//! the divergence would surface as one file meaning two things. Refusing is what
//! keeps the two readers honest, and skipping a line silently is the one thing
//! this crate may never do.
//!
//! IT IS A CRATE OF ITS OWN, standing beside the kennel rather than inside it
//! (BKSNC-Kennelcraft.adoc "The Rust Backend"). Six hand-rolled readers of
//! regime files stand across the estate, each admitting a different subset; the
//! kennel's collar reader would have been the seventh. It is the kennel's first
//! consumer and not its only intended one, so nothing here knows what a collar
//! is — the crate reads regime files and stops, and every question about what a
//! particular field MEANS belongs to the family that declared it.
//!
//! Open source and delivered in the kennel's own parcel, which is why the
//! license header stands on every file.

#![deny(warnings)]
#![allow(non_camel_case_types)]

pub mod bklrc_catena;

#[cfg(test)]
mod bkltc_catena;

// eof
