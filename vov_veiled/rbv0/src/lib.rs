// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBV Hierophant — the veiled delivery-ceremony conductor. Library root.
//
// The hierophant conducts the delivery ceremony as the theurge conducts the
// tests (RBSHC). It owns four commands split on the one load-bearing seam,
// reversible vs irreversible: essai (the repair lap), docimasy (the reveal's
// reversible proving act), ostend (the reveal's irreversible showing), and
// harbinger (the stranger walk). This crate is withheld from delivery by
// directory construction; it may therefore cite the closed spec
// (RBSHE/RBSHC/RBSHD/RBSHO/RBSHH, rbshc_ quoins) directly, which shipped
// source may never do.

#![allow(non_camel_case_types)]
#![deny(warnings)]

pub mod rbvgeb_breviary;
pub mod rbvcel_log;
pub mod rbvcer_run;
pub mod rbvcep_repo;
pub mod rbvcem_perambulation;
pub mod rbvceu_loupe;
pub mod rbvcex_expede;
pub mod rbvceg_temenos;
pub mod rbvcet_cachet;
pub mod rbvcce_essai;
pub mod rbvccd_docimasy;
pub mod rbvcco_ostend;
pub mod rbvcch_harbinger;

#[cfg(test)]
mod rbvtem_perambulation;
#[cfg(test)]
mod rbvteu_loupe;
#[cfg(test)]
mod rbvtet_cachet;
