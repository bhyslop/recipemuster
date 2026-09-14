// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHD Hierophant — the veiled delivery-ceremony conductor. Library root.
//
// The hierophant conducts the delivery ceremony as the theurge conducts the
// tests (RBSHC). It owns four commands split on the one load-bearing seam,
// reversible vs irreversible: essai (the repair lap), docimasy (the reveal's
// reversible proving act), ostend (the reveal's irreversible showing), and
// harbinger (the stranger rig). This crate is withheld from delivery by
// directory construction; it may therefore cite the closed spec
// (RBSHE/RBSHC/RBSHD/RBSHO/RBSHH, rbth_ quoins) directly, which shipped
// source may never do.

#![allow(non_camel_case_types)]
#![deny(warnings)]

pub mod rbthdr_log;
pub mod rbthdr_run;
pub mod rbthdr_repo;
pub mod rbthdr_perambulation;
pub mod rbthdr_loupe;
pub mod rbthdr_expede;
pub mod rbthdr_rig;
pub mod rbthdr_cachet;
pub mod rbthdr_essai;
pub mod rbthdr_docimasy;
pub mod rbthdr_ostend;
pub mod rbthdr_harbinger;

#[cfg(test)]
mod rbthdt_perambulation;
#[cfg(test)]
mod rbthdt_loupe;
#[cfg(test)]
mod rbthdt_cachet;
