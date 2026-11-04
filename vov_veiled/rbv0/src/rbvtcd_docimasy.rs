// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTCD — docimasy's narthex gate (RBSHD step 1): the freshness predicate
// admits an empty narthex and this cut's own preview and nothing else, and the
// not-fresh refusal carries the procedure's narthex steps in the palaestra
// alone — the console recreate sentence naming the practice narthex and the
// read proving it empty — while the agora's refusal carries none of them. The
// expected lines are the procedure's own, spelled here as the independent
// oracle.

use crate::rbvccd_docimasy::{
    zrbvccd_narthex_fresh,
    zrbvccd_narthex_refusal, //
};
use crate::rbvcew_world::rbvcew_World;
use crate::rbvcex_expede::RBVCEX_REGULUS_BRANCH;

/// The practice narthex, as the console shows it.
const ZRBVTCD_NARTHEX_SLUG: &str = "bhyslop/recipebottle-picadero-practice";

/// The console recreate sentence the procedure gives the practice narthex,
/// the governed word cited rather than spelled.
fn zrbvtcd_console() -> String {
    format!(
        "At the GitHub console, by hand, delete the practice {} {} and create it again, private and empty, with no README, ignore file or license.",
        crate::rbvgeb_breviary::RBVGEB_NARTHEX_BASE,
        ZRBVTCD_NARTHEX_SLUG, //
    )
}

/// The read proving the recreated narthex empty.
const ZRBVTCD_EMPTY_READ: &str =
    "git ls-remote git@github.com:bhyslop/recipebottle-picadero-practice.git";

const ZRBVTCD_TIP: &str = "1111111111111111111111111111111111111111";
const ZRBVTCD_OTHER: &str = "2222222222222222222222222222222222222222";

fn zrbvtcd_ref(sha: &str, name: &str) -> (String, String) {
    (sha.to_string(), name.to_string())
}

fn zrbvtcd_regulus_ref() -> String {
    format!(
        "refs/heads/{}",
        RBVCEX_REGULUS_BRANCH, //
    )
}

/// An empty narthex and this cut's own preview are fresh; a prior cut, a
/// second ref and another branch are not.
#[test]
fn rbvtcd_narthex_freshness_admits_empty_and_own_preview_only() {
    let own = zrbvtcd_regulus_ref();
    assert!(zrbvccd_narthex_fresh(&[], ZRBVTCD_TIP));
    assert!(zrbvccd_narthex_fresh(
        &[zrbvtcd_ref(ZRBVTCD_TIP, &own)],
        ZRBVTCD_TIP
    ));
    assert!(!zrbvccd_narthex_fresh(
        &[zrbvtcd_ref(ZRBVTCD_OTHER, &own)],
        ZRBVTCD_TIP
    ));
    assert!(!zrbvccd_narthex_fresh(
        &[zrbvtcd_ref(ZRBVTCD_TIP, "refs/heads/main")],
        ZRBVTCD_TIP
    ));
    assert!(!zrbvccd_narthex_fresh(
        &[
            zrbvtcd_ref(ZRBVTCD_TIP, &own),
            zrbvtcd_ref(ZRBVTCD_TIP, "refs/heads/main")
        ],
        ZRBVTCD_TIP
    ));
}

/// The practice refusal names the narthex, its console sentence and the empty
/// read beside the ref listing; the agora refusal carries none of the three.
#[test]
fn rbvtcd_practice_refusal_prints_the_narthex_steps_and_agora_does_not() {
    let refs = vec![zrbvtcd_ref(ZRBVTCD_OTHER, &zrbvtcd_regulus_ref())];
    let listing = format!(
        "  {} {}",
        ZRBVTCD_OTHER,
        zrbvtcd_regulus_ref(), //
    );

    let console = zrbvtcd_console();
    let practice = zrbvccd_narthex_refusal(&refs, rbvcew_World::Palaestra);
    for needle in [
        ZRBVTCD_NARTHEX_SLUG,
        console.as_str(),
        ZRBVTCD_EMPTY_READ,
        listing.as_str(),
    ] {
        assert!(
            practice.contains(needle),
            "the practice refusal lacks '{}':\n{}",
            needle,
            practice, //
        );
    }

    let agora = zrbvccd_narthex_refusal(&refs, rbvcew_World::Agora);
    assert!(
        agora.contains(&listing),
        "the real-world refusal lost its ref listing:\n{}",
        agora, //
    );
    for needle in [ZRBVTCD_NARTHEX_SLUG, console.as_str(), ZRBVTCD_EMPTY_READ] {
        assert!(
            !agora.contains(needle),
            "the real-world refusal carries '{}':\n{}",
            needle,
            agora, //
        );
    }
    assert!(
        practice.starts_with(&agora),
        "the practice refusal does not open on the real world's text", //
    );
}
